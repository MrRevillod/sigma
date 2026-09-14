use super::*;
use crate::academic::{AcademicOption, AcademicPlanta, DegreeKind, Sex};
use crate::shared::{CLf64, Country};

use chrono::NaiveDate;
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AcademicImportRowDto {
	#[validate(regex(
        path = *RUT_REGEX,
        message = "El RUT debe tener el formato XXXXXXXX-X"
    ))]
	#[serde(rename = "RUT")]
	pub rut: String,

	#[validate(length(
		min = 1,
		max = 255,
		message = "Los nombres deben tener entre 1 y 255 caracteres"
	))]
	#[serde(rename = "NOMBRES")]
	pub names: String,

	#[validate(length(
		min = 1,
		max = 255,
		message = "El apellido paterno debe tener entre 1 y 255 caracteres"
	))]
	#[serde(rename = "APELLIDO PATERNO")]
	pub paternal_surname: String,

	#[validate(length(
		min = 1,
		max = 255,
		message = "El apellido materno debe tener entre 1 y 255 caracteres"
	))]
	#[serde(rename = "APELLIDO MATERNO")]
	pub maternal_surname: String,

	#[validate(email(message = "El email debe ser válido"))]
	#[serde(rename = "CORREO")]
	pub email: String,

	#[serde(rename = "ORCID")]
	pub orcid: Option<String>,

	#[serde(rename = "SEXO")]
	pub sex: Sex,

	#[validate(custom(function = "validate_birth_date"))]
	#[serde(rename = "FECHA DE NACIMIENTO")]
	pub birth_date: NaiveDate,

	#[validate(custom(function = "validate_joined_at"))]
	#[serde(rename = "FECHA DE INGRESO")]
	pub joined_at: NaiveDate,

	#[serde(rename = "CARGO")]
	pub work_position_name: String,

	#[serde(rename = "DEPARTAMENTO")]
	pub department_name: String,

	#[serde(rename = "CARRERA")]
	#[serde(default)]
	pub career_name: Option<String>,

	#[serde(rename = "PLANTA")]
	pub planta: ImportedAcademicPlanta,

	#[serde(rename = "CATEGORIA")]
	pub category_name: String,

	#[serde(rename = "OPCION")]
	pub option: ImportedAcademicOption,

	#[serde(rename = "JCE")]
	pub jce: CLf64,

	#[serde(rename = "HRS DD CATEGORIA/OPCION")]
	pub acad_category_hours: CLf64,

	#[serde(rename = "HRS DD DESC PROM ANUAL")]
	pub annual_discount_hours: CLf64,

	#[serde(rename = "PAIS DE NACIONALIDAD")]
	pub nationality_country: Country,

	#[validate(length(
		min = 1,
		max = 255,
		message = "La ciudad debe tener entre 1 y 255 caracteres"
	))]
	#[serde(rename = "CIUDAD")]
	pub city: String,

	#[serde(rename = "TITULO PROFESIONAL")]
	#[serde(default)]
	pub degree_1_name: Option<String>,

	#[serde(rename = "UNIVERSIDAD (I)")]
	#[serde(default)]
	pub degree_1_university: Option<String>,

	#[serde(rename = "FECHA (I)")]
	#[serde(default)]
	pub degree_1_date: Option<NaiveDate>,

	#[serde(rename = "PAIS (I)")]
	#[serde(default)]
	pub degree_1_country: Option<Country>,

	#[serde(rename = "GRADO ACADEMICO")]
	#[serde(default)]
	pub degree_2_name: Option<String>,

	#[serde(rename = "UNIVERSIDAD (II)")]
	#[serde(default)]
	pub degree_2_university: Option<String>,

	#[serde(rename = "FECHA (II)")]
	#[serde(default)]
	pub degree_2_date: Option<NaiveDate>,

	#[serde(rename = "PAIS (II)")]
	#[serde(default)]
	pub degree_2_country: Option<Country>,

	#[serde(rename = "TIPO DE GRADO")]
	#[serde(default)]
	pub degree_2_kind: Option<ImportedDegreeKind>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ImportedAcademicOption {
	#[serde(rename = "Docencia")]
	Teaching,
	#[serde(rename = "Investigación")]
	Research,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ImportedDegreeKind {
	#[serde(rename = "Magister")]
	Magister,
	#[serde(rename = "Doctor")]
	Doctor,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ImportedAcademicPlanta {
	#[serde(rename = "Adjunta")]
	Adjunta,
	#[serde(rename = "Permanente")]
	Permanente,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
	pub imported: usize,
	pub updated: usize,
	pub errors: Vec<ImportRowError>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportRowError {
	pub row: usize,
	pub reasons: Vec<String>,
}

impl ImportRowError {
	pub fn from_validation(row: usize, errors: &ValidationErrors) -> Self {
		let reasons = errors
			.field_errors()
			.iter()
			.flat_map(|(field, errs)| {
				errs.iter().map(move |e| {
					let msg = e
						.message
						.as_ref()
						.map(ToString::to_string)
						.unwrap_or_default();

					format!("{field}: {msg}")
				})
			})
			.collect();

		Self { row, reasons }
	}
}

impl From<&ImportedAcademicOption> for AcademicOption {
	fn from(value: &ImportedAcademicOption) -> Self {
		match value {
			ImportedAcademicOption::Teaching => AcademicOption::Teaching,
			ImportedAcademicOption::Research => AcademicOption::Research,
		}
	}
}

impl From<&ImportedDegreeKind> for DegreeKind {
	fn from(value: &ImportedDegreeKind) -> Self {
		match value {
			ImportedDegreeKind::Magister => DegreeKind::Magister,
			ImportedDegreeKind::Doctor => DegreeKind::Doctor,
		}
	}
}

impl From<&ImportedAcademicPlanta> for AcademicPlanta {
	fn from(value: &ImportedAcademicPlanta) -> Self {
		match value {
			ImportedAcademicPlanta::Adjunta => AcademicPlanta::Adjunta,
			ImportedAcademicPlanta::Permanente => AcademicPlanta::Permanente,
		}
	}
}
