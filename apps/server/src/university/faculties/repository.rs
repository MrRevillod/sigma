use crate::shared::{AppResult, Database};
use crate::university::faculties::Faculty;
use crate::university::{FacultyFilter, FacultyId};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct FacultiesRepository {
	database: Arc<Database>,
}

impl FacultiesRepository {
	pub async fn list(&self, filter: FacultyFilter) -> AppResult<Vec<Faculty>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT id, name FROM faculties WHERE 1=1");

		if let Some(n) = filter.name {
			let pattern = format!("%{}%", n.trim());

			query
				.push(" AND (name ILIKE ")
				.push_bind(pattern.clone())
				.push(")");
		}

		let faculties = query
			.build_query_as::<Faculty>()
			.fetch_all(self.database.pool())
			.await?;

		Ok(faculties)
	}

	pub async fn find_by_id(&self, id: &FacultyId) -> AppResult<Option<Faculty>> {
		let item = sqlx::query_as::<_, Faculty>("SELECT id, name FROM faculties WHERE id = $1")
			.bind(id)
			.fetch_optional(self.database.pool())
			.await?;

		Ok(item)
	}

	pub async fn save(&self, faculty: &Faculty) -> AppResult<()> {
		sqlx::query("INSERT INTO faculties (id, name) VALUES ($1, $2)")
			.bind(faculty.id)
			.bind(&faculty.name)
			.execute(self.database.pool())
			.await?;

		Ok(())
	}
}
