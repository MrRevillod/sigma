use crate::auth::PasswordResetReqEvent;
use crate::shared::*;

use std::collections::HashMap;
use std::sync::Arc;
use sword::events::*;
use sword::prelude::*;

#[controller(kind = Controller::EventHandler, source = EventSource::Memory)]
pub struct AuthEventsController {
	mailer: Arc<Mailer>,
}

impl AuthEventsController {
	#[handle("auth.events.password-reset-req")]
	async fn password_reset_req(&self, e: PasswordResetReqEvent) -> EventHandlerResult<()> {
		let template_variables = HashMap::from([
			("USER_NAME".to_string(), e.user_name),
			("RESET_URL".to_string(), e.reset_url),
			("EXPIRES_MINUTES".to_string(), e.expires_minutes.to_string()),
		]);

		let template = TemplateRenderer::render("password-reset", &template_variables);

		let mail = Mail::builder()
			.to(e.user_email)
			.subject("Recuperación de contraseña".into())
			.html(template)
			.build();

		self.mailer.send(mail).await.ok();

		Ok(())
	}
}
