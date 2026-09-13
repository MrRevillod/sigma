use crate::academic::{
	Academic, AcademicId, AcademicListFilter, AcademicView, EditCode, EditCodeId,
};
use crate::shared::{AppResult, Database, Tx};

use sqlx::QueryBuilder;
use sqlx::types::chrono::Utc;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicsRepository {
	database: Arc<Database>,
}

impl AcademicsRepository {
	pub async fn list(&self, filter: AcademicListFilter) -> AppResult<Vec<AcademicView>> {
		let mut query = QueryBuilder::new(
			"SELECT a.id, a.names, a.paternal_surname, a.maternal_surname,
			        a.email, a.orcid, a.sex, a.birth_date, a.joined_at, a.left_at,
			        a.work_position_id,
			        wp.name AS work_position,
			        a.department_id,
			        d.name AS department,
			        a.career_id,
			        c.name AS career,
			        a.jce,
			        a.acad_category_options_id,
			        ac.name AS category,
			        ac.planta,
			        aco.option,
			        aco.hours AS acad_category_hours, a.annual_discount_hours,
			        a.nationality_code AS nationality,
			        a.city
			    FROM academics a
			    LEFT JOIN academic_work_positions wp ON a.work_position_id = wp.id
			    JOIN departments d ON a.department_id = d.id
			    LEFT JOIN careers c ON a.career_id = c.id
			    JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
			    JOIN academic_categories ac ON aco.category_id = ac.id
			    WHERE 1=1",
		);

		if let Some(q) = filter.search {
			let pattern = format!("%{}%", q.trim());

			query
				.push(" AND (a.names ILIKE ")
				.push_bind(pattern.clone())
				.push(" OR a.paternal_surname ILIKE ")
				.push_bind(pattern.clone())
				.push(" OR a.maternal_surname ILIKE ")
				.push_bind(pattern.clone())
				.push(" OR a.email ILIKE ")
				.push_bind(pattern)
				.push(")");
		}

		if let Some(id) = filter.department_id {
			query.push(" AND a.department_id = ").push_bind(id);
		}

		if let Some(id) = filter.career_id {
			query.push(" AND a.career_id = ").push_bind(id);
		}

		if let Some(id) = filter.category_id {
			query.push(" AND aco.category_id = ").push_bind(id);
		}

		if let Some(planta) = filter.planta {
			query.push(" AND ac.planta = ").push_bind(planta);
		}

		if let Some(option) = filter.option {
			query.push(" AND aco.option = ").push_bind(option);
		}

		if filter.include_unlinked != Some(true) {
			query.push(" AND a.left_at IS NULL");
		}

		let items = query
			.build_query_as::<AcademicView>()
			.fetch_all(self.database.pool())
			.await?;

		Ok(items)
	}

	pub async fn find_view_by_id(&self, id: &AcademicId) -> AppResult<Option<AcademicView>> {
		let item = sqlx::query_as::<_, AcademicView>(
			"SELECT a.id, a.names, a.paternal_surname, a.maternal_surname,
			        a.email, a.orcid, a.sex, a.birth_date, a.joined_at, a.left_at,
			        a.work_position_id,
			        wp.name AS work_position,
			        a.department_id,
			        d.name AS department,
			        a.career_id,
			        c.name AS career,
			        a.jce,
			        a.acad_category_options_id,
			        ac.name AS category,
			        ac.planta,
			        aco.option,
			        aco.hours AS acad_category_hours, a.annual_discount_hours,
			        a.nationality_code AS nationality,
			        a.city
			    FROM academics a
			    LEFT JOIN academic_work_positions wp ON a.work_position_id = wp.id
			    JOIN departments d ON a.department_id = d.id
			    LEFT JOIN careers c ON a.career_id = c.id
			    JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
			    JOIN academic_categories ac ON aco.category_id = ac.id
			    WHERE a.id = $1",
		)
		.bind(id)
		.fetch_optional(self.database.pool())
		.await?;

		Ok(item)
	}

