use csv::ReaderBuilder;
use std::path::Path;

fn normalize(value: &str) -> Option<String> {
	let cleaned = value.trim().replace('-', "").to_uppercase();

	if cleaned.is_empty() {
		None
	} else {
		Some(cleaned)
	}
}

pub fn read_csv(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let mut reader = ReaderBuilder::new()
		.has_headers(true)
		.trim(csv::Trim::All)
		.from_path(path)?;

	let mut records = Vec::new();

	for result in reader.records() {
		let row = result?;

		if let Some(issn) = normalize(&row[0]) {
			records.push(issn);
		}
		if let Some(eissn) = normalize(&row[1]) {
			records.push(eissn);
		}
	}

	Ok(records)
}
