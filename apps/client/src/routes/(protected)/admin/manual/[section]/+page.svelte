<script lang="ts">
	import { page } from "$app/state"

	import ManualArticle from "$manual/components/manual-article.svelte"
	import ManualNotFound from "$manual/components/manual-not-found.svelte"
	import { findPage } from "$manual/sections"

	const entry = $derived(findPage(page.params.section))
</script>

<svelte:head>
	<title>{entry ? `${entry.page.title} | Manual` : "Manual"} | Administración</title>
</svelte:head>

{#if entry}
	<ManualArticle {entry} />
{:else}
	<ManualNotFound />
{/if}
