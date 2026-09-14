<script lang="ts">
	import type { ResearchLineStatsResponse } from "$stats/dtos"

	import { ChartBar, Trophy } from "@lucide/svelte"

	import BarsList from "./bars-list.svelte"
	import ProductivitySection from "./productivity-section.svelte"
	import StatCard from "./stat-card.svelte"
	import StatsSection from "./stats-section.svelte"
	import TopLimitSelect from "./top-limit-select.svelte"
	import TopPublishersTable from "./top-publishers-table.svelte"
	import TrendLine from "./trend-line.svelte"

	import type { ProductivitySectionProps } from "../productivity-labels"

	interface Props {
		data: ResearchLineStatsResponse
		productivity: ProductivitySectionProps
		limit?: string
	}

	let { data, productivity, limit = $bindable("10") }: Props = $props()

	let openSection = $state<"trend" | "ranking" | "productivity">("trend")

	const unindexed = $derived(data.totalWorks - data.wosCount - data.scopusCount)
</script>

{#snippet rankingAction()}
	<TopLimitSelect bind:value={limit} />
{/snippet}

<div class="space-y-4">
	<div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
		<StatCard label="Total publicaciones" value={data.totalWorks} />
		<StatCard label="WoS" value={data.wosCount} variant="blue" />
		<StatCard label="Scopus" value={data.scopusCount} variant="gold" />
		<StatCard label="Sin indexar" value={unindexed} variant="muted" />
	</div>

	<div class="overflow-hidden rounded-xl border border-corp-gray/20 bg-white">
		<StatsSection
			title="Tendencia anual de publicaciones"
			icon={ChartBar}
			open={openSection === "trend"}
			ontoggle={() => (openSection = "trend")}
			first
			description="Evolución anual de las publicaciones de la línea, según tipo de indexación."
		>
			<TrendLine journalKind={data.byJournalKind} />
		</StatsSection>

		<StatsSection
			title="Ranking de publicadores"
			icon={Trophy}
			open={openSection === "ranking"}
			ontoggle={() => (openSection = "ranking")}
			action={rankingAction}
			description="Top {limit} publicadores del periodo, ordenados por total de publicaciones."
		>
			<div class="grid grid-cols-1 items-start gap-8 lg:grid-cols-[320px_1fr]">
				<div class="w-full">
					<h2 class="mb-4 text-sm font-semibold tracking-wide uppercase text-corp-blue">
						Por departamento
					</h2>
					<BarsList data={data.byDepartment} />
				</div>
				<div>
					<TopPublishersTable publishers={data.topPublishers} />
				</div>
			</div>
		</StatsSection>

		<ProductivitySection
			{productivity}
			open={openSection === "productivity"}
			ontoggle={() => (openSection = "productivity")}
		/>
	</div>
</div>
