<script lang="ts">
	import { statsParamsDTOSchema, type StatsQuery } from "$stats/dtos"

	import { useSearchParams } from "runed/kit"
	import { useWorksStatsQuery } from "$stats/queries"
	import { CircleAlert, Loader, RotateCcw } from "@lucide/svelte"

	import Button from "$shared/components/ui/button.svelte"
	import YearRange from "$shared/components/ui/year-range.svelte"
	import KpiStrip from "$stats/components/kpi-strip.svelte"
	import StatsHub from "$stats/components/stats-hub.svelte"

	const params = useSearchParams(statsParamsDTOSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const queryParams = $derived<StatsQuery>({
		yearFrom: Number(params.yearFrom),
		yearTo: Number(params.yearTo),
		limit: Number(params.limit),
	})

	const statsQuery = useWorksStatsQuery(() => queryParams)

	const unindexedCount = $derived.by(() => {
		const d = statsQuery.data
		if (!d) return 0
		return (
			d.facultySummary.totalWorks - d.facultySummary.wosCount - d.facultySummary.scopusCount
		)
	})
</script>

<div class="flex flex-col">
	{#if statsQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if statsQuery.isError || !statsQuery.data}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar las estadísticas.</p>
		</div>
	{:else}
		<div class="mb-6 flex flex-wrap items-start justify-between gap-4">
			<div>
				<h1 class="text-xl font-semibold text-corp-ink">Estadísticas de Publicaciones</h1>
				<p class="mt-1 text-sm text-corp-gray">Facultad de Ingeniería</p>
			</div>
			<div class="flex items-end gap-3">
				<YearRange
					bind:yearFrom={params.yearFrom}
					bind:yearTo={params.yearTo}
					label="Rango anual de publicación"
					showLabels={false}
					placeholderFrom="DESDE"
					placeholderTo="HASTA"
				/>
				<Button variant="secondary" onclick={() => params.reset()}>
					<RotateCcw class="size-3.5" />
					Limpiar
				</Button>
			</div>
		</div>

		<KpiStrip
			total={statsQuery.data.facultySummary.totalWorks}
			wos={statsQuery.data.facultySummary.wosCount}
			scopus={statsQuery.data.facultySummary.scopusCount}
			unindexed={unindexedCount}
		/>

		<div class="mt-4">
			<StatsHub
				data={statsQuery.data}
				bind:limit={params.limit}
				productivity={{
					denominator: "de la facultad",
					degree: "all",
					scope: "faculty",
					yearFrom: Number(params.yearFrom),
					yearTo: Number(params.yearTo),
				}}
			/>
		</div>
	{/if}
</div>
