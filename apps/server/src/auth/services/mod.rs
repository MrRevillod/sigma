mod cookies;
mod hasher;

pub use cookies::*;
pub use hasher::*;

use crate::{auth::*, shared::*};

use chrono::{DateTime, Duration, Utc};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use sword::events::EventPublisher;
use sword::prelude::*;

#[injectable]
pub struct AuthService {
	config: AuthConfig,
	users: Arc<UsersRepository>,
	jwt_service: Arc<JsonWebTokenService>,
	sessions: Arc<SessionRepository>,
	hasher: Arc<Hasher>,
	events: Arc<EventPublisher>,
}

impl AuthService {
	pub async fn login(&self, input: &LoginDto) -> AppResult<LoginResponse> {
		let LoginDto { email, password } = input;

		let Some(user) = self.users.find_by_email(email).await? else {
			return Err(AuthError::InvalidCredentials)?;
		};

		if !self.hasher.verify(password, &user.password_hash)? {
			Err(AuthError::InvalidCredentials)?;
		}

		let session_id = SessionId::new();

		let (access_token, access_token_exp) = self.generate_access_token(&session_id, &user.id)?;
		let (refresh_token, refresh_token_exp) =
			self.generate_refresh_token(&session_id, &user.id)?;

		let session = Session::builder()
			.id(session_id)
			.user_id(user.id)
			.refresh_token_hash(Self::hash_token(&refresh_token))
			.created_at(Utc::now())
			.expires_at(access_token_exp)
			.refresh_expires_at(refresh_token_exp)
			.build();

		self.sessions.save(&session).await?;

		Ok(LoginResponse {
			user: user.into(),
			access_token,
			access_token_exp,
			refresh_token,
			refresh_token_exp,
		})
	}

	pub async fn refresh(&self, token: &String) -> AppResult<RefreshResponse> {
		let claims = self
			.jwt_service
			.decode::<SessionClaims>(token, self.config.jwt_secret.as_ref())?;

		if claims.typ != "refresh" {
			Err(AuthError::InvalidToken)?;
		}

		let session = self
			.sessions
			.find_active_by_refresh_id(&claims.session_id)
			.await?
			.ok_or(AuthError::TokenNotFound)?;

		let (access_token, access_token_exp) =
			self.generate_access_token(&session.id, &session.user_id)?;

		let mut session = session;
		session.expires_at = access_token_exp;
		self.sessions.save(&session).await?;

		Ok(RefreshResponse {
			access_token,
			access_token_exp,
		})
	}

	pub async fn logout(&self, session_id: &SessionId) -> AppResult<()> {
		if let Some(mut session) = self.sessions.find_active_by_id(session_id).await? {
			session.revoked_at = Some(Utc::now());
			self.sessions.save(&session).await?;
		}

		Ok(())
	}

	pub async fn forgot_password(&self, input: &ForgotPasswordDto) -> AppResult<()> {
		let Some(user) = self.users.find_by_email(&input.email).await? else {
			return Ok(());
		};

		let expiration = Utc::now() + Duration::minutes(self.config.reset_exp_minutes);

		let claims = PasswordResetClaims {
			user_id: user.id,
			pwd: Self::hash_token(&user.password_hash),
			exp: expiration.timestamp(),
			typ: "password_reset".to_string(),
		};

		let token = self
			.jwt_service
			.encode(&claims, self.config.jwt_secret.as_ref())?;

		let reset_url = format!(
			"{}/reset-password?token={}",
			self.config.frontend_url, token
		);

		self.events
			.publish(PasswordResetReqEvent {
				user_name: user.name,
				user_email: user.email,
				reset_url,
				expires_minutes: self.config.reset_exp_minutes,
			})
			.await;

		Ok(())
	}

	pub async fn reset_password(&self, input: &ResetPasswordDto) -> AppResult<()> {
		let claims = self
			.jwt_service
			.decode::<PasswordResetClaims>(&input.token, self.config.jwt_secret.as_ref())
			.map_err(|_| AuthError::InvalidResetToken)?;

		if claims.typ != "password_reset" {
			Err(AuthError::InvalidResetToken)?;
		}

		let Some(mut user) = self.users.find_by_id(&claims.user_id).await? else {
			return Err(AuthError::InvalidResetToken)?;
		};

		if claims.pwd != Self::hash_token(&user.password_hash) {
			Err(AuthError::InvalidResetToken)?;
		}

		user.password_hash = self.hasher.hash(&input.password)?;
		self.users.save(&user).await?;

		self.sessions.revoke_all_for_user(&user.id).await?;

		Ok(())
	}

	fn generate_access_token(
		&self,
		session_id: &SessionId,
		user_id: &UserId,
	) -> AppResult<(String, DateTime<Utc>)> {
		let expiration = Utc::now() + Duration::minutes(self.config.access_exp_minutes);

		let claims = SessionClaims {
			session_id: *session_id,
			user_id: *user_id,
			exp: expiration.timestamp(),
			typ: "access".to_string(),
		};

		let token = self
			.jwt_service
			.encode(&claims, self.config.jwt_secret.as_ref())?;

		Ok((token, expiration))
	}

	fn generate_refresh_token(
		&self,
		session_id: &SessionId,
		user_id: &UserId,
	) -> AppResult<(String, DateTime<Utc>)> {
		let expiration = Utc::now() + Duration::days(self.config.refresh_exp_days);

		let claims = SessionClaims {
			session_id: *session_id,
			user_id: *user_id,
			exp: expiration.timestamp(),
			typ: "refresh".to_string(),
		};

		let token = self
			.jwt_service
			.encode(&claims, self.config.jwt_secret.as_ref())?;

		Ok((token, expiration))
	}

	fn hash_token(token: &str) -> String {
		let result = Sha256::digest(token.as_bytes());
		result.iter().map(|b| format!("{b:02x}")).collect()
	}
}
