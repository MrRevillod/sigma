use crate::academic::AcademicOption;
use crate::research::JournalKind;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorksStatsQuery {
	pub journal_kind: Option<JournalKind>,

	pub department_id: Option<Uuid>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,

	#[validate(range(min = 1, max = 100))]
	pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YearValue {
	pub year: i16,
	pub value: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesStat {
	pub id: Option<String>,
	pub key: String,
	pub values: Vec<YearValue>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeSeries {
	pub id: Option<String>,
	pub name: String,
	pub total: i64,
	pub wos: Vec<YearValue>,
	pub scopus: Vec<YearValue>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeTotal {
	pub id: Option<String>,
	pub name: String,
	pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FacultySummary {
	pub total_works: i64,
	pub wos_count: i64,
	pub scopus_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorksStatsResponse {
	pub faculty_summary: FacultySummary,
	pub by_journal_kind: Vec<TimeSeriesStat>,
	pub by_department: Vec<ScopeSeries>,
	pub by_research_line: Vec<ScopeSeries>,
	pub top_publishers: Vec<TopPublisher>,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentDetailQuery {
	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,

	pub option: Option<AcademicOption>,

	pub journal_kind: Option<JournalKind>,

	#[validate(range(min = 1, max = 100))]
	pub limit: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct AcademicStatsQuery {
	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLineStat {
	pub research_line_id: String,
	pub name: String,
	pub count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicContribution {
	pub academic_works: i64,
	pub faculty_works: i64,
	pub department_works: i64,
	pub department_name: Option<String>,
	pub dominant_line_works: i64,
	pub line_total_works: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicStatsResponse {
	pub by_research_line: Vec<ResearchLineStat>,
	pub dominant_research_line_id: Option<String>,
	pub by_journal_kind: Vec<TimeSeriesStat>,
	pub contribution: AcademicContribution,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopPublisher {
	pub academic_id: String,
	pub name: String,
	pub total: i64,
	pub scopus: i64,
	pub wos: i64,
	pub unindexed: i64,
	pub option: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentDetailResponse {
	pub department: String,
	pub total_works: i64,
	pub scopus_count: i64,
	pub wos_count: i64,
	pub teaching_count: i64,
	pub research_count: i64,
	pub by_journal_kind: Vec<TimeSeriesStat>,
	pub top_publishers: Vec<TopPublisher>,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLineStatsQuery {
	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,

	#[validate(range(min = 1, max = 100))]
	pub limit: Option<i32>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProductivityDegree {
	All,
	Magister,
	Doctor,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProductivityScope {
	Faculty,
	Department,
	ResearchLine,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ProductivityJceScope {
	#[default]
	Doctor,
	All,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductivityQuery {
	pub degree: Option<ProductivityDegree>,
	pub scope: Option<ProductivityScope>,
	pub jce_scope: Option<ProductivityJceScope>,
	pub department_id: Option<Uuid>,
	pub research_line_id: Option<Uuid>,

	#[validate(range(min = 1, max = 12))]
	pub month: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductivityYearValue {
	pub year: i16,
	pub value: f64,
	pub pubs: i64,
	pub jce: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductivitySeries {
	pub key: String,
	pub values: Vec<ProductivityYearValue>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductivityResponse {
	pub trend: Vec<ProductivitySeries>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLineStatsResponse {
	pub name: String,
	pub total_works: i64,
	pub wos_count: i64,
	pub scopus_count: i64,
	pub by_journal_kind: Vec<TimeSeriesStat>,
	pub by_department: Vec<ScopeTotal>,
	pub top_publishers: Vec<TopPublisher>,
}
