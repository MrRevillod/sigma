use crate::shared::{Entity, Id};

use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Type, Serialize, Deserialize, PartialEq, Eq)]
#[sqlx(type_name = "academic_planta", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AcademicPlanta {
	Adjunta,
	Permanente,
}

pub type AcademicCategoryId = Id<AcademicCategory>;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct AcademicCategory {
	#[builder(default)]
	pub id: AcademicCategoryId,
	pub name: String,
	pub planta: AcademicPlanta,
}

impl Entity for AcademicCategory {
	fn key_name() -> &'static str {
		"academic_category"
	}
}
