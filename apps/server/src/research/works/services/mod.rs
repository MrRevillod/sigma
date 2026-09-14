use crate::academic::AcademicId;
use crate::research::*;
use crate::shared::AppResult;

mod import;
mod openalex;
mod orcid;

pub use import::WorksImportService;
pub use openalex::*;
pub use orcid::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorksService {
	works: Arc<WorksRepository>,
	sources: Arc<SourcesRepository>,
	authorships: Arc<AuthorshipsRepository>,
}

impl WorksService {
	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<WorkView>> {
		let mut views = self.works.list(query).await?;

		for view in &mut views {
			view.work = view.work.resolve();
		}

		Ok(views)
	}

	pub async fn exists(&self, work_id: &WorkId) -> AppResult<bool> {
		Ok(self.works.find_work(work_id).await?.is_some())
	}

	pub async fn is_local_author(
		&self,
		work_id: &WorkId,
		academic_id: &AcademicId,
	) -> AppResult<bool> {
		self.authorships
			.exists_local_author(work_id, academic_id)
			.await
	}

	pub async fn find_by_id(&self, id: WorkId) -> AppResult<WorkView> {
		let Some(mut view) = self.works.find_by_id(&id).await? else {
			return Err(WorksError::NotFound)?;
		};

		view.work = view.work.resolve();

		let work_id = view.work.id;

		view.source = match view.work.source_id {
			Some(source_id) => self.sources.find_source_view_by_id(&source_id).await?,
			None => None,
		};

		let mut authorships = self.authorships.list(&work_id).await?;

		if let Some(orcid) = view.work.overrides.corresponding_orcid.as_ref() {
			for authorship in &mut authorships {
				authorship.is_corresponding = &authorship.orcid == orcid;
			}
		} else {
			let mut found = false;
			for authorship in &mut authorships {
				if authorship.is_corresponding {
					if found {
						authorship.is_corresponding = false;
					} else {
						found = true;
					}
				}
			}
		}

		authorships.sort_by_key(|a| {
			let position_rank = match a.position {
				AuthorshipPosition::First => 0,
				AuthorshipPosition::Middle => 1,
				AuthorshipPosition::Last => 2,
			};
			(std::cmp::Reverse(a.is_corresponding), position_rank)
		});

		view.authorships = Some(authorships);
		view.topics = Some(self.works.list_topics_by_work(&work_id).await?);
		view.keywords = Some(self.works.list_keywords_by_work(&work_id).await?);

		Ok(view)
	}

	pub async fn update_overrides(
		&self,
		work_id: WorkId,
		input: WorkOverridesInput,
	) -> AppResult<()> {
		let Some(mut work) = self.works.find_work(&work_id).await? else {
			return Err(WorksError::NotFound)?;
		};

		if let Some(v) = input.title {
			work.overrides.title = v;
		}
		if let Some(v) = input.abstract_text {
			work.overrides.abstract_text = v;
		}
		if let Some(v) = input.doi {
			work.overrides.doi = v;
		}
		if let Some(v) = input.publication_year {
			work.overrides.publication_year = v;
		}
		if let Some(v) = input.is_accepted {
			work.overrides.is_accepted = v;
		}
		if let Some(v) = input.is_published {
			work.overrides.is_published = v;
		}
		if let Some(v) = input.research_line_id {
			work.overrides.research_line_id = v;
		}
		if let Some(v) = input.corresponding_orcid {
			if let Some(orcid) = v.as_ref() {
				let authorships = self.authorships.list(&work_id).await?;
				if !authorships.iter().any(|a| &a.orcid == orcid) {
					Err(WorksError::AuthorshipNotInWork)?;
				}
			}
			work.overrides.corresponding_orcid = v;
		}

		if let Some(v) = input.journal_kind {
			work.overrides.journal_kind = v;
		}

		work.updated_at = chrono::Utc::now();
		self.works.save(&work).await
	}

	pub async fn update_authorship_affiliations(
		&self,
		work_id: WorkId,
		orcid: String,
		affiliations: Vec<String>,
	) -> AppResult<()> {
		let Some(_work) = self.works.find_work(&work_id).await? else {
			return Err(WorksError::NotFound)?;
		};

		let updated = self
			.authorships
			.update_affiliations(&work_id, &orcid, &affiliations)
			.await?;

		if !updated {
			Err(WorksError::AuthorshipNotInWork)?;
		}

		Ok(())
	}

	pub async fn clear_overrides(&self, work_id: WorkId) -> AppResult<()> {
		let Some(mut work) = self.works.find_work(&work_id).await? else {
			return Err(WorksError::NotFound)?;
		};

		work.overrides = WorkOverrides::default();
		work.updated_at = chrono::Utc::now();
		self.works.save(&work).await
	}
}
