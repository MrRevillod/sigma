<script lang="ts">
	import type { TocEntry } from "$manual/markdown"

	let { entries, scrollRoot = null }: { entries: TocEntry[]; scrollRoot?: HTMLElement | null } =
		$props()

	let activeId = $state<string | null>(null)

	$effect(() => {
		const ids = entries.map((entry) => entry.id)
		if (ids.length === 0) return

		const elements = ids
			.map((id) => document.getElementById(id))
			.filter((el): el is HTMLElement => el !== null)
		if (elements.length === 0) return

		let frame = 0

		const update = () => {
			frame = 0
			const offset = (scrollRoot?.getBoundingClientRect().top ?? 0) + 24
			let current = elements[0].id
			for (const el of elements) {
				if (el.getBoundingClientRect().top <= offset) current = el.id
				else break
			}
			activeId = current
		}

		const onScroll = () => {
			if (frame) return
			frame = requestAnimationFrame(update)
		}

		update()
		scrollRoot?.addEventListener("scroll", onScroll, { passive: true })
		window.addEventListener("resize", onScroll)

		return () => {
			if (frame) cancelAnimationFrame(frame)
			scrollRoot?.removeEventListener("scroll", onScroll)
			window.removeEventListener("resize", onScroll)
		}
	})

	function handleClick(event: MouseEvent, id: string) {
		event.preventDefault()
		const target = document.getElementById(id)
		if (!target) return
		target.scrollIntoView({ behavior: "smooth", block: "start" })
		history.replaceState(null, "", `#${id}`)
	}
</script>

<nav aria-label="En esta página">
	<p class="mb-3 text-xs font-semibold uppercase tracking-wider text-corp-gray">En esta página</p>
	<ul class="border-l border-corp-gray/20">
		{#each entries as entry (entry.id)}
			<li>
				<a
					href="#{entry.id}"
					onclick={(event) => handleClick(event, entry.id)}
					class="-ml-px block border-l-2 py-1 pr-2 text-sm leading-snug transition-colors {entry.depth ===
					3
						? 'pl-6'
						: 'pl-3'} {activeId === entry.id
						? 'border-corp-blue font-medium text-corp-blue'
						: 'border-transparent text-corp-gray hover:text-corp-ink'}"
				>
					{entry.text}
				</a>
			</li>
		{/each}
	</ul>
</nav>
