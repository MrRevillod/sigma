use crate::shared::{Entity, Id};

use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize, Type, Copy)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
	Admin,
}

pub type UserId = Id<User>;

#[derive(Debug, Clone, FromRow, Serialize, Builder)]
pub struct User {
	#[builder(default = UserId::new())]
	pub id: UserId,
	pub name: String,
	pub email: String,
	pub role: UserRole,
	pub password_hash: String,
}

#[derive(Debug)]
pub struct UserFilter {
	pub search: Option<String>,
	pub role: Option<UserRole>,
}

impl Entity for User {
	fn key_name() -> &'static str {
		"user"
	}
}
