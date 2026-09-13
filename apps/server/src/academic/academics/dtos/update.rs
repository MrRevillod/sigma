use super::ORCID_ID_REGEX;
use crate::academic::{AcademicCategoryOptionId, Sex};
use crate::university::{AcademicWorkPositionId, CareerId, DepartmentId};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAcademicDto {
	#[validate(length(
		min = 1,
		max = 255,
		message = "Los nombres deben tener entre 1 y 255 caracteres"
	))]
	pub names: Option<String>,

	#[validate(length(
		min = 1,
		max = 255,
		message = "El apellido paterno debe tener entre 1 y 255 caracteres"
	))]
	pub paternal_surname: Option<String>,

	#[validate(length(
		min = 1,
		max = 255,
		message = "El apellido materno debe tener entre 1 y 255 caracteres"
	))]
	pub maternal_surname: Option<String>,

	#[validate(email(message = "El email debe ser válido"))]
	pub email: Option<String>,

	#[validate(regex(
		path = *ORCID_ID_REGEX,
		message = "El ORCID debe ser una URL válida (https://orcid.org/XXXX-XXXX-XXXX-XXXX)"
	))]
	pub orcid: Option<String>,
	pub sex: Option<Sex>,

	#[validate(custom(function = "super::validate_birth_date"))]
	pub birth_date: Option<NaiveDate>,

	#[validate(custom(function = "super::validate_joined_at"))]
	pub joined_at: Option<NaiveDate>,
	pub work_position_id: Option<AcademicWorkPositionId>,
	pub department_id: Option<DepartmentId>,
	pub career_id: Option<CareerId>,

	pub acad_category_options_id: Option<AcademicCategoryOptionId>,

	#[validate(range(min = 0.0, message = "La JCE no puede ser negativa"))]
	pub jce: Option<f64>,

	#[validate(range(
		min = 0.0,
		message = "Las horas de descuento anual no pueden ser negativas"
	))]
	pub annual_discount_hours: Option<f64>,

	#[validate(length(
		min = 2,
		max = 2,
		message = "El código de país debe tener 2 caracteres"
	))]
	pub nationality_code: Option<String>,

	#[validate(length(
		min = 1,
		max = 255,
		message = "La ciudad debe tener entre 1 y 255 caracteres"
	))]
	pub city: Option<String>,
}
