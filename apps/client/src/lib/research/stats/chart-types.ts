export interface Segment {
	label: string
	value: number
	color: string
}

export interface RadarItem {
	name: string
	acronym: string
	count: number
}

export interface BarsItem {
	id: string | null
	name: string
	total: number
	color?: string
}

export type AcademicStatsSection = "lines" | "trend" | "impact"
