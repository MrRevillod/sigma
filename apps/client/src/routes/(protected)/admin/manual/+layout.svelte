<script lang="ts">
	import type { Snippet } from "svelte"

	import { page } from "$app/state"

	import ManualNav from "$manual/components/manual-nav.svelte"
	import ManualToc from "$manual/components/manual-toc.svelte"
	import { extractToc } from "$manual/markdown"
	import { findPage } from "$manual/sections"

	import "$manual/manual.css"

	let { children }: { children: Snippet } = $props()

	const entry = $derived(findPage(page.params.section, page.params.page))
	const toc = $derived(entry ? extractToc(entry.page.body) : [])

	let container = $state<HTMLElement>()
	const scrollRoot = $derived(container?.parentElement ?? null)
</script>

<div bind:this={container} class="flex w-full flex-col gap-6 lg:flex-row lg:gap-12">
	<aside class="lg:sticky lg:top-0 lg:self-start">
		<ManualNav />
	</aside>

	<div class="min-w-0 flex-1">
		{@render children()}
	</div>

	{#if toc.length > 0}
		<aside class="hidden shrink-0 self-start xl:sticky xl:top-0 xl:block xl:w-52">
			<ManualToc entries={toc} {scrollRoot} />
		</aside>
	{/if}
</div>
