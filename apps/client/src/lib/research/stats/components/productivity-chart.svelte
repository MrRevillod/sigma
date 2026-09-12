<script lang="ts">
	import { CircleAlert, Loader } from "@lucide/svelte"
	import { LineChart, Tooltip } from "layerchart"

	import type { ProductivityResponse } from "../dtos"
	import type { ProductivityIndexation, ProductivityPrecision } from "../productivity-labels"

	export interface ProductivityChartProps {
		data?: ProductivityResponse
		isPending: boolean
		isError: boolean
		indexation: ProductivityIndexation
		precision: ProductivityPrecision
	}

	let { data, isPending, isError, indexation, precision }: ProductivityChartProps = $props()

	const series = $derived.by(() => {
		const key = { all: "total", wos: "wos", scopus: "scopus" }[indexation]
		return [
			{
				key: "total",
				color: "#1F2937",
				label: "Total",
			},
			{
				key: "wos",
				color: "#0075B4",
				label: "WoS",
			},
			{
				key: "scopus",
				color: "#C9A500",
				label: "Scopus",
			},
		].filter((s) => s.key === key)
	})

	const allYears = $derived(
		[...new Set((data?.trend ?? []).flatMap((s) => s.values.map((v) => v.year)))].sort(
			(a, b) => a - b,
		),
	)

	const wideData = $derived(
		allYears.map((year) => {
			const row: Record<string, number> = { year }
			for (const s of data?.trend ?? []) {
				const value = s.values.find((v) => v.year === year)
				row[s.key] = value?.value ?? 0
				row[`pubs_${s.key}`] = value?.pubs ?? 0
				row["jce"] = value?.jce ?? 0
			}
			return row
		}),
	)

	const maxVal = $derived(
		Math.max(0, ...wideData.flatMap((r) => series.map((s) => r[s.key] ?? 0))),
	)

	const yTicks = $derived.by(() => {
		const max = Math.max(maxVal, 1e-9)
		const raw = max / 6
		const mag = 10 ** Math.floor(Math.log10(raw))
		const norm = raw / mag
		const nice = norm < 1.5 ? 1 : norm < 3 ? 2 : norm < 7 ? 5 : 10
		const step = nice * mag
		const top = Math.ceil(max / step) * step
		const ticks: number[] = []
		for (let v = 0; v <= top + 1e-9; v += step) ticks.push(v)
		return ticks
	})

	const yMax = $derived(yTicks[yTicks.length - 1] ?? 1)

	const truncate = $derived.by(() => {
		if (precision === "auto") {
			return (d: number) => d
		}
		const f = 10 ** Number(precision)
		return (d: number) => Math.trunc(d * f) / f
	})

	const yTickFmt = $derived.by(() => {
		if (precision === "auto") {
			const step = yTicks[1] - yTicks[0]
			const decimals = step >= 1 ? 0 : Math.max(0, Math.ceil(-Math.log10(step) - 1e-9))
			return (d: number) => d.toFixed(decimals)
		}
		const decimals = Number(precision)
		return (d: number) => truncate(d).toFixed(decimals)
	})

	const valueFmt = $derived.by(() => {
		if (precision === "auto") return (d: number) => String(d)
		return (d: number) => truncate(d).toFixed(Number(precision))
	})

	const minYear = $derived(allYears[0] ?? 0)
	const maxYear = $derived(allYears[allYears.length - 1] ?? 0)
</script>

{#if isPending}
	<div class="flex items-center justify-center py-16">
		<Loader class="size-6 animate-spin text-corp-gray" />
	</div>
{:else if isError || !data}
	<div class="flex flex-col items-center justify-center py-16 text-center">
		<CircleAlert class="size-8 text-red-500" />
		<p class="mt-3 text-sm text-corp-gray">Error al cargar la productividad.</p>
	</div>
{:else if allYears.length === 0}
	<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
{:else}
	<LineChart
		data={wideData}
		x="year"
		{series}
		height={280}
		padding={{ left: 50, right: 20, bottom: 40, top: 10 }}
		xDomain={[minYear, maxYear]}
		xNice={false}
		yDomain={[0, yMax]}
		yNice={false}
		legend={false}
		points={true}
		props={{
			xAxis: { ticks: allYears.length, format: (d: number) => String(d) },
			yAxis: { ticks: yTicks, format: yTickFmt },
		}}
	>
		{#snippet tooltip()}
			<Tooltip.Root>
				{#snippet children({ data: row })}
					{@const s = series[0]}
					{@const pubs = row[`pubs_${s.key}`] ?? 0}
					{@const jce = row["jce"] ?? 0}
					{@const jceFmt = jce.toLocaleString("es-CL", { maximumFractionDigits: 2 })}
					<Tooltip.List>
						<Tooltip.Item label={s.label} color={s.color} valueAlign="right">
							<span class="inline-flex items-center gap-1.5">
								<span class="flex flex-col items-center leading-none">
									<span>{pubs} publicaciones</span>
									<span class="my-0.5 w-7 grow border-t border-current"></span>
									<span>{jceFmt} JCE</span>
								</span>
								<span class="text-corp-gray">
									= {valueFmt(row[s.key])} publicaciones por JCE
								</span>
							</span>
						</Tooltip.Item>
					</Tooltip.List>
					<Tooltip.Header
						value={row.year}
						classes={{ root: "productivity-tooltip-year" }}
					/>
				{/snippet}
			</Tooltip.Root>
		{/snippet}
	</LineChart>
{/if}

<style>
	:global(.lc-tooltip-header.productivity-tooltip-year) {
		border-bottom: none;
		margin-bottom: 0;
		padding-bottom: 0;
		border-top: 1px solid
			color-mix(in oklab, var(--color-surface-content, currentColor) 20%, transparent);
		margin-top: 8px;
		padding-top: 4px;
	}
</style>
