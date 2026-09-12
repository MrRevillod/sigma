import adapter from "@sveltejs/adapter-static"

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			fallback: "index.html",
		}),
		alias: {
			"$academics": "src/lib/academic/academics",
			"$categories": "src/lib/academic/categories",
			"$degrees": "src/lib/academic/degrees",
			"$options": "src/lib/academic/options",
			"$auth": "src/lib/auth",
			"$shared": "src/lib/shared",
			"$research": "src/lib/research",
			"$works": "src/lib/research/works",
			"$stats": "src/lib/research/stats",
			"$collaborations": "src/lib/research/collaborations",
			"$manual": "src/lib/manual",
			"$careers": "src/lib/university/careers",
			"$departments": "src/lib/university/departments",
			"$faculties": "src/lib/university/faculties",
			"$work-positions": "src/lib/university/work-positions",
			"$users": "src/lib/users",
		},
	},
	compilerOptions: {
		runes: ({ filename }) =>
			filename.split(/[/\\]/).includes("node_modules") ? undefined : true,
	},
}

export default config
