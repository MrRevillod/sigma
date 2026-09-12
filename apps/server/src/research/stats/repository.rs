use crate::academic::{AcademicId, DegreeKind};
use crate::research::stats::*;
use crate::shared::{AppResult, Database};

use sqlx::types::Uuid;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct StatsRepository {
	database: Arc<Database>,
}

impl StatsRepository {
	pub async fn stats_by_journal_kind(
		&self,
		query: &WorksStatsQuery,
	) -> AppResult<Vec<JournalKindRow>> {
		sqlx::query_as::<_, JournalKindRow>(
			"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a         ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    JOIN departments d       ON a.department_id = d.id
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
		        AND ($2::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
		        AND ($3::uuid IS NULL OR a.department_id = $3)
		        AND ($4::journal_kind IS NULL OR COALESCE((w.overrides).journal_kind, ji.kind) = $4)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		    GROUP BY year
		    ORDER BY year",
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn stats_by_department(
		&self,
		query: &WorksStatsQuery,
	) -> AppResult<Vec<DepartmentRow>> {
		sqlx::query_as::<_, DepartmentRow>(
			"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
		        d.id   AS department_id,
		        d.name AS department,
		        COUNT(DISTINCT w.id)::bigint AS count,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a         ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    JOIN departments d       ON a.department_id = d.id
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
		        AND ($2::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
		        AND ($3::uuid IS NULL OR a.department_id = $3)
		        AND ($4::journal_kind IS NULL OR COALESCE((w.overrides).journal_kind, ji.kind) = $4)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		    GROUP BY year, d.id, d.name
		    ORDER BY d.name, year",
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn stats_by_research_line(
		&self,
		query: &WorksStatsQuery,
	) -> AppResult<Vec<ResearchLineDistributionRow>> {
		sqlx::query_as::<_, ResearchLineDistributionRow>(
			"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
		        rl.id   AS research_line_id,
		        rl.name AS name,
		        COUNT(DISTINCT w.id)::bigint AS count,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a         ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    JOIN research_lines rl ON rl.id = COALESCE(
		            (w.overrides).research_line_id,
		            (
		                SELECT sf.research_line_id
		                FROM work_topic_scores wt
		                JOIN topics t ON t.id = wt.topic_id
		                JOIN subfields sf ON sf.id = t.subfield_id
		                WHERE wt.work_id = w.id
		                ORDER BY wt.score DESC
		                LIMIT 1
		            ),
		            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		        )
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
		        AND ($2::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
		        AND ($3::uuid IS NULL OR a.department_id = $3)
		        AND ($4::journal_kind IS NULL OR COALESCE((w.overrides).journal_kind, ji.kind) = $4)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        AND rl.slug <> 'sin-asignar'
		    GROUP BY year, rl.id, rl.name
		    ORDER BY rl.name, year",
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn faculty_summary(&self, query: &WorksStatsQuery) -> AppResult<FacultySummaryRow> {
		sqlx::query_as::<_, FacultySummaryRow>(
			"SELECT COUNT(DISTINCT w.id)::bigint AS total,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a         ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
		        AND ($2::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
		        AND ($3::uuid IS NULL OR a.department_id = $3)
		        AND ($4::journal_kind IS NULL OR COALESCE((w.overrides).journal_kind, ji.kind) = $4)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)",
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.journal_kind)
		.fetch_one(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn department_summary(
		&self,
		id: &Uuid,
		query: &DepartmentDetailQuery,
	) -> AppResult<DeptSummaryRow> {
		sqlx::query_as::<_, DeptSummaryRow>(
			"SELECT d.name AS department,
		        COUNT(DISTINCT w.id)::bigint AS total,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint  AS scopus,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint     AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'teaching')::bigint AS teaching,
		        COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'research')::bigint AS research
		    FROM departments d
		    LEFT JOIN academics a ON a.department_id = d.id
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    LEFT JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
		    LEFT JOIN work_authorships wa ON wa.orcid = a.orcid AND wa.is_external = false
		    LEFT JOIN works w       ON w.id = wa.work_id
		        AND COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND ($3::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $3)
		        AND ($4::academic_option IS NULL OR aco.option = $4)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE d.id = $1
		        AND ($5::journal_kind IS NULL OR COALESCE((w.overrides).journal_kind, ji.kind) = $5)
		    GROUP BY d.id, d.name",
		)
		.bind(id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_one(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn top_publishers(
		&self,
		id: &Uuid,
		query: &DepartmentDetailQuery,
		limit: i32,
	) -> AppResult<Vec<TopPublisherRow>> {
		sqlx::query_as::<_, TopPublisherRow>(
			"SELECT a.id AS academic_id,
		        a.names || ' ' || a.paternal_surname || ' ' || a.maternal_surname AS name,
		        COUNT(DISTINCT w.id)::bigint AS total,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) IS NULL)::bigint    AS unindexed,
		        aco.option::text AS option
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a         ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    JOIN departments d       ON a.department_id = d.id
		    JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE d.id = $1
		        AND COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND ($3::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $3)
		        AND ($4::academic_option IS NULL OR aco.option = $4)
		        AND ($5::journal_kind IS NULL OR COALESCE((w.overrides).journal_kind, ji.kind) = $5)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		    GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, aco.option
		    ORDER BY total DESC
		    LIMIT $6",
		)
		.bind(id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.option)
		.bind(query.journal_kind)
		.bind(limit)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn top_publishers_faculty(
		&self,
		query: &WorksStatsQuery,
		limit: i32,
	) -> AppResult<Vec<TopPublisherRow>> {
		sqlx::query_as::<_, TopPublisherRow>(
			"SELECT a.id AS academic_id,
		        a.names || ' ' || a.paternal_surname || ' ' || a.maternal_surname AS name,
		        COUNT(DISTINCT w.id)::bigint AS total,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) IS NULL)::bigint    AS unindexed,
		        aco.option::text AS option
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a         ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
		        AND ($2::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		    GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, aco.option
		    ORDER BY total DESC
		    LIMIT $3",
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(limit)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn department_journal_kind_trend(
		&self,
		id: &Uuid,
		query: &DepartmentDetailQuery,
	) -> AppResult<Vec<JournalKindRow>> {
		sqlx::query_as::<_, JournalKindRow>(
			"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a ON a.orcid = wa.orcid AND a.department_id = $1
		    JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
		    LEFT JOIN sources src ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND ($3::smallint IS NULL
		            OR COALESCE((w.overrides).publication_year, w.publication_year) <= $3)
		        AND ($4::academic_option IS NULL OR aco.option = $4)
		        AND ($5::journal_kind IS NULL OR COALESCE((w.overrides).journal_kind, ji.kind) = $5)
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		    GROUP BY year
		    ORDER BY year",
		)
		.bind(id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn academic_line_distribution(
		&self,
		academic_id: &AcademicId,
		query: &AcademicStatsQuery,
	) -> AppResult<Vec<ResearchLineRow>> {
		sqlx::query_as::<_, ResearchLineRow>(
			"SELECT rl.id AS research_line_id, rl.name AS name,
		        COALESCE(lc.count, 0)::bigint AS count
		    FROM research_lines rl
		    LEFT JOIN (
		        SELECT x.line_id, COUNT(*)::bigint AS count
		        FROM (
		            SELECT COALESCE(
		                (w.overrides).research_line_id,
		                (
		                    SELECT sf.research_line_id
		                    FROM work_topic_scores wt
		                    JOIN topics t ON t.id = wt.topic_id
		                    JOIN subfields sf ON sf.id = t.subfield_id
		                    WHERE wt.work_id = w.id
		                    ORDER BY wt.score DESC
		                    LIMIT 1
		                ),
		                (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		            ) AS line_id
		            FROM works w
		            JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		            JOIN academics a ON a.orcid = wa.orcid AND a.id = $1
		            WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		                AND COALESCE((w.overrides).publication_year, w.publication_year) <= $3
		                AND COALESCE(w.publication_date,
		                    make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		                    BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        ) x
		        GROUP BY x.line_id
		    ) lc ON lc.line_id = rl.id
		    WHERE rl.slug <> 'sin-asignar'
		    GROUP BY rl.id, rl.name, lc.count
		    ORDER BY count DESC, rl.name",
		)
		.bind(academic_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn academic_journal_kind_trend(
		&self,
		academic_id: &AcademicId,
		query: &AcademicStatsQuery,
	) -> AppResult<Vec<JournalKindRow>> {
		sqlx::query_as::<_, JournalKindRow>(
			"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a ON a.orcid = wa.orcid AND a.id = $1
		    LEFT JOIN sources src ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND COALESCE((w.overrides).publication_year, w.publication_year) <= $3
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		    GROUP BY year
		    ORDER BY year",
		)
		.bind(academic_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn academic_contribution(
		&self,
		academic_id: &AcademicId,
		query: &AcademicStatsQuery,
	) -> AppResult<ContributionRow> {
		sqlx::query_as::<_, ContributionRow>(
			"SELECT
		        (SELECT COUNT(DISTINCT w1.id)::bigint
		         FROM works w1
		         JOIN work_authorships wa1 ON w1.id = wa1.work_id AND wa1.is_external = false
		         JOIN academics a1 ON a1.orcid = wa1.orcid AND a1.id = $1
		         WHERE COALESCE((w1.overrides).publication_year, w1.publication_year) BETWEEN $2 AND $3
		            AND COALESCE(w1.publication_date,
		                make_date(COALESCE((w1.overrides).publication_year, w1.publication_year), 1, 1))
		                BETWEEN a1.joined_at AND COALESCE(a1.left_at, CURRENT_DATE)) AS academic_works,
		        (SELECT COUNT(DISTINCT w2.id)::bigint
		         FROM works w2
		         JOIN work_authorships wa2 ON w2.id = wa2.work_id AND wa2.is_external = false
		         JOIN academics a2 ON a2.orcid = wa2.orcid
		            AND a2.orcid != 'https://orcid.org/0000-0000-0000-0000'
		         WHERE COALESCE((w2.overrides).publication_year, w2.publication_year) BETWEEN $2 AND $3
		            AND COALESCE(w2.publication_date,
		                make_date(COALESCE((w2.overrides).publication_year, w2.publication_year), 1, 1))
		                BETWEEN a2.joined_at AND COALESCE(a2.left_at, CURRENT_DATE)) AS faculty_works,
		        (SELECT COUNT(DISTINCT w3.id)::bigint
		         FROM works w3
		         JOIN work_authorships wa3 ON w3.id = wa3.work_id AND wa3.is_external = false
		         JOIN academics a3 ON a3.orcid = wa3.orcid
		            AND a3.department_id = (SELECT department_id FROM academics WHERE id = $1)
		         WHERE COALESCE((w3.overrides).publication_year, w3.publication_year) BETWEEN $2 AND $3
		            AND COALESCE(w3.publication_date,
		                make_date(COALESCE((w3.overrides).publication_year, w3.publication_year), 1, 1))
		                BETWEEN a3.joined_at AND COALESCE(a3.left_at, CURRENT_DATE)) AS department_works,
		        (SELECT d.name FROM departments d
		         JOIN academics a4 ON a4.department_id = d.id WHERE a4.id = $1) AS department_name",
		)
		.bind(academic_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.fetch_one(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn works_in_research_line(
		&self,
		line_id: &Uuid,
		query: &AcademicStatsQuery,
	) -> AppResult<i64> {
		sqlx::query_scalar::<_, i64>(
			"SELECT COUNT(DISTINCT w.id)::bigint
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) BETWEEN $2 AND $3
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        AND COALESCE(
		            (w.overrides).research_line_id,
		            (
		                SELECT sf.research_line_id
		                FROM work_topic_scores wt
		                JOIN topics t ON t.id = wt.topic_id
		                JOIN subfields sf ON sf.id = t.subfield_id
		                WHERE wt.work_id = w.id
		                ORDER BY wt.score DESC
		                LIMIT 1
		            ),
		            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		        ) = $1",
		)
		.bind(line_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.fetch_one(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn research_line_summary(
		&self,
		line_id: &Uuid,
		query: &ResearchLineStatsQuery,
	) -> AppResult<ResearchLineSummaryRow> {
		sqlx::query_as::<_, ResearchLineSummaryRow>(
			"SELECT rl.name AS name,
		        COUNT(DISTINCT w.id)::bigint AS total,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos
		    FROM research_lines rl
		    LEFT JOIN works w ON rl.id = COALESCE(
		            (w.overrides).research_line_id,
		            (
		                SELECT sf.research_line_id
		                FROM work_topic_scores wt
		                JOIN topics t ON t.id = wt.topic_id
		                JOIN subfields sf ON sf.id = t.subfield_id
		                WHERE wt.work_id = w.id
		                ORDER BY wt.score DESC
		                LIMIT 1
		            ),
		            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		        )
		        AND COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND COALESCE((w.overrides).publication_year, w.publication_year) <= $3
		        AND EXISTS (
		            SELECT 1
		            FROM work_authorships wa
		            JOIN academics a ON a.orcid = wa.orcid
		                AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		            WHERE wa.work_id = w.id AND wa.is_external = false
		                AND COALESCE(w.publication_date,
		                    make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		                    BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        )
		    LEFT JOIN sources src    ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE rl.id = $1
		    GROUP BY rl.id, rl.name",
		)
		.bind(line_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.fetch_one(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn research_line_journal_kind_trend(
		&self,
		line_id: &Uuid,
		query: &ResearchLineStatsQuery,
	) -> AppResult<Vec<JournalKindRow>> {
		sqlx::query_as::<_, JournalKindRow>(
			"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    LEFT JOIN sources src ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND COALESCE((w.overrides).publication_year, w.publication_year) <= $3
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        AND COALESCE(
		            (w.overrides).research_line_id,
		            (
		                SELECT sf.research_line_id
		                FROM work_topic_scores wt
		                JOIN topics t ON t.id = wt.topic_id
		                JOIN subfields sf ON sf.id = t.subfield_id
		                WHERE wt.work_id = w.id
		                ORDER BY wt.score DESC
		                LIMIT 1
		            ),
		            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		        ) = $1
		    GROUP BY year
		    ORDER BY year",
		)
		.bind(line_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn research_line_department_distribution(
		&self,
		line_id: &Uuid,
		query: &ResearchLineStatsQuery,
	) -> AppResult<Vec<ResearchLineDeptRow>> {
		sqlx::query_as::<_, ResearchLineDeptRow>(
			"SELECT d.id AS department_id, d.name AS department,
		        COUNT(DISTINCT w.id)::bigint AS count
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    JOIN departments d ON a.department_id = d.id
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND COALESCE((w.overrides).publication_year, w.publication_year) <= $3
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        AND COALESCE(
		            (w.overrides).research_line_id,
		            (
		                SELECT sf.research_line_id
		                FROM work_topic_scores wt
		                JOIN topics t ON t.id = wt.topic_id
		                JOIN subfields sf ON sf.id = t.subfield_id
		                WHERE wt.work_id = w.id
		                ORDER BY wt.score DESC
		                LIMIT 1
		            ),
		            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		        ) = $1
		    GROUP BY d.id, d.name
		    ORDER BY count DESC",
		)
		.bind(line_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn research_line_top_publishers(
		&self,
		line_id: &Uuid,
		query: &ResearchLineStatsQuery,
		limit: i32,
	) -> AppResult<Vec<TopPublisherRow>> {
		sqlx::query_as::<_, TopPublisherRow>(
			"SELECT a.id AS academic_id,
		        a.names || ' ' || a.paternal_surname || ' ' || a.maternal_surname AS name,
		        COUNT(DISTINCT w.id)::bigint AS total,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
		        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) IS NULL)::bigint    AS unindexed,
		        aco.option::text AS option
		    FROM works w
		    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
		    JOIN academics a ON a.orcid = wa.orcid
		        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
		    JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
		    LEFT JOIN sources src ON w.source_id = src.id
		    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $2
		        AND COALESCE((w.overrides).publication_year, w.publication_year) <= $3
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        AND COALESCE(
		            (w.overrides).research_line_id,
		            (
		                SELECT sf.research_line_id
		                FROM work_topic_scores wt
		                JOIN topics t ON t.id = wt.topic_id
		                JOIN subfields sf ON sf.id = t.subfield_id
		                WHERE wt.work_id = w.id
		                ORDER BY wt.score DESC
		                LIMIT 1
		            ),
		            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		        ) = $1
		    GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, aco.option
		    ORDER BY total DESC
		    LIMIT $4",
		)
		.bind(line_id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to.unwrap_or(2100))
		.bind(limit)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn productivity_numerator(
		&self,
		query: &ProductivityQuery,
		month: i16,
		year_from: i16,
		year_to: i16,
		degree: Option<DegreeKind>,
	) -> AppResult<Vec<ProductivityTrendRow>> {
		sqlx::query_as::<_, ProductivityTrendRow>(
			"SELECT (COALESCE((w.overrides).publication_year, w.publication_year)
			        - CASE WHEN EXTRACT(MONTH FROM w.publication_date) < $1 THEN 1 ELSE 0 END)::smallint
			       AS period,
			        COUNT(DISTINCT w.id)::bigint AS total,
			        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'wos')::bigint    AS wos,
			        COUNT(DISTINCT w.id) FILTER (WHERE COALESCE((w.overrides).journal_kind, ji.kind) = 'scopus')::bigint AS scopus
			    FROM works w
			    JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
			    JOIN academics a         ON a.orcid = wa.orcid
			        AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
			    LEFT JOIN sources src    ON w.source_id = src.id
			    LEFT JOIN journal_issn ji ON ji.issn = src.issn
		    WHERE (COALESCE((w.overrides).publication_year, w.publication_year)
		            - CASE WHEN EXTRACT(MONTH FROM w.publication_date) < $1 THEN 1 ELSE 0 END)::smallint
		            BETWEEN $2 AND $3
		        AND COALESCE(w.publication_date,
		            make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
		            BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
		        AND ($4::degree_kind IS NULL
			            OR a.id IN (SELECT academic_id FROM degrees WHERE kind = $4))
			        AND ($5::uuid IS NULL OR a.department_id = $5)
			        AND ($6::uuid IS NULL OR COALESCE(
			            (w.overrides).research_line_id,
			            (
			                SELECT sf.research_line_id
			                FROM work_topic_scores wt
			                JOIN topics t ON t.id = wt.topic_id
			                JOIN subfields sf ON sf.id = t.subfield_id
			                WHERE wt.work_id = w.id
			                ORDER BY wt.score DESC
			                LIMIT 1
			            ),
			            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
			        ) = $6)
			    GROUP BY period
			    ORDER BY period",
		)
		.bind(month)
		.bind(year_from)
		.bind(year_to)
		.bind(degree)
		.bind(query.department_id)
		.bind(query.research_line_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn jce_by_period(
		&self,
		department_id: Option<Uuid>,
		year_from: i16,
		year_to: i16,
		degree_kind: Option<DegreeKind>,
	) -> AppResult<Vec<JcePeriodRow>> {
		sqlx::query_as::<_, JcePeriodRow>(
			"SELECT y.period::smallint AS period,
			        COALESCE(SUM(a.jce), 0)::float8 AS jce
			    FROM generate_series($1::int, $2::int) AS y(period)
			    LEFT JOIN academics a
			        ON a.joined_at < make_date(y.period + 1, 1, 1)
			        AND COALESCE(a.left_at, DATE 'infinity') >= make_date(y.period, 1, 1)
			        AND ($3::uuid IS NULL OR a.department_id = $3)
			        AND ($4::degree_kind IS NULL
			            OR a.id IN (SELECT academic_id FROM degrees WHERE kind = $4))
			    GROUP BY y.period
			    ORDER BY y.period",
		)
		.bind(year_from as i32)
		.bind(year_to as i32)
		.bind(department_id)
		.bind(degree_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn jce_by_period_research_line(
		&self,
		research_line_id: &Uuid,
		year_from: i16,
		year_to: i16,
		degree_kind: Option<DegreeKind>,
	) -> AppResult<Vec<JcePeriodRow>> {
		sqlx::query_as::<_, JcePeriodRow>(
			"WITH doc AS (
			        SELECT a.id AS academic_id, a.orcid, a.jce,
			            a.joined_at, a.left_at
			        FROM academics a
			        WHERE ($3::degree_kind IS NULL
			                OR a.id IN (SELECT academic_id FROM degrees WHERE kind = $3))
			    ),
			    dw AS (
			        SELECT d.academic_id, d.jce, d.joined_at, d.left_at,
			            COALESCE(
			                (w.overrides).research_line_id,
			                (
			                    SELECT sf.research_line_id
			                    FROM work_topic_scores wt
			                    JOIN topics t ON t.id = wt.topic_id
			                    JOIN subfields sf ON sf.id = t.subfield_id
			                    WHERE wt.work_id = w.id
			                    ORDER BY wt.score DESC
			                    LIMIT 1
			                ),
			                (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
			            ) AS line_id
			        FROM doc d
			        JOIN work_authorships wa ON wa.orcid = d.orcid AND wa.is_external = false
			        JOIN works w ON w.id = wa.work_id
			            AND COALESCE(w.publication_date,
			                make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
			                BETWEEN d.joined_at AND COALESCE(d.left_at, CURRENT_DATE)
			    ),
			    cnt AS (
			        SELECT academic_id, jce, joined_at, left_at, line_id, COUNT(*) AS n
			        FROM dw GROUP BY academic_id, jce, joined_at, left_at, line_id
			    ),
			    dom AS (
			        SELECT DISTINCT ON (academic_id)
			            academic_id, jce, joined_at, left_at, line_id
			        FROM cnt ORDER BY academic_id, n DESC, line_id
			    )
			    SELECT y.period::smallint AS period,
			        COALESCE(SUM(dom.jce), 0)::float8 AS jce
			    FROM generate_series($1::int, $2::int) AS y(period)
			    LEFT JOIN dom
			        ON dom.line_id = $4::uuid
			        AND dom.joined_at < make_date(y.period + 1, 1, 1)
			        AND COALESCE(dom.left_at, DATE 'infinity') >= make_date(y.period, 1, 1)
			    GROUP BY y.period
			    ORDER BY y.period",
		)
		.bind(year_from as i32)
		.bind(year_to as i32)
		.bind(degree_kind)
		.bind(research_line_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}
}
