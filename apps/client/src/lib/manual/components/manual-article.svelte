<script lang="ts">
	import { ArrowLeft, ArrowRight } from "@lucide/svelte"

	import ManualLightbox from "./manual-lightbox.svelte"
	import { renderMarkdown } from "$manual/markdown"
	import { MANUAL_FLAT_PAGES, type FlatPage } from "$manual/sections"

	let { entry }: { entry: FlatPage } = $props()

	const index = $derived(MANUAL_FLAT_PAGES.indexOf(entry))
	const previous = $derived(index > 0 ? MANUAL_FLAT_PAGES[index - 1] : undefined)
	const next = $derived(
		index >= 0 && index < MANUAL_FLAT_PAGES.length - 1
			? MANUAL_FLAT_PAGES[index + 1]
			: undefined,
	)
	const html = $derived(renderMarkdown(entry.page.body))

	let contentEl = $state<HTMLDivElement | null>(null)
	let preview = $state<{ src: string; alt: string } | null>(null)

	$effect(() => {
		const root = contentEl
		if (!root || !html) return

		const images = Array.from(root.querySelectorAll<HTMLImageElement>("img"))
		const onImageClick = (event: Event) => {
			const img = event.currentTarget as HTMLImageElement
			preview = { src: img.currentSrc || img.src, alt: img.alt }
		}

		for (const img of images) {
			img.classList.add("manual-zoomable")
			img.title = "Clic para ampliar"
			img.addEventListener("click", onImageClick)
		}

		return () => {
			for (const img of images) img.removeEventListener("click", onImageClick)
		}
	})
</script>

<article class="w-full">
	<header class="mb-4 border-b border-corp-gray/20 pb-6">
		<div
			class="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-corp-blue"
		>
			<entry.chapter.icon class="size-4" />
			<span>
				{entry.chapter.page
					? "Manual de administración"
					: `Manual de administración · ${entry.chapter.title}`}
			</span>
		</div>
		<h1 class="mt-3 text-2xl font-semibold text-corp-ink">{entry.page.title}</h1>
		<p class="mt-2 text-sm text-corp-gray">{entry.page.description}</p>
	</header>

	<div bind:this={contentEl} class="manual-content">{@html html}</div>

	{#if preview}
		<ManualLightbox src={preview.src} alt={preview.alt} onclose={() => (preview = null)} />
	{/if}

	{#if previous || next}
		<nav
			class="mt-12 flex items-stretch justify-between gap-4 border-t border-corp-gray/20 pt-6"
			aria-label="Navegación entre páginas"
		>
			{#if previous}
				<a
					href={previous.href}
					class="group flex max-w-[48%] flex-col items-start rounded-lg border border-corp-gray/20 px-4 py-3 text-left transition-colors hover:border-corp-blue/40 hover:bg-corp-blue/5"
				>
					<span
						class="flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wider text-corp-gray"
					>
						<ArrowLeft class="size-3.5" />
						Anterior
					</span>
					<span class="mt-1 text-sm font-medium text-corp-ink group-hover:text-corp-blue">
						{previous.page.title}
					</span>
				</a>
			{:else}
				<span></span>
			{/if}

			{#if next}
				<a
					href={next.href}
					class="group flex max-w-[48%] flex-col items-end rounded-lg border border-corp-gray/20 px-4 py-3 text-right transition-colors hover:border-corp-blue/40 hover:bg-corp-blue/5"
				>
					<span
						class="flex items-center gap-1.5 text-[11px] font-semibold uppercase tracking-wider text-corp-gray"
					>
						Siguiente
						<ArrowRight class="size-3.5" />
					</span>
					<span class="mt-1 text-sm font-medium text-corp-ink group-hover:text-corp-blue">
						{next.page.title}
					</span>
				</a>
			{/if}
		</nav>
	{/if}
</article>
