use crate::{
	academic::AcademicId,
	research::{KeywordId, ResearchLineId, TopicId, WorkId},
};

use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CollaborationNodeRow {
	pub id: AcademicId,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub department: String,
	pub total_works: i64,
}

#[derive(Debug, FromRow)]
pub struct CollaborationEdgeRow {
	pub source_id: AcademicId,
	pub target_id: AcademicId,
	pub weight: i64,
	pub work_ids: Vec<WorkId>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct WorkRef {
	pub id: WorkId,
	pub title: String,
	pub publication_year: Option<i16>,
}

#[derive(Debug, FromRow)]
pub struct RecommendationCandidateRow {
	pub id: AcademicId,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub department: String,
	pub total_works: i64,
}

#[derive(Debug, FromRow)]
pub struct AcademicTopicRow {
	pub academic_id: AcademicId,
	pub topic_id: TopicId,
	pub topic_name: String,
	pub work_id: WorkId,
	pub work_title: String,
	pub publication_year: Option<i16>,
	pub score: f64,
}

#[derive(Debug, FromRow)]
pub struct AcademicKeywordRow {
	pub academic_id: AcademicId,
	pub keyword_id: KeywordId,
	pub keyword_name: String,
	pub work_id: WorkId,
	pub work_title: String,
	pub publication_year: Option<i16>,
	pub score: f64,
}

#[derive(Debug, FromRow)]
pub struct AcademicLineRow {
	pub academic_id: AcademicId,
	pub research_line_id: ResearchLineId,
}
