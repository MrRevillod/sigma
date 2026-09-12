import { redirect } from "@sveltejs/kit"

import { getChapter, getFirstChildHref } from "$manual/sections"

export const load = async ({ params }) => {
	const chapter = getChapter(params.section)

	if (chapter && !chapter.page) {
		redirect(302, getFirstChildHref(chapter))
	}

	return {}
}
