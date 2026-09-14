use crate::university::DepartmentId;

use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum StatsError {
	#[http(code = 404, message = "Departamento no encontrado")]
	#[error("Department not found: {0}")]
	DepartmentNotFound(DepartmentId),

	#[http(code = 400, message = "Faltan parámetros para el alcance seleccionado")]
	#[error("Invalid scope parameters for productivity")]
	InvalidScopeParams,
}
