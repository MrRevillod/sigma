<script lang="ts">
	import type {
		CollaborationRecommendationDTO,
		RecommendationWorkDTO,
	} from "$collaborations/dtos"
	import type { Interest } from "$collaborations/types"

	import { Flame, Tag } from "@lucide/svelte"
	import { goto } from "$app/navigation"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import HtmlRenderer from "$shared/components/ui/html-renderer.svelte"
	import { authStore } from "$lib/auth/store.svelte"
	import { FullName } from "$shared/value-objects/full-name.value"

	interface Props {
		open: boolean
		recommendation: CollaborationRecommendationDTO | null
		focusName: string
		focusDepartment: string
	}

	let { open = $bindable(false), recommendation, focusName, focusDepartment }: Props = $props()

	const fullName = $derived(
		recommendation
			? FullName.of(
					recommendation.names,
					recommendation.paternalSurname,
					recommendation.maternalSurname,
				).format()
			: "",
	)

	const profileHref = $derived(
		recommendation
			? `${authStore.isAuthenticated ? "/academics" : "/public/academics"}/${recommendation.academicId}`
			: "#",
	)

	const interests = $derived.by<Interest[]>(() => {
		const byKey: Record<string, Interest> = {}
		for (const work of recommendation?.works ?? []) {
			for (const item of work.shared) {
				const key = `${item.type}:${item.id}`
				const existing = byKey[key]
				if (existing) {
					existing.bestScore = Math.max(existing.bestScore, item.score)
					existing.candidateWorks.push(work)
				} else {
					byKey[key] = {
						type: item.type,
						id: item.id,
						name: item.name,
						bestScore: item.score,
						focusWorks: [],
						candidateWorks: [work],
					}
				}
			}
		}
		for (const work of recommendation?.focusWorks ?? []) {
			for (const item of work.shared) {
				const existing = byKey[`${item.type}:${item.id}`]
				if (existing) {
					existing.focusWorks.push(work)
				}
			}
		}
		return Object.values(byKey).sort((a, b) => b.bestScore - a.bestScore)
	})

	function typePillClass(type: "topic" | "keyword"): string {
		return type === "topic"
			? "bg-green-600/10 text-green-600"
			: "bg-corp-gray/10 text-corp-gray"
	}

	function scorePillClass(score: number): string {
		if (score >= 0.8) return "bg-green-600/10 text-green-700"
		if (score >= 0.6) return "bg-corp-blue/10 text-corp-blue"
		return "bg-corp-gray/10 text-corp-gray"
	}
</script>

<Dialog
	bind:open
	title="Posible colaboración"
	description="Aún sin publicaciones compartidas, comparten intereses de investigación."
	class="max-w-3xl"
>
	{#if !recommendation}
		<p class="py-8 text-center text-sm text-corp-gray">No hay información para mostrar.</p>
	{:else}
		<div class="flex max-h-[70vh] flex-col">
			<div class="min-h-0 flex-1 space-y-5 overflow-y-auto pr-1">
				<div class="grid grid-cols-2 gap-3">
					<div class="rounded-xl border border-corp-gray/20 bg-white p-3">
						<p class="flex items-center gap-1.5 text-sm font-medium text-[#1A1A1A]">
							<span class="size-2.5 shrink-0 rounded-full bg-corp-blue"></span>
							<span class="truncate">{focusName}</span>
						</p>
						<p class="mt-0.5 truncate text-xs text-corp-gray">{focusDepartment}</p>
					</div>
					<div class="rounded-xl border border-corp-gray/20 bg-white p-3">
						<p class="flex items-center gap-1.5 text-sm font-medium text-[#1A1A1A]">
							<span class="size-2.5 shrink-0 rounded-full bg-green-500"></span>
							<span class="truncate">{fullName}</span>
						</p>
						<p class="mt-0.5 truncate text-xs text-corp-gray">
							{recommendation.department}
						</p>
					</div>
				</div>

				<section>
					<h3
						class="flex items-center gap-2 text-xs font-semibold uppercase tracking-widest text-corp-blue"
					>
						<Tag class="size-3.5 shrink-0" />
						En qué coinciden
					</h3>
					{#if interests.length === 0}
						<p class="mt-3 text-sm text-corp-gray">
							No hay intereses compartidos para mostrar.
						</p>
					{:else}
						<ul class="mt-2 space-y-2">
							{#each interests as interest (`${interest.type}:${interest.id}`)}
								<li class="rounded-xl border border-corp-gray/15 bg-white p-3">
									<div class="flex items-center gap-3">
										<span
											class={`shrink-0 rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide ${typePillClass(interest.type)}`}
										>
											{interest.type}
										</span>
										<span
											class="min-w-0 flex-1 truncate text-sm font-medium text-[#1A1A1A]"
										>
											{interest.name}
										</span>
										<span
											class="hidden h-1 w-20 shrink-0 overflow-hidden rounded-full bg-corp-gray/10 sm:block"
										>
											<span
												class="block h-full rounded-full bg-corp-blue"
												style:width={`${Math.round(interest.bestScore * 100)}%`}
											></span>
										</span>
										<span
											class={`shrink-0 rounded-full px-2 py-0.5 text-[10px] font-semibold tabular-nums ${scorePillClass(interest.bestScore)}`}
										>
											{Math.round(interest.bestScore * 100)}%
										</span>
									</div>

									<div
										class="mt-2 flex flex-wrap items-center gap-x-1.5 gap-y-1 text-xs text-corp-gray"
									>
										<span class="shrink-0">Aparece en:</span>
										{#each interest.focusWorks as work (work.workId)}
											{@render WorkChip(work, "focus")}
										{/each}
										{#if interest.focusWorks.length > 0}
											<span class="shrink-0">(foco)</span>
										{/if}
										{#if interest.focusWorks.length > 0 && interest.candidateWorks.length > 0}
											<span class="shrink-0">·</span>
										{/if}
										{#each interest.candidateWorks as work (work.workId)}
											{@render WorkChip(work, "candidate")}
										{/each}
										{#if interest.candidateWorks.length > 0}
											<span class="shrink-0">(candidato)</span>
										{/if}
									</div>
								</li>
							{/each}
						</ul>
					{/if}
				</section>
			</div>

			<div
				class="mt-4 flex shrink-0 items-center justify-between gap-2 border-t border-corp-gray/10 pt-3"
			>
				<div class="flex items-center gap-2">
					<Flame class="size-4 shrink-0 text-corp-gray" />
					<p class="text-xs text-corp-gray">
						{fullName} tiene {recommendation.totalWorks} publicaciones registradas.
					</p>
				</div>
				<a
					href={profileHref}
					class="shrink-0 text-xs font-semibold text-corp-blue hover:underline"
				>
					Ir al perfil académico →
				</a>
			</div>
		</div>
	{/if}
</Dialog>

{#snippet WorkChip(work: RecommendationWorkDTO, side: "focus" | "candidate")}
	<button
		type="button"
		class={`flex min-w-0 max-w-52 items-center gap-1 rounded-full px-2 py-0.5 text-[11px] font-medium hover:underline ${
			side === "focus" ? "bg-corp-blue/5 text-corp-blue" : "bg-green-600/5 text-green-700"
		}`}
		title={work.title}
		onclick={() => goto(`/works/${work.workId}`)}
	>
		{#if work.publicationYear}
			<span class="shrink-0 tabular-nums">{work.publicationYear}</span>
		{/if}
		<HtmlRenderer html={work.title} class="pointer-events-none min-w-0 truncate leading-snug" />
	</button>
{/snippet}
