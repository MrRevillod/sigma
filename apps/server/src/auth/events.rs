use sword::events::*;

#[event(key = "auth.events.password-reset-req")]
pub struct PasswordResetReqEvent {
	pub user_name: String,
	pub user_email: String,
	pub reset_url: String,
	pub expires_minutes: i64,
}
