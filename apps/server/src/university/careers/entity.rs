use crate::shared::{Entity, Id};
use crate::university::DepartmentId;

use bon::Builder;
use serde::Serialize;
use sqlx::FromRow;

pub type CareerId = Id<Career>;

#[derive(Debug, Clone, FromRow, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Career {
	#[builder(default)]
	pub id: CareerId,
	pub name: String,
	pub department_id: DepartmentId,
}

#[derive(Debug)]
pub struct CareerFilter {
	pub name: Option<String>,
	pub department_id: Option<DepartmentId>,
}

impl Entity for Career {
	fn key_name() -> &'static str {
		"career"
	}
}
