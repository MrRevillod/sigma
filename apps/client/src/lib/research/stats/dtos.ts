import * as v from "valibot"

export interface StatsQuery {
	journalKind?: "wos" | "scopus"
	departmentId?: string
	yearFrom?: number
	yearTo?: number
	limit?: number
}

export interface YearValue {
	year: number
	value: number
}

export interface TimeSeriesStat {
	id?: string | null
	key: string
	values: YearValue[]
}

export interface ScopeSeries {
	id: string | null
	name: string
	total: number
	wos: YearValue[]
	scopus: YearValue[]
}

export interface ScopeTotal {
	id: string | null
	name: string
	total: number
}

export interface FacultySummary {
	totalWorks: number
	wosCount: number
	scopusCount: number
}

export interface WorksStatsResponse {
	facultySummary: FacultySummary
	byJournalKind: TimeSeriesStat[]
	byDepartment: ScopeSeries[]
	byResearchLine: ScopeSeries[]
	topPublishers: TopPublisher[]
}

export interface TopPublisher {
	academicId: string
	name: string
	total: number
	scopus: number
	wos: number
	unindexed: number
	option: string
}

export interface DepartmentDetail {
	department: string
	totalWorks: number
	scopusCount: number
	wosCount: number
	teachingCount: number
	researchCount: number
	byJournalKind: TimeSeriesStat[]
	topPublishers: TopPublisher[]
}

export interface DepartmentDetailQuery {
	yearFrom?: number
	yearTo?: number
	option?: "teaching" | "research"
	journalKind?: "wos" | "scopus"
	limit?: number
}

export interface AcademicStatsQuery {
	yearFrom?: number
	yearTo?: number
}

export interface ResearchLineStat {
	researchLineId: string
	name: string
	count: number
}

export interface AcademicContribution {
	academicWorks: number
	facultyWorks: number
	departmentWorks: number
	departmentName: string | null
	dominantLineWorks: number
	lineTotalWorks: number
}

export interface AcademicStatsResponse {
	byResearchLine: ResearchLineStat[]
	dominantResearchLineId: string | null
	byJournalKind: TimeSeriesStat[]
	contribution: AcademicContribution
}

export interface ResearchLineStatsQuery {
	yearFrom?: number
	yearTo?: number
	limit?: number
}

export interface ResearchLineStatsResponse {
	name: string
	totalWorks: number
	wosCount: number
	scopusCount: number
	byJournalKind: TimeSeriesStat[]
	byDepartment: ScopeTotal[]
	topPublishers: TopPublisher[]
}

export type ProductivityDegree = "all" | "magister" | "doctor"
export type ProductivityScope = "faculty" | "department" | "researchLine"
export type ProductivityJceScope = "doctor" | "all"

export interface ProductivityQuery {
	degree?: ProductivityDegree
	scope?: ProductivityScope
	jceScope?: ProductivityJceScope
	departmentId?: string
	researchLineId?: string
	month?: number
	yearFrom?: number
	yearTo?: number
}

export interface ProductivityYearValue {
	year: number
	value: number
	pubs: number
	jce: number
}

export interface ProductivitySeries {
	key: string
	values: ProductivityYearValue[]
}

export interface ProductivityResponse {
	trend: ProductivitySeries[]
}

// Stats search params schema

const currentYear = new Date().getFullYear()
const defaultYearFrom = String(currentYear - 5)
const defaultYearTo = String(currentYear)

export const statsParamsDTOSchema = v.object({
	yearFrom: v.optional(v.fallback(v.string(), defaultYearFrom), defaultYearFrom),
	yearTo: v.optional(v.fallback(v.string(), defaultYearTo), defaultYearTo),
	limit: v.optional(v.fallback(v.string(), "10"), "10"),
})
