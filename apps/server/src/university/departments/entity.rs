use crate::shared::{Entity, Id};
use crate::university::FacultyId;

use bon::Builder;
use serde::Serialize;
use sqlx::FromRow;

pub type DepartmentId = Id<Department>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Department {
	#[builder(default)]
	pub id: DepartmentId,
	pub name: String,
	pub faculty_id: FacultyId,
}

#[derive(Debug)]
pub struct DepartmentFilter {
	pub name: Option<String>,
	pub faculty_id: Option<FacultyId>,
}

impl Entity for Department {
	fn key_name() -> &'static str {
		"department"
	}
}
