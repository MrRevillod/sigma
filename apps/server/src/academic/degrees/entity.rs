use crate::academic::academics::AcademicId;
use crate::shared::{Entity, Id};

use bon::Builder;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize)]
#[sqlx(type_name = "degree_kind", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DegreeKind {
	Professional,
	Magister,
	Doctor,
}

pub type DegreeId = Id<Degree>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Degree {
	#[builder(default)]
	pub id: DegreeId,
	pub academic_id: AcademicId,
	pub name: String,
	pub university: String,
	pub obtained_at: NaiveDate,
	pub kind: DegreeKind,
	pub country_code: String,
}

impl Entity for Degree {
	fn key_name() -> &'static str {
		"degree"
	}
}
