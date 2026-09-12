use crate::academic::*;
use crate::shared::{Entity, Id};
use crate::university::{AcademicWorkPositionId, CareerId, DepartmentId};

use bon::Builder;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

pub type AcademicId = Id<Academic>;

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize)]
#[sqlx(type_name = "sex", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum Sex {
	H,
	M,
	O,
}

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct Academic {
	#[builder(default)]
	pub id: AcademicId,
	pub rut: String,
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
	pub department_id: DepartmentId,
	pub career_id: Option<CareerId>,
	pub jce: f64,
	pub acad_category_options_id: AcademicCategoryOptionId,
	pub annual_discount_hours: f64,
	pub nationality_code: String,
	pub city: String,

	#[builder(default = Utc::now())]
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct AcademicListFilter {
	pub search: Option<String>,
	pub career_id: Option<CareerId>,
	pub department_id: Option<DepartmentId>,
	pub category_id: Option<AcademicCategoryId>,
	pub planta: Option<AcademicPlanta>,
	pub option: Option<AcademicOption>,
	pub include_unlinked: Option<bool>,
}

impl Academic {
	pub fn full_name(&self) -> String {
		format!(
			"{} {} {}",
			self.names, self.paternal_surname, self.maternal_surname
		)
	}
}

impl Entity for Academic {
	fn key_name() -> &'static str {
		"academic"
	}
}

pub type EditCodeId = Id<EditCode>;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct EditCode {
	pub id: EditCodeId,
	pub academic_id: AcademicId,
	pub code: String,
	pub used_at: Option<DateTime<Utc>>,
	pub created_at: DateTime<Utc>,
}

impl Entity for EditCode {
	fn key_name() -> &'static str {
		"edit_code"
	}
}
