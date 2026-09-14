<script lang="ts">
	import type { Segment } from "../chart-types"

	interface Props {
		segments: Segment[]
		total: number
		class?: string
	}

	let { segments, total, class: className = "" }: Props = $props()

	const active = $derived(segments.filter((s) => s.value > 0))
	const radius = 80
	const stroke = 28
	const center = $derived(radius + stroke / 2)

	const arcs = $derived.by(() => {
		let offset = -Math.PI / 2
		return active.map((seg) => {
			const ratio = total > 0 ? seg.value / total : 0
			const start = offset
			offset += ratio * 2 * Math.PI
			const end = offset
			const large = ratio > 0.5 ? 1 : 0

			const x1 = center + radius * Math.cos(start)
			const y1 = center + radius * Math.sin(start)
			const x2 = center + radius * Math.cos(end)
			const y2 = center + radius * Math.sin(end)

			return {
				d: `M ${x1} ${y1} A ${radius} ${radius} 0 ${large} 1 ${x2} ${y2}`,
				...seg,
				pct: Math.round(ratio * 100),
				isFull: ratio >= 1,
			}
		})
	})

	const viewBox = $derived(`0 0 ${center * 2} ${center * 2}`)
</script>

<div class="flex flex-col items-center gap-2 {className}">
	<svg {viewBox} class="w-40 h-40" role="img">
		{#each arcs as arc (arc.label)}
			{#if arc.isFull}
				<circle
					cx={center}
					cy={center}
					r={radius}
					fill="none"
					stroke={arc.color}
					stroke-width={stroke}
				/>
			{:else}
				<path d={arc.d} fill="none" stroke={arc.color} stroke-width={stroke} />
			{/if}
		{/each}
		<text x={center} y={center - 4} text-anchor="middle" class="fill-corp-ink text-xl font-bold"
			>{total}</text
		>
		<text x={center} y={center + 16} text-anchor="middle" class="fill-corp-gray text-xs"
			>publicaciones</text
		>
	</svg>

	<div class="flex flex-wrap justify-center gap-x-4 gap-y-1 text-xs font-medium">
		{#each arcs as arc (arc.label)}
			<span class="inline-flex items-center gap-1.5">
				<span class="inline-block size-2.5 rounded-full" style="background:{arc.color}"
				></span>
				{arc.label} <span class="text-corp-gray">{arc.value} ({arc.pct}%)</span>
			</span>
		{/each}
	</div>
</div>
