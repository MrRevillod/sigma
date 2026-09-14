use crate::academic::*;
use crate::shared::AppResult;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DegreesService {
	degrees: Arc<DegreesRepository>,
}

impl DegreesService {
	pub async fn find(&self, academic_id: &AcademicId) -> AppResult<Vec<Degree>> {
		self.degrees.list(academic_id).await
	}

	pub async fn create(&self, input: CreateDegreeDto) -> AppResult<Degree> {
		if self.is_superior_degree(input.kind)
			&& self.degrees.count_superior(&input.academic_id).await? > 0
		{
			Err(AcademicError::DegreeSuperiorAlreadyExists)?;
		}

		let degree = Degree::builder()
			.academic_id(input.academic_id)
			.name(input.name)
			.university(input.university)
			.obtained_at(input.obtained_at)
			.kind(input.kind)
			.country_code(input.country_code)
			.build();

		self.degrees.save(&degree).await?;

		Ok(degree)
	}

	pub async fn update(&self, degree_id: &DegreeId, input: UpdateDegreeDto) -> AppResult<Degree> {
		let Some(mut degree) = self.degrees.find_by_id(degree_id).await? else {
			return Err(AcademicError::DegreeNotFound)?;
		};

		let new_kind = input.kind.unwrap_or(degree.kind);

		if self.is_superior_degree(new_kind)
			&& self
				.degrees
				.count_superior_excluding(&degree.academic_id, degree_id)
				.await? > 0
		{
			Err(AcademicError::DegreeSuperiorAlreadyExists)?;
		}

		if let Some(name) = input.name {
			degree.name = name;
		}

		if let Some(university) = input.university {
			degree.university = university;
		}

		if let Some(obtained_at) = input.obtained_at {
			degree.obtained_at = obtained_at;
		}

		if let Some(country_code) = input.country_code {
			degree.country_code = country_code;
		}

		if let Some(kind) = input.kind {
			degree.kind = kind;
		}

		self.degrees.save(&degree).await?;

		Ok(degree)
	}

	fn is_superior_degree(&self, kind: DegreeKind) -> bool {
		matches!(kind, DegreeKind::Magister | DegreeKind::Doctor)
	}
}
