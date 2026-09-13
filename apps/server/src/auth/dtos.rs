use crate::auth::{UserView, validate_password};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
	#[validate(length(
		min = 1,
		max = 255,
		message = "El correo electrónico es obligatorio y debe tener entre 1 y 255 caracteres."
	))]
	pub email: String,

	#[validate(length(
		min = 1,
		max = 255,
		message = "La contraseña es obligatoria y debe tener entre 1 y 255 caracteres."
	))]
	pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordDto {
	#[validate(email(message = "El atributo 'email' debe ser un correo electrónico válido"))]
	pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordDto {
	#[validate(length(min = 1, message = "El token es obligatorio"))]
	pub token: String,

	#[validate(custom(function = "validate_password"))]
	pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
	pub user: UserView,
	pub access_token: String,
	pub access_token_exp: DateTime<Utc>,
	pub refresh_token: String,
	pub refresh_token_exp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
	pub access_token: String,
	pub access_token_exp: DateTime<Utc>,
}
