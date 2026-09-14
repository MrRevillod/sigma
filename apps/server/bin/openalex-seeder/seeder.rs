use papers_openalex::{Domain, Field, Subfield, Topic};
use sqlx::PgPool;
use uuid::Uuid;

const UNKNOWN_ID: Uuid = Uuid::from_u128(1);

pub async fn seed_unknown_taxonomy(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
	sqlx::query(
        "INSERT INTO domains (id, openalex_id, name) VALUES ($1, 'unknown', 'Unknown') ON CONFLICT (openalex_id) DO NOTHING",
    )
    .bind(UNKNOWN_ID)
    .execute(pool)
    .await?;

	sqlx::query(
        "INSERT INTO fields (id, openalex_id, name, domain_id) VALUES ($1, 'unknown', 'Unknown', $1) ON CONFLICT (openalex_id) DO NOTHING",
    )
    .bind(UNKNOWN_ID)
    .execute(pool)
    .await?;

	sqlx::query(
        "INSERT INTO subfields (id, openalex_id, name, field_id) VALUES ($1, 'unknown', 'Unknown', $1) ON CONFLICT (openalex_id) DO NOTHING",
    )
    .bind(UNKNOWN_ID)
    .execute(pool)
    .await?;

	sqlx::query(
        "INSERT INTO topics (id, openalex_id, name, subfield_id) VALUES ($1, 'unknown', 'Unknown', $1) ON CONFLICT (openalex_id) DO NOTHING",
    )
    .bind(UNKNOWN_ID)
    .execute(pool)
    .await?;

	sqlx::query(
        "INSERT INTO keywords (id, openalex_id, name) VALUES ($1, 'unknown', 'Unknown') ON CONFLICT (openalex_id) DO NOTHING",
    )
    .bind(UNKNOWN_ID)
    .execute(pool)
    .await?;

	Ok(())
}

pub async fn seed_domains(
	pool: &PgPool,
	domains: &[Domain],
) -> Result<u64, Box<dyn std::error::Error>> {
	let mut inserted = 0u64;

	for domain in domains {
		let name = domain.display_name.as_deref().unwrap_or("Unknown");

		let result = sqlx::query(
			"INSERT INTO domains (openalex_id, name) VALUES ($1, $2) ON CONFLICT (openalex_id) DO NOTHING",
		)
		.bind(&domain.id)
		.bind(name)
		.execute(pool)
		.await?;

		inserted += result.rows_affected();
	}

	Ok(inserted)
}

pub async fn seed_fields(
	pool: &PgPool,
	fields: &[Field],
) -> Result<u64, Box<dyn std::error::Error>> {
	let mut inserted = 0u64;

	for field in fields {
		let name = field.display_name.as_deref().unwrap_or("Unknown");
		let domain_openalex_id = field
			.domain
			.as_ref()
			.and_then(|d| d.id.as_deref())
			.unwrap_or("unknown");

		let result = sqlx::query(
            "INSERT INTO fields (openalex_id, name, domain_id) VALUES ($1, $2, COALESCE((SELECT id FROM domains WHERE openalex_id = $3), $4::uuid)) ON CONFLICT (openalex_id) DO NOTHING",
        )
        .bind(&field.id)
        .bind(name)
        .bind(domain_openalex_id)
        .bind(UNKNOWN_ID)
        .execute(pool)
        .await?;

		inserted += result.rows_affected();
	}

	Ok(inserted)
}

pub async fn seed_subfields(
	pool: &PgPool,
	subfields: &[Subfield],
) -> Result<u64, Box<dyn std::error::Error>> {
	let mut inserted = 0u64;

	for subfield in subfields {
		let name = subfield.display_name.as_deref().unwrap_or("Unknown");
		let field_openalex_id = subfield
			.field
			.as_ref()
			.and_then(|f| f.id.as_deref())
			.unwrap_or("unknown");

		let result = sqlx::query(
            "INSERT INTO subfields (openalex_id, name, field_id) VALUES ($1, $2, COALESCE((SELECT id FROM fields WHERE openalex_id = $3), $4::uuid)) ON CONFLICT (openalex_id) DO NOTHING",
        )
        .bind(&subfield.id)
        .bind(name)
        .bind(field_openalex_id)
        .bind(UNKNOWN_ID)
        .execute(pool)
        .await?;

		inserted += result.rows_affected();
	}

	Ok(inserted)
}

pub async fn seed_topics(
	pool: &PgPool,
	topics: &[Topic],
) -> Result<u64, Box<dyn std::error::Error>> {
	let mut inserted = 0u64;

	for topic in topics {
		let name = topic.display_name.as_deref().unwrap_or("Unknown");
		let subfield_openalex_id = topic
			.subfield
			.as_ref()
			.and_then(|s| extract_hierarchy_id(&s.id))
			.unwrap_or_else(|| "unknown".to_string());

		let result = sqlx::query(
            "INSERT INTO topics (openalex_id, name, subfield_id) VALUES ($1, $2, COALESCE((SELECT id FROM subfields WHERE openalex_id = $3), $4::uuid)) ON CONFLICT (openalex_id) DO NOTHING",
        )
        .bind(&topic.id)
        .bind(name)
        .bind(&subfield_openalex_id)
        .bind(UNKNOWN_ID)
        .execute(pool)
        .await?;

		inserted += result.rows_affected();
	}

	Ok(inserted)
}

fn extract_hierarchy_id(value: &Option<serde_json::Value>) -> Option<String> {
	value.as_ref().and_then(|v| match v {
		serde_json::Value::String(s) => Some(s.clone()),
		serde_json::Value::Number(n) => n.as_i64().map(|i| i.to_string()),
		_ => None,
	})
}
