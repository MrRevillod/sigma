import * as v from "valibot"

import type { JournalKind } from "./value-objects/journal-kind.value"
import type { AuthorshipPosition } from "./value-objects/position.value"

export interface WorkOverridesDTO {
	title?: string | null
	abstractText?: string | null
	doi?: string | null
	publicationYear?: number | null
	isAccepted?: boolean | null
	isPublished?: boolean | null
	researchLineId?: string | null
	correspondingOrcid?: string | null
	journalKind?: JournalKind | null
}

export interface WorkDTO {
	id: string
	openalexId: string
	title: string
	abstractText: string | null
	doi: string | null
	publicationDate: string | null
	publicationYear: number | null
	ty: string
	lang: string
	isAccepted: boolean
	isPublished: boolean
	sourceId: string | null
	journalKind: string | null
	researchLineId?: string | null
	researchLineName?: string | null
	overrides?: WorkOverridesDTO
	source?: SourceDTO | null
	authorships?: AuthorshipDTO[]
	topics?: WorkTopicDTO[]
	keywords?: WorkKeywordDTO[]
}

export const WORK_TYPE_LABELS: Record<string, string> = {
	"article": "Artículo",
	"book": "Libro",
	"book-chapter": "Capítulo de libro",
	"book-review": "Reseña de libro",
	"conference-abstract": "Abstract de conferencia",
	"conference-paper": "Paper de conferencia",
	"data-paper": "Paper de datos",
	"dissertation": "Tesis",
	"editorial": "Editorial",
	"erratum": "Errata",
	"letter": "Carta",
	"libguide": "Guía",
	"other": "Otro",
	"paratext": "Paratexto",
	"peer-review": "Revisión por pares",
	"preprint": "Preprint",
	"reference-entry": "Entrada de referencia",
	"report": "Reporte",
	"retraction": "Retracción",
	"review": "Revisión",
	"software": "Software",
	"software-paper": "Paper de software",
	"standard": "Estándar",
	"supplementary-materials": "Materiales suplementarios",
}

export interface SourceDTO {
	id: string
	openalexId: string
	name: string
	ty: string
	issn: string | null
	kind: JournalKind | null
}

export interface AuthorshipDTO {
	orcid: string
	name: string
	isExternal: boolean
	isCorresponding: boolean
	affiliations: string[]
	position: AuthorshipPosition
	academicId?: string
}

export interface WorkTopicDTO {
	topicId: string
	name: string
	score: number
	subfieldId: string
	subfieldName: string
	fieldId: string
	fieldName: string
	domainId: string
	domainName: string
}

export interface WorkKeywordDTO {
	keywordId: string
	name: string
	score: number
}

/** Detail response is the same shape; relations are present when loaded. */
export type WorkDetailDTO = WorkDTO

export interface GetWorksParams {
	academicId?: string
	search?: string
	yearFrom?: number
	yearTo?: number
	isAccepted?: boolean
	isPublished?: boolean
	departmentId?: string
	careerId?: string
	size?: number
	journalKind?: JournalKind
	researchLineId?: string
}

export interface WorkOverridesInput {
	title?: string | null
	abstractText?: string | null
	doi?: string | null
	publicationYear?: number | null
	isAccepted?: boolean | null
	isPublished?: boolean | null
	researchLineId?: string | null
	correspondingOrcid?: string | null
	journalKind?: JournalKind | null
}

export interface SyncResult {
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

// Works filter DTOs =========================

const yearFromDefault = String(new Date().getFullYear() - 5)

export const worksFilterDTOSchema = v.object({
	search: v.optional(v.fallback(v.string(), ""), ""),
	departmentId: v.optional(v.fallback(v.string(), ""), ""),
	careerId: v.optional(v.fallback(v.string(), ""), ""),
	yearFrom: v.optional(v.fallback(v.string(), yearFromDefault), yearFromDefault),
	yearTo: v.optional(v.fallback(v.string(), ""), ""),
	journalKind: v.optional(v.fallback(v.string(), ""), ""),
	researchLineId: v.optional(v.fallback(v.string(), ""), ""),
})
