use crate::shared::{Entity, Id};

use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

pub type SourceId = Id<Source>;

#[derive(Debug, Clone, Serialize, FromRow, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct Source {
	#[builder(default)]
	pub id: SourceId,
	pub openalex_id: String,
	pub name: String,
	pub ty: String,
	pub issn: Option<String>,
}

impl Source {
	pub fn normalize_issn(issn: &str) -> Option<String> {
		let normalized = issn.replace("-", "").to_uppercase();

		if normalized.is_empty() {
			None
		} else {
			Some(normalized)
		}
	}
}

impl Entity for Source {
	fn key_name() -> &'static str {
		"source"
	}
}

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[sqlx(type_name = "journal_kind", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum JournalKind {
	Wos,
	Scopus,
}
