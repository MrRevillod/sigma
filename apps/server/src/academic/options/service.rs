use crate::academic::*;
use crate::shared::AppResult;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoryOptionsService {
	options: Arc<AcademicCategoryOptionsRepository>,
}

impl AcademicCategoryOptionsService {
	pub async fn find(
		&self,
		query: GetCategoryOptionsQuery,
	) -> AppResult<Vec<AcademicCategoryOption>> {
		let filter = AcademicCategoryOptionFilter {
			category_id: query.category_id,
			..Default::default()
		};

		self.options.list(filter).await
	}

	pub async fn find_by_id(
		&self,
		id: &AcademicCategoryOptionId,
	) -> AppResult<AcademicCategoryOption> {
		let Some(option) = self.options.find_by_id(id).await? else {
			return Err(AcademicError::CategoryOptionNotFound)?;
		};

		Ok(option)
	}

	pub async fn create(
		&self,
		input: CreateCategoryOptionDto,
	) -> AppResult<AcademicCategoryOption> {
		let option = AcademicCategoryOption::builder()
			.category_id(input.category_id)
			.option(input.option)
			.maybe_hours(input.hours)
			.build();

		self.options.save(&option).await?;

		Ok(option)
	}
}
