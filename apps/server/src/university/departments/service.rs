use crate::shared::AppResult;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DepartmentsService {
	departments: Arc<DepartmentsRepository>,
}

impl DepartmentsService {
	pub async fn find(&self, query: GetDepartmentsQuery) -> AppResult<Vec<Department>> {
		let filter = DepartmentFilter {
			name: query.name,
			faculty_id: query.faculty_id,
		};

		self.departments.list(filter).await
	}

	pub async fn find_by_id(&self, id: &DepartmentId) -> AppResult<Department> {
		let Some(department) = self.departments.find_by_id(id).await? else {
			return Err(UniversityError::DepartmentNotFound)?;
		};

		Ok(department)
	}

	pub async fn create(&self, input: CreateDepartmentDto) -> AppResult<Department> {
		let department = Department::builder()
			.name(input.name)
			.faculty_id(input.faculty_id)
			.build();

		self.departments.save(&department).await?;

		Ok(department)
	}
}
