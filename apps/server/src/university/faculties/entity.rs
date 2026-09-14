use crate::shared::{Entity, Id};

use bon::Builder;
use serde::Serialize;
use sqlx::FromRow;

pub type FacultyId = Id<Faculty>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct Faculty {
	#[builder(default)]
	pub id: FacultyId,
	pub name: String,
}

pub struct FacultyFilter {
	pub name: Option<String>,
}

impl Entity for Faculty {
	fn key_name() -> &'static str {
		"faculty"
	}
}
