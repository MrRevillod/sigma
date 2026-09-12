mod create;
mod imports;
mod self_update;
mod token;
mod update;

pub use create::*;
pub use imports::*;
pub use self_update::*;
pub use token::*;
pub use update::*;

use crate::{
	academic::{AcademicCategoryId, AcademicOption, AcademicPlanta},
	university::{CareerId, DepartmentId},
};

use chrono::{NaiveDate, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static RUT_REGEX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"^\d{7,8}-[\dkK]$").expect("regex inválida"));

static ORCID_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"^https://orcid\.org/\d{4}-\d{4}-\d{4}-\d{3}[\dX]$").expect("regex inválida")
});

static UCT_FOUNDATION_DATE: NaiveDate = NaiveDate::from_ymd_opt(1959, 9, 8).unwrap();

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAcademicsQuery {
	#[validate(length(
		min = 1,
		max = 255,
		message = "El atributo 'search' debe tener entre 1 y 255 caracteres"
	))]
	pub search: Option<String>,
	pub career_id: Option<CareerId>,
	pub department_id: Option<DepartmentId>,
	pub category_id: Option<AcademicCategoryId>,
	pub planta: Option<AcademicPlanta>,
	pub option: Option<AcademicOption>,
	pub include_unlinked: Option<bool>,
}

fn validate_birth_date(date: &NaiveDate) -> Result<(), ValidationError> {
	validate_future_date(date, "La fecha de nacimiento no puede ser en el futuro")
}

fn validate_joined_at(joined_at: &NaiveDate) -> Result<(), ValidationError> {
	validate_future_date(joined_at, "La fecha de ingreso no puede ser en el futuro")?;

	if *joined_at < UCT_FOUNDATION_DATE {
		return Err(ValidationError::new(
			"La fecha de ingreso no puede ser anterior al año de fundación de la universidad (1959)",
		));
	};

	Ok(())
}

fn validate_future_date(date: &NaiveDate, message: &'static str) -> Result<(), ValidationError> {
	if *date > Utc::now().naive_utc().date() {
		Err(ValidationError::new(message))
	} else {
		Ok(())
	}
}
