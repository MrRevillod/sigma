use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
#[http_error(code = 401, message = "Unauthorized")]
pub enum AuthError {
	#[error("The requested token was not found")]
	TokenNotFound,

	#[error("The provided token is invalid")]
	InvalidToken,

	#[http(code = 401, message = "Las credenciales de acceso son inválidas")]
	#[error("Invalid credentials provided")]
	InvalidCredentials,

	#[http(code = 404, message = "Usuario no encontrado")]
	#[error("User not found")]
	UserNotFound,

	#[http(code = 500, message = "Internal Server Error")]
	#[tracing(error)]
	#[error("Encryption error: {0}")]
	EncryptionError(#[from] BcryptError),

	#[http(code = 409, message = "El email ya está registrado")]
	#[error("Email already exists")]
	EmailAlreadyExists,

	#[http(
		code = 400,
		message = "El enlace de recuperación es inválido o ha expirado"
	)]
	#[error("Invalid password reset token")]
	InvalidResetToken,

	#[http(code = 403)]
	#[tracing(error)]
	#[error("JWT Error: {0}")]
	Jwt(#[from] JwtError),
}
