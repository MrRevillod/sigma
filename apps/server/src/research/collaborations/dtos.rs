use crate::academic::AcademicId;
use crate::research::{WorkId, WorkRef};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Default, Validate, Deserialize)]
pub struct CollaborationsQuery {
	#[validate(range(
		min = 0.0,
		max = 1.0,
		message = "El score de coincidencia debe estar entre 0 y 1"
	))]
	pub score_threshold: Option<f64>,
	#[validate(range(
		min = 1,
		max = 10,
		message = "El mínimo de coincidencias por work debe estar entre 1 y 10"
	))]
	pub min_coincidences: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationGraph {
	pub academic_id: AcademicId,
	pub nodes: Vec<CollaborationNode>,
	pub edges: Vec<CollaborationEdge>,
	pub recommendations: Vec<CollaborationRecommendation>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationNode {
	pub id: AcademicId,
	pub name: String,
	pub department: String,
	pub total_works: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationEdge {
	pub source_id: AcademicId,
	pub target_id: AcademicId,
	pub weight: i64,
	pub works: Vec<WorkRef>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationRecommendation {
	pub academic_id: AcademicId,
	pub name: String,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub department: String,
	pub total_works: i64,
	pub weight: i64,
	pub works: Vec<RecommendationWork>,
	pub focus_works: Vec<RecommendationWork>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationWork {
	pub work_id: WorkId,
	pub title: String,
	pub publication_year: Option<i16>,
	pub side: RecommendationWorkSide,
	pub shared: Vec<RecommendationSharedItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RecommendationWorkSide {
	Focus,
	Candidate,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationSharedItem {
	pub r#type: RecommendationMatchType,
	pub id: Uuid,
	pub name: String,
	pub score: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchWorkRef {
	pub work_id: WorkId,
	pub title: String,
	pub publication_year: Option<i16>,
	pub score: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RecommendationMatchType {
	Topic,
	Keyword,
}
