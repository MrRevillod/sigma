const ACRONYMS: Record<string, string> = {
	"Materiales Avanzados y Bioproductos": "MAB",
	"Ciencias de la Tierra": "CT",
	"Sostenibilidad": "SO",
	"IA, Sistemas Complejos y Modelamiento Matemático": "IA",
	"Educación en Ingeniería": "EI",
}

const STOPWORDS = new Set([
	"de",
	"del",
	"la",
	"las",
	"los",
	"el",
	"en",
	"y",
	"e",
	"a",
	"para",
	"por",
	"su",
	"un",
	"una",
])

export function acronymOf(name: string): string {
	if (ACRONYMS[name]) return ACRONYMS[name]

	const words = name
		.replace(/[^\p{L}\s]/gu, " ")
		.split(/\s+/)
		.filter((w) => w.length > 0 && !STOPWORDS.has(w.toLowerCase()))

	if (words.length === 0) return name.slice(0, 2).toUpperCase()
	if (words.length === 1) return words[0].slice(0, 2).toUpperCase()

	return words
		.slice(0, 3)
		.map((w) => w[0].toUpperCase())
		.join("")
}
