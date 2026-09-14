use crate::auth::{AuthConfig, SessionClaims, SessionRepository};
use crate::shared::JsonWebTokenService;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[derive(Interceptor)]
pub struct SessionCheck {
	config: AuthConfig,
	jwt_service: Arc<JsonWebTokenService>,
	sessions: Arc<SessionRepository>,
}

impl OnRequest for SessionCheck {
	async fn on_request(&self, mut req: Request) -> WebInterceptorResult {
		let method = req.method().to_string();
		let path = req.uri();

		let token = req
            .cookies()?
            .get("ACAD_MGR_ACCESS_TOKEN")
            .map(|c| c.value().to_string())
            .ok_or_else(|| {
	            tracing::warn!(method = %method, path = %path, "SessionCheck rejected: missing Access token cookie");
	            JsonResponse::Unauthorized()
            })?;

		let claims: SessionClaims = self
            .jwt_service
            .decode(&token, self.config.jwt_secret.as_ref())
            .inspect_err(|error| {
                tracing::warn!(method = %method, path = %path, error = %error, "SessionCheck rejected: token decode failed");
            })?;

		if claims.typ != "access" {
			tracing::warn!(
				method = %method,
				path = %path,
				session_id = %claims.session_id,
				user_id = %claims.user_id,
				token_type = %claims.typ,
				"SessionCheck rejected: token type is not access"
			);

			return Err(JsonResponse::Unauthorized());
		}

		if !self.sessions.is_active(&claims.session_id).await? {
			tracing::warn!(
				method = %method,
				path = %path,
				session_id = %claims.session_id,
				user_id = %claims.user_id,
				"SessionCheck rejected: session is not active"
			);

			return Err(JsonResponse::Unauthorized());
		}

		tracing::debug!(
			method = %method,
			path = %path,
			session_id = %claims.session_id,
			user_id = %claims.user_id,
			"SessionCheck accepted"
		);

		req.extensions.insert(claims);
		req.next().await
	}
}
