use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum AcademicError {
	// Academic category errors
	#[http(code = 404, message = "Categoría académica no encontrada")]
	#[error("Categoría académica no encontrada")]
	CategoryNotFound,

	#[http(code = 404, message = "Opción de categoría no encontrada")]
	#[error("Opción de categoría no encontrada")]
	CategoryOptionNotFound,

	#[http(
		code = 400,
		message = "La opción de categoría no coincide con la planta académica"
	)]
	#[error("La opción de categoría no coincide con la planta académica")]
	CategoryPlantaMismatch,

	#[http(
		code = 400,
		message = "La opción de categoría no coincide con la categoría académica"
	)]
	#[error("La opción de categoría no coincide con la categoría académica")]
	CategoryOptionCategoryMismatch,

	#[http(
		code = 400,
		message = "La opción de categoría no coincide con la categoría académica"
	)]
	#[error("La opción de categoría no coincide con la categoría académica")]
	CategoryOptionMismatch,

	#[http(
		code = 400,
		message = "Las horas de la opción de categoría no coinciden con las horas proporcionadas"
	)]
	#[error("Las horas de la opción de categoría no coinciden con las horas proporcionadas")]
	CategoryOptionHoursMismatch,

	#[http(code = 404, message = "Grado académico no encontrado")]
	#[error("Grado académico no encontrado")]
	DegreeNotFound,

	#[http(
		code = 409,
		message = "El académico ya tiene asignado un grado superior (magíster o doctor)"
	)]
	#[error("El académico ya tiene asignado un grado superior (magíster o doctor)")]
	DegreeSuperiorAlreadyExists,

	#[http(code = 409, message = "Ya existe un académico con el mismo RUT")]
	#[error("Ya existe un académico con el mismo RUT")]
	AcademicRutAlreadyExists,

	#[http(code = 409, message = "Ya existe un académico con el mismo ORCID")]
	#[error("Ya existe un académico con el mismo ORCID")]
	AcademicOrcidAlreadyExists,

	#[http(code = 409, message = "Ya existe un académico con el mismo email")]
	#[error("Ya existe un académico con el mismo email")]
	AcademicEmailAlreadyExists,

	#[http(code = 404, message = "Académico no encontrado")]
	#[error("Académico no encontrado")]
	AcademicNotFound,

	#[http(code = 409, message = "El académico ya está desvinculado")]
	#[error("El académico ya está desvinculado")]
	AcademicAlreadyUnlinked,

	#[http(
		code = 409,
		message = "El académico está desvinculado y no puede realizar esta acción"
	)]
	#[error("El académico está desvinculado y no puede realizar esta acción")]
	AcademicUnlinked,

	#[http(
		code = 400,
		message = "La JCE no puede superar el valor máximo configurado"
	)]
	#[error("La JCE no puede superar el valor máximo configurado")]
	JceExceedsMax,

	#[http(
		code = 400,
		message = "El código de autorización es inválido o ya fue utilizado"
	)]
	#[error("El código de autorización es inválido o ya fue utilizado")]
	InvalidEditCode,

	#[http(
		code = 401,
		message = "El enlace de actualización es inválido o ha expirado"
	)]
	#[error("El enlace de actualización es inválido o ha expirado")]
	InvalidOneTimeToken,

	#[http(
		code = 403,
		message = "Solo el autor de esta publicación puede editarla"
	)]
	#[error("Solo el autor de esta publicación puede editarla")]
	NotLocalAuthor,
}
