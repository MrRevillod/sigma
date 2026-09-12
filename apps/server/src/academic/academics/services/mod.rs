mod edition_codes;
mod imports;

pub use edition_codes::*;
pub use imports::*;
use serde_json::{Value, json};

use crate::{
	academic::*,
	auth::AuthConfig,
	config::ConfigService,
	research::{
		SyncResultView, WorkId, WorkOverridesInput, WorksError, WorksImportService, WorksService,
	},
	shared::{AppResult, JsonWebTokenService},
	university::*,
};
use std::sync::Arc;
use sword::{events::EventPublisher, prelude::*};

#[injectable]
pub struct AcademicsService {
	academics: Arc<AcademicsRepository>,
	departments: Arc<DepartmentsRepository>,
	careers: Arc<CareersRepository>,
	work_positions: Arc<AcademicWorkPositionsRepository>,
	countries: Arc<CountriesRepository>,
	category_options: Arc<AcademicCategoryOptionsRepository>,
	events: Arc<EventPublisher>,

	auth_config: AuthConfig,
	jwt_service: Arc<JsonWebTokenService>,

	config: Arc<ConfigService>,
	edit_codes: Arc<EditCodesService>,

	works_import: Arc<WorksImportService>,
	works: Arc<WorksService>,
}

impl AcademicsService {
	pub async fn find(&self, query: GetAcademicsQuery) -> AppResult<Vec<AcademicView>> {
		let filter = AcademicListFilter {
			search: query.search,
			career_id: query.career_id,
			category_id: query.category_id,
			department_id: query.department_id,
			option: query.option,
			planta: query.planta,
			include_unlinked: query.include_unlinked,
		};

		self.academics.list(filter).await
	}

	pub async fn find_active(&self, query: GetAcademicsQuery) -> AppResult<Vec<AcademicView>> {
		let filter = AcademicListFilter {
			search: query.search,
			career_id: query.career_id,
			category_id: query.category_id,
			department_id: query.department_id,
			option: query.option,
			planta: query.planta,
			include_unlinked: None,
		};

		self.academics.list(filter).await
	}

	pub async fn find_view_by_id(&self, id: &AcademicId) -> AppResult<AcademicView> {
		let Some(view) = self.academics.find_view_by_id(id).await? else {
			return Err(AcademicError::AcademicNotFound)?;
		};

		Ok(view)
	}

	pub async fn find_public_view_by_id(&self, id: &AcademicId) -> AppResult<AcademicView> {
		let view = self.find_view_by_id(id).await?;

		if view.left_at.is_some() {
			Err(AcademicError::AcademicNotFound)?;
		}

		Ok(view)
	}

	pub async fn unlink(&self, id: &AcademicId) -> AppResult<AcademicView> {
		let Some(academic) = self.academics.find_by_id(id).await? else {
			return Err(AcademicError::AcademicNotFound.into());
		};

		if academic.left_at.is_some() {
			Err(AcademicError::AcademicAlreadyUnlinked)?;
		}

		self.academics.mark_unlinked(id).await?;
		self.find_view_by_id(id).await
	}

	fn ensure_linked(academic: &Academic) -> AppResult<()> {
		if academic.left_at.is_some() {
			Err(AcademicError::AcademicUnlinked)?;
		}

		Ok(())
	}

	pub async fn create(&self, input: CreateAcademicDto) -> AppResult<AcademicView> {
		let academic = Academic::from(input.clone());

		if self.academics.find_by_rut(&academic.rut).await?.is_some() {
			Err(AcademicError::AcademicRutAlreadyExists)?;
		}

		if let Some(ref orcid) = input.orcid
			&& self.academics.find_by_orcid(orcid).await?.is_some()
		{
			Err(AcademicError::AcademicOrcidAlreadyExists)?;
		}

		let jce_max = self.config.jce_max().await?;
		if academic.jce > jce_max {
			Err(AcademicError::JceExceedsMax)?;
		}

		let Some(depto) = self.departments.find_by_id(&academic.department_id).await? else {
			return Err(UniversityError::DepartmentNotFound)?;
		};

		if let Some(career_id) = academic.career_id {
			let Some(career) = self.careers.find_by_id(&career_id).await? else {
				return Err(UniversityError::CareerNotFound)?;
			};

			if career.department_id != depto.id {
				Err(UniversityError::CareerDepartmentMismatch)?;
			}
		}

		self.academics.save(&academic).await?;
		self.find_view_by_id(&academic.id).await
	}