	pub async fn find_by_id(&self, id: &AcademicId) -> AppResult<Option<Academic>> {
		let item = sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE id = $1")
			.bind(id)
			.fetch_optional(self.database.pool())
			.await?;

		Ok(item)
	}

	pub async fn find_by_email(&self, email: &str) -> AppResult<Option<Academic>> {
		let item = sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE email = $1")
			.bind(email)
			.fetch_optional(self.database.pool())
			.await?;

		Ok(item)
	}

	pub async fn find_by_rut(&self, rut: &str) -> AppResult<Option<Academic>> {
		let item = sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE rut = $1")
			.bind(rut)
			.fetch_optional(self.database.pool())
			.await?;

		Ok(item)
	}

	pub async fn find_by_orcid(&self, orcid: &str) -> AppResult<Option<Academic>> {
		let item = sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE orcid = $1")
			.bind(orcid)
			.fetch_optional(self.database.pool())
			.await?;

		Ok(item)
	}

	pub async fn mark_unlinked(&self, id: &AcademicId) -> AppResult<bool> {
		let result = sqlx::query(
			"UPDATE academics
			 SET left_at = CURRENT_DATE, updated_at = NOW()
			 WHERE id = $1 AND left_at IS NULL",
		)
		.bind(id)
		.execute(self.database.pool())
		.await?;

		Ok(result.rows_affected() > 0)
	}

	pub async fn save(&self, academic: &Academic) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO academics (
			        id, rut, names, paternal_surname, maternal_surname, email, orcid, sex,
			        birth_date, joined_at, work_position_id,
			        department_id, career_id, jce, acad_category_options_id,
			        annual_discount_hours, nationality_code, city, updated_at
			    ) VALUES (
			        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
			        $11, $12, $13, $14, $15, $16, $17, $18, $19
			    ) ON CONFLICT (id) DO UPDATE SET
			        names                 = EXCLUDED.names,
			        paternal_surname      = EXCLUDED.paternal_surname,
			        maternal_surname      = EXCLUDED.maternal_surname,
			        email                 = EXCLUDED.email,
			        orcid                 = EXCLUDED.orcid,
			        sex                   = EXCLUDED.sex,
			        birth_date            = EXCLUDED.birth_date,
			        joined_at             = EXCLUDED.joined_at,
			        work_position_id      = EXCLUDED.work_position_id,
			        department_id         = EXCLUDED.department_id,
			        career_id             = EXCLUDED.career_id,
			        jce                   = EXCLUDED.jce,
			        acad_category_options_id = EXCLUDED.acad_category_options_id,
			        annual_discount_hours = EXCLUDED.annual_discount_hours,
			        nationality_code      = EXCLUDED.nationality_code,
			        city                  = EXCLUDED.city,
			        updated_at            = NOW()",
		)
		.bind(academic.id)
		.bind(&academic.rut)
		.bind(&academic.names)
		.bind(&academic.paternal_surname)
		.bind(&academic.maternal_surname)
		.bind(&academic.email)
		.bind(&academic.orcid)
		.bind(academic.sex)
		.bind(academic.birth_date)
		.bind(academic.joined_at)
		.bind(academic.work_position_id)
		.bind(academic.department_id)
		.bind(academic.career_id)
		.bind(academic.jce)
		.bind(academic.acad_category_options_id)
		.bind(academic.annual_discount_hours)
		.bind(&academic.nationality_code)
		.bind(&academic.city)
		.bind(academic.updated_at)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn save_tx_upsert(&self, tx: &mut Tx<'_>, academic: &Academic) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO academics (
			        id, rut, names, paternal_surname, maternal_surname, email, orcid, sex,
			        birth_date, joined_at, work_position_id,
			        department_id, career_id, jce, acad_category_options_id,
			        annual_discount_hours, nationality_code, city, updated_at
			    ) VALUES (
			        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
			        $11, $12, $13, $14, $15, $16, $17, $18, $19
			    ) ON CONFLICT (id) DO UPDATE SET
			        rut                   = EXCLUDED.rut,
			        names                 = EXCLUDED.names,
			        paternal_surname      = EXCLUDED.paternal_surname,
			        maternal_surname      = EXCLUDED.maternal_surname,
			        email                 = EXCLUDED.email,
			        orcid                 = EXCLUDED.orcid,
			        sex                   = EXCLUDED.sex,
			        birth_date            = EXCLUDED.birth_date,
			        joined_at             = EXCLUDED.joined_at,
			        work_position_id      = EXCLUDED.work_position_id,
			        department_id         = EXCLUDED.department_id,
			        career_id             = EXCLUDED.career_id,
			        jce                   = EXCLUDED.jce,
			        acad_category_options_id = EXCLUDED.acad_category_options_id,
			        annual_discount_hours = EXCLUDED.annual_discount_hours,
			        nationality_code      = EXCLUDED.nationality_code,
			        city                  = EXCLUDED.city,
			        updated_at            = NOW()",
		)
		.bind(academic.id)
		.bind(&academic.rut)
		.bind(&academic.names)
		.bind(&academic.paternal_surname)
		.bind(&academic.maternal_surname)
		.bind(&academic.email)
		.bind(&academic.orcid)
		.bind(academic.sex)
		.bind(academic.birth_date)
		.bind(academic.joined_at)
		.bind(academic.work_position_id)
		.bind(academic.department_id)
		.bind(academic.career_id)
		.bind(academic.jce)
		.bind(academic.acad_category_options_id)
		.bind(academic.annual_discount_hours)
		.bind(&academic.nationality_code)
		.bind(&academic.city)
		.bind(academic.updated_at)
		.execute(&mut **tx)
		.await?;

		Ok(())
	}
}

