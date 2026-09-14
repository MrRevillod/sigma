use crate::research::{SyncSummary, SyncWorksRequest, WorksImportService};
use crate::shared::{Mail, Mailer, TemplateRenderer};

use std::collections::HashMap;
use std::sync::Arc;

use sword::events::*;
use sword::prelude::*;

#[controller(kind = Controller::EventHandler, source = EventSource::Memory)]
pub struct WorksEventsController {
	mailer: Arc<Mailer>,
	works_import: Arc<WorksImportService>,
}

impl WorksEventsController {
	#[handle("works.sync-requested")]
	async fn handle_sync_requested(&self, event: SyncWorksRequest) -> EventHandlerResult<()> {
		tracing::info!("processing sync-requested event for {}", event.user_email);
		let result = self.works_import.sync_all_academics().await;

		match result {
			Ok(summary) => {
				let SyncSummary {
					results,
					skipped_without_orcid,
				} = summary;

				let total = results.len();

				let mut errors_by_message: HashMap<&String, Vec<String>> = HashMap::new();
				for r in &results {
					if let Some(err) = r.errors.first() {
						errors_by_message
							.entry(err)
							.or_default()
							.push(r.academic_id.to_string());
					}
				}
				let error_count: usize = errors_by_message.values().map(Vec::len).sum();

				tracing::info!(
					"sync completed: {} synced, {} skipped (no ORCID), {} errors",
					total,
					skipped_without_orcid,
					error_count
				);

				let (status_suffix, error_details, subject) = if errors_by_message.is_empty() {
					(
						format!(
							" exitosamente ({total} académicos sincronizados, {skipped_without_orcid} omitidos por no tener ORCID)"
						),
						String::new(),
						"Sincronización completada exitosamente",
					)
				} else {
					let items: String = errors_by_message
						.iter()
						.map(|(msg, ids)| {
							let id_list: String =
								ids.iter().map(|o| format!("<li>{o}</li>")).collect();
							format!(
								"<li><b>{msg}</b> — {} académico(s)<ul>{id_list}</ul></li>",
								ids.len()
							)
						})
						.collect();
					(
						format!(
							" con {error_count} académico(s) con errores ({total} sincronizados, {skipped_without_orcid} omitidos por no tener ORCID)"
						),
						format!(
							"<p style=\"margin:0;padding:0;font-size:1em;padding-top:0.5em;padding-bottom:0.5em;color:#b91c1c\">Académicos con errores ({error_count}):</p><ul style=\"font-size:0.875em;color:#555\">{items}</ul>"
						),
						"Sincronización completada con errores",
					)
				};

				let context = HashMap::from([
					("STATUS_SUFFIX".into(), status_suffix),
					("ERROR_DETAILS".into(), error_details),
				]);
				let html = TemplateRenderer::render("sync-results", &context);

				let mail = Mail::builder()
					.to(event.user_email)
					.subject(subject.into())
					.html(html)
					.build();

				self.mailer.send(mail).await.ok();
			}
			Err(e) => {
				tracing::error!("sync failed: {e}");
				let html = format!("<p>La sincronización falló: {e}</p>");
				let mail = Mail::builder()
					.to(event.user_email)
					.subject("Error en sincronización de publicaciones".into())
					.html(html)
					.build();

				self.mailer.send(mail).await.ok();
			}
		}

		Ok(())
	}
}
