<script lang="ts">
	import { ChevronDown } from "@lucide/svelte"
	import { page } from "$app/state"

	import { MANUAL_BASE_PATH, MANUAL_CHAPTERS, findPage } from "$manual/sections"

	const active = $derived(findPage(page.params.section, page.params.page))

	let open = $state<Record<string, boolean>>({})

	$effect(() => {
		const chapter = active?.chapter
		if (chapter && !chapter.page) {
			open[chapter.slug] = true
		}
	})

	function isChapterActive(slug: string): boolean {
		return active?.chapter.slug === slug
	}

	function isPageActive(chapterSlug: string, pageSlug: string): boolean {
		return active?.chapter.slug === chapterSlug && active.page.slug === pageSlug
	}
</script>

<nav class="flex flex-col gap-0.5 lg:w-52" aria-label="Secciones del manual">
	{#each MANUAL_CHAPTERS as chapter (chapter.slug)}
		{#if chapter.page}
			<a
				href="{MANUAL_BASE_PATH}/{chapter.slug}"
				aria-current={isChapterActive(chapter.slug) ? "page" : undefined}
				class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-medium transition-colors {isChapterActive(
					chapter.slug,
				)
					? 'bg-corp-blue/10 text-corp-blue'
					: 'text-corp-gray hover:bg-corp-gray/5 hover:text-corp-ink'}"
			>
				<chapter.icon class="size-4 shrink-0" />
				<span>{chapter.title}</span>
			</a>
		{:else}
			<button
				type="button"
				onclick={() => (open[chapter.slug] = !open[chapter.slug])}
				aria-expanded={open[chapter.slug]}
				class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-medium transition-colors {isChapterActive(
					chapter.slug,
				)
					? 'text-corp-ink'
					: 'text-corp-gray hover:bg-corp-gray/5 hover:text-corp-ink'}"
			>
				<chapter.icon class="size-4 shrink-0" />
				<span class="flex-1 text-left">{chapter.title}</span>
				<ChevronDown
					class="size-4 shrink-0 transition-transform {open[chapter.slug]
						? 'rotate-180'
						: ''}"
				/>
			</button>

			{#if open[chapter.slug]}
				<div class="mb-1 ml-4 flex flex-col gap-0.5 border-l border-corp-gray/20 pl-2">
					{#each chapter.children ?? [] as child (child.slug)}
						<a
							href="{MANUAL_BASE_PATH}/{chapter.slug}/{child.slug}"
							aria-current={isPageActive(chapter.slug, child.slug)
								? "page"
								: undefined}
							class="rounded-md px-2.5 py-1.5 text-[13px] font-medium transition-colors {isPageActive(
								chapter.slug,
								child.slug,
							)
								? 'bg-corp-blue/10 text-corp-blue'
								: 'text-corp-gray hover:bg-corp-gray/5 hover:text-corp-ink'}"
						>
							{child.title}
						</a>
					{/each}
				</div>
			{/if}
		{/if}
	{/each}
</nav>
