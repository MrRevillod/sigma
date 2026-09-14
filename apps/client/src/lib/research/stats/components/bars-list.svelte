<script lang="ts">
	import { ArrowRight } from "@lucide/svelte"

	import type { BarsItem } from "../chart-types"

	interface Props {
		data: BarsItem[]
		hrefFor?: (id: string) => string
	}

	let { data, hrefFor }: Props = $props()

	const items = $derived([...data].sort((a, b) => b.total - a.total))
	const max = $derived(items[0]?.total ?? 1)
</script>

{#if items.length === 0}
	<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
{:else}
	<div class="space-y-2.5">
		{#each items as item (item.name)}
			{@const w = Math.max((item.total / max) * 100, 2)}
			<div class="rounded-lg px-3 py-2 transition-colors hover:bg-corp-blue/4">
				<div class="flex items-center justify-between gap-3">
					<span class="flex min-w-0 items-center gap-2">
						{#if item.color}
							<span
								class="size-2 shrink-0 rounded-full"
								style="background:{item.color}"
							></span>
						{/if}
						<span class="min-w-0 truncate text-sm font-medium text-corp-ink">
							{item.name}
						</span>
					</span>
					{#if item.id && hrefFor}
						<a
							href={hrefFor(item.id)}
							class="flex shrink-0 items-center gap-1 text-xs font-medium text-corp-blue transition-colors hover:text-corp-blue/80 hover:underline"
						>
							Ver estadísticas <ArrowRight class="size-3" />
						</a>
					{/if}
				</div>
				<div class="mt-1.5 flex items-center gap-2">
					<div
						class="relative h-2.5 min-w-0 flex-1 overflow-hidden rounded-sm bg-corp-gray/10"
					>
						<div
							class="absolute inset-y-0 left-0 rounded-sm"
							style="background:{item.color ?? '#0075B4'};width:{w}%"
						></div>
					</div>
					<span class="text-sm font-semibold text-corp-ink tabular-nums">
						{item.total}
					</span>
				</div>
			</div>
		{/each}
	</div>
{/if}
