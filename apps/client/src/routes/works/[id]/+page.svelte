<script lang="ts">
	import {
		CircleAlert,
		ArrowLeft,
		Check,
		ChevronDown,
		ExternalLink,
		FileText,
		Folder,
		FolderOpen,
		Loader,
		Network,
		Pencil,
		RotateCcw,
		Tag,
		TriangleAlert,
		X,
	} from "@lucide/svelte"

	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { authStore } from "$auth/store.svelte"
	import { DateValue } from "$shared/value-objects/date.value"
	import { WORK_TYPE_LABELS } from "$works/dtos"
	import { useWorkDetailQuery } from "$works/queries"

	import Badge from "$shared/components/ui/badge.svelte"
	import Dialog from "$shared/components/ui/dialog.svelte"
	import HtmlRenderer from "$shared/components/ui/html-renderer.svelte"
	import WorkEditForm from "$works/components/work-edit-form.svelte"
	import WorkAuthorsList from "$works/components/work-authors-list.svelte"

	const id = $derived(page.params.id ?? "")

	const query = useWorkDetailQuery(() => id)

	let editing = $state(false)
	let editSubmit = $state<(() => Promise<void>) | null>(null)
	let editRestore = $state<(() => Promise<void>) | null>(null)
	let showRestoreConfirm = $state(false)
	let isSaving = $state(false)
	let expandedTopicId = $state<string | null>(null)

	function toggleTopic(id: string) {
		expandedTopicId = expandedTopicId === id ? null : id
	}

	function goBack() {
		if (window.history.length > 1) {
			history.back()
		} else {
			void goto("/works")
		}
	}

	async function confirmRestore() {
		showRestoreConfirm = false
		await editRestore?.()
	}
</script>

