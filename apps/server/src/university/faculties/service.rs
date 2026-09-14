use crate::shared::AppResult;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct FacultiesService {
	faculties: Arc<FacultiesRepository>,
}

impl FacultiesService {
	pub async fn find(&self, query: GetFacultiesQuery) -> AppResult<Vec<Faculty>> {
		let filter = FacultyFilter { name: query.name };

		self.faculties.list(filter).await
	}

	pub async fn find_by_id(&self, id: &FacultyId) -> AppResult<Faculty> {
		let Some(faculty) = self.faculties.find_by_id(id).await? else {
			return Err(UniversityError::FacultyNotFound)?;
		};

		Ok(faculty)
	}

	pub async fn create(&self, input: CreateFacultyDto) -> AppResult<Faculty> {
		let faculty = Faculty::builder().name(input.name).build();

		self.faculties.save(&faculty).await?;

		Ok(faculty)
	}
}
