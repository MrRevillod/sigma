<script lang="ts">
	import { useProductivityQuery } from "../queries"

	import type { ProductivityJceScope, ProductivityDegree, ProductivityScope } from "../dtos"
	import type { ProductivityIndexation, ProductivityPrecision } from "../productivity-labels"
	import ProductivityChart from "./productivity-chart.svelte"
	import ProductivityFilters from "./productivity-filters.svelte"

	export interface ProductivityPanelProps {
		scope: ProductivityScope
		departmentId?: string
		researchLineId?: string
		yearFrom: number
		yearTo: number
		degree?: ProductivityDegree
		jceScope?: ProductivityJceScope
	}

	let {
		scope,
		departmentId,
		researchLineId,
		yearFrom,
		yearTo,
		degree = $bindable("all"),
		jceScope = $bindable<ProductivityJceScope>("doctor"),
	}: ProductivityPanelProps = $props()

	let month = $state("1")
	let indexation = $state<ProductivityIndexation>("all")
	let precision = $state<ProductivityPrecision>("3")

	const queryParams = $derived({
		degree,
		scope,
		jceScope,
		...(departmentId ? { departmentId } : {}),
		...(researchLineId ? { researchLineId } : {}),
		month: Number(month),
		yearFrom,
		yearTo,
	})

	const productivity = useProductivityQuery(() => queryParams)
</script>

<div class="space-y-6">
	<ProductivityFilters bind:degree bind:jceScope bind:month bind:indexation bind:precision />

	<ProductivityChart
		data={productivity.data}
		isPending={productivity.isPending}
		isError={productivity.isError}
		{indexation}
		{precision}
	/>
</div>
