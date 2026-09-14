<script lang="ts">
	import { ChartBar, CircleAlert, Gauge, Loader, TrendingUp } from "@lucide/svelte"

	import YearRange from "$shared/components/ui/year-range.svelte"
	import { useAcademicStatsQuery } from "$stats/queries"

	import DonutChart from "./donut-chart.svelte"
	import RadarChart from "./radar-chart.svelte"
	import StatsSection from "./stats-section.svelte"
	import TrendLine from "./trend-line.svelte"
	import { acronymOf } from "../utils"
	import type { AcademicStatsSection } from "../chart-types"

	interface Props {
		academicId: string
	}

	let { academicId }: Props = $props()

	const currentYear = new Date().getFullYear()

	let yearFrom = $state(String(currentYear - 5))
	let yearTo = $state(String(currentYear))

	let openSection = $state<AcademicStatsSection>("lines")

	function toggleSection(section: AcademicStatsSection) {
		openSection = section
	}

	const query = useAcademicStatsQuery(
		() => academicId,
		() => ({
			...(yearFrom && { yearFrom: Number(yearFrom) }),
			...(yearTo && { yearTo: Number(yearTo) }),
		}),
	)

	const lines = $derived(query.data?.byResearchLine ?? [])
	const total = $derived(lines.reduce((a, l) => a + l.count, 0))
	const maxCount = $derived(Math.max(0, ...lines.map((l) => l.count)))

	const radarItems = $derived(
		lines.map((l) => ({
			name: l.name,
			acronym: acronymOf(l.name),
			count: l.count,
		})),
	)

	const contribution = $derived(query.data?.contribution)
	const dominant = $derived(
		lines.find((l) => l.researchLineId === query.data?.dominantResearchLineId) ?? null,
	)

	const impactDonuts = $derived.by(() => {
		if (!contribution) return []

		const mk = (caption: string, theirs: number, scopeTotal: number) => ({
			caption,
			total: scopeTotal,
			segments: [
				{ label: "Sus publicaciones", value: Math.max(0, theirs), color: "#C9A500" },
				{ label: "Otras", value: Math.max(0, scopeTotal - theirs), color: "#0075B4" },
			],
		})

		return [
			mk("Facultad de Ingeniería", contribution.academicWorks, contribution.facultyWorks),
			mk(
				contribution.departmentName
					? `Departamento de ${contribution.departmentName}`
					: "Departamento",
				contribution.academicWorks,
				contribution.departmentWorks,
			),
			mk(
				dominant ? `Línea: ${dominant.name}` : "Línea de investigación",
				contribution.dominantLineWorks,
				contribution.lineTotalWorks,
			),
		]
	})
</script>

<div class="space-y-4">
	<div class="flex flex-wrap items-end justify-between gap-3">
		<h2 class="text-lg font-semibold text-corp-blue">Estadísticas del académico</h2>
		<YearRange
			bind:yearFrom
			bind:yearTo
			minYear={1900}
			label="Rango anual de publicación"
			showLabels={false}
			placeholderFrom="DESDE"
			placeholderTo="HASTA"
		/>
	</div>

	{#if query.isPending}
		<div
			class="flex items-center justify-center rounded-xl border border-corp-gray/20 bg-white py-12"
		>
			<Loader class="size-5 animate-spin text-corp-gray" />
		</div>
	{:else if query.isError}
		<div
			class="flex flex-col items-center rounded-xl border border-corp-gray/20 bg-white py-12 text-center"
		>
			<CircleAlert class="size-6 text-red-500" />
			<p class="mt-2 text-sm text-corp-gray">Error al cargar las estadísticas.</p>
		</div>
	{:else}
		<div class="overflow-hidden rounded-xl border border-corp-gray/20 bg-white">
			<StatsSection
				title="Líneas de investigación"
				icon={ChartBar}
				open={openSection === "lines"}
				ontoggle={() => toggleSection("lines")}
				first
				description="Distribución de las publicaciones del académico por línea de investigación."
			>
				{#if lines.length === 0}
					<p class="py-6 text-center text-sm text-corp-gray">
						Sin líneas de investigación asignadas en el rango seleccionado.
					</p>
				{:else}
					<div
						class="grid grid-cols-1 items-center gap-8 lg:grid-cols-[minmax(0,1fr)_auto]"
					>
						<div class="space-y-3 flex flex-col gap-2">
							{#each lines as line (line.researchLineId)}
								{@const pct =
									total > 0 ? Math.round((line.count / total) * 100) : 0}
								{@const w =
									maxCount > 0 ? Math.max((line.count / maxCount) * 100, 0) : 0}
								<div>
									<div class="mb-1 flex items-center justify-between gap-3">
										<span class="flex min-w-0 items-center gap-2">
											<span
												class="shrink-0 rounded bg-corp-blue/10 px-1.5 py-0.5 text-[10px] font-bold text-corp-blue"
											>
												{acronymOf(line.name)}
											</span>
											<span
												class="truncate text-sm font-medium text-corp-ink"
											>
												{line.name}
											</span>
										</span>
										<span class="shrink-0 text-xs text-corp-gray tabular-nums"
											>{line.count} · {pct}%</span
										>
									</div>
									<div
										class="mt-3 relative h-2.5 min-w-0 overflow-hidden rounded-sm bg-corp-gray/10"
									>
										<div
											class="absolute inset-y-0 left-0 rounded-sm bg-corp-blue"
											style="width:{w}%"
										></div>
									</div>
								</div>
							{/each}
						</div>

						<div class="flex justify-center">
							<RadarChart items={radarItems} />
						</div>
					</div>
				{/if}
			</StatsSection>

			<StatsSection
				title="Tendencia anual de publicaciones"
				icon={TrendingUp}
				open={openSection === "trend"}
				ontoggle={() => toggleSection("trend")}
				description="Evolución anual de las publicaciones del académico, según tipo de indexación."
			>
				<TrendLine journalKind={query.data?.byJournalKind ?? []} />
			</StatsSection>

			<StatsSection
				title="Impacto y Desempeño"
				icon={Gauge}
				open={openSection === "impact"}
				ontoggle={() => toggleSection("impact")}
				description="Contribución del académico respecto a su facultad, departamento y línea."
			>
				{#if impactDonuts.length === 0}
					<p class="py-6 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
				{:else}
					<div class="grid grid-cols-1 gap-8 md:grid-cols-3">
						{#each impactDonuts as d (d.caption)}
							<div class="flex flex-col items-center gap-2">
								<p class="text-center text-sm font-medium text-corp-ink">
									{d.caption}
								</p>
								{#if d.total > 0}
									<DonutChart segments={d.segments} total={d.total} />
								{:else}
									<p class="py-6 text-xs text-corp-gray">
										Sin datos en el rango.
									</p>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</StatsSection>
		</div>
	{/if}
</div>
