import * as v from "valibot"

// Academic View DTOs ------------------------------------------------

export interface AcademicDTO {
	id: string
	names: string
	paternalSurname: string
	maternalSurname: string
	email: string
	orcid: string | null
	sex: "H" | "M" | "O"
	birthDate: string
	joinedAt: string
	leftAt: string | null
	workPositionId: string
	workPosition: string | null
	departmentId: string
	department: string
	careerId: string | null
	career: string | null
	jce: number
	acadCategoryOptionsId: string
	category: string
	planta: "adjunta" | "permanente"
	option: "teaching" | "research"
	acadCategoryHours: number | null
	annualDiscountHours: number
	nationality: string
	city: string
}

export interface PublicAcademicDTO {
	id: string
	names: string
	paternalSurname: string
	maternalSurname: string
	email: string
	orcid: string | null
	sex: "H" | "M" | "O"
	birthDate: string
	joinedAt: string
	department: string
	career: string | null
	nationality: string
	city: string
}

// Query Academic DTOs ------------------------------------------------

export interface GetAcademicsParams {
	search?: string
	departmentId?: string
	careerId?: string
	categoryId?: string
	planta?: "adjunta" | "permanente"
	option?: "teaching" | "research"
	includeUnlinked?: boolean
	sort?: AcademicSortField
}

export const ACADEMIC_SORT_FIELD = [
	"names",
	"paternal_surname",
	"maternal_surname",
	"joined_at",
	"birth_date",
] as const

export type AcademicSortField = (typeof ACADEMIC_SORT_FIELD)[number]

// Shared schemas ------------------------------------------------

const ORCID_REGEX = /^https:\/\/orcid\.org\/\d{4}-\d{4}-\d{4}-\d{3}[\dX]$/
const RUT_REGEX = /^\d{7,8}-[\dkK]$/

const normalizeDecimal = (v: unknown) => (typeof v === "string" ? v.replace(",", ".") : v)
const coerceNumber = (v: unknown) => (v === "" ? 0 : Number(v))
const textField = (msg: string) => v.pipe(v.string(), v.minLength(1, msg), v.maxLength(255, msg))

const jceSchema = (jceMax: number) =>
	v.optional(
		v.pipe(
			v.unknown(),
			v.transform(normalizeDecimal),
			v.transform(coerceNumber),
			v.number(),
			v.minValue(0, "La JCE no puede ser negativa"),
			v.maxValue(jceMax, `La JCE no puede superar ${jceMax} horas`),
		),
	)

const annualDiscountHoursSchema = v.optional(
	v.pipe(
		v.unknown(),
		v.transform(normalizeDecimal),
		v.transform(coerceNumber),
		v.number(),
		v.minValue(0, "Las horas de descuento anual no pueden ser negativas"),
	),
)

const requiredNumber = v.pipe(
	v.unknown(),
	v.transform(normalizeDecimal),
	v.transform(coerceNumber),
	v.number(),
)

// Create Academic DTOs ------------------------------------------------

export const createAcademicDTOSchema = (jceMax: number) =>
	v.object({
		rut: v.pipe(v.string(), v.regex(RUT_REGEX, "Formato: XXXXXXXX-X")),
		names: textField("Los nombres deben tener entre 1 y 255 caracteres"),
		paternalSurname: textField("El apellido paterno debe tener entre 1 y 255 caracteres"),
		maternalSurname: textField("El apellido materno debe tener entre 1 y 255 caracteres"),
		email: v.pipe(v.string(), v.email("El email debe ser válido")),
		orcid: v.optional(
			v.nullable(
				v.pipe(
					v.string(),
					v.regex(
						ORCID_REGEX,
						"El ORCID debe ser una URL válida (https://orcid.org/XXXX-XXXX-XXXX-XXXX)",
					),
				),
			),
		),
		sex: v.picklist(["H", "M", "O"], "Seleccione una opción válida"),
		birthDate: v.pipe(v.string(), v.nonEmpty("La fecha de nacimiento es obligatoria")),
		joinedAt: v.pipe(v.string(), v.nonEmpty("La fecha de ingreso es obligatoria")),
		workPositionId: v.pipe(v.string(), v.nonEmpty("Seleccione un cargo")),
		departmentId: v.pipe(v.string(), v.nonEmpty("Seleccione un departamento")),
		careerId: v.optional(v.nullable(v.string())),
		acadCategoryOptionsId: v.pipe(v.string(), v.nonEmpty("Seleccione una opción de categoría")),
		jce: v.pipe(
			requiredNumber,
			v.minValue(0, "La JCE no puede ser negativa"),
			v.maxValue(jceMax, `La JCE no puede superar ${jceMax} horas`),
		),
		annualDiscountHours: v.pipe(
			requiredNumber,
			v.minValue(0, "Las horas de descuento anual no pueden ser negativas"),
		),
		nationalityCode: v.pipe(
			v.string(),
			v.length(2, "El código de país debe tener 2 caracteres"),
		),
		city: textField("La ciudad debe tener entre 1 y 255 caracteres"),
	})

