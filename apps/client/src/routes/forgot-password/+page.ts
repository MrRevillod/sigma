import { redirect } from "@sveltejs/kit"
import { authStore } from "$auth/store.svelte"

export const load = async () => {
	if (authStore.isAuthenticated) {
		redirect(302, "/")
	}
}
