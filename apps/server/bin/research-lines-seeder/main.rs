use std::error::Error;

const MAPPINGS: &[(&str, &str)] = &[
	// IA, Sistemas Complejos y Modelamiento Matemático
	(
		"https://openalex.org/subfields/1203",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1302",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1304",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1306",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1311",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1315",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1702",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1703",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1704",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1705",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1706",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1707",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1708",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1709",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1710",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1711",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1712",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1800",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1802",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1803",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/1804",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2206",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2207",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2208",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2214",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2602",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2604",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2605",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2607",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2608",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2610",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2611",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2612",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2613",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2614",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2716",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2718",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2728",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2802",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2803",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2804",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2805",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2806",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2807",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2808",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/2809",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3109",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3200",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3202",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3203",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3206",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3207",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3307",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3309",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3310",
		"ia-sistemas-complejos",
	),
	(
		"https://openalex.org/subfields/3315",
		"ia-sistemas-complejos",
	),
	// Materiales Avanzados y Bioproductos
	(
		"https://openalex.org/subfields/1106",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1303",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1305",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1307",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1308",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1312",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1313",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1502",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1503",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1504",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1506",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1507",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1508",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1602",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1603",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1604",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1605",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1606",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/1607",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2202",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2203",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2204",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2209",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2210",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2211",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2402",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2403",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2404",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2405",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2406",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2500",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2502",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2503",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2504",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2505",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2506",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2507",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2508",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2704",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2723",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2725",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2726",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/2736",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3002",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3003",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3004",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3102",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3104",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3105",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3106",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3107",
		"materiales-avanzados",
	),
	(
		"https://openalex.org/subfields/3611",
		"materiales-avanzados",
	),
	// Ciencias de la Tierra
	("https://openalex.org/subfields/1104", "ciencias-tierra"),
	("https://openalex.org/subfields/1111", "ciencias-tierra"),
	("https://openalex.org/subfields/1204", "ciencias-tierra"),
	("https://openalex.org/subfields/1902", "ciencias-tierra"),
	("https://openalex.org/subfields/1904", "ciencias-tierra"),
	("https://openalex.org/subfields/1906", "ciencias-tierra"),
	("https://openalex.org/subfields/1907", "ciencias-tierra"),
	("https://openalex.org/subfields/1908", "ciencias-tierra"),
	("https://openalex.org/subfields/1910", "ciencias-tierra"),
	("https://openalex.org/subfields/1911", "ciencias-tierra"),
	("https://openalex.org/subfields/1912", "ciencias-tierra"),
	("https://openalex.org/subfields/2212", "ciencias-tierra"),
	("https://openalex.org/subfields/3103", "ciencias-tierra"),
	("https://openalex.org/subfields/3302", "ciencias-tierra"),
	("https://openalex.org/subfields/3305", "ciencias-tierra"),
	// Sostenibilidad
	("https://openalex.org/subfields/1100", "sostenibilidad"),
	("https://openalex.org/subfields/1102", "sostenibilidad"),
	("https://openalex.org/subfields/1103", "sostenibilidad"),
	("https://openalex.org/subfields/1105", "sostenibilidad"),
	("https://openalex.org/subfields/1107", "sostenibilidad"),
	("https://openalex.org/subfields/1108", "sostenibilidad"),
	("https://openalex.org/subfields/1109", "sostenibilidad"),
	("https://openalex.org/subfields/1110", "sostenibilidad"),
	("https://openalex.org/subfields/1206", "sostenibilidad"),
	("https://openalex.org/subfields/1402", "sostenibilidad"),
	("https://openalex.org/subfields/1403", "sostenibilidad"),
	("https://openalex.org/subfields/1404", "sostenibilidad"),
	("https://openalex.org/subfields/1405", "sostenibilidad"),
	("https://openalex.org/subfields/1406", "sostenibilidad"),
	("https://openalex.org/subfields/1407", "sostenibilidad"),
	("https://openalex.org/subfields/1408", "sostenibilidad"),
	("https://openalex.org/subfields/1409", "sostenibilidad"),
	("https://openalex.org/subfields/1410", "sostenibilidad"),
	("https://openalex.org/subfields/2000", "sostenibilidad"),
	("https://openalex.org/subfields/2002", "sostenibilidad"),
	("https://openalex.org/subfields/2003", "sostenibilidad"),
	("https://openalex.org/subfields/2100", "sostenibilidad"),
	("https://openalex.org/subfields/2102", "sostenibilidad"),
	("https://openalex.org/subfields/2103", "sostenibilidad"),
	("https://openalex.org/subfields/2104", "sostenibilidad"),
	("https://openalex.org/subfields/2105", "sostenibilidad"),
	("https://openalex.org/subfields/2205", "sostenibilidad"),
	("https://openalex.org/subfields/2213", "sostenibilidad"),
	("https://openalex.org/subfields/2215", "sostenibilidad"),
	("https://openalex.org/subfields/2216", "sostenibilidad"),
	("https://openalex.org/subfields/2302", "sostenibilidad"),
	("https://openalex.org/subfields/2303", "sostenibilidad"),
	("https://openalex.org/subfields/2304", "sostenibilidad"),
	("https://openalex.org/subfields/2305", "sostenibilidad"),
	("https://openalex.org/subfields/2306", "sostenibilidad"),
	("https://openalex.org/subfields/2307", "sostenibilidad"),
	("https://openalex.org/subfields/2308", "sostenibilidad"),
	("https://openalex.org/subfields/2309", "sostenibilidad"),
	("https://openalex.org/subfields/2310", "sostenibilidad"),
	("https://openalex.org/subfields/2311", "sostenibilidad"),
	("https://openalex.org/subfields/2312", "sostenibilidad"),
	("https://openalex.org/subfields/2707", "sostenibilidad"),
	("https://openalex.org/subfields/2713", "sostenibilidad"),
	("https://openalex.org/subfields/2714", "sostenibilidad"),
	("https://openalex.org/subfields/2717", "sostenibilidad"),
	("https://openalex.org/subfields/2735", "sostenibilidad"),
	("https://openalex.org/subfields/2738", "sostenibilidad"),
	("https://openalex.org/subfields/2739", "sostenibilidad"),
	("https://openalex.org/subfields/2740", "sostenibilidad"),
	("https://openalex.org/subfields/2742", "sostenibilidad"),
	("https://openalex.org/subfields/2910", "sostenibilidad"),
	("https://openalex.org/subfields/2911", "sostenibilidad"),
	("https://openalex.org/subfields/2916", "sostenibilidad"),
	("https://openalex.org/subfields/2922", "sostenibilidad"),
	("https://openalex.org/subfields/3005", "sostenibilidad"),
	("https://openalex.org/subfields/3108", "sostenibilidad"),
	("https://openalex.org/subfields/3300", "sostenibilidad"),
	("https://openalex.org/subfields/3303", "sostenibilidad"),
	("https://openalex.org/subfields/3306", "sostenibilidad"),
	("https://openalex.org/subfields/3308", "sostenibilidad"),
	("https://openalex.org/subfields/3311", "sostenibilidad"),
	("https://openalex.org/subfields/3312", "sostenibilidad"),
	("https://openalex.org/subfields/3313", "sostenibilidad"),
	("https://openalex.org/subfields/3314", "sostenibilidad"),
	("https://openalex.org/subfields/3316", "sostenibilidad"),
	("https://openalex.org/subfields/3317", "sostenibilidad"),
	("https://openalex.org/subfields/3318", "sostenibilidad"),
	("https://openalex.org/subfields/3319", "sostenibilidad"),
	("https://openalex.org/subfields/3320", "sostenibilidad"),
	("https://openalex.org/subfields/3321", "sostenibilidad"),
	("https://openalex.org/subfields/3322", "sostenibilidad"),
	("https://openalex.org/subfields/3600", "sostenibilidad"),
	("https://openalex.org/subfields/3603", "sostenibilidad"),
	("https://openalex.org/subfields/3604", "sostenibilidad"),
	("https://openalex.org/subfields/3605", "sostenibilidad"),
	("https://openalex.org/subfields/3607", "sostenibilidad"),
	("https://openalex.org/subfields/3608", "sostenibilidad"),
	("https://openalex.org/subfields/3609", "sostenibilidad"),
	("https://openalex.org/subfields/3612", "sostenibilidad"),
	("https://openalex.org/subfields/3614", "sostenibilidad"),
	("https://openalex.org/subfields/3616", "sostenibilidad"),
	// Educación en Ingeniería
	(
		"https://openalex.org/subfields/1200",
		"educacion-ingenieria",
	),
	(
		"https://openalex.org/subfields/1202",
		"educacion-ingenieria",
	),
	(
		"https://openalex.org/subfields/1207",
		"educacion-ingenieria",
	),
	(
		"https://openalex.org/subfields/1211",
		"educacion-ingenieria",
	),
	(
		"https://openalex.org/subfields/2200",
		"educacion-ingenieria",
	),
	(
		"https://openalex.org/subfields/3204",
		"educacion-ingenieria",
	),
	(
		"https://openalex.org/subfields/3205",
		"educacion-ingenieria",
	),
	(
		"https://openalex.org/subfields/3304",
		"educacion-ingenieria",
	),
	// Sin Asignar
	("https://openalex.org/subfields/1205", "sin-asignar"),
	("https://openalex.org/subfields/1208", "sin-asignar"),
	("https://openalex.org/subfields/1209", "sin-asignar"),
	("https://openalex.org/subfields/1210", "sin-asignar"),
	("https://openalex.org/subfields/1212", "sin-asignar"),
	("https://openalex.org/subfields/1213", "sin-asignar"),
	("https://openalex.org/subfields/1309", "sin-asignar"),
	("https://openalex.org/subfields/1310", "sin-asignar"),
	("https://openalex.org/subfields/1314", "sin-asignar"),
	("https://openalex.org/subfields/2702", "sin-asignar"),
	("https://openalex.org/subfields/2703", "sin-asignar"),
	("https://openalex.org/subfields/2705", "sin-asignar"),
	("https://openalex.org/subfields/2706", "sin-asignar"),
	("https://openalex.org/subfields/2708", "sin-asignar"),
	("https://openalex.org/subfields/2711", "sin-asignar"),
	("https://openalex.org/subfields/2712", "sin-asignar"),
	("https://openalex.org/subfields/2715", "sin-asignar"),
	("https://openalex.org/subfields/2720", "sin-asignar"),
	("https://openalex.org/subfields/2721", "sin-asignar"),
	("https://openalex.org/subfields/2724", "sin-asignar"),
	("https://openalex.org/subfields/2727", "sin-asignar"),
	("https://openalex.org/subfields/2729", "sin-asignar"),
	("https://openalex.org/subfields/2730", "sin-asignar"),
	("https://openalex.org/subfields/2731", "sin-asignar"),
	("https://openalex.org/subfields/2732", "sin-asignar"),
	("https://openalex.org/subfields/2733", "sin-asignar"),
	("https://openalex.org/subfields/2734", "sin-asignar"),
	("https://openalex.org/subfields/2737", "sin-asignar"),
	("https://openalex.org/subfields/2741", "sin-asignar"),
	("https://openalex.org/subfields/2743", "sin-asignar"),
	("https://openalex.org/subfields/2745", "sin-asignar"),
	("https://openalex.org/subfields/2746", "sin-asignar"),
	("https://openalex.org/subfields/2747", "sin-asignar"),
	("https://openalex.org/subfields/2748", "sin-asignar"),
	("https://openalex.org/subfields/3402", "sin-asignar"),
	("https://openalex.org/subfields/3404", "sin-asignar"),
	("https://openalex.org/subfields/3500", "sin-asignar"),
	("https://openalex.org/subfields/3504", "sin-asignar"),
	("https://openalex.org/subfields/3505", "sin-asignar"),
	("https://openalex.org/subfields/3506", "sin-asignar"),
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let database_url = std::env::var("LOCAL_POSTGRES_DATABASE_URL")
		.or_else(|_| std::env::var("POSTGRES_DATABASE_URL"))?;

	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&database_url)
		.await?;

	eprintln!("Connected to database");

	let mut updated = 0u64;

	for (openalex_id, slug) in MAPPINGS {
		let result = sqlx::query(
			"UPDATE subfields
			    SET research_line_id = (SELECT id FROM research_lines WHERE slug = $1)
			    WHERE openalex_id = $2",
		)
		.bind(slug)
		.bind(openalex_id)
		.execute(&pool)
		.await?;

		updated += result.rows_affected();
	}

	eprintln!(
		"Done — {updated} subfields updated ({} total)",
		MAPPINGS.len()
	);

	Ok(())
}