	pub async fn update(
		&self,
		id: &AcademicId,
		input: UpdateAcademicDto,
	) -> AppResult<AcademicView> {
		let Some(mut academic) = self.academics.find_by_id(id).await? else {
			return Err(AcademicError::AcademicNotFound)?;
		};

		if let Some(names) = &input.names {
			academic.names = names.clone();
		}

		if let Some(paternal_surname) = &input.paternal_surname {
			academic.paternal_surname = paternal_surname.clone();
		}

		if let Some(maternal_surname) = &input.maternal_surname {
			academic.maternal_surname = maternal_surname.clone();
		}

		if let Some(ref orcid) = input.orcid {
			if self.academics.find_by_orcid(orcid).await?.is_some()
				&& Some(orcid.as_str()) != academic.orcid.as_deref()
			{
				Err(AcademicError::AcademicOrcidAlreadyExists)?;
			}

			academic.orcid = Some(orcid.clone());
		}

		if let Some(sex) = input.sex {
			academic.sex = sex;
		}

		if let Some(birth_date) = input.birth_date {
			academic.birth_date = birth_date;
		}

		if let Some(joined_at) = input.joined_at {
			academic.joined_at = joined_at;
		}

		if let Some(wp_id) = input.work_position_id {
			if self.work_positions.find_by_id(&wp_id).await?.is_none() {
				Err(UniversityError::WorkPositionNotFound)?;
			}

			academic.work_position_id = wp_id;
		}

		if let Some(dept_id) = input.department_id {
			if self.departments.find_by_id(&dept_id).await?.is_none() {
				Err(UniversityError::DepartmentNotFound)?;
			}

			academic.department_id = dept_id;

			if let Some(career_id) = input.career_id {
				let Some(career) = self.careers.find_by_id(&career_id).await? else {
					return Err(UniversityError::CareerNotFound)?;
				};

				if career.department_id != dept_id {
					Err(UniversityError::CareerDepartmentMismatch)?;
				}

				academic.career_id = Some(career_id);
			} else {
				academic.career_id = None;
			}
		} else if let Some(career_id) = input.career_id {
			let Some(career) = self.careers.find_by_id(&career_id).await? else {
				return Err(UniversityError::CareerNotFound)?;
			};

			if career.department_id != academic.department_id {
				Err(UniversityError::CareerDepartmentMismatch)?;
			}

			academic.career_id = Some(career_id);
		}

		if let Some(cat_opt_id) = input.acad_category_options_id {
			if self
				.category_options
				.find_by_id(&cat_opt_id)
				.await?
				.is_none()
			{
				Err(AcademicError::CategoryOptionNotFound)?;
			}

			academic.acad_category_options_id = cat_opt_id;
		}

		if let Some(jce) = input.jce {
			academic.jce = jce;
		}

		if let Some(hours) = input.annual_discount_hours {
			academic.annual_discount_hours = hours;
		}

		if let Some(code) = &input.nationality_code {
			if self.countries.find_by_code(code).await?.is_none() {
				Err(UniversityError::CountryNotFound(code.clone()))?;
			}

			academic.nationality_code = code.clone();
		}

		if let Some(city) = input.city {
			academic.city = city;
		}

		let jce_max = self.config.jce_max().await?;
		if academic.jce > jce_max {
			Err(AcademicError::JceExceedsMax)?;
		}

		self.academics.save(&academic).await?;
		self.find_view_by_id(&academic.id).await
	}

	pub async fn update_profile_request(
		&self,
		id: &AcademicId,
		code: &str,
	) -> AppResult<AcademicView> {
		let Some(academic) = self.academics.find_by_id(id).await? else {
			return Err(AcademicError::AcademicNotFound)?;
		};

		Self::ensure_linked(&academic)?;

		let Some(code_academic_id) = self.edit_codes.consume(code).await? else {
			return Err(AcademicError::InvalidEditCode)?;
		};

		if &code_academic_id != id {
			Err(AcademicError::InvalidEditCode)?;
		}

		let mut academic = academic;
		academic.updated_at = chrono::Utc::now();
		self.academics.save(&academic).await?;
		let updated_at = academic.updated_at;

		let claims = json!({
			"academic_id": academic.id.to_string(),
			"updated_at": updated_at.timestamp(),
			"exp": (updated_at + chrono::Duration::days(7)).timestamp(),
		});

		let one_time_token = self
			.jwt_service
			.encode(&claims, self.auth_config.jwt_secret.as_bytes())?;

		let form_url = format!(
			"{}/public/academics/profile/update?token={}",
			self.auth_config.frontend_url, one_time_token
		);

		self.events
			.publish(UpdateAcademicFormEvent {
				academic_name: academic.full_name(),
				academic_email: academic.email.clone(),
				form_url,
			})
			.await;

		self.find_view_by_id(&academic.id).await
	}

	pub async fn send_edit_codes(&self, id: &AcademicId) -> AppResult<AcademicView> {
		let Some(academic) = self.academics.find_by_id(id).await? else {
			return Err(AcademicError::AcademicNotFound)?;
		};

		Self::ensure_linked(&academic)?;

		let codes = self.edit_codes.ensure_vigentes(id).await?;

		self.events
			.publish(SendEditCodesEvent {
				academic_name: academic.full_name(),
				academic_email: academic.email.clone(),
				codes: codes.into_iter().map(|c| c.code).collect(),
			})
			.await;

		self.find_view_by_id(id).await
	}

	pub async fn send_edit_codes_all(&self) -> AppResult<usize> {
		let academics = self.academics.list(AcademicListFilter::default()).await?;
		let mut sent = 0;

		for academic in academics {
			let codes = self.edit_codes.ensure_vigentes(&academic.id).await?;

			self.events
				.publish(SendEditCodesEvent {
					academic_name: format!(
						"{} {} {}",
						academic.names, academic.paternal_surname, academic.maternal_surname
					),
					academic_email: academic.email.clone(),
					codes: codes.into_iter().map(|c| c.code).collect(),
				})
				.await;

			sent += 1;
		}

		Ok(sent)
	}

