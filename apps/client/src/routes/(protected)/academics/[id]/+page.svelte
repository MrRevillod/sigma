<script lang="ts">
	import * as v from "valibot"
	import type { Degree } from "$degrees/entity"
	import type { JournalKind } from "$works/value-objects/journal-kind.value"

	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import { useSearchParams } from "runed/kit"
	import {
		GraduationCap,
		Briefcase,
		BookOpen,
		Loader,
		CircleAlert,
		Pencil,
		Plus,
		Network,
		Info,
		Mail,
		UserMinus,
	} from "@lucide/svelte"

	import { authStore } from "$lib/auth/store.svelte"
	import { CLf64Value } from "$shared/value-objects/cl-f64.value"
	import { degreeService } from "$degrees/service"
	import { academicService } from "$academics/service"
	import { DegreeKindValue } from "$degrees/value-objects/kind.value"
	import { useQuery, useMutation, queryClient } from "$shared/http/tanstack"

	import Badge from "$shared/components/ui/badge.svelte"
	import Button from "$shared/components/ui/button.svelte"
	import Dialog from "$shared/components/ui/dialog.svelte"
	import DegreeDialog from "$degrees/components/degree-dialog.svelte"
	import WorksSection from "$works/components/works-section.svelte"
	import AcademicStats from "$stats/components/academic-stats.svelte"
	import AcademicSidebar from "$academics/components/academic-sidebar.svelte"
	import AcademicEditDialog from "$academics/components/academic-edit-dialog.svelte"
	import CollaborationGraph from "$collaborations/components/collaboration-graph.svelte"
	import CollaborationGraphHelpDialog from "$collaborations/components/collaboration-graph-help-dialog.svelte"

	const id = $derived(page.params.id ?? "")

	const yearFromDefault = String(new Date().getFullYear() - 5)

	const filtersParamsSchema = v.object({
		yearFrom: v.optional(v.fallback(v.string(), yearFromDefault), yearFromDefault),
		yearTo: v.optional(v.fallback(v.string(), ""), ""),
		researchLineId: v.optional(v.fallback(v.string(), ""), ""),
		journalKind: v.optional(v.fallback(v.string(), ""), ""),
	})

	const filtersParams = useSearchParams(filtersParamsSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const tabParamsSchema = v.object({
		tab: v.optional(
			v.fallback(
				v.picklist(["academic-info", "publications", "stats", "collaborations"]),
				"academic-info",
			),
			"academic-info",
		),
	})

	const tabParams = useSearchParams(tabParamsSchema, { pushHistory: true })
	const activeTab = $derived(tabParams.tab)

	const academicQuery = useQuery(() => ({
		queryKey: ["academic", id],
		queryFn: () => academicService.get(id),
		enabled: Boolean(id),
	}))

	const degreesQuery = useQuery(() => ({
		queryKey: ["degrees", id],
		queryFn: () => degreeService.listByAcademic(id),
		enabled: Boolean(id),
	}))

	const academic = $derived(academicQuery.data)
	const isUnlinked = $derived(academic?.isUnlinked ?? false)
	const degreeSlots = $derived.by<
		Array<
			| (Degree & { isPlaceholder: false })
			| { kind: (typeof DegreeKindValue.KINDS)[number]; isPlaceholder: true }
		>
	>(() => {
		const degrees = degreesQuery.data ?? []
		const hasSuperior = degrees.some(
			(d) => d.kind.code === "magister" || d.kind.code === "doctor",
		)
		const slots: Array<
			| (Degree & { isPlaceholder: false })
			| { kind: (typeof DegreeKindValue.KINDS)[number]; isPlaceholder: true }
		> = []

		for (const kind of DegreeKindValue.KINDS) {
			const found = degrees.find((d) => d.kind.code === kind)
			if (found) {
				slots.push({ ...found, isPlaceholder: false as const })
			} else if (!(hasSuperior && kind !== "professional")) {
				slots.push({ kind, isPlaceholder: true as const })
			}
		}

		return slots
	})

	const degreeKindMeta: Record<
		string,
		{ label: string; badge: "base" | "advanced" | "doctor"; dot: string }
	> = {
		professional: {
			label: DegreeKindValue.LABELS.professional,
			badge: "base",
			dot: "bg-corp-blue",
		},
		magister: {
			label: DegreeKindValue.LABELS.magister,
			badge: "advanced",
			dot: "bg-corp-yellow",
		},
		doctor: {
			label: DegreeKindValue.LABELS.doctor,
			badge: "doctor",
			dot: "bg-corp-gold",
		},
	}

	const takenSuperiorKind = $derived.by<(typeof DegreeKindValue.KINDS)[number] | null>(() => {
		const current = editingDegree
		if (!current) return null
		const other = (degreesQuery.data ?? []).find(
			(d) => d.id !== current.id && (d.kind.code === "magister" || d.kind.code === "doctor"),
		)
		return other ? (other.kind.code as (typeof DegreeKindValue.KINDS)[number]) : null
	})

	let showDegreeDialog = $state(false)
	let editingDegree = $state<Degree | null>(null)
	let createKind = $state<(typeof DegreeKindValue.KINDS)[number]>("professional")

	function openCreate(k: (typeof DegreeKindValue.KINDS)[number]) {
		editingDegree = null
		createKind = k
		showDegreeDialog = true
	}

	function openEdit(deg: Degree) {
		editingDegree = deg
		showDegreeDialog = true
	}

	let showEditAcademicDialog = $state(false)
	let showGraphHelp = $state(false)
	let showSendEditCodesConfirmDialog = $state(false)
	let showUnlinkConfirmDialog = $state(false)

	const isAdmin = $derived(authStore.isAuthenticated)

	const unlinkMutation = useMutation(() => ({
		mutationFn: () => academicService.unlink(id),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academic", id] })
			void queryClient.invalidateQueries({ queryKey: ["academics"] })
			toast.success("Académico desvinculado")
			showUnlinkConfirmDialog = false
		},
		onError: () => toast.error("Error al desvincular al académico"),
	}))

	const sendEditCodesMutation = useMutation(() => ({
		mutationFn: () => academicService.sendEditCodes(id),
		onSuccess: () => toast.success("Códigos enviados al correo del académico"),
		onError: () => toast.error("Error al enviar los códigos de edición"),
	}))

	function handleSendEditCodes() {
		showSendEditCodesConfirmDialog = true
	}

	function confirmSendEditCodes() {
		showSendEditCodesConfirmDialog = false
		sendEditCodesMutation.mutate()
	}

	function closeEditAcademic() {
		showEditAcademicDialog = false
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
			<a href="/academics" class="mt-4 text-sm font-medium text-corp-blue hover:underline"
				>Volver al listado</a
			>
		</div>
	{:else}
		<div class="mx-auto max-w-[1600px] px-4 py-8 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
				<AcademicSidebar
					{academic}
					onEdit={isUnlinked ? undefined : () => (showEditAcademicDialog = true)}
					onSendCodes={isUnlinked ? undefined : handleSendEditCodes}
					onUnlink={isAdmin && !isUnlinked
						? () => (showUnlinkConfirmDialog = true)
						: undefined}
				/>

				<div class="flex h-[calc(100dvh-10rem)] flex-col">
					<div class="mb-4 flex shrink-0 rounded-lg bg-corp-gray/10 p-1">
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'academic-info'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-ink'}"
							onclick={() => (tabParams.tab = "academic-info")}
						>
							Información Académica
						</button>
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'publications'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-ink'}"
							onclick={() => (tabParams.tab = "publications")}
						>
							Publicaciones
						</button>
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'stats'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-ink'}"
							onclick={() => (tabParams.tab = "stats")}
						>
							Estadísticas
						</button>
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'collaborations'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-corp-ink'}"
							onclick={() => (tabParams.tab = "collaborations")}
						>
							Colaboraciones
						</button>
					</div>
					<div class="min-h-0 flex-1 space-y-6 overflow-y-auto">
						{#if activeTab === "academic-info"}
							<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
								<div
									class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
								>
									<Briefcase class="size-4 text-corp-blue" />
									Información Laboral
								</div>
								<div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-3">
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Departamento
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.department}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Carrera
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.career ?? "—"}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Ingreso
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.joinedAt.toDisplayDate()}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Cargo
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.workPosition ?? "—"}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Jornada Completa Equivalente
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{CLf64Value.format(academic.jce.number)} horas
										</p>
									</div>
								</div>
							</section>

							<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
								<div
									class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
								>
									<BookOpen class="size-4 text-corp-blue" />
									Categorización Académica
								</div>
								<div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-3">
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Planta
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.planta.toDisplay()}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Categoría
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.category}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Opción
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.option.toDisplay()}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Horas de categoría y opción
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{academic.acadCategoryHours?.toLocaleString("es-CL") ??
												"—"} horas
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Descuento anual
										</p>
										<p class="mt-1 text-[15px] font-medium text-corp-ink">
											{CLf64Value.format(academic.annualDiscountHours)} horas
										</p>
									</div>
								</div>
							</section>

							<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
								<div
									class="mb-6 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
								>
									<GraduationCap class="size-4 text-corp-blue" />
									Grados Académicos
								</div>

								{#if degreesQuery.isPending}
									<div class="flex items-center justify-center py-8">
										<Loader class="size-5 animate-spin text-corp-gray" />
									</div>
								{:else}
									<div class="relative">
										{#each degreeSlots as slot, i (slot.kind)}
											{@const meta =
												degreeKindMeta[
													slot.isPlaceholder ? slot.kind : slot.kind.code
												]}
											<div
												class="relative flex gap-5 {i <
												degreeSlots.length - 1
													? 'pb-8'
													: ''}"
											>
												<div class="flex flex-col items-center">
													<div
														class="z-10 size-3 shrink-0 rounded-full {slot.isPlaceholder
															? 'bg-corp-gray/30'
															: meta.dot}"
													></div>
													{#if i < degreeSlots.length - 1}
														<div
															class="mt-1 w-px grow bg-corp-gray/20"
														></div>
													{/if}
												</div>
												<div class="min-w-0 flex-1">
													<div class="mb-1 flex items-center gap-2">
														<Badge variant={meta.badge}>
															{meta.label}
														</Badge>
														{#if !slot.isPlaceholder && isAdmin}
															<button
																class="flex size-6 items-center justify-center rounded-md text-corp-gray/40 transition-colors hover:text-corp-blue"
																onclick={() => openEdit(slot)}
															>
																<Pencil class="size-3.5" />
															</button>
														{/if}
													</div>
													{#if slot.isPlaceholder && isAdmin}
														<button
															class="mt-1 inline-flex items-center gap-1.5 text-sm text-corp-gray/50 transition-colors hover:text-corp-blue"
															onclick={() => openCreate(slot.kind)}
														>
															<Plus class="size-3.5" />
															Agregar
														</button>
													{:else}
														{@const degree = slot as Degree}
														<p
															class="text-[15px] font-medium text-corp-ink"
														>
															{degree.name}
														</p>
														<p class="mt-1 text-sm text-corp-gray">
															{degree.university}
															<span class="mx-1.5 text-corp-gray/40"
																>·</span
															>
															{degree.country.toDisplay()}
															<span class="mx-1.5 text-corp-gray/40"
																>·</span
															>
															{degree.obtainedAt.toDisplayDate()}
														</p>
													{/if}
												</div>
											</div>
										{/each}
									</div>
								{/if}
							</section>
						{:else if activeTab === "collaborations"}
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
</div>

<DegreeDialog
	academicId={id}
	degree={editingDegree}
	{createKind}
	{takenSuperiorKind}
	bind:open={showDegreeDialog}
	onClose={() => (showDegreeDialog = false)}
/>

<CollaborationGraphHelpDialog bind:open={showGraphHelp} />

{#if isAdmin && academic}
	<AcademicEditDialog {academic} bind:open={showEditAcademicDialog} onClose={closeEditAcademic} />
{/if}

<Dialog
	bind:open={showSendEditCodesConfirmDialog}
	title="Enviar códigos de edición"
	description="Usa esta función para permitir que el académico edite su perfil de forma segura."
>
	<div class="space-y-4">
		<div class="rounded-lg bg-corp-blue/5 p-4">
			<p class="text-sm text-corp-ink">
				Se enviará un <strong>código de 8 caracteres</strong> al correo del académico. Con este
				código, podrá generar un enlace temporal para editar:
			</p>
			<ul class="mt-3 space-y-1 text-xs text-corp-gray">
				<li class="flex items-start gap-2">
					<span class="mt-1.5 inline-block size-1.5 shrink-0 rounded-full bg-corp-blue"
					></span>
					<span>Nombres y apellidos</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="mt-1.5 inline-block size-1.5 shrink-0 rounded-full bg-corp-blue"
					></span>
					<span>ORCID y otras ID externas</span>
				</li>
			</ul>
		</div>

		<p class="text-xs text-corp-gray">
			Esta es una forma segura de delegación: el académico no tiene acceso directo a la
			plataforma, pero puede mantener su información actualizada.
		</p>

		<div class="flex justify-end gap-2">
			<Button variant="secondary" onclick={() => (showSendEditCodesConfirmDialog = false)}>
				Cancelar
			</Button>
			<Button
				variant="primary"
				disabled={sendEditCodesMutation.isPending}
				onclick={confirmSendEditCodes}
			>
				{#if sendEditCodesMutation.isPending}
					<Loader class="size-4 animate-spin" />
					Enviando...
				{:else}
					<Mail class="size-4" />
					Enviar código
				{/if}
			</Button>
		</div>
	</div>
</Dialog>

<Dialog
	bind:open={showUnlinkConfirmDialog}
	title="Desvincular académico"
	description="El académico dejará de pertenecer a la universidad. Su historial y sus publicaciones se conservan."
>
	<div class="space-y-4">
		<div class="rounded-lg bg-red-50 p-4 text-sm text-red-800">
			<p>
				Las publicaciones del académico seguirán contando para los años en que estuvo
				vinculado, desde su fecha de ingreso hasta hoy. A partir de la desvinculación dejará
				de contar para la JCE.
			</p>
			<p class="mt-2">Esta acción no se puede revertir desde la plataforma.</p>
		</div>

		<div class="flex justify-end gap-2">
			<Button variant="secondary" onclick={() => (showUnlinkConfirmDialog = false)}>
				Cancelar
			</Button>
			<Button
				variant="danger"
				disabled={unlinkMutation.isPending}
				onclick={() => unlinkMutation.mutate()}
			>
				{#if unlinkMutation.isPending}
					<Loader class="size-4 animate-spin" />
					Desvinculando...
				{:else}
					<UserMinus class="size-4" />
					Desvincular
				{/if}
			</Button>
		</div>
	</div>
</Dialog>