export type CreateAcademicDTO = v.InferInput<ReturnType<typeof createAcademicDTOSchema>>

export const createAcademicDTOInitialInput = {
	rut: "",
	names: "",
	paternalSurname: "",
	maternalSurname: "",
	email: "",
	orcid: null,
	sex: "",
	birthDate: "",
	joinedAt: "",
	workPositionId: "",
	departmentId: "",
	careerId: null,
	acadCategoryOptionsId: "",
	jce: 0,
	annualDiscountHours: 0,
	nationalityCode: "CL",
	city: "",
} as unknown as CreateAcademicDTO

// Update Academic DTOs ------------------------------------------------

export const updateAcademicDTOSchema = (jceMax: number) =>
	v.object({
		names: v.optional(textField("Los nombres deben tener entre 1 y 255 caracteres")),
		paternalSurname: v.optional(
			textField("El apellido paterno debe tener entre 1 y 255 caracteres"),
		),
		maternalSurname: v.optional(
			textField("El apellido materno debe tener entre 1 y 255 caracteres"),
		),
		email: v.optional(v.pipe(v.string(), v.email("El email debe ser válido"))),
		orcid: v.optional(
			v.nullable(
				v.pipe(
					v.string(),
					v.regex(
						ORCID_REGEX,
						"El ORCID debe ser una URL válida (https://orcid.org/XXXX-XXXX-XXXX-XXXX)",
					),
				),
			),
		),
		sex: v.optional(v.picklist(["H", "M", "O"], "Seleccione una opción válida")),
		birthDate: v.optional(v.string()),
		joinedAt: v.optional(v.string()),
		workPositionId: v.optional(v.pipe(v.string(), v.nonEmpty("Seleccione un cargo"))),
		departmentId: v.optional(v.pipe(v.string(), v.nonEmpty("Seleccione un departamento"))),
		careerId: v.optional(
			v.nullable(
				v.pipe(
					v.string(),
					v.transform((s) => (s === "" ? null : s)),
				),
			),
		),
		acadCategoryOptionsId: v.optional(
			v.pipe(v.string(), v.nonEmpty("Seleccione una opción de categoría")),
		),
		city: v.optional(textField("La ciudad debe tener entre 1 y 255 caracteres")),
		nationalityCode: v.optional(
			v.pipe(v.string(), v.length(2, "El código de país debe tener 2 caracteres")),
		),
		jce: jceSchema(jceMax),
		annualDiscountHours: annualDiscountHoursSchema,
	})

export type UpdateAcademicDTO = v.InferInput<ReturnType<typeof updateAcademicDTOSchema>>

// Self-Update Academic DTOs ------------------------------------------------

export const selfUpdateAcademicDTOSchema = v.object({
	names: v.optional(textField("Los nombres deben tener entre 1 y 255 caracteres")),
	paternalSurname: v.optional(
		textField("El apellido paterno debe tener entre 1 y 255 caracteres"),
	),
	maternalSurname: v.optional(
		textField("El apellido materno debe tener entre 1 y 255 caracteres"),
	),
	orcid: v.optional(
		v.nullable(
			v.pipe(
				v.string(),
				v.regex(
					ORCID_REGEX,
					"El ORCID debe ser una URL válida (https://orcid.org/XXXX-XXXX-XXXX-XXXX)",
				),
			),
		),
	),
	sex: v.optional(v.picklist(["H", "M", "O"], "Seleccione una opción válida")),
	birthDate: v.optional(v.string()),
	city: v.optional(textField("La ciudad debe tener entre 1 y 255 caracteres")),
	nationalityCode: v.optional(
		v.pipe(v.string(), v.length(2, "El código de país debe tener 2 caracteres")),
	),
})

export type SelfUpdateAcademicDTOSchema = typeof selfUpdateAcademicDTOSchema
export type SelfUpdateDTO = v.InferInput<typeof selfUpdateAcademicDTOSchema>

export interface SyncResultDTO {
	academicId: string
	orcidWorks: number
	worksWithoutDoi: number
	notFoundInOpenalex: number
	worksCreated: number
	authorshipsInserted: number
	authorshipsUnlinked: number
	topicsLinked: number
	keywordsLinked: number
	errors: string[]
}

// Data Import DTOs ------------------------------------------------

export interface ImportResult {
	imported: number
	updated: number
	errors: ImportRowError[]
}

export interface ImportRowError {
	row: number
	reasons: string[]
}

// Academic detail filter params --------------------------------------

const yearFromDefault = String(new Date().getFullYear() - 5)

export const academicFiltersParamsDTOSchema = v.object({
	yearFrom: v.optional(v.fallback(v.string(), yearFromDefault), yearFromDefault),
	yearTo: v.optional(v.fallback(v.string(), ""), ""),
	researchLineId: v.optional(v.fallback(v.string(), ""), ""),
	journalKind: v.optional(v.fallback(v.string(), ""), ""),
})