<div class="h-full overflow-y-auto">
	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<div class="flex w-full items-center justify-between gap-3">
			<button
				type="button"
				onclick={goBack}
				class="inline-flex items-center gap-1 text-sm text-corp-blue transition-colors hover:text-corp-blue/80 active:scale-[0.96]"
			>
				<ArrowLeft class="size-3.5" />
				Volver al catálogo
			</button>

			{#if query.data}
				{#if editing}
					<div class="flex items-center gap-2">
						<button
							type="button"
							onclick={() => (showRestoreConfirm = true)}
							disabled={isSaving}
							class="inline-flex items-center gap-1.5 rounded-lg border border-red-200 px-3 py-1.5 text-sm font-medium text-red-600 transition-colors hover:border-red-300 hover:bg-red-50 active:scale-[0.96] disabled:pointer-events-none disabled:opacity-50"
						>
							<RotateCcw class="size-4" />
							Restaurar originales
						</button>
						<button
							type="button"
							onclick={() => (editing = false)}
							disabled={isSaving}
							class="inline-flex items-center gap-1.5 rounded-lg border border-corp-gray/20 px-3 py-1.5 text-sm font-medium text-corp-gray transition-colors hover:border-corp-gray/40 hover:text-[#1A1A1A] active:scale-[0.96] disabled:pointer-events-none disabled:opacity-50"
						>
							<X class="size-4" />
							Cancelar
						</button>
						<button
							type="button"
							onclick={() => editSubmit?.()}
							disabled={isSaving}
							class="inline-flex items-center gap-1.5 rounded-lg bg-corp-blue px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-corp-blue/90 active:scale-[0.96] disabled:pointer-events-none disabled:opacity-60"
						>
							<Check class="size-4" />
							{isSaving ? "Guardando…" : "Guardar"}
						</button>
					</div>
				{:else if authStore.isAuthenticated}
					<button
						type="button"
						onclick={() => (editing = true)}
						class="inline-flex items-center gap-1.5 rounded-lg border border-corp-gray/20 px-3 py-1.5 text-sm font-medium text-corp-gray transition-colors hover:border-corp-blue/30 hover:text-corp-blue active:scale-[0.96]"
					>
						<Pencil class="size-4" />
						Editar
					</button>
				{/if}
			{/if}
		</div>

		{#if query.isPending}
			<div class="flex justify-center py-12">
				<Loader class="size-6 animate-spin text-corp-gray" />
			</div>
		{:else if query.isError || !query.data}
			<div class="flex flex-col items-center py-12 text-center">
				<CircleAlert class="size-8 text-red-500" />
				<p class="mt-3 text-sm text-corp-gray">Error al cargar la obra.</p>
			</div>
		{:else}
			{#if editing}
				<WorkEditForm
					work={query.data}
					bind:submit={editSubmit}
					bind:restore={editRestore}
					bind:isSaving
					onSaved={() => (editing = false)}
				/>
			{:else}
				{@const work = query.data}
				<article class="mt-6">
					<header>
						<div class="flex flex-wrap items-center gap-2">
							<Badge variant="base">{WORK_TYPE_LABELS[work.ty] ?? work.ty}</Badge>
							{#if work.isAccepted}
								<span
									class="inline-flex items-center rounded-full bg-emerald-50 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-emerald-700 uppercase"
								>
									Aceptado
								</span>
							{/if}
							{#if work.isPublished}
								<span
									class="inline-flex items-center rounded-full bg-corp-blue/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-corp-blue uppercase"
								>
									Publicado
								</span>
							{/if}
							{#if work.publicationDate}
								<span class="text-sm text-corp-gray tabular-nums">
									{DateValue.formatDate(work.publicationDate)}
								</span>
							{:else if work.publicationYear}
								<span class="text-sm text-corp-gray tabular-nums">
									{work.publicationYear}
								</span>
							{/if}
						</div>
						<h1 class="mt-3 text-2xl font-semibold text-balance text-[#1A1A1A]">
							<HtmlRenderer html={work.title} />
						</h1>
						<div
							class="mt-3 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-corp-gray"
						>
							{#if work.doi}
								<a
									href={work.doi}
									target="_blank"
									rel="noopener"
									class="inline-flex items-center gap-1 text-corp-blue hover:underline"
								>
									<span>Ver DOI</span>
									<ExternalLink class="size-3" />
								</a>
							{/if}
							<a
								href={work.openalexId}
								target="_blank"
								rel="noopener"
								class="inline-flex items-center gap-1 text-corp-blue hover:underline"
							>
								<span>Ver en OpenAlex</span>
								<ExternalLink class="size-3" />
							</a>
							<span>Idioma: {work.lang}</span>
						</div>
					</header>

					<div
						class="mt-8 lg:grid lg:grid-cols-[minmax(0,1fr)_22rem] lg:items-start lg:gap-10"
					>
						<div class="min-w-0 space-y-8">
							<section>
								<h2
									class="mb-3 text-xs font-semibold tracking-widest uppercase text-corp-blue"
								>
									Abstract
									{#if work.isFieldOverridden("abstractText")}
										<span
											class="ml-1 text-[10px] italic font-normal text-corp-blue/60"
											>(editado)</span
										>
									{/if}
								</h2>
								{#if work.abstractText}
									<HtmlRenderer
										tag="p"
										html={work.abstractText}
										class="text-pretty text-sm leading-relaxed text-[#1A1A1A]"
									/>
								{:else}
									<p class="text-sm italic text-corp-gray">
										No se encuentra disponible. Puedes añadirlo manualmente
										editando esta publicación.
									</p>
								{/if}
							</section>

							{#if work.authorships.length > 0}
								<section>
									<h2
										class="mb-3 flex items-center gap-1.5 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										Autores
										<span
											class="rounded-full bg-corp-gray/10 px-1.5 py-0.5 text-[10px] font-semibold text-corp-gray"
										>
											{work.authorships.length}
										</span>
									</h2>
									<WorkAuthorsList authors={work.authorships} />
								</section>
							{/if}
						</div>

						<aside class="mt-8 space-y-4 lg:mt-0">
							{#if work.source}
								<section class="rounded-xl border border-corp-gray/20 bg-white p-4">
									<h2
										class="mb-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										Publicado en
									</h2>
									<p class="text-sm font-medium text-[#1A1A1A]">
										{work.source.name}
									</p>
									<hr class="my-3 border-corp-gray/10" />
									<div class="flex items-center gap-2">
										<span
											class="text-xs font-semibold uppercase tracking-widest text-corp-blue"
											>Indexación:</span
										>
										{#if work.journalKind.code}
											<Badge
												variant={work.journalKind.code === "scopus"
													? "advanced"
													: "base"}
											>
												{work.journalKind.toDisplay()}
											</Badge>
										{:else}
											<span
												class="inline-flex items-center rounded-full bg-red-100 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-red-700 uppercase"
											>
												Sin indexar
											</span>
										{/if}
										{#if work.isFieldOverridden("journalKind")}
											<span class="text-[10px] italic text-corp-blue/60">
												(editado)
											</span>
										{/if}
									</div>
								</section>
							{:else if work.journalKind.code || work.isFieldOverridden("journalKind")}
								<section class="rounded-xl border border-corp-gray/20 bg-white p-4">
									<h2
										class="mb-2 flex items-center gap-1.5 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										<Tag class="size-3" />
										Indexación
									</h2>
									<div class="flex items-center gap-2">
										{#if work.journalKind.code}
											<Badge
												variant={work.journalKind.code === "scopus"
													? "advanced"
													: "base"}
											>
												{work.journalKind.toDisplay()}
											</Badge>
										{:else}
											<span
												class="inline-flex items-center rounded-full bg-red-100 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-red-700 uppercase"
											>
												Sin indexar
											</span>
										{/if}
										{#if work.isFieldOverridden("journalKind")}
											<span class="text-[10px] italic text-corp-blue/60">
												(editado)
											</span>
										{/if}
									</div>
								</section>
							{/if}

							{#if work.researchLineName}
								<section class="rounded-xl border border-corp-gray/20 bg-white p-4">
									<h2
										class="mb-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										Línea de Investigación
										{#if work.isFieldOverridden("researchLineId")}
											<span
												class="ml-1 text-[10px] italic font-normal text-corp-blue/60"
												>(editado)</span
											>
										{/if}
									</h2>
									<p class="text-sm font-medium text-[#1A1A1A]">
										{work.researchLineName}
									</p>
								</section>
							{/if}

							{#if work.topics.length > 0}
								<section class="rounded-xl border border-corp-gray/20 bg-white p-4">
									<h2
										class="mb-2 flex items-center gap-1.5 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										<Network class="size-3" />
										Topics
									</h2>
									<div class="space-y-1">
										{#each work.topics as t (t.topicId)}
											<div>
												<button
													type="button"
													class="flex w-full items-center gap-2 rounded-md px-1 py-0.5 text-left transition-colors hover:bg-corp-gray/5"
													onclick={() => toggleTopic(t.topicId)}
													aria-expanded={expandedTopicId === t.topicId}
												>
													<span
														class="min-w-0 flex-1 truncate text-sm font-medium text-[#1A1A1A]"
													>
														{t.name}
													</span>
													<span
														class="shrink-0 rounded-full bg-corp-blue/10 px-2 py-0.5 text-xs font-semibold text-corp-blue tabular-nums"
													>
														{(t.score * 100).toFixed(1)}%
													</span>
													<ChevronDown
														class={`size-4 shrink-0 text-corp-gray transition-transform ${
															expandedTopicId === t.topicId
																? "rotate-180"
																: ""
														}`}
													/>
												</button>

												{#if expandedTopicId === t.topicId}
													<div
														class="mt-1 rounded-md bg-corp-gray/3 p-2 text-xs"
													>
														<div class="flex items-center gap-1.5">
															<FolderOpen
																class="size-3.5 shrink-0 text-corp-gray/60"
															/>
															<span class="truncate text-corp-gray">
																{t.domainName}
															</span>
														</div>
														<div
															class="ml-1.5 mt-1 space-y-1 border-l border-corp-gray/15 pl-3"
														>
															<div class="flex items-center gap-1.5">
																<Folder
																	class="size-3.5 shrink-0 text-corp-gray/60"
																/>
																<span
																	class="truncate text-corp-gray"
																>
																	{t.fieldName}
																</span>
															</div>
															<div class="flex items-center gap-1.5">
																<Folder
																	class="size-3.5 shrink-0 text-corp-gray/60"
																/>
																<span
																	class="truncate text-corp-gray"
																>
																	{t.subfieldName}
																</span>
															</div>
															<div class="flex items-center gap-1.5">
																<FileText
																	class="size-3.5 shrink-0 text-corp-blue/70"
																/>
																<span
																	class="truncate font-medium text-[#1A1A1A]"
																>
																	{t.name}
																</span>
															</div>
														</div>
													</div>
												{/if}
											</div>
										{/each}
									</div>
								</section>
							{/if}

							{#if work.keywords.length > 0}
								<section class="rounded-xl border border-corp-gray/20 bg-white p-4">
									<h2
										class="mb-2 flex items-center gap-1.5 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										<Tag class="size-3" />
										Keywords
									</h2>
									<div class="flex flex-wrap gap-1.5">
										{#each work.keywords as k (k.keywordId)}
											<span
												class="inline-flex items-center gap-1 rounded-full bg-corp-gray/10 px-2.5 py-1 text-xs text-corp-gray"
											>
												{k.name}
												<span
													class="tabular-nums text-[10px] text-corp-gray/70"
												>
													{(k.score * 100).toFixed(0)}%
												</span>
											</span>
										{/each}
									</div>
								</section>
							{/if}
						</aside>
					</div>
				</article>
			{/if}
		{/if}
	</div>
</div>

<Dialog
	bind:open={showRestoreConfirm}
	title="Restaurar valores originales"
	description="Esta acción no se puede deshacer."
	class="max-w-md"
>
	<div class="flex items-start gap-3">
		<div class="flex size-9 shrink-0 items-center justify-center rounded-full bg-red-50">
			<TriangleAlert class="size-4.5 text-red-600" />
		</div>
		<div>
			<p class="text-sm text-[#1A1A1A]">
				Se descartarán todos los cambios manuales y la obra volverá a mostrar sus valores
				originales.
			</p>
			<p class="mt-1 text-xs text-corp-gray">
				Si luego quieres volver a editar, podrás hacerlo desde el botón "Editar".
			</p>
		</div>
	</div>
	<div class="mt-5 flex justify-end gap-2">
		<button
			type="button"
			onclick={() => (showRestoreConfirm = false)}
			class="inline-flex items-center rounded-lg border border-corp-gray/20 px-3 py-1.5 text-sm font-medium text-corp-gray transition-colors hover:border-corp-gray/40 hover:text-[#1A1A1A] active:scale-[0.96]"
		>
			Cancelar
		</button>
		<button
			type="button"
			onclick={confirmRestore}
			class="inline-flex items-center gap-1.5 rounded-lg bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-red-700 active:scale-[0.96]"
		>
			<RotateCcw class="size-4" />
			Restaurar
		</button>
	</div>
</Dialog>
