<script lang="ts">
	import type { RadarItem } from "../chart-types"

	interface Props {
		items: RadarItem[]
		color?: string
		size?: number
	}

	let { items, color = "#0075B4", size = 220 }: Props = $props()

	const n = $derived(items.length)
	const max = $derived(Math.max(1, ...items.map((i) => i.count)))

	const center = $derived(size / 2)
	const radius = $derived(size / 2 - 32)

	function angle(index: number): number {
		return -Math.PI / 2 + (index * 2 * Math.PI) / n
	}

	function point(index: number, fraction: number) {
		const a = angle(index)
		const r = radius * fraction
		return { x: center + r * Math.cos(a), y: center + r * Math.sin(a) }
	}

	function polygon(fraction: number): string {
		const pts = items.map((_, i) => point(i, fraction))
		return (
			pts
				.map((p, i) => `${i === 0 ? "M" : "L"} ${p.x.toFixed(2)} ${p.y.toFixed(2)}`)
				.join(" ") + " Z"
		)
	}

	const rings = [0.25, 0.5, 0.75, 1]
</script>

{#if n === 0}
	<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
{:else}
	<svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} role="img">
		{#each rings as f (f)}
			<path
				d={polygon(f)}
				fill="none"
				stroke="#878787"
				stroke-opacity={f === 1 ? "0.4" : "0.25"}
				stroke-width="1"
			/>
		{/each}
		<path
			d={(() => {
				const pts = items.map((item, i) => point(i, item.count / max))
				return (
					pts
						.map((p, i) => `${i === 0 ? "M" : "L"} ${p.x.toFixed(2)} ${p.y.toFixed(2)}`)
						.join(" ") + " Z"
				)
			})()}
			fill={color}
			fill-opacity="0.15"
			stroke={color}
			stroke-width="2"
			stroke-linejoin="round"
		/>
		{#each items as item, i (item.name)}
			{@const p = point(i, item.count / max)}
			{@const a = angle(i)}
			{@const lx = center + (radius + 20) * Math.cos(a)}
			{@const ly = center + (radius + 20) * Math.sin(a)}
			<circle cx={p.x} cy={p.y} r="3" fill={color} />
			<text
				x={lx}
				y={ly}
				text-anchor={Math.abs(Math.cos(a)) < 0.3
					? "middle"
					: Math.cos(a) > 0
						? "start"
						: "end"}
				dy={Math.sin(a) > 0 ? "0.9em" : "-0.1em"}
				class="text-[10px] font-bold"
				fill={color}
			>
				{item.acronym}
			</text>
		{/each}
	</svg>
{/if}
