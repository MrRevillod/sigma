use crate::academic::{AcademicCategoryOptionId, AcademicId, AcademicOption, AcademicPlanta, Sex};
use crate::university::{AcademicWorkPositionId, CareerId, DepartmentId};

use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AcademicView {
	pub id: AcademicId,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub email: String,
	pub orcid: Option<String>,
	pub sex: Sex,
	pub birth_date: NaiveDate,
	pub joined_at: NaiveDate,
	pub left_at: Option<NaiveDate>,
	pub work_position_id: AcademicWorkPositionId,
	pub work_position: Option<String>,
	pub department_id: DepartmentId,
	pub department: String,
	pub career_id: Option<CareerId>,
	pub career: Option<String>,
	pub jce: f64,
	pub acad_category_options_id: AcademicCategoryOptionId,
	pub category: String,
	pub planta: AcademicPlanta,
	pub option: AcademicOption,
	pub acad_category_hours: Option<f64>,
	pub annual_discount_hours: f64,
	pub nationality: String,
	pub city: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicPublicView {
	pub id: AcademicId,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub email: String,
	pub orcid: Option<String>,
	pub sex: Sex,
	pub birth_date: NaiveDate,
	pub joined_at: NaiveDate,
	pub department: String,
	pub career: Option<String>,
	pub nationality: String,
	pub city: String,
}

impl From<AcademicView> for AcademicPublicView {
	fn from(view: AcademicView) -> Self {
		Self {
			id: view.id,
			names: view.names,
			paternal_surname: view.paternal_surname,
			maternal_surname: view.maternal_surname,
			email: view.email,
			orcid: view.orcid,
			sex: view.sex,
			birth_date: view.birth_date,
			joined_at: view.joined_at,
			department: view.department,
			career: view.career,
			nationality: view.nationality,
			city: view.city,
		}
	}
}
