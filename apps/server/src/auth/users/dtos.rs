use crate::auth::{User, UserId, UserRole};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationError};

#[derive(Debug, Default, Validate, Deserialize)]
pub struct GetUsersQuery {
	#[validate(length(
		max = 255,
		message = "El atributo 'search' no puede tener más de 255 caracteres"
	))]
	pub search: Option<String>,
	pub role: Option<UserRole>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserDto {
	#[validate(length(
		min = 1,
		max = 255,
		message = "El atributo 'name' no puede tener más de 255 caracteres"
	))]
	pub name: String,

	#[validate(email(message = "El atributo 'email' debe ser un correo electrónico válido"))]
	pub email: String,

	#[validate(custom(function = "validate_password"))]
	pub password: String,
	pub role: UserRole,
}

pub(crate) fn validate_password(password: &str) -> Result<(), ValidationError> {
	let mut missing = Vec::new();

	if password.len() < 8 {
		missing.push("al menos 8 caracteres");
	}
	if !password.chars().any(|c| c.is_ascii_lowercase()) {
		missing.push("una letra minúscula");
	}
	if !password.chars().any(|c| c.is_ascii_uppercase()) {
		missing.push("una letra mayúscula");
	}
	if !password.chars().any(|c| c.is_ascii_digit()) {
		missing.push("un número");
	}
	if !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
		missing.push("un carácter especial");
	}

	if password.len() > 255 {
		missing.push("no más de 255 caracteres");
	}

	if missing.is_empty() {
		Ok(())
	} else {
		let mut err = ValidationError::new("password");
		err.message = Some(format!("La contraseña debe tener {}", missing.join(", ")).into());
		Err(err)
	}
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserView {
	pub id: UserId,
	pub name: String,
	pub email: String,
	pub role: UserRole,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserDto {
	#[validate(length(
		min = 1,
		max = 255,
		message = "El atributo 'name' no puede tener más de 255 caracteres"
	))]
	pub name: Option<String>,

	#[validate(email(message = "El atributo 'email' debe ser un correo electrónico válido"))]
	pub email: Option<String>,

	pub role: Option<UserRole>,

	#[validate(custom(function = "validate_password"))]
	pub password: Option<String>,
}

impl From<User> for UserView {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			name: user.name,
			email: user.email,
			role: user.role,
		}
	}
}
