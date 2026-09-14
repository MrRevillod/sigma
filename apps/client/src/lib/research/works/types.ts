import type { FieldElementProps } from "@formisch/svelte"

export interface AuthorDraft {
	orcid: string
	name: string
	isExternal: boolean
	affiliations: string[]
	draftAffiliation: string
}

export interface AutoGrowField {
	props: FieldElementProps
	input: string | null | undefined
	errors: [string, ...string[]] | null
}
