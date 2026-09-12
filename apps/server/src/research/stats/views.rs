use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(Debug, FromRow)]
pub struct JournalKindRow {
	pub year: i16,
	pub wos: Option<i64>,
	pub scopus: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct FacultySummaryRow {
	pub total: Option<i64>,
	pub wos: Option<i64>,
	pub scopus: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct DepartmentRow {
	pub year: i16,
	pub department_id: Uuid,
	pub department: String,
	pub count: Option<i64>,
	pub wos: Option<i64>,
	pub scopus: Option<i64>,
}
#[derive(Debug, FromRow)]
pub struct ResearchLineDistributionRow {
	pub year: i16,
	pub research_line_id: Uuid,
	pub name: String,
	pub count: Option<i64>,
	pub wos: Option<i64>,
	pub scopus: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct ResearchLineSummaryRow {
	pub name: String,
	pub total: Option<i64>,
	pub wos: Option<i64>,
	pub scopus: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct ResearchLineDeptRow {
	pub department_id: Uuid,
	pub department: String,
	pub count: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct TopPublisherRow {
	pub academic_id: Uuid,
	pub name: String,
	pub total: Option<i64>,
	pub scopus: Option<i64>,
	pub wos: Option<i64>,
	pub unindexed: Option<i64>,
	pub option: String,
}

#[derive(Debug, FromRow)]
pub struct DeptSummaryRow {
	pub department: String,
	pub total: Option<i64>,
	pub scopus: Option<i64>,
	pub wos: Option<i64>,
	pub teaching: Option<i64>,
	pub research: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct ResearchLineRow {
	pub research_line_id: Uuid,
	pub name: String,
	pub count: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct ContributionRow {
	pub academic_works: Option<i64>,
	pub faculty_works: Option<i64>,
	pub department_works: Option<i64>,
	pub department_name: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct ProductivityTrendRow {
	pub period: i16,
	pub total: Option<i64>,
	pub wos: Option<i64>,
	pub scopus: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct JcePeriodRow {
	pub period: i16,
	pub jce: Option<f64>,
}
