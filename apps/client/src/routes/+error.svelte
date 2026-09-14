<script lang="ts">
	import { page } from "$app/state"
	import { goto } from "$app/navigation"

	import Button from "$shared/components/ui/button.svelte"

	const status = $derived(page.status)
	const is404 = $derived(status === 404)
	const message = $derived(page.error?.message)

	const title = $derived(is404 ? "Página no encontrada" : "Algo salió mal")
	const description = $derived(
		is404
			? "La página que buscas no existe o fue movida."
			: (message ?? "Ocurrió un error inesperado. Inténtalo nuevamente."),
	)
</script>

<div class="flex h-full flex-col items-center justify-center px-4 py-8 text-center">
	<p class="text-6xl font-semibold text-corp-blue">{status}</p>
	<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">{title}</h1>
	<p class="mt-2 max-w-md text-sm text-corp-gray">{description}</p>
	<Button class="mt-6" onclick={() => goto("/works")}>Volver al inicio</Button>
</div>