#[injectable]
pub struct EditCodesRepository {
	database: Arc<Database>,
}

impl EditCodesRepository {
	pub async fn count_vigentes(&self, academic_id: &AcademicId) -> AppResult<i64> {
		let count = sqlx::query_scalar::<_, i64>(
			"SELECT COUNT(*) FROM academic_edit_codes
			 WHERE academic_id = $1 AND used_at IS NULL",
		)
		.bind(academic_id)
		.fetch_one(self.database.pool())
		.await?;

		Ok(count)
	}

	pub async fn list_vigentes(&self, academic_id: &AcademicId) -> AppResult<Vec<EditCode>> {
		let items = sqlx::query_as::<_, EditCode>(
			"SELECT id, academic_id, code, used_at, created_at
			 FROM academic_edit_codes
			 WHERE academic_id = $1 AND used_at IS NULL
			 ORDER BY created_at",
		)
		.bind(academic_id)
		.fetch_all(self.database.pool())
		.await?;

		Ok(items)
	}

	pub async fn insert_many(&self, codes: &[EditCode]) -> AppResult<()> {
		for code in codes {
			sqlx::query(
				"INSERT INTO academic_edit_codes (id, academic_id, code, created_at)
				 VALUES ($1, $2, $3, $4)",
			)
			.bind(code.id)
			.bind(code.academic_id)
			.bind(&code.code)
			.bind(code.created_at)
			.execute(self.database.pool())
			.await?;
		}

		Ok(())
	}

	pub async fn find_by_code(&self, code: &str) -> AppResult<Option<EditCode>> {
		let item = sqlx::query_as::<_, EditCode>(
			"SELECT id, academic_id, code, used_at, created_at
			 FROM academic_edit_codes
			 WHERE code = $1 AND used_at IS NULL",
		)
		.bind(code)
		.fetch_optional(self.database.pool())
		.await?;

		Ok(item)
	}

	pub async fn mark_used(&self, id: &EditCodeId) -> AppResult<()> {
		sqlx::query("UPDATE academic_edit_codes SET used_at = $1 WHERE id = $2")
			.bind(Utc::now())
			.bind(id)
			.execute(self.database.pool())
			.await?;

		Ok(())
	}
}
