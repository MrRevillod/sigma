use crate::academic::*;
use crate::shared::{AppResult, Database};

use sqlx::types::chrono::Utc;
use std::sync::Arc;
use sword::prelude::*;

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
