<script lang="ts">
	import type { Academic } from "$academics/entity"
	import type { UpdateAcademicDTO } from "$academics/dtos"

	import { toast } from "svelte-sonner"
	import { SexValue } from "$shared/value-objects/sex.value"
	import { countryItems } from "$shared/countries"
	import { careerService } from "$careers/service"
	import { optionService } from "$options/service"
	import { positionService } from "$work-positions/service"
	import { categoryService } from "$categories/service"
	import { departmentService } from "$departments/service"
	import { academicService } from "$academics/service"
	import { updateAcademicDTOSchema } from "$academics/dtos"
	import { useConfig } from "$shared/config/queries"

	import { queryClient, useMutation, useQuery } from "$shared/http/tanstack"
	import { createForm, Field, Form, reset, setInput } from "@formisch/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import DatePicker from "$shared/components/ui/form/date-picker.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import NumberInput from "$shared/components/ui/form/number-input.svelte"
	import CountrySelect from "$shared/components/ui/form/country-select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	interface Props {
		academic: Academic
		open: boolean
		onClose: () => void
	}

	let { academic, open = $bindable(), onClose }: Props = $props()

	const configQuery = useConfig()
	const jceMax = $derived(configQuery.data?.jceMax ?? 42.5)

	const form = $derived.by(() => createForm({ schema: updateAcademicDTOSchema(jceMax) }))

	let selectedDeptId = $state("")
	let selectedCategoryId = $state("")

	const departmentsQuery = useQuery(() => ({
		queryKey: ["departments"],
		queryFn: () => departmentService.list(),
	}))

	const positionsQuery = useQuery(() => ({
		queryKey: ["positions"],
		queryFn: () => positionService.list(),
	}))

	const categoriesQuery = useQuery(() => ({
		queryKey: ["categories"],
		queryFn: () => categoryService.list(),
	}))

	const careersQuery = useQuery(() => ({
		queryKey: ["careers", selectedDeptId],
		queryFn: () =>
			careerService.list(selectedDeptId ? { department_id: selectedDeptId } : undefined),
		enabled: Boolean(selectedDeptId),
	}))

	const optionsQuery = useQuery(() => ({
		queryKey: ["category-options", selectedCategoryId],
		queryFn: () => optionService.list({ category_id: selectedCategoryId }),
		enabled: Boolean(selectedCategoryId),
	}))

	$effect(() => {
		if (!open) return
		const matched = countryItems.find((i) => i.label.includes(academic.nationality.toDisplay()))

		selectedDeptId = academic.departmentId
		selectedCategoryId = ""

		reset(form, {
			initialInput: {
				names: academic.names,
				paternalSurname: academic.paternalSurname,
				maternalSurname: academic.maternalSurname,
				email: academic.email,
				orcid: academic.orcid ?? null,
				sex: academic.sex.code,
				birthDate: academic.birthDate.iso ?? "",
				joinedAt: academic.joinedAt.iso ?? "",
				workPositionId: academic.workPositionId,
				departmentId: academic.departmentId,
				careerId: academic.careerId,
				acadCategoryOptionsId: academic.acadCategoryOptionsId,
				city: academic.city,
				nationalityCode: matched?.value ?? "CL",
				jce: academic.jce.number,
				annualDiscountHours: academic.annualDiscountHours,
			},
		})

		void optionService.get(academic.acadCategoryOptionsId).then((option) => {
			selectedCategoryId = option.categoryId
		})
	})

	const updateAcademic = useMutation(() => ({
		mutationFn: (output: UpdateAcademicDTO) => academicService.update(academic.id, output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academic", academic.id] })
			void queryClient.invalidateQueries({ queryKey: ["academics"] })
			toast.success("Académico actualizado")
			open = false
		},
		onError: () => toast.error("Error al actualizar el académico"),
	}))

	const sexOptions = $derived(
		Object.entries(SexValue.LABELS).map(([value, label]) => ({ label, value })),
	)

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

	const categoryOptions = $derived(
		(categoriesQuery.data ?? []).map((c) => ({ label: c.name, value: c.id })),
	)

	const optionOptions = $derived(
		(optionsQuery.data ?? []).map((opt) => ({
			label: `${opt.option.toDisplay()}${opt.hours != null ? ` · ${opt.hours} hrs` : ""}`,
			value: opt.id,
		})),
	)

	function handleDepartmentChange(value: string) {
		selectedDeptId = value
		setInput(form, { path: ["careerId"], input: null })
	}
</script>

<Dialog bind:open title="Editar académico" class="max-w-4xl">
	<Form of={form} onsubmit={(output) => updateAcademic.mutate(output)}>
		<div class="grid gap-6">
			<section class="grid gap-4">
				<h3 class="text-xs font-semibold tracking-widest text-corp-blue uppercase">
					Datos personales
				</h3>
				<div class="grid grid-cols-2 gap-4 md:grid-cols-3">
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
					<Field of={form} path={["birthDate"]}>
						{#snippet children(field)}
							<DatePicker
								{...field.props}
								input={field.input}
								errors={field.errors}
								label="Fecha de nacimiento"
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
					<Field of={form} path={["nationalityCode"]}>
						{#snippet children(field)}
							<CountrySelect
								{...field.props}
								input={field.input}
								errors={field.errors}
							/>
						{/snippet}
					</Field>
				</div>
			</section>

			<section class="grid gap-4">
				<h3 class="text-xs font-semibold tracking-widest text-corp-blue uppercase">
					Información laboral
				</h3>
				<div class="grid grid-cols-2 gap-4 md:grid-cols-3">
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
					<Field of={form} path={["departmentId"]}>
						{#snippet children(field)}
							<Select
								{...field.props}
								input={field.input}
								errors={field.errors}
								label="Departamento"
								placeholder="Seleccionar departamento..."
								options={deptOptions}
								onValueChange={handleDepartmentChange}
							/>
						{/snippet}
					</Field>
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
				</div>
			</section>

			<section class="grid gap-4">
				<h3 class="text-xs font-semibold tracking-widest text-corp-blue uppercase">
					Categorización académica
				</h3>
				<div class="grid grid-cols-2 gap-4 md:grid-cols-3">
					<Select
						name="category"
						label="Categoría"
						placeholder="Seleccionar categoría..."
						options={categoryOptions}
						input={selectedCategoryId}
						errors={null}
						onValueChange={(value) => (selectedCategoryId = value)}
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
					<Field of={form} path={["annualDiscountHours"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type="number"
								label="Horas descuento anual"
								min={0}
								step={1}
							/>
						{/snippet}
					</Field>
				</div>
			</section>
		</div>

		<FormFooter onCancel={onClose} submitLabel="Guardar" isPending={updateAcademic.isPending} />
	</Form>
</Dialog>
