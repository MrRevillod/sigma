import type { ProductivityDegree, ProductivityJceScope, ProductivityScope } from "./dtos"

export type ProductivityPrecision = "auto" | "0" | "1" | "2" | "3" | "4" | "5"

export interface ProductivitySectionProps {
	denominator: string
	degree: ProductivityDegree
	scope: ProductivityScope
	jceScope?: ProductivityJceScope
	departmentId?: string
	researchLineId?: string
	yearFrom: number
	yearTo: number
}

export const degreeItems = [
	{ value: "all", label: "Todas" },
	{ value: "doctor", label: "Doctores" },
	{ value: "magister", label: "Magísteres" },
]

export const jceScopeItems = [
	{ value: "doctor", label: "Solo doctores" },
	{ value: "all", label: "Todos los académicos del alcance" },
]

export const monthNames = [
	"Enero",
	"Febrero",
	"Marzo",
	"Abril",
	"Mayo",
	"Junio",
	"Julio",
	"Agosto",
	"Septiembre",
	"Octubre",
	"Noviembre",
	"Diciembre",
]

export const monthItems = monthNames.map((label, i) => ({ value: String(i + 1), label }))

export const degreePhrases: Record<ProductivityDegree, string> = {
	all: "Publicaciones",
	doctor: "Publicaciones de autores con grado de doctor",
	magister: "Publicaciones de autores con grado de magíster",
}

export const jceScopePhrases: Record<ProductivityJceScope, string> = {
	doctor: "la JCE de los doctores",
	all: "la JCE de todos los académicos",
}

export function buildProductivityDescription(
	degree: ProductivityDegree,
	jceScope: ProductivityJceScope,
	denominator: string,
): string {
	return `${degreePhrases[degree]} por año, divididas por ${jceScopePhrases[jceScope]} ${denominator}. La JCE se calcula por año con los académicos vinculados ese año.`
}

export type ProductivityIndexation = "all" | "wos" | "scopus"

export const indexationItems = [
	{ value: "all", label: "Ambas" },
	{ value: "wos", label: "WoS" },
	{ value: "scopus", label: "Scopus" },
]

const precisionLabels: Record<Exclude<ProductivityPrecision, "auto">, string> = {
	"0": "0 decimales",
	"1": "1 decimal",
	"2": "2 decimales",
	"3": "3 decimales",
	"4": "4 decimales",
	"5": "5 decimales",
}

export const precisionItems = [
	{ value: "auto", label: "Auto" },
	...Object.entries(precisionLabels).map(([value, label]) => ({ value, label })),
]
