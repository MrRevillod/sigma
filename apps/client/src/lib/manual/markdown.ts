import DOMPurify from "dompurify"
import { marked, type Tokens } from "marked"

export interface TocEntry {
	depth: number
	text: string
	id: string
}

function slugify(value: string): string {
	return value
		.toLowerCase()
		.normalize("NFD")
		.replace(/[\u0300-\u036f]/g, "")
		.replace(/<[^>]*>/g, "")
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "")
}

marked.use({
	gfm: true,
	breaks: false,
	renderer: {
		heading({ tokens, depth }: Tokens.Heading) {
			const text = this.parser.parseInline(tokens)
			const id = slugify(text) || `seccion-${depth}`
			return `<h${depth} id="${id}">${text}</h${depth}>\n`
		},
	},
})

export function renderMarkdown(markdown: string): string {
	const html = marked.parse(markdown, { async: false })
	return DOMPurify.sanitize(html, {
		ADD_TAGS: ["figure", "figcaption", "img"],
		ADD_ATTR: ["target", "rel"],
	})
}

export function extractToc(markdown: string): TocEntry[] {
	const entries: TocEntry[] = []

	for (const token of marked.lexer(markdown, { gfm: true })) {
		if (token.type !== "heading") continue

		const heading = token as Tokens.Heading
		if (heading.depth < 2 || heading.depth > 3) continue

		const html = marked.Parser.parseInline(heading.tokens)
		entries.push({
			depth: heading.depth,
			text: html.replace(/<[^>]*>/g, ""),
			id: slugify(html) || `seccion-${heading.depth}`,
		})
	}

	return entries
}
