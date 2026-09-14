use std::error::Error;
use std::path::Path;

mod reader;
mod seeder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let database_url = std::env::var("LOCAL_POSTGRES_DATABASE_URL")
		.or_else(|_| std::env::var("POSTGRES_DATABASE_URL"))?;

	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&database_url)
		.await?;

	eprintln!("Connected to database");

	let data_dir = Path::new("data/ISSN");

	if !data_dir.join("SCOPUS.csv").exists()
		&& let Ok(zip_url) = std::env::var("ISSN_ZIP_URL")
	{
		eprintln!("Downloading ISSN CSVs from: {zip_url}");

		std::fs::create_dir_all(data_dir)?;

		let response = reqwest::get(&zip_url).await?;
		let bytes = response.bytes().await?;

		let mut reader = std::io::Cursor::new(&bytes);
		let mut archive = zip::ZipArchive::new(&mut reader)?;

		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			let out_path = data_dir.join(file.name());
			let mut out = std::fs::File::create(&out_path)?;

			std::io::copy(&mut file, &mut out)?;
		}

		eprintln!("  Extracted {} files", archive.len());
	}

	let files: &[(&str, &str)] = &[
		("WoS-AHCI.csv", "wos"),
		("WoS-ESCI.csv", "wos"),
		("WoS-SCIE.csv", "wos"),
		("WoS-SSCI.csv", "wos"),
		("SCOPUS.csv", "scopus"),
	];

	for (filename, kind) in files {
		let path = data_dir.join(filename);

		if !path.exists() {
			eprintln!("Skipping {filename} (file not found)");
			continue;
		}

		eprintln!("Reading {filename}...");

		let records = reader::read_csv(&path)?;

		eprintln!("  {} records loaded, inserting...", records.len());

		let affected = seeder::seed_records(&pool, &records, kind).await?;

		eprintln!("  Done — {} rows affected", affected);
	}

	eprintln!("\nSeeding complete.");

	Ok(())
}
