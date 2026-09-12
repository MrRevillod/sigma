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

<style>
	:global(.manual-content) {
		color: #1a1a1a;
		font-size: 0.9375rem;
		line-height: 1.7;
	}

	:global(.manual-content > :first-child) {
		margin-top: 0;
	}

	:global(.manual-content h2),
	:global(.manual-content h3),
	:global(.manual-content h4) {
		scroll-margin-top: 1rem;
	}

	:global(.manual-content h2) {
		margin: 2rem 0 0.75rem;
		color: var(--color-corp-blue);
		font-size: 1.125rem;
		font-weight: 600;
	}

	:global(.manual-content h3) {
		margin: 1.5rem 0 0.5rem;
		font-size: 1rem;
		font-weight: 600;
	}

	:global(.manual-content h4) {
		margin: 1.25rem 0 0.5rem;
		font-size: 0.9375rem;
		font-weight: 600;
	}

	:global(.manual-content p) {
		margin: 0.75rem 0;
	}

	:global(.manual-content ul) {
		margin: 0.75rem 0;
		padding-left: 1.25rem;
		list-style: disc;
	}

	:global(.manual-content ol) {
		margin: 0.75rem 0;
		padding-left: 1.25rem;
		list-style: decimal;
	}

	:global(.manual-content li) {
		margin: 0.3rem 0;
	}

	:global(.manual-content li > ul),
	:global(.manual-content li > ol) {
		margin: 0.3rem 0;
	}

	:global(.manual-content a) {
		color: var(--color-corp-blue);
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	:global(.manual-content a:hover) {
		color: #00598a;
	}

	:global(.manual-content strong) {
		font-weight: 600;
	}

	:global(.manual-content code) {
		background: #f1f1f1;
		border-radius: 4px;
		padding: 0.1rem 0.35rem;
		font-size: 0.85em;
	}

	:global(.manual-content pre) {
		margin: 1rem 0;
		overflow-x: auto;
		border: 1px solid rgb(135 135 135 / 0.2);
		border-radius: 8px;
		background: #f7f7f7;
		padding: 1rem;
	}

	:global(.manual-content pre code) {
		background: transparent;
		padding: 0;
	}

	:global(.manual-content blockquote) {
		margin: 1rem 0;
		border-left: 3px solid var(--color-corp-blue);
		background: rgb(0 117 180 / 0.05);
		padding: 0.5rem 1rem;
		color: #444;
	}

	:global(.manual-content blockquote p) {
		margin: 0.25rem 0;
	}

	:global(.manual-content hr) {
		margin: 2rem 0;
		border: 0;
		border-top: 1px solid rgb(135 135 135 / 0.2);
	}

	:global(.manual-content table) {
		display: block;
		width: 100%;
		margin: 1rem 0;
		overflow-x: auto;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	:global(.manual-content th) {
		border-bottom: 1px solid rgb(135 135 135 / 0.3);
		background: #fafafa;
		padding: 0.5rem 0.75rem;
		text-align: left;
		font-weight: 600;
	}

	:global(.manual-content td) {
		border-bottom: 1px solid rgb(135 135 135 / 0.15);
		padding: 0.5rem 0.75rem;
		vertical-align: top;
	}

	:global(.manual-content details.manual-disclosure) {
		margin: 1.25rem 0;
		border: 1px solid rgb(135 135 135 / 0.25);
		border-radius: 8px;
		background: #fafafa;
		padding: 0 1rem;
	}

	:global(.manual-content details.manual-disclosure[open]) {
		background: #fff;
		padding-bottom: 1rem;
	}

	:global(.manual-content details.manual-disclosure > summary) {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		list-style: none;
		padding: 0.75rem 0;
		font-weight: 600;
		color: var(--color-corp-blue);
	}

	:global(.manual-content details.manual-disclosure > summary)::-webkit-details-marker {
		display: none;
	}

	:global(.manual-content details.manual-disclosure > summary)::before {
		content: "";
		display: inline-block;
		width: 0.45rem;
		height: 0.45rem;
		border-right: 2px solid currentColor;
		border-bottom: 2px solid currentColor;
		transform: rotate(-45deg);
		transition: transform 0.15s ease;
	}

	:global(.manual-content details.manual-disclosure[open] > summary)::before {
		transform: rotate(45deg);
	}

	:global(.manual-content details.manual-disclosure > summary):hover {
		color: #00598a;
	}

	:global(.manual-content figure.manual-figure) {
		margin: 1.25rem 0;
	}

	:global(.manual-content img) {
		max-width: 100%;
		border: 1px solid rgb(135 135 135 / 0.25);
		border-radius: 8px;
		background: #fafafa;
	}

	:global(.manual-content img.manual-zoomable) {
		cursor: zoom-in;
		transition:
			box-shadow 0.15s ease,
			border-color 0.15s ease;
	}

	:global(.manual-content img.manual-zoomable:hover) {
		border-color: rgb(0 117 180 / 0.5);
		box-shadow: 0 6px 20px rgb(0 0 0 / 0.15);
	}

	:global(.manual-content figure.manual-figure figcaption) {
		margin-top: 0.5rem;
		color: var(--color-corp-gray);
		font-size: 0.75rem;
	}
</style>
