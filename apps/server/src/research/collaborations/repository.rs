use crate::academic::AcademicId;
use crate::research::*;
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct CollaborationsRepository {
	database: Arc<Database>,
}

impl CollaborationsRepository {
	pub async fn find_nodes(
		&self,
		academic_id: &AcademicId,
	) -> AppResult<Vec<CollaborationNodeRow>> {
		sqlx::query_as::<_, CollaborationNodeRow>(
			"WITH coauthors AS (
				SELECT DISTINCT wa2.orcid
				FROM work_authorships wa1
				JOIN work_authorships wa2 ON wa2.work_id = wa1.work_id
				JOIN academics f ON f.id = $1 AND f.orcid = wa1.orcid
				JOIN works w ON w.id = wa1.work_id
					AND COALESCE(w.publication_date,
						make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
						BETWEEN f.joined_at AND COALESCE(f.left_at, CURRENT_DATE)
				WHERE NOT wa1.is_external AND NOT wa2.is_external AND wa2.orcid <> f.orcid
			)
			SELECT a.id, a.names, a.paternal_surname, a.maternal_surname,
				d.name AS department,
				COUNT(DISTINCT w.id) AS total_works
			FROM academics a
			LEFT JOIN work_authorships wa ON wa.orcid = a.orcid AND NOT wa.is_external
			LEFT JOIN works w ON w.id = wa.work_id
				AND COALESCE(w.publication_date,
					make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
					BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
			LEFT JOIN departments d ON d.id = a.department_id
			WHERE a.id = $1
				OR a.orcid IN (SELECT orcid FROM coauthors)
			GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, d.name",
		)
		.bind(academic_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_edges(
		&self,
		academic_id: &AcademicId,
	) -> AppResult<Vec<CollaborationEdgeRow>> {
		sqlx::query_as::<_, CollaborationEdgeRow>(
			"WITH coauthors AS (
				SELECT DISTINCT wa2.orcid
				FROM work_authorships wa1
				JOIN work_authorships wa2 ON wa2.work_id = wa1.work_id
				JOIN academics f ON f.id = $1 AND f.orcid = wa1.orcid
				JOIN works w ON w.id = wa1.work_id
					AND COALESCE(w.publication_date,
						make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
						BETWEEN f.joined_at AND COALESCE(f.left_at, CURRENT_DATE)
				WHERE NOT wa1.is_external AND NOT wa2.is_external AND wa2.orcid <> f.orcid
			),
			ego AS (
				SELECT orcid FROM academics WHERE id = $1
				UNION
				SELECT orcid FROM coauthors
			)
			SELECT a1.id AS source_id, a2.id AS target_id,
				COUNT(DISTINCT w.id) AS weight,
				array_agg(DISTINCT w.id) AS work_ids
			FROM work_authorships wa1
			JOIN work_authorships wa2 ON wa2.work_id = wa1.work_id AND wa2.orcid > wa1.orcid
			JOIN academics a1 ON a1.orcid = wa1.orcid
			JOIN academics a2 ON a2.orcid = wa2.orcid
			JOIN works w ON w.id = wa1.work_id
				AND COALESCE(w.publication_date,
					make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
					BETWEEN a1.joined_at AND COALESCE(a1.left_at, CURRENT_DATE)
				AND COALESCE(w.publication_date,
					make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
					BETWEEN a2.joined_at AND COALESCE(a2.left_at, CURRENT_DATE)
			WHERE NOT wa1.is_external AND NOT wa2.is_external
				AND a1.orcid IN (SELECT orcid FROM ego)
				AND a2.orcid IN (SELECT orcid FROM ego)
			GROUP BY a1.id, a2.id",
		)
		.bind(academic_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_works(&self, work_ids: &[WorkId]) -> AppResult<Vec<WorkRef>> {
		sqlx::query_as::<_, WorkRef>(
			"SELECT w.id, w.title,
				COALESCE((w.overrides).publication_year, w.publication_year) AS publication_year
			FROM works w
			WHERE w.id = ANY($1)
			ORDER BY COALESCE((w.overrides).publication_year, w.publication_year) DESC",
		)
		.bind(work_ids)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_recommendation_candidates(
		&self,
		academic_id: &AcademicId,
	) -> AppResult<Vec<RecommendationCandidateRow>> {
		sqlx::query_as::<_, RecommendationCandidateRow>(
			"SELECT a.id, a.names, a.paternal_surname, a.maternal_surname,
				d.name AS department,
				COUNT(DISTINCT w.id) AS total_works
			FROM academics a
			JOIN departments d ON d.id = a.department_id
			LEFT JOIN work_authorships wa ON wa.orcid = a.orcid AND NOT wa.is_external
			LEFT JOIN works w ON w.id = wa.work_id
				AND COALESCE(w.publication_date,
					make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
					BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
			WHERE a.id <> $1
			GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, d.name",
		)
		.bind(academic_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_academic_topics(
		&self,
		score_threshold: f64,
	) -> AppResult<Vec<AcademicTopicRow>> {
		sqlx::query_as::<_, AcademicTopicRow>(
			"SELECT a.id AS academic_id, t.id AS topic_id, t.name AS topic_name,
				wts.work_id, w.title AS work_title,
				COALESCE((w.overrides).publication_year, w.publication_year) AS publication_year,
				wts.score
			FROM work_topic_scores wts
			JOIN work_authorships wa ON wa.work_id = wts.work_id AND NOT wa.is_external
			JOIN academics a ON a.orcid = wa.orcid
			JOIN topics t ON t.id = wts.topic_id
			JOIN works w ON w.id = wts.work_id
				AND COALESCE(w.publication_date,
					make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
					BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
			JOIN subfields sf ON sf.id = t.subfield_id
			JOIN research_lines rl ON rl.id = sf.research_line_id AND rl.slug <> 'sin-asignar'
			WHERE wts.score >= $1",
		)
		.bind(score_threshold)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_academic_keywords(
		&self,
		score_threshold: f64,
	) -> AppResult<Vec<AcademicKeywordRow>> {
		sqlx::query_as::<_, AcademicKeywordRow>(
			"SELECT a.id AS academic_id, k.id AS keyword_id, k.name AS keyword_name,
				wks.work_id, w.title AS work_title,
				COALESCE((w.overrides).publication_year, w.publication_year) AS publication_year,
				wks.score
			FROM work_keyword_scores wks
			JOIN work_authorships wa ON wa.work_id = wks.work_id AND NOT wa.is_external
			JOIN academics a ON a.orcid = wa.orcid
			JOIN keywords k ON k.id = wks.keyword_id
			JOIN works w ON w.id = wks.work_id
			WHERE wks.score >= $1
				AND COALESCE(w.publication_date,
					make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
					BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)",
		)
		.bind(score_threshold)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_academic_lines(
		&self,
		score_threshold: f64,
	) -> AppResult<Vec<AcademicLineRow>> {
		sqlx::query_as::<_, AcademicLineRow>(
			"WITH topic_lines AS (
				SELECT DISTINCT a.id AS academic_id, rl.id AS research_line_id
				FROM work_topic_scores wts
				JOIN work_authorships wa ON wa.work_id = wts.work_id AND NOT wa.is_external
				JOIN academics a ON a.orcid = wa.orcid
				JOIN works w ON w.id = wts.work_id
					AND COALESCE(w.publication_date,
						make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
						BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
				JOIN topics t ON t.id = wts.topic_id
				JOIN subfields sf ON sf.id = t.subfield_id
				JOIN research_lines rl ON rl.id = sf.research_line_id AND rl.slug <> 'sin-asignar'
				WHERE wts.score >= $1
			),
			override_lines AS (
				SELECT DISTINCT a.id AS academic_id, rl.id AS research_line_id
				FROM works w
				JOIN work_authorships wa ON wa.work_id = w.id AND NOT wa.is_external
				JOIN academics a ON a.orcid = wa.orcid
				JOIN research_lines rl ON rl.id = (w.overrides).research_line_id AND rl.slug <> 'sin-asignar'
					AND COALESCE(w.publication_date,
						make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
						BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
			)
			SELECT * FROM topic_lines
			UNION
			SELECT * FROM override_lines",
		)
		.bind(score_threshold)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}
}
