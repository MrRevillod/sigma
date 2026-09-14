use crate::academic::AcademicId;
use crate::research::*;

use serde::Serialize;
use sqlx::{FromRow, Row, postgres::PgRow};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkView {
	#[serde(flatten)]
	pub work: Work,

	pub journal_kind: Option<JournalKind>,
	pub research_line_id: Option<Uuid>,
	pub research_line_name: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub source: Option<SourceView>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub authorships: Option<Vec<Authorship>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub topics: Option<Vec<TopicView>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub keywords: Option<Vec<KeywordView>>,
}

impl<'r> FromRow<'r, PgRow> for WorkView {
	fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
		Ok(Self {
			work: Work::from_row(row)?,
			journal_kind: row.try_get("journal_kind")?,
			research_line_id: row.try_get("research_line_id")?,
			research_line_name: row.try_get("research_line_name")?,
			source: None,
			authorships: None,
			topics: None,
			keywords: None,
		})
	}
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TopicView {
	pub topic_id: TopicId,
	pub name: String,
	pub score: f64,
	pub subfield_id: SubfieldId,
	pub subfield_name: String,
	pub field_id: FieldId,
	pub field_name: String,
	pub domain_id: DomainId,
	pub domain_name: String,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct KeywordView {
	pub keyword_id: KeywordId,
	pub name: String,
	pub score: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResultView {
	pub academic_id: AcademicId,
	pub orcid_works: usize,
	pub works_without_doi: usize,
	pub not_found_in_openalex: usize,
	pub works_created: usize,
	pub authorships_inserted: usize,
	pub authorships_unlinked: usize,
	pub topics_linked: usize,
	pub keywords_linked: usize,
	pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct SyncSummary {
	pub results: Vec<SyncResultView>,
	pub skipped_without_orcid: usize,
}
