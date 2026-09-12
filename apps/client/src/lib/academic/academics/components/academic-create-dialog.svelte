<script lang="ts">
	import type { CreateAcademicDTO } from "$academics/dtos"

	import { toast } from "svelte-sonner"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { useMutation, useQuery, queryClient } from "$shared/http/tanstack"

	import { careerService } from "$careers/service"
	import { optionService } from "$options/service"
	import { academicService } from "$academics/service"
	import { positionService } from "$work-positions/service"
	import { categoryService } from "$categories/service"
	import { departmentService } from "$departments/service"
	import { createAcademicDTOInitialInput, createAcademicDTOSchema } from "$academics/dtos"
	import { useConfig } from "$shared/config/queries"

	import { SexValue } from "$shared/value-objects/sex.value"

	import { RotateCcw } from "@lucide/svelte"
	import Button from "$shared/components/ui/button.svelte"
	import Dialog from "$shared/components/ui/dialog.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import DatePicker from "$shared/components/ui/form/date-picker.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import NumberInput from "$shared/components/ui/form/number-input.svelte"
	import CountrySelect from "$shared/components/ui/form/country-select.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const configQuery = useConfig()
	const jceMax = $derived(configQuery.data?.jceMax ?? 42.5)

	const form = $derived.by(() => createForm({ schema: createAcademicDTOSchema(jceMax) }))

	let selectedCategoryId = $state("")
	let selectedCategoryError: string | undefined = $state(undefined)
	let selectedDeptId = $state("")

	$effect(() => {
		if (!open) return
		reset(form, { initialInput: createAcademicDTOInitialInput })
		selectedCategoryId = ""
		selectedCategoryError = undefined
		selectedDeptId = ""
	})

	const createAcad = useMutation(() => ({
		mutationFn: (output: CreateAcademicDTO) => academicService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academics"] })
			toast.success("Académico creado")
			open = false
		},
		onError: () => toast.error("Error al crear el académico"),
	}))

	const departmentsQuery = useQuery(() => ({
		queryKey: ["departments"],
		queryFn: () => departmentService.list(),
	}))

	const careersQuery = useQuery(() => ({
		queryKey: ["careers", selectedDeptId],
		queryFn: () =>
			careerService.list(selectedDeptId ? { department_id: selectedDeptId } : undefined),
		enabled: Boolean(selectedDeptId),
	}))

	const positionsQuery = useQuery(() => ({
		queryKey: ["positions"],
		queryFn: () => positionService.list(),
	}))

	const categoriesQuery = useQuery(() => ({
		queryKey: ["categories"],
		queryFn: () => categoryService.list(),
	}))

	const optionsQuery = useQuery(() => ({
		queryKey: ["category-options", selectedCategoryId],
		queryFn: () => optionService.list({ category_id: selectedCategoryId }),
		enabled: Boolean(selectedCategoryId),
	}))

	function handleSubmit(output: CreateAcademicDTO) {
		if (!selectedCategoryId) {
			selectedCategoryError = "Seleccione primero una categoría"
			return
		}
		selectedCategoryError = undefined
		createAcad.mutate(output)
	}

	function handleReset() {
		reset(form, { initialInput: createAcademicDTOInitialInput })
		selectedCategoryId = ""
		selectedCategoryError = undefined
		selectedDeptId = ""
	}

	function handleClose() {
		open = false
		onClose()
	}

	const sexOptions = $derived([
		{ label: "Seleccionar sexo...", value: "" },
		...Object.entries(SexValue.LABELS).map(([value, label]) => ({ label, value })),
	])

	const categoryData = $derived(categoriesQuery.data ?? [])
	const categoryOptions = $derived(categoryData.map((c) => ({ label: c.name, value: c.id })))

	const deptOptions = $derived(
		(departmentsQuery.data ?? []).map((d) => ({ label: d.name, value: d.id })),
	)

	const positionOptions = $derived(
		(positionsQuery.data ?? []).map((p) => ({ label: p.name, value: p.id })),
	)

	const careerOptions = $derived([
		{ label: "Sin carrera", value: "" },
		...(careersQuery.data ?? []).map((c) => ({ label: c.name, value: c.id })),
	])

	const optionOptions = $derived(
		(optionsQuery.data ?? []).map((opt) => {
			const catLabel =
				categoryData.find((c) => c.id === opt.categoryId)?.name ?? opt.categoryId
			return {
				label: `${catLabel} · ${opt.option.toDisplay()}${opt.hours != null ? ` · ${opt.hours} hrs` : ""}`,
				value: opt.id,
			}
		}),
	)
</script>

<Dialog bind:open title="Nuevo académico" class="max-w-3xl">
	<Form of={form} onsubmit={(output) => handleSubmit(output)}>
		<div class="grid gap-4">
			<div class="grid grid-cols-3 gap-4">
				<Field of={form} path={["rut"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="RUT"
							placeholder="XXXXXXXX-X"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["email"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="email"
							label="Email"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["sex"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Sexo"
							options={sexOptions}
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-3 gap-4">
				<Field of={form} path={["names"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="Nombres"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["paternalSurname"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="Apellido paterno"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["maternalSurname"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="Apellido materno"
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["birthDate"]}>
					{#snippet children(field)}
						<DatePicker
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Fecha de nacimiento"
							hint="DD/MM/AAAA"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["joinedAt"]}>
					{#snippet children(field)}
						<DatePicker
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Fecha de ingreso UCT"
							hint="DD/MM/AAAA"
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["departmentId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Departamento"
							placeholder="Seleccionar departamento..."
							options={deptOptions}
							onValueChange={(v) => (selectedDeptId = v)}
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["workPositionId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Cargo"
							placeholder="Seleccionar cargo..."
							options={positionOptions}
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["careerId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							label="Carrera"
							placeholder="Sin carrera"
							options={careerOptions}
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["nationalityCode"]}>
					{#snippet children(field)}
						<CountrySelect {...field.props} input={field.input} errors={field.errors} />
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Select
					name="category"
					label="Categoría"
					placeholder="Seleccionar categoría..."
					options={categoryOptions}
					input={selectedCategoryId}
					onValueChange={(v) => {
						selectedCategoryId = v
						selectedCategoryError = undefined
					}}
					errors={selectedCategoryError ? [selectedCategoryError] : null}
				/>
				<Field of={form} path={["acadCategoryOptionsId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Opción"
							placeholder={selectedCategoryId
								? "Seleccionar opción..."
								: "Seleccione una categoría primero"}
							options={optionOptions}
							disabled={!selectedCategoryId}
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["jce"]}>
					{#snippet children(field)}
						<NumberInput
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							label="JCE"
							hint="Horas de la jornada completa equivalente"
							min={0}
							max={jceMax}
							step={0.25}
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["annualDiscountHours"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							type="number"
							label="Horas descuento anual"
							min={0}
							step={1}
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["orcid"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							type="text"
							label="ORCID"
							placeholder="https://orcid.org/0000-0000-0000-0000"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["city"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="Ciudad"
						/>
					{/snippet}
				</Field>
			</div>

			<FormFooter onCancel={handleClose} submitLabel="Crear" isPending={createAcad.isPending}>
				<Button variant="secondary" type="button" onclick={handleReset}>
					<RotateCcw class="size-3.5" />
					Limpiar Datos
				</Button>
			</FormFooter>
		</div>
	</Form>
</Dialog>
