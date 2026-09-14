use crate::shared::AppResult;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsService {
	positions: Arc<AcademicWorkPositionsRepository>,
}

impl AcademicWorkPositionsService {
	pub async fn find(&self, query: GetWorkPositionsQuery) -> AppResult<Vec<AcademicWorkPosition>> {
		let filter = WorkPositionFilter { name: query.name };

		self.positions.list(filter).await
	}

	pub async fn create(
		&self,
		input: CreateAcademicWorkPositionDto,
	) -> AppResult<AcademicWorkPosition> {
		let position = AcademicWorkPosition::new(input.name);

		self.positions.save(&position).await?;

		Ok(position)
	}
}
