<script lang="ts">
	import { Plus, X } from "@lucide/svelte"
	import { createForm, Field, Form, handleSubmit, reset } from "@formisch/svelte"
	import { onMount } from "svelte"
	import * as v from "valibot"

	import type { WorkDetail } from "$works/entity"
	import type { AuthorDraft, AutoGrowField } from "$works/types"

	import { academicService } from "$academics/service"
	import Select from "$shared/components/ui/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import { FullName } from "$shared/value-objects/full-name.value"

	import { useResearchLinesQuery } from "$research/classification/queries"
	import {
		useClearOverridesMutation,
		useUpdateAuthorshipAffiliationsMutation,
		useUpdateOverridesMutation,
	} from "$works/queries"

	const currentYear = new Date().getFullYear()

	const yearItems = Array.from({ length: currentYear - 1900 + 1 }, (_, i) => {
		const v = String(1900 + i)
		return { value: v, label: v }
	}).toReversed()

	const editSchema = v.object({
		title: v.pipe(v.string(), v.maxLength(2000)),
		abstractText: v.nullable(v.pipe(v.string())),
		doi: v.nullable(v.pipe(v.string(), v.maxLength(500))),
		publicationYear: v.nullable(v.pipe(v.string())),
		isAccepted: v.boolean(),
		isPublished: v.boolean(),
		researchLineId: v.nullable(v.pipe(v.string())),
		journalKind: v.nullable(v.picklist(["wos", "scopus"])),
	})

	type EditData = v.InferInput<typeof editSchema>

	const journalKindItems = [
		{ value: "wos", label: "WoS" },
		{ value: "scopus", label: "Scopus" },
		{ value: "", label: "Automática (ISSN)" },
	]

	function parseJournalKind(value: string | null | undefined): "wos" | "scopus" | null {
		if (value === "wos" || value === "scopus") return value
		return null
	}

	interface Props {
		work: WorkDetail
		token?: string
		submit?: (() => Promise<void>) | null
		restore?: (() => Promise<void>) | null
		isSaving?: boolean
		onSaved: () => void
	}

	let {
		work,
		token,
		// eslint-disable-next-line no-useless-assignment -- `$bindable` writes are read by the parent
		submit = $bindable(null),
		// eslint-disable-next-line no-useless-assignment -- `$bindable` writes are read by the parent
		restore = $bindable(null),
		// eslint-disable-next-line no-useless-assignment -- `$bindable` writes are read by the parent
		isSaving = $bindable(false),
		onSaved,
	}: Props = $props()

	const form = createForm({ schema: editSchema })

	const researchLinesQuery = useResearchLinesQuery()

	const researchLineItems = $derived(
		researchLinesQuery.data?.map((rl) => ({ value: rl.id, label: rl.name })) ?? [],
	)

	const authorships = $derived(work.authorships ?? [])

	let authorDrafts = $state<AuthorDraft[]>([])
	let selectedCorrespondingOrcid = $state<string | null>(null)

	onMount(() => {
		submit = handleSubmit(form, doSave)
		restore = handleRestoreAll
		reset(form, {
			initialInput: {
				title: work.title,
				abstractText: work.abstractText,
				doi: work.doi,
				publicationYear: work.publicationYear?.toString() ?? null,
				isAccepted: work.isAccepted,
				isPublished: work.isPublished,
				researchLineId: work.researchLineId ?? null,
				journalKind: parseJournalKind(work.journalKind.code),
			} satisfies EditData,
		})
		authorDrafts = authorships.map((a) => ({
			orcid: a.orcid,
			name: a.name,
			isExternal: a.isExternal,
			affiliations: [...a.affiliations],
			draftAffiliation: "",
		}))
		selectedCorrespondingOrcid = authorships.find((a) => a.isCorresponding)?.orcid ?? null
	})

	const updateMutation = useUpdateOverridesMutation()
	const clearMutation = useClearOverridesMutation()
	const affiliationsMutation = useUpdateAuthorshipAffiliationsMutation()

	function addAffiliation(index: number) {
		const draft = authorDrafts[index]
		const value = draft.draftAffiliation.trim()
		if (!value) return
		draft.affiliations = [...draft.affiliations, value]
		draft.draftAffiliation = ""
	}

	function removeAffiliation(index: number, affIndex: number) {
		const draft = authorDrafts[index]
		draft.affiliations = draft.affiliations.filter((_, i) => i !== affIndex)
	}

	async function doSave(output: EditData) {
		isSaving = true
		try {
			const data: Record<string, unknown> = {}
			if (output.title !== work.title) data.title = output.title
			if (output.abstractText !== work.abstractText) data.abstractText = output.abstractText
			if (output.doi !== work.doi) data.doi = output.doi
			if (output.publicationYear !== (work.publicationYear?.toString() ?? null))
				data.publicationYear = output.publicationYear
					? Number(output.publicationYear)
					: null
			if (output.isAccepted !== work.isAccepted) data.isAccepted = output.isAccepted
			if (output.isPublished !== work.isPublished) data.isPublished = output.isPublished

			const nextLine = output.researchLineId || null
			const prevLine = work.researchLineId ?? null
			if (nextLine !== prevLine) {
				data.researchLineId = nextLine
			}

			const currentKind = work.journalKind.code || null
			if (output.journalKind !== currentKind) {
				data.journalKind = output.journalKind
			}

			const currentCorresponding = authorships.find((a) => a.isCorresponding)?.orcid ?? null
			if (selectedCorrespondingOrcid !== currentCorresponding) {
				data.correspondingOrcid = selectedCorrespondingOrcid
			}

			const affiliationChanges: { orcid: string; affiliations: string[] }[] = []
			for (const draft of authorDrafts) {
				const original =
					authorships.find((a) => a.orcid === draft.orcid)?.affiliations ?? []
				const cleaned = draft.affiliations.filter((a) => a.trim() !== "")
				if (JSON.stringify(original) !== JSON.stringify(cleaned)) {
					affiliationChanges.push({
						orcid: draft.orcid,
						affiliations: cleaned,
					})
				}
			}

			if (Object.keys(data).length > 0) {
				if (token) {
					await academicService.updateWorkOverridesByToken(token, work.id, data)
				} else {
					await updateMutation.mutateAsync({ id: work.id, data })
				}
			}
			for (const change of affiliationChanges) {
				if (token) {
					await academicService.updateAuthorshipAffiliationsByToken(
						token,
						work.id,
						change.orcid,
						change.affiliations,
					)
				} else {
					await affiliationsMutation.mutateAsync({
						workId: work.id,
						orcid: change.orcid,
						affiliations: change.affiliations,
					})
				}
			}
			onSaved()
		} finally {
			isSaving = false
		}
	}

	async function handleRestoreAll() {
		if (token) {
			await academicService.clearWorkOverridesByToken(token, work.id)
		} else {
			await clearMutation.mutateAsync(work.id)
		}
		onSaved()
	}
