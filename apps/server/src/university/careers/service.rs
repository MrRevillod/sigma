use crate::shared::AppResult;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct CareersService {
	careers: Arc<CareersRepository>,
}

impl CareersService {
	pub async fn find(&self, query: GetCareersQuery) -> AppResult<Vec<Career>> {
		let filter = CareerFilter {
			name: query.name,
			department_id: query.department_id,
		};

		self.careers.list(filter).await
	}

	pub async fn find_by_id(&self, id: &CareerId) -> AppResult<Career> {
		let Some(career) = self.careers.find_by_id(id).await? else {
			return Err(UniversityError::CareerNotFound)?;
		};

		Ok(career)
	}

	pub async fn create(&self, input: CreateCareerDto) -> AppResult<Career> {
		let career = Career::builder()
			.name(input.name)
			.department_id(input.department_id)
			.build();

		self.careers.save(&career).await?;

		Ok(career)
	}
}
