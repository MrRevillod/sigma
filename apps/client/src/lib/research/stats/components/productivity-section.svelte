<script lang="ts">
	import { Gauge, Info } from "@lucide/svelte"

	import type { ProductivityDegree, ProductivityJceScope } from "../dtos"
	import type { ProductivitySectionProps } from "../productivity-labels"
	import { buildProductivityDescription } from "../productivity-labels"

	import ProductivityHelpDialog from "./productivity-help-dialog.svelte"
	import ProductivityPanel from "./productivity-panel.svelte"
	import StatsSection from "./stats-section.svelte"

	interface Props {
		productivity: ProductivitySectionProps
		open: boolean
		ontoggle: () => void
	}

	let { productivity, open, ontoggle }: Props = $props()

	function initialDegree() {
		return productivity.degree
	}

	let selectedDegree = $state<ProductivityDegree>(initialDegree())

	function initialJceScope() {
		return productivity.jceScope ?? "doctor"
	}

	let selectedJceScope = $state<ProductivityJceScope>(initialJceScope())
	let showProductivityInfo = $state(false)

	const productivityDescription = $derived(
		buildProductivityDescription(selectedDegree, selectedJceScope, productivity.denominator),
	)
</script>

{#snippet productivityInfoAction()}
	<button
		type="button"
		class="flex size-10 shrink-0 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-corp-ink"
		title="Cómo se calcula este indicador"
		aria-label="Cómo se calcula este indicador"
		onclick={() => (showProductivityInfo = true)}
	>
		<Info class="size-5" />
	</button>
{/snippet}

<StatsSection
	title="Productividad por jornada completa"
	icon={Gauge}
	{open}
	{ontoggle}
	barAction={productivityInfoAction}
	description={productivityDescription}
>
	<ProductivityPanel
		bind:degree={selectedDegree}
		bind:jceScope={selectedJceScope}
		scope={productivity.scope}
		departmentId={productivity.departmentId}
		researchLineId={productivity.researchLineId}
		yearFrom={productivity.yearFrom}
		yearTo={productivity.yearTo}
	/>
</StatsSection>

<ProductivityHelpDialog bind:open={showProductivityInfo} />
