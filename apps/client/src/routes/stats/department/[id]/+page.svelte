<script lang="ts">
	import * as v from "valibot"

	import type { DepartmentDetailQuery } from "$stats/dtos"

	import { page } from "$app/state"
	import { useSearchParams } from "runed/kit"
	import { useDepartmentDetailQuery } from "$stats/queries"
	import { CircleAlert, Loader, RotateCcw } from "@lucide/svelte"

	import Button from "$shared/components/ui/button.svelte"
	import Select from "$shared/components/ui/select.svelte"
	import YearRange from "$shared/components/ui/year-range.svelte"
	import DepartmentStats from "$stats/components/department-stats.svelte"

	const deptId = $derived(page.params.id ?? "")
	const currentYear = new Date().getFullYear()
	const defaultYearFrom = String(currentYear - 5)
	const defaultYearTo = String(currentYear)

	const searchParamsSchema = v.object({
		yearFrom: v.optional(v.fallback(v.string(), defaultYearFrom), defaultYearFrom),
		yearTo: v.optional(v.fallback(v.string(), defaultYearTo), defaultYearTo),
		option: v.optional(v.fallback(v.string(), ""), ""),
		journalKind: v.optional(v.fallback(v.string(), ""), ""),
		limit: v.optional(v.fallback(v.string(), "10"), "10"),
	})

	const params = useSearchParams(searchParamsSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const queryParams = $derived<DepartmentDetailQuery>({
		yearFrom: Number(params.yearFrom),
		yearTo: Number(params.yearTo),
		...(params.option && {
			option: params.option as "teaching" | "research",
		}),
		...(params.journalKind && {
			journalKind: params.journalKind as "wos" | "scopus",
		}),
		limit: Number(params.limit),
	})

	const optionItems = [
		{ value: "", label: "Todas" },
		{ value: "teaching", label: "Docencia" },
		{ value: "research", label: "Investigación" },
	]

	const detailQuery = useDepartmentDetailQuery(
		() => deptId,
		() => queryParams,
	)
</script>

<div class="flex flex-col">
	{#if detailQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if detailQuery.isError || !detailQuery.data}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar los datos del departamento.</p>
		</div>
	{:else}
		<div class="mb-6 flex flex-wrap items-start justify-between gap-4">
			<div>
				<h1 class="text-xl font-semibold text-corp-ink">
					Departamento de {detailQuery.data.department}
				</h1>
				<p class="mt-1 text-sm text-corp-gray">Detalle de publicaciones por departamento</p>
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
				<Select
					items={optionItems}
					bind:value={params.option}
					placeholder="Opción Académica"
					class="min-w-48"
				/>
				<Button variant="secondary" onclick={() => params.reset()}>
					<RotateCcw class="size-3.5" />
					Limpiar
				</Button>
			</div>
		</div>

		<DepartmentStats
			data={detailQuery.data}
			bind:limit={params.limit}
			productivity={{
				denominator: "del departamento",
				degree: "magister",
				scope: "department",
				departmentId: deptId,
				yearFrom: Number(params.yearFrom),
				yearTo: Number(params.yearTo),
			}}
		/>
	{/if}
</div>