	pub async fn validate_one_time_token(&self, token: &str) -> AppResult<AcademicId> {
		let claims = self
			.jwt_service
			.decode::<Value>(&token.to_string(), self.auth_config.jwt_secret.as_bytes())
			.map_err(|_| AcademicError::InvalidOneTimeToken)?;

		let academic_id_str = claims
			.get("academic_id")
			.and_then(|v| v.as_str())
			.ok_or(AcademicError::InvalidOneTimeToken)?;

		let token_updated_at = claims
			.get("updated_at")
			.and_then(|v| v.as_i64())
			.ok_or(AcademicError::InvalidOneTimeToken)?;

		let academic_id =
			AcademicId::parse(academic_id_str).map_err(|_| AcademicError::InvalidOneTimeToken)?;

		let Some(academic) = self.academics.find_by_id(&academic_id).await? else {
			return Err(AcademicError::AcademicNotFound)?;
		};

		Self::ensure_linked(&academic)?;

		if token_updated_at != academic.updated_at.timestamp() {
			Err(AcademicError::InvalidOneTimeToken)?;
		}

		Ok(academic_id)
	}

	fn apply_self_update(
		&self,
		academic: &mut Academic,
		input: &SelfUpdateAcademicDto,
	) -> AppResult<()> {
		if let Some(names) = &input.names {
			academic.names = names.clone();
		}

		if let Some(paternal_surname) = &input.paternal_surname {
			academic.paternal_surname = paternal_surname.clone();
		}

		if let Some(maternal_surname) = &input.maternal_surname {
			academic.maternal_surname = maternal_surname.clone();
		}

		if let Some(ref orcid) = input.orcid {
			academic.orcid = Some(orcid.clone());
		}

		if let Some(sex) = input.sex {
			academic.sex = sex;
		}

		if let Some(birth_date) = input.birth_date {
			academic.birth_date = birth_date;
		}

		if let Some(code) = &input.nationality_code {
			academic.nationality_code = code.clone();
		}

		if let Some(ref city) = input.city {
			academic.city = city.clone();
		}

		Ok(())
	}

	pub async fn update_from_one_time_link(
		&self,
		token: &str,
		input: SelfUpdateAcademicDto,
	) -> AppResult<AcademicView> {
		let academic_id = self.validate_one_time_token(token).await?;

		let Some(mut academic) = self.academics.find_by_id(&academic_id).await? else {
			return Err(AcademicError::AcademicNotFound)?;
		};

		if let Some(ref orcid) = input.orcid
			&& self.academics.find_by_orcid(orcid).await?.is_some()
			&& Some(orcid.as_str()) != academic.orcid.as_deref()
		{
			Err(AcademicError::AcademicOrcidAlreadyExists)?;
		}

		if let Some(code) = &input.nationality_code
			&& self.countries.find_by_code(code).await?.is_none()
		{
			Err(UniversityError::CountryNotFound(code.clone()))?;
		}

		self.apply_self_update(&mut academic, &input)?;

		self.academics.save(&academic).await?;
		self.find_view_by_id(&academic.id).await
	}

	pub async fn sync_works_by_token(&self, token: &str) -> AppResult<SyncResultView> {
		let academic_id = self.validate_one_time_token(token).await?;

		self.works_import.sync_works(academic_id).await
	}

	async fn ensure_can_edit_work(
		&self,
		work_id: &WorkId,
		academic_id: &AcademicId,
	) -> AppResult<()> {
		if !self.works.exists(work_id).await? {
			Err(WorksError::NotFound)?;
		}

		if !self.works.is_local_author(work_id, academic_id).await? {
			Err(AcademicError::NotLocalAuthor)?;
		}

		Ok(())
	}

	pub async fn update_work_overrides_by_token(
		&self,
		token: &str,
		work_id: WorkId,
		input: WorkOverridesInput,
	) -> AppResult<()> {
		let academic_id = self.validate_one_time_token(token).await?;
		self.ensure_can_edit_work(&work_id, &academic_id).await?;
		self.works.update_overrides(work_id, input).await
	}

	pub async fn clear_work_overrides_by_token(
		&self,
		token: &str,
		work_id: WorkId,
	) -> AppResult<()> {
		let academic_id = self.validate_one_time_token(token).await?;
		self.ensure_can_edit_work(&work_id, &academic_id).await?;
		self.works.clear_overrides(work_id).await
	}

	pub async fn update_authorship_affiliations_by_token(
		&self,
		token: &str,
		work_id: WorkId,
		orcid: String,
		affiliations: Vec<String>,
	) -> AppResult<()> {
		let academic_id = self.validate_one_time_token(token).await?;
		self.ensure_can_edit_work(&work_id, &academic_id).await?;
		self.works
			.update_authorship_affiliations(work_id, orcid, affiliations)
			.await
	}
}
