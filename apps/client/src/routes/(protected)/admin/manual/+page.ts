import { redirect } from "@sveltejs/kit"

import { FIRST_PAGE } from "$manual/sections"

export const load = async () => {
	redirect(302, FIRST_PAGE.href)
}
