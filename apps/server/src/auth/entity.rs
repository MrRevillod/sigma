use crate::{
	auth::UserId,
	shared::{Entity, Id},
};

use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub type SessionId = Id<Session>;

#[derive(Debug, Serialize, Deserialize, FromRow, Builder)]
pub struct Session {
	#[builder(default)]
	pub id: SessionId,
	pub user_id: UserId,
	pub refresh_token_hash: String,
	pub created_at: DateTime<Utc>,
	pub expires_at: DateTime<Utc>,
	pub refresh_expires_at: DateTime<Utc>,
	pub revoked_at: Option<DateTime<Utc>>,
}

impl Entity for Session {
	fn key_name() -> &'static str {
		"session"
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionClaims {
	pub session_id: SessionId,
	pub user_id: UserId,
	pub exp: i64,
	pub typ: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordResetClaims {
	pub user_id: UserId,
	pub pwd: String,
	pub exp: i64,
	pub typ: String,
}
