use sqlx::PgPool;

const CHUNK_SIZE: usize = 500;

pub async fn seed_records(
	pool: &PgPool,
	records: &[String],
	kind: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let mut total_affected = 0u64;

	for chunk in records.chunks(CHUNK_SIZE) {
		let mut tx = pool.begin().await?;

		for issn in chunk {
			total_affected += upsert_one(&mut tx, issn, kind).await?;
		}

		tx.commit().await?;
	}

	Ok(total_affected)
}

async fn upsert_one(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	issn: &str,
	kind: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let result = match kind {
		"wos" => {
			sqlx::query(
				"INSERT INTO journal_issn (issn, kind) VALUES ($1, 'wos'::journal_kind)
				 ON CONFLICT (issn) DO UPDATE SET kind = EXCLUDED.kind",
			)
			.bind(issn)
			.execute(&mut **tx)
			.await?
		}
		"scopus" => {
			sqlx::query(
				"INSERT INTO journal_issn (issn, kind) VALUES ($1, 'scopus'::journal_kind)
				 ON CONFLICT (issn) DO NOTHING",
			)
			.bind(issn)
			.execute(&mut **tx)
			.await?
		}
		_ => return Ok(0),
	};

	Ok(result.rows_affected())
}