</script>

{#snippet autoGrowText(field: AutoGrowField, label: string)}
	<div class="space-y-1">
		<span class="block text-xs font-medium text-corp-gray">{label}</span>
		<textarea
			{...field.props}
			value={field.input ?? ""}
			rows={1}
			class="field-sizing-content w-full resize-none rounded-lg border border-corp-gray/20 bg-white px-3 py-2 text-sm leading-6 text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50"
		></textarea>
		{#if field.errors}
			<p class="text-xs text-red-500">{field.errors[0]}</p>
		{/if}
	</div>
{/snippet}

<Form of={form} onsubmit={doSave}>
	<div class="mt-6 space-y-8">
		<section>
			<Field of={form} path={["title"]}>
				{#snippet children(field)}
					{@render autoGrowText(field, "Título")}
				{/snippet}
			</Field>
		</section>

		<div class="lg:grid lg:grid-cols-3 lg:items-start lg:gap-6">
			<div class="min-w-0 space-y-6 lg:col-span-2">
				<section>
					<Field of={form} path={["abstractText"]}>
						{#snippet children(field)}
							{@render autoGrowText(field, "Abstract")}
						{/snippet}
					</Field>
				</section>
			</div>

			<aside class="mt-6 space-y-4 lg:mt-0">
				<section>
					<Field of={form} path={["publicationYear"]}>
						{#snippet children(field)}
							<div class="space-y-1">
								<span class="block text-xs font-medium text-corp-gray"
									>Año de publicación</span
								>
								<Select
									items={yearItems}
									value={field.input ?? ""}
									onValueChange={(v) => field.onInput(v || null)}
									placeholder="Seleccionar"
									class="w-full"
								/>
								{#if field.errors}
									<p class="text-xs text-red-500">{field.errors[0]}</p>
								{/if}
							</div>
						{/snippet}
					</Field>
				</section>

				<section>
					<Field of={form} path={["doi"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type="text"
								label="DOI"
							/>
						{/snippet}
					</Field>
				</section>

				<section>
					<Field of={form} path={["researchLineId"]}>
						{#snippet children(field)}
							<div class="space-y-1">
								<span class="block text-xs font-medium text-corp-gray"
									>Línea de investigación</span
								>
								<Select
									items={researchLineItems}
									value={field.input ?? ""}
									onValueChange={(v) => field.onInput(v || null)}
									placeholder="Seleccionar"
									class="w-full"
								/>
								{#if field.errors}
									<p class="text-xs text-red-500">{field.errors[0]}</p>
								{/if}
							</div>
						{/snippet}
					</Field>
				</section>

				<section>
					<Field of={form} path={["journalKind"]}>
						{#snippet children(field)}
							<div class="space-y-1">
								<span class="block text-xs font-medium text-corp-gray"
									>Indexación</span
								>
								<Select
									items={journalKindItems}
									value={field.input ?? ""}
									onValueChange={(v) => field.onInput(parseJournalKind(v))}
									placeholder="Seleccionar"
									class="w-full"
								/>
								<p class="text-[11px] text-corp-gray">
									Forzar WoS/Scopus o volver a la indexación automática del ISSN.
								</p>
								{#if field.errors}
									<p class="text-xs text-red-500">{field.errors[0]}</p>
								{/if}
							</div>
						{/snippet}
					</Field>
				</section>

				<section class="space-y-2">
					<span class="block text-xs font-medium text-corp-gray">Estado</span>
					<div class="flex w-full items-center justify-between">
						<Field of={form} path={["isAccepted"]}>
							{#snippet children(field)}
								<label class="flex items-center gap-2 text-sm">
									<input
										type="checkbox"
										{...field.props}
										checked={field.input ?? false}
										class="size-4 rounded border-corp-gray/30 text-corp-blue focus:ring-corp-blue/30"
									/>
									Aceptado
								</label>
							{/snippet}
						</Field>

						<Field of={form} path={["isPublished"]}>
							{#snippet children(field)}
								<label class="flex items-center gap-2 text-sm">
									<input
										type="checkbox"
										{...field.props}
										checked={field.input ?? false}
										class="size-4 rounded border-corp-gray/30 text-corp-blue focus:ring-corp-blue/30"
									/>
									Publicado
								</label>
							{/snippet}
						</Field>
					</div>
				</section>
			</aside>
		</div>

		{#if authorships.length > 0}
			<section>
				<div class="mb-3 flex flex-wrap items-center justify-between gap-2">
					<p class="text-xs font-semibold tracking-widest uppercase text-corp-blue">
						Autores
					</p>
					<p class="text-[11px] text-corp-gray">
						Solo un autor puede ser el correspondiente.
					</p>
				</div>
				<div class="grid items-start gap-4 sm:grid-cols-2">
					{#each authorDrafts as draft, index (draft.orcid)}
						<div class="min-w-0 rounded-lg border border-corp-gray/10 p-4">
							<label
								class="flex cursor-pointer items-center gap-3"
								title="Marcar como autor correspondiente"
							>
								<input
									type="radio"
									name="corresponding-author"
									value={draft.orcid}
									bind:group={selectedCorrespondingOrcid}
									class="size-4 text-corp-blue focus:ring-corp-blue/30"
								/>
								<span class="min-w-0 flex-1 text-sm font-medium text-[#1A1A1A]">
									{FullName.fromFullString(draft.name)}
								</span>
								{#if draft.isExternal}
									<span
										class="shrink-0 rounded-full bg-corp-gray/10 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-corp-gray uppercase"
									>
										Externo
									</span>
								{/if}
							</label>

							<div class="mt-4">
								<p
									class="mb-1 text-[11px] font-semibold uppercase tracking-wide text-corp-gray"
								>
									Afiliaciones
								</p>
								{#if draft.affiliations.length === 0}
									<p class="text-xs text-corp-gray">
										Sin afiliaciones registradas.
									</p>
								{:else}
									<ul class="space-y-2">
										{#each draft.affiliations as _, affIndex (affIndex)}
											<li class="flex items-center gap-2">
												<input
													type="text"
													bind:value={draft.affiliations[affIndex]}
													class="min-w-0 flex-1 rounded-lg border border-corp-gray/25 bg-white px-2.5 py-1.5 text-xs text-[#1A1A1A] focus:border-corp-blue focus:outline-none"
												/>
												<button
													type="button"
													title="Quitar afiliación"
													class="flex size-8 shrink-0 items-center justify-center rounded-md text-corp-gray transition-colors hover:bg-red-50 hover:text-red-600"
													onclick={() =>
														removeAffiliation(index, affIndex)}
												>
													<X class="size-3.5" />
												</button>
											</li>
										{/each}
									</ul>
								{/if}
								<div class="mt-2.5 flex items-center gap-2">
									<input
										type="text"
										bind:value={draft.draftAffiliation}
										placeholder="Nueva afiliación"
										class="min-w-0 flex-1 rounded-lg border border-corp-gray/25 bg-white px-2.5 py-1.5 text-sm text-[#1A1A1A] placeholder:text-corp-gray/70 focus:border-corp-blue focus:outline-none"
										onkeydown={(e) => {
											if (e.key === "Enter") {
												e.preventDefault()
												addAffiliation(index)
											}
										}}
									/>
									<button
										type="button"
										title="Añadir afiliación"
										class="flex size-8 shrink-0 items-center justify-center rounded-lg border border-corp-gray/20 text-corp-blue transition-colors hover:bg-corp-blue/5"
										onclick={() => addAffiliation(index)}
									>
										<Plus class="size-3.5" />
									</button>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</section>
		{/if}
	</div>
</Form>
