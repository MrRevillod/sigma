use crate::research::*;
use crate::shared::{AppResult, Database};

use sqlx::QueryBuilder;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorksRepository {
	database: Arc<Database>,
}

impl WorksRepository {
	pub async fn find_by_id(&self, id: &WorkId) -> AppResult<Option<WorkView>> {
		sqlx::query_as::<_, WorkView>(
			"SELECT w.id, w.openalex_id, w.title, w.abstract_text, w.doi,
			        w.publication_date, w.publication_year, w.ty, w.lang,
			        w.is_accepted, w.is_published, w.source_id, w.updated_at, w.overrides,
			        COALESCE((w.overrides).journal_kind,
			            (SELECT ji.kind FROM journal_issn ji WHERE ji.issn = s.issn))
			            AS journal_kind,
			        rl.id   AS research_line_id,
			        rl.name AS research_line_name
			    FROM works w
			    LEFT JOIN sources s ON s.id = w.source_id
			    LEFT JOIN research_lines rl
			        ON rl.id = COALESCE(
			            (w.overrides).research_line_id,
			            (
			                SELECT sf.research_line_id
			                FROM work_topic_scores wt
			                JOIN topics t     ON t.id = wt.topic_id
			                JOIN subfields sf ON sf.id = t.subfield_id
			                WHERE wt.work_id = w.id
			                ORDER BY wt.score DESC
			                LIMIT 1
			            ),
			            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
			        )
			    WHERE w.id = $1",
		)
		.bind(id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_work(&self, id: &WorkId) -> AppResult<Option<Work>> {
		sqlx::query_as::<_, Work>(
			"SELECT id, openalex_id, title, abstract_text, doi,
			        publication_date, publication_year, ty, lang,
			        is_accepted, is_published, source_id, updated_at, overrides
			    FROM works WHERE id = $1",
		)
		.bind(id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_by_openalex_id(&self, openalex_id: &str) -> AppResult<Option<Work>> {
		sqlx::query_as::<_, Work>(
			"SELECT id, openalex_id, title, abstract_text, doi,
		        publication_date, publication_year, ty, lang,
		        is_accepted, is_published, source_id, updated_at, overrides
		    FROM works WHERE openalex_id = $1",
		)
		.bind(openalex_id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<WorkView>> {
		let mut qb = QueryBuilder::new(
			"SELECT w.id, w.openalex_id, w.title, w.abstract_text,
		        w.doi, w.publication_date, w.publication_year,
		        w.ty, w.lang, w.is_accepted, w.is_published,
		        w.source_id, w.updated_at, w.overrides,
		        COALESCE((w.overrides).journal_kind,
		            (SELECT ji.kind FROM journal_issn ji WHERE ji.issn = s.issn))
		            AS journal_kind,
		        rl.id   AS research_line_id,
		        rl.name AS research_line_name
		    FROM works w
		    LEFT JOIN sources s ON s.id = w.source_id
		    LEFT JOIN research_lines rl
		        ON rl.id = COALESCE(
		            (w.overrides).research_line_id,
		            (
		                SELECT sf.research_line_id
		                FROM work_topic_scores wt
		                JOIN topics t     ON t.id = wt.topic_id
		                JOIN subfields sf ON sf.id = t.subfield_id
		                WHERE wt.work_id = w.id
		                ORDER BY wt.score DESC
		                LIMIT 1
		            ),
		            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
		        )
		    WHERE TRUE ",
		);

		if let Some(academic_id) = query.academic_id {
			qb.push(
				"    AND w.id IN (
			        SELECT wa2.work_id
			        FROM work_authorships wa2
			        JOIN academics a ON a.orcid = wa2.orcid
			        WHERE wa2.is_external = false
			            AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
			            AND COALESCE(w.publication_date,
			                make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
			                BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
			            AND a.id = ",
			);
			qb.push_bind(academic_id);
			qb.push(")");
		}

		if let Some(department_id) = query.department_id {
			qb.push(
				"    AND w.id IN (
			        SELECT wa2.work_id
			        FROM work_authorships wa2
			        JOIN academics a ON a.orcid = wa2.orcid
			        WHERE wa2.is_external = false
			            AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
			            AND COALESCE(w.publication_date,
			                make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
			                BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
			            AND a.department_id = ",
			);
			qb.push_bind(department_id);
			qb.push(")");
		}

		if let Some(career_id) = query.career_id {
			qb.push(
				"    AND w.id IN (
			        SELECT wa2.work_id
			        FROM work_authorships wa2
			        JOIN academics a ON a.orcid = wa2.orcid
			        WHERE wa2.is_external = false
			            AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
			            AND COALESCE(w.publication_date,
			                make_date(COALESCE((w.overrides).publication_year, w.publication_year), 1, 1))
			                BETWEEN a.joined_at AND COALESCE(a.left_at, CURRENT_DATE)
			            AND a.career_id = ",
			);
			qb.push_bind(career_id);
			qb.push(")");
		}

		if let Some(ref search) = query.search {
			qb.push("    AND w.title ILIKE ");
			qb.push_bind(format!("%{}%", search));
		}

		if let Some(year_from) = query.year_from {
			qb.push("    AND COALESCE((w.overrides).publication_year, w.publication_year) >= ");
			qb.push_bind(year_from);
		}

		if let Some(year_to) = query.year_to {
			qb.push("    AND COALESCE((w.overrides).publication_year, w.publication_year) <= ");
			qb.push_bind(year_to);
		}

		if let Some(is_accepted) = query.is_accepted {
			qb.push("    AND COALESCE((w.overrides).is_accepted, w.is_accepted) = ");
			qb.push_bind(is_accepted);
		}

		if let Some(is_published) = query.is_published {
			qb.push("    AND COALESCE((w.overrides).is_published, w.is_published) = ");
			qb.push_bind(is_published);
		}

		if let Some(ref journal_kind) = query.journal_kind {
			qb.push(
				"    AND COALESCE((w.overrides).journal_kind,
			            (SELECT ji.kind FROM journal_issn ji WHERE ji.issn = s.issn)) = ",
			);
			qb.push_bind(journal_kind);
		}

		if let Some(research_line_id) = query.research_line_id {
			qb.push(
				"    AND COALESCE(
			            (w.overrides).research_line_id,
			            (
			                SELECT rs.research_line_id
			                FROM work_topic_scores wt
			                JOIN topics rt    ON rt.id = wt.topic_id
			                JOIN subfields rs ON rs.id = rt.subfield_id
			                WHERE wt.work_id = w.id
			                ORDER BY wt.score DESC
			                LIMIT 1
			            ),
			            (SELECT id FROM research_lines WHERE slug = 'sin-asignar')
			        ) = ",
			);
			qb.push_bind(research_line_id);
		}

		qb.push(
			"    ORDER BY COALESCE((w.overrides).publication_year, w.publication_year) DESC
			            NULLS LAST,
			            w.publication_date DESC NULLS LAST,
			            w.id
			    LIMIT ",
		);

		qb.push_bind(query.size.unwrap_or(100) as i64);

		qb.build_query_as()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn save(&self, work: &Work) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO works (
		        id, openalex_id, title, abstract_text, doi,
		        publication_date, publication_year, ty, lang,
		        is_accepted, is_published, source_id, overrides, updated_at
		    ) VALUES (
		        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
		        $11, $12, $13, $14
		    ) ON CONFLICT (id) DO UPDATE SET
		        title            = EXCLUDED.title,
		        abstract_text    = EXCLUDED.abstract_text,
		        doi              = EXCLUDED.doi,
		        publication_date = EXCLUDED.publication_date,
		        publication_year = EXCLUDED.publication_year,
		        ty               = EXCLUDED.ty,
		        lang             = EXCLUDED.lang,
		        is_accepted      = EXCLUDED.is_accepted,
		        is_published     = EXCLUDED.is_published,
		        source_id        = EXCLUDED.source_id,
		        overrides        = EXCLUDED.overrides,
		        updated_at       = EXCLUDED.updated_at",
		)
		.bind(work.id)
		.bind(&work.openalex_id)
		.bind(&work.title)
		.bind(&work.abstract_text)
		.bind(&work.doi)
		.bind(work.publication_date)
		.bind(work.publication_year)
		.bind(work.ty)
		.bind(&work.lang)
		.bind(work.is_accepted)
		.bind(work.is_published)
		.bind(work.source_id)
		.bind(&work.overrides)
		.bind(work.updated_at)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn save_topic_score(&self, score: &WorkTopicScore) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO work_topic_scores (work_id, topic_id, score)
			VALUES ($1, $2, $3)
			ON CONFLICT (work_id, topic_id) DO NOTHING",
		)
		.bind(score.work_id)
		.bind(score.topic_id)
		.bind(score.score)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn save_keyword_score(&self, score: &WorkKeywordScore) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO work_keyword_scores (work_id, keyword_id, score)
			VALUES ($1, $2, $3)
			ON CONFLICT (work_id, keyword_id) DO NOTHING",
		)
		.bind(score.work_id)
		.bind(score.keyword_id)
		.bind(score.score)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn list_topics_by_work(&self, work_id: &WorkId) -> AppResult<Vec<TopicView>> {
		sqlx::query_as::<_, TopicView>(
			"SELECT wt.topic_id, t.name, wt.score,
		        s.id AS subfield_id,  s.name AS subfield_name,
		        f.id AS field_id,     f.name AS field_name,
		        d.id AS domain_id,    d.name AS domain_name
		    FROM work_topic_scores wt
		    JOIN topics t     ON t.id  = wt.topic_id
		    JOIN subfields s  ON s.id  = t.subfield_id
		    JOIN fields f     ON f.id  = s.field_id
		    JOIN domains d    ON d.id  = f.domain_id
		    WHERE wt.work_id = $1
		    ORDER BY wt.score DESC",
		)
		.bind(work_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn list_keywords_by_work(&self, work_id: &WorkId) -> AppResult<Vec<KeywordView>> {
		sqlx::query_as::<_, KeywordView>(
			"SELECT wk.keyword_id, k.name, wk.score
		    FROM work_keyword_scores wk
		    JOIN keywords k ON k.id = wk.keyword_id
		    WHERE wk.work_id = $1
		    ORDER BY wk.score DESC",
		)
		.bind(work_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}
}
