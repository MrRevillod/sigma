<script lang="ts">
	import * as v from "valibot"
	import type { JournalKind } from "$works/value-objects/journal-kind.value"

	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import { Form, Field, createForm, reset } from "@formisch/svelte"
	import { useQuery, useMutation } from "$shared/http/tanstack"
	import { useSearchParams } from "runed/kit"
	import { academicService } from "$academics/service"
	import { academicFiltersParamsDTOSchema } from "$academics/dtos"
	import { Loader, CircleAlert, CircleCheck, Network, Info, Send } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import Button from "$shared/components/ui/button.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import WorksSection from "$works/components/works-section.svelte"
	import AcademicStats from "$stats/components/academic-stats.svelte"
	import AcademicSidebar from "$academics/components/academic-sidebar.svelte"
	import CollaborationGraph from "$collaborations/components/collaboration-graph.svelte"
	import CollaborationGraphHelpDialog from "$collaborations/components/collaboration-graph-help-dialog.svelte"

	const id = $derived(page.params.id ?? "")

	const filtersParams = useSearchParams(academicFiltersParamsDTOSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const tabParamsSchema = v.object({
		tab: v.optional(
			v.fallback(v.picklist(["publications", "stats", "collaborations"]), "publications"),
			"publications",
		),
	})

	const tabParams = useSearchParams(tabParamsSchema, { pushHistory: true })
	const activeTab = $derived(tabParams.tab)

	const tabs = [
		{ key: "publications", label: "Publicaciones" },
		{ key: "stats", label: "Estadísticas" },
		{ key: "collaborations", label: "Colaboraciones" },
	] as const

	const academicQuery = useQuery(() => ({
		queryKey: ["public-academic", id],
		queryFn: () => academicService.getPublic(id),
		enabled: Boolean(id),
	}))

	const academic = $derived(academicQuery.data)

	let requestEditDialogOpen = $state(false)
	let showGraphHelp = $state(false)
	let requestSent = $state(false)

	const editCodeSchema = v.object({
		code: v.pipe(v.string(), v.trim(), v.length(8, "El código debe tener 8 caracteres")),
	})

	const editCodeForm = createForm({ schema: editCodeSchema })

	const requestEditMutation = useMutation(() => ({
		mutationFn: (code: string) => academicService.requestProfileUpdate(id, code),
		onSuccess: () => {
			requestSent = true
			toast.success("Enlace enviado a tu correo electrónico")
		},
		onError: (error) => {
			const message =
				error.code === 400
					? "Código inválido. Verifica el código enviado a tu correo."
					: "Error al solicitar la edición del perfil"
			toast.error(message)
		},
	}))

	function handleCloseRequestDialog() {
		requestEditDialogOpen = false
		requestSent = false
		reset(editCodeForm)
	}
</script>

<div class="h-full overflow-y-auto">
	{#if academicQuery.isPending}
		<div class="flex h-full items-center justify-center">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if academicQuery.isError || !academic}
		<div class="flex h-full flex-col items-center justify-center text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Académico no encontrado.</p>
		</div>
	{:else}
		<div class="mx-auto max-w-[1600px] px-4 py-8 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
				<AcademicSidebar
					{academic}
					readonly
					onRequestEdit={() => (requestEditDialogOpen = true)}
				/>
				<div class="flex h-[calc(100dvh-10rem)] flex-col">
					<div class="mb-4 flex shrink-0 rounded-lg bg-corp-gray/10 p-1">
						{#each tabs as tab (tab.key)}
							<button
								type="button"
								class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
								tab.key
									? 'bg-white text-corp-blue shadow-sm'
									: 'text-corp-gray hover:text-corp-ink'}"
								onclick={() => (tabParams.tab = tab.key)}
							>
								{tab.label}
							</button>
						{/each}
					</div>
					<div class="min-h-0 flex-1 space-y-6 overflow-y-auto">
						{#if activeTab === "collaborations"}
							<section
								class="flex h-full min-h-0 flex-col overflow-hidden rounded-xl border border-corp-gray/20 bg-white"
							>
								<div
									class="flex shrink-0 items-center justify-between gap-2 border-b border-corp-gray/10 px-6 py-4"
								>
									<div
										class="flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										<Network class="size-4 text-corp-blue" />
										Red de colaboración
									</div>
									<button
										type="button"
										title="Ayuda: cómo leer esta red"
										class="flex size-7 items-center justify-center rounded-full text-corp-gray transition-colors hover:bg-corp-gray/10 hover:text-[#1A1A1A]"
										onclick={() => (showGraphHelp = true)}
									>
										<Info class="size-4" />
									</button>
								</div>
								<div class="min-h-0 flex-1">
									<CollaborationGraph {academic} />
								</div>
							</section>
						{:else if activeTab === "stats"}
							<AcademicStats academicId={id} />
						{:else}
							<WorksSection
								{academic}
								readonly
								bind:yearFrom={filtersParams.yearFrom}
								bind:yearTo={filtersParams.yearTo}
								bind:researchLineId={filtersParams.researchLineId}
								bind:journalKind={filtersParams.journalKind as JournalKind}
							/>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}

	<Dialog
		bind:open={requestEditDialogOpen}
		title="Solicitar edición de perfil"
		description="Ingresa el código de 8 caracteres que recibiste por correo electrónico para recibir un enlace de edición."
	>
		{#if requestSent}
			<div class="flex flex-col items-center py-4 text-center">
				<CircleCheck class="size-10 text-green-500" />
				<p class="mt-3 text-sm text-corp-gray">
					Revisa tu correo electrónico. Hemos enviado un enlace para editar tu perfil.
				</p>
			</div>

			<div class="flex justify-end gap-2">
				<Button variant="primary" onclick={handleCloseRequestDialog}>Aceptar</Button>
			</div>
		{:else}
			<Form of={editCodeForm} onsubmit={(output) => requestEditMutation.mutate(output.code)}>
				<div class="space-y-4">
					<Field of={editCodeForm} path={["code"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type="text"
								label="Código de autorización"
								placeholder="XXXXXXXX"
							/>
						{/snippet}
					</Field>

					<div class="flex justify-end gap-2">
						<Button variant="secondary" onclick={handleCloseRequestDialog}>
							Cancelar
						</Button>
						<Button
							type="submit"
							variant="primary"
							disabled={requestEditMutation.isPending}
						>
							{#if requestEditMutation.isPending}
								<Loader class="size-4 animate-spin" />
								Solicitando...
							{:else}
								<Send class="size-4" />
								Solicitar enlace
							{/if}
						</Button>
					</div>
				</div>
			</Form>
		{/if}
	</Dialog>

	<CollaborationGraphHelpDialog bind:open={showGraphHelp} />
</div>
