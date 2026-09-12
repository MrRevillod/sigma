<script lang="ts">
	import * as v from "valibot"
	import type { Academic } from "$academics/entity"
	import type { TableFeatures } from "@tanstack/svelte-table"
	import type { GetAcademicsParams } from "$academics/dtos"

	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { useSearchParams } from "runed/kit"
	import { createColumnHelper } from "@tanstack/svelte-table"
	import { Loader, CircleAlert, TriangleAlert } from "@lucide/svelte"
	import { queryClient, useMutation, useQuery } from "$shared/http/tanstack"

	import { FullName } from "$shared/value-objects/full-name.value"
	import { careerService } from "$careers/service"
	import { academicService } from "$academics/service"
	import { departmentService } from "$departments/service"
	import { categoryService } from "$categories/service"
	import type { ImportResult } from "$academics/dtos"

	import DataTable from "$shared/components/ui/data-table.svelte"
	import Dialog from "$shared/components/ui/dialog.svelte"
	import Button from "$shared/components/ui/button.svelte"
	import AcademicsFilters from "$academics/components/academics-filters.svelte"
	import AcademicCreateDialog from "$academics/components/academic-create-dialog.svelte"

	const searchParamsSchema = v.object({
		search: v.optional(v.fallback(v.string(), ""), ""),
		departmentId: v.optional(v.fallback(v.string(), ""), ""),
		careerId: v.optional(v.fallback(v.string(), ""), ""),
		categoryId: v.optional(v.fallback(v.string(), ""), ""),
		planta: v.optional(v.fallback(v.string(), ""), ""),
		option: v.optional(v.fallback(v.string(), ""), ""),
		includeUnlinked: v.optional(v.fallback(v.string(), ""), ""),
	})

	const params = useSearchParams(searchParamsSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const departmentsQuery = useQuery(() => ({
		queryKey: ["departments"],
		queryFn: () => departmentService.list(),
	}))

	const careersQuery = useQuery(() => ({
		queryKey: ["careers", params.departmentId],
		queryFn: () =>
			careerService.list(
				params.departmentId ? { department_id: params.departmentId } : undefined,
			),
	}))

	const categoriesQuery = useQuery(() => ({
		queryKey: ["categories"],
		queryFn: () => categoryService.list(),
	}))

	let filters = $derived<GetAcademicsParams>({
		...(params.search && { search: params.search }),
		...(params.departmentId && { departmentId: params.departmentId }),
		...(params.careerId && { careerId: params.careerId }),
		...(params.categoryId && { categoryId: params.categoryId }),
		...(params.planta && { planta: params.planta as GetAcademicsParams["planta"] }),
		...(params.option && { option: params.option as GetAcademicsParams["option"] }),
		...(params.includeUnlinked === "true" && { includeUnlinked: true }),
	})

	const showUnlinked = $derived(params.includeUnlinked === "true")

	function toggleUnlinked() {
		params.includeUnlinked = showUnlinked ? "" : "true"
	}

	let showCreateDialog = $state(false)
	let importResult = $state<ImportResult | null>(null)
	let showImportErrors = $state(false)

	function clearFilters() {
		params.reset()
	}

	const query = useQuery(() => ({
		queryKey: ["academics", filters],
		queryFn: () => academicService.list(filters),
	}))

	const importMutation = useMutation(() => ({
		mutationFn: (file: File) => academicService.import(file),
		onSuccess: (result) => {
			void queryClient.invalidateQueries({ queryKey: ["academics"] })
			toast.success(
				`${result.imported} importados · ${result.updated} actualizados · ${result.errors.length} con errores`,
			)
			if (result.errors.length > 0) {
				importResult = result
				showImportErrors = true
			}
		},
		onError: () => toast.error("Error al importar el archivo"),
	}))

	const helper = createColumnHelper<TableFeatures, Academic>()

	const columns = $derived([
		helper.accessor(
			(row) => FullName.of(row.names, row.paternalSurname, row.maternalSurname).format(),
			{
				id: "name",
				header: "Nombre",
			},
		),
		helper.accessor("email", { header: "Email" }),
		helper.accessor("department", { header: "Departamento" }),
		helper.accessor("category", { header: "Categoría" }),
		helper.accessor((row) => row.planta.toDisplay(), {
			id: "planta",
			header: "Planta",
		}),
		helper.accessor((row) => row.option.toDisplay(), {
			id: "option",
			header: "Opción",
		}),
		...(showUnlinked
			? [
					helper.accessor((row) => (row.isUnlinked ? "Desvinculado" : "Activo"), {
						id: "status",
						header: "Estado",
					}),
				]
			: []),
	])
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col px-4 py-8 sm:px-6 lg:px-8">
	<div class="flex min-h-0 flex-1 gap-8">
		<AcademicsFilters
			bind:search={params.search}
			bind:deptFilter={params.departmentId}
			bind:careerFilter={params.careerId}
			bind:catFilter={params.categoryId}
			bind:plantaFilter={params.planta}
			bind:optionFilter={params.option}
			departments={departmentsQuery.data}
			careers={careersQuery.data}
			categories={categoriesQuery.data}
			onClear={clearFilters}
			onCreate={() => (showCreateDialog = true)}
			onImport={(file) => importMutation.mutate(file)}
			{showUnlinked}
			onToggleUnlinked={toggleUnlinked}
		/>

		<main class="min-w-0 flex-1 overflow-y-auto">
			{#if query.isPending}
				<div class="flex items-center justify-center py-16">
					<Loader class="size-6 animate-spin text-corp-gray" />
				</div>
			{:else if query.isError}
				<div class="flex flex-col items-center justify-center py-16 text-center">
					<CircleAlert class="size-8 text-red-500" />
					<p class="mt-3 text-sm text-corp-gray">Error al cargar los académicos.</p>
				</div>
			{:else}
				<DataTable
					data={query.data ?? []}
					{columns}
					onRowClick={(row: Academic) => void goto(`/academics/${row.id}`)}
				/>
			{/if}
		</main>
	</div>
</div>

<AcademicCreateDialog bind:open={showCreateDialog} onClose={() => (showCreateDialog = false)} />

{#if importResult}
	<Dialog
		bind:open={showImportErrors}
		title="Resultado de la importación"
		description={`${importResult.imported} importados · ${importResult.updated} actualizados · ${importResult.errors.length} filas con errores`}
		contentProps={{ class: "max-w-2xl" }}
	>
		<div class="max-h-[60vh] overflow-y-auto pr-1">
			<div
				class="mb-3 flex items-center gap-2 rounded-lg bg-red-50 px-3 py-2 text-sm text-red-700"
			>
				<TriangleAlert class="size-4 shrink-0" />
				<p>
					Las filas con errores no se procesaron. Revisa los motivos y corrige el archivo
					antes de volver a intentarlo.
				</p>
			</div>
			<ul class="space-y-3">
				{#each importResult.errors as error (error.row)}
					<li class="rounded-lg border border-corp-gray/15 bg-gray-50 p-3">
						<p class="text-xs font-semibold tracking-wide text-corp-gray uppercase">
							Fila {error.row + 1}
						</p>
						<ul class="mt-1.5 list-inside list-disc space-y-0.5 text-sm text-[#1A1A1A]">
							{#each error.reasons as reason (reason)}
								<li>{reason}</li>
							{/each}
						</ul>
					</li>
				{/each}
			</ul>
		</div>
		<div class="mt-4 flex justify-end">
			<Button variant="primary" onclick={() => (showImportErrors = false)}>Entendido</Button>
		</div>
	</Dialog>
{/if}
