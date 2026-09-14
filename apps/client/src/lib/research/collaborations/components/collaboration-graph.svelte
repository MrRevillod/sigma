<script lang="ts">
	import type { Academic } from "$academics/entity"
	import type {
		CollaborationRecommendationDTO,
		CollaborationWorkRefDTO,
	} from "$collaborations/dtos"

	import { forceCenter, forceCollide, forceLink, forceManyBody } from "d3-force"
	import { Chart, Circle, Layer, Link, Tooltip, type ChartState } from "layerchart"
	import { ForceSimulation } from "layerchart/force"
	import { CircleAlert, Loader, Network, SlidersHorizontal, X } from "@lucide/svelte"
	import { Switch } from "bits-ui"

	import { useCollaborationGraphQuery } from "$collaborations/queries"
	import { FullName } from "$shared/value-objects/full-name.value"
	import { useDebounce } from "runed"

	import type { GraphLink, GraphNode } from "../graph-types"

	import EdgeWorksDialog from "./edge-works-dialog.svelte"
	import RecommendationDetailDialog from "./recommendation-detail-dialog.svelte"

	interface Props {
		academic: Academic
	}

	let { academic }: Props = $props()

	let scoreThreshold = $state(60)
	let minCoincidences = $state(2)
	let controlsOpen = $state(true)

	let appliedScoreThreshold = $state(0.6)
	let appliedMinCoincidences = $state(2)

	const applyScoreThreshold = useDebounce((value: number) => {
		appliedScoreThreshold = value / 100
	}, 300)
	const applyMinCoincidences = useDebounce((value: number) => {
		appliedMinCoincidences = value
	}, 300)

	const query = useCollaborationGraphQuery(
		() => academic.id,
		() => ({
			scoreThreshold: appliedScoreThreshold,
			minCoincidences: appliedMinCoincidences,
		}),
	)
	const graph = $derived(query.data)

	const isEmptyGraph = $derived(
		!graph || (graph.nodes.length <= 1 && graph.recommendations.length === 0),
	)

	const focusName = $derived(
		FullName.of(academic.names, academic.paternalSurname, academic.maternalSurname).format(),
	)

	const graphNodes = $derived.by<GraphNode[]>(() => {
		const coauthorNodes: GraphNode[] = (graph?.nodes ?? []).map((n) => ({
			...n,
			displayName: FullName.fromFullString(n.name),
			kind: n.id === academic.id ? "focus" : "coauthor",
			weight: 0,
		}))
		const recommendationNodes: GraphNode[] = (graph?.recommendations ?? [])
			.filter((r) => !graph?.nodes.some((n) => n.id === r.academicId))
			.map((r) => ({
				id: r.academicId,
				name: r.name,
				department: r.department,
				totalWorks: r.totalWorks,
				displayName: FullName.fromFullString(r.name),
				kind: "recommendation",
				weight: r.weight,
			}))
		return [...coauthorNodes, ...recommendationNodes]
	})

	const graphLinks = $derived.by<GraphLink[]>(() => {
		const coauthorLinks: GraphLink[] = (graph?.edges ?? []).map((e) => ({
			source: e.sourceId,
			target: e.targetId,
			weight: e.weight,
			kind: "coauthor",
			works: e.works,
		}))
		const recommendationLinks: GraphLink[] = (graph?.recommendations ?? []).map((r) => ({
			source: academic.id,
			target: r.academicId,
			weight: r.weight,
			kind: "recommendation",
			works: [],
		}))
		return [...coauthorLinks, ...recommendationLinks]
	})

	const maxWorks = $derived(Math.max(1, ...graphNodes.map((n) => n.totalWorks)))
	const maxWeight = $derived(Math.max(1, ...graphLinks.map((l) => l.weight)))

	function nodeRadius(node: GraphNode): number {
		if (node.kind === "focus") return 12
		return 7 + (node.totalWorks / maxWorks) * 9
	}

	function edgeWidth(link: GraphLink): number {
		if (link.kind === "recommendation") return 0.75 + (link.weight / maxWeight) * 1.25
		return 1 + (link.weight / maxWeight) * 2
	}

	function linkEndpointId(value: string | number | GraphNode): string {
		return typeof value === "object" && value !== null ? value.id : String(value)
	}

	function linkMidpoint(link: GraphLink): { x: number; y: number } {
		const s = link.source as unknown as GraphNode | string
		const t = link.target as unknown as GraphNode | string
		const sx = typeof s === "object" && s !== null ? (s.x ?? 0) : 0
		const sy = typeof s === "object" && s !== null ? (s.y ?? 0) : 0
		const tx = typeof t === "object" && t !== null ? (t.x ?? 0) : 0
		const ty = typeof t === "object" && t !== null ? (t.y ?? 0) : 0
		return { x: (sx + tx) / 2, y: (sy + ty) / 2 }
	}

	function isFocusEdge(link: GraphLink): boolean {
		const s = linkEndpointId(link.source)
		const t = linkEndpointId(link.target)
		return s === academic.id || t === academic.id
	}

	function nodeFill(node: GraphNode): string {
		if (node.kind === "focus") return "#0075B4"
		if (node.kind === "recommendation") return "#F0FDF4"
		return "#5B9FD4"
	}

	function nodeStroke(node: GraphNode): string {
		if (node.kind === "focus") return "#0075B4"
		if (node.kind === "recommendation") return "#22C55E"
		return "#B9CFDF"
	}

	function edgeClass(link: GraphLink): string {
		if (link.kind === "recommendation") return "stroke-green-500"
		return isFocusEdge(link) ? "stroke-corp-blue" : "stroke-corp-gray"
	}

	let hoveredId = $state<string | null>(null)
	let showCollaborations = $state(true)
	let showSuggestions = $state(true)
	let showWeights = $state(false)
	let linkDistance = $state(130)
	let chartContext = $state<ChartState>()

	const zoomPercent = $derived(Math.round((chartContext?.transform.scale ?? 1) * 100))

	function setZoomPercent(value: number) {
		chartContext?.transform.setScale(value / 100)
	}

	const neighbors = $derived.by(() => {
		const map: Record<string, string[]> = {}
		for (const e of graph?.edges ?? []) {
			;(map[e.sourceId] ??= []).push(e.targetId)
			;(map[e.targetId] ??= []).push(e.sourceId)
		}
		if (showSuggestions) {
			for (const r of graph?.recommendations ?? []) {
				;(map[academic.id] ??= []).push(r.academicId)
				;(map[r.academicId] ??= []).push(academic.id)
			}
		}
		return map
	})

	function isDimmed(node: GraphNode): boolean {
		if (!hoveredId) return false
		if (node.id === hoveredId) return false
		return !neighbors[hoveredId]?.includes(node.id)
	}

	function edgeOpacity(link: GraphLink): number {
		const base = link.kind === "recommendation" ? 0.3 : isFocusEdge(link) ? 0.6 : 0.65
		if (!hoveredId) return base
		const s = linkEndpointId(link.source)
		const t = linkEndpointId(link.target)
		if (s === hoveredId || t === hoveredId) return 0.8
		return 0.05
	}

	function nodeOpacity(node: GraphNode): number {
		return isDimmed(node) ? 0.25 : 1
	}

	let selectedEdge = $state<{
		works: CollaborationWorkRefDTO[]
		coauthor: string
		coauthorId: string
	} | null>(null)
	let edgeDialogOpen = $state(false)

	let selectedRecommendation = $state<CollaborationRecommendationDTO | null>(null)
	let recommendationDialogOpen = $state(false)

	function openEdge(link: GraphLink) {
		const s = linkEndpointId(link.source)
		const t = linkEndpointId(link.target)

		if (link.kind === "recommendation") {
			const recommendation = graph?.recommendations.find((r) => r.academicId === t)
			if (!recommendation) return
			selectedRecommendation = recommendation
			recommendationDialogOpen = true
			return
		}

		const edge = graph?.edges.find(
			(e) => (e.sourceId === s && e.targetId === t) || (e.sourceId === t && e.targetId === s),
		)
		if (!edge) return
		const coauthorId = s === academic.id ? t : s
		const coauthorNode = graphNodes.find((n) => n.id === coauthorId)
		selectedEdge = {
			works: edge.works,
			coauthor: coauthorNode?.displayName ?? "Coautor",
			coauthorId,
		}
		edgeDialogOpen = true
	}

	function openNode(node: GraphNode) {
		if (node.kind === "focus") return
		if (node.kind === "recommendation") {
			const recommendation = graph?.recommendations.find((r) => r.academicId === node.id)
			if (!recommendation) return
			selectedRecommendation = recommendation
			recommendationDialogOpen = true
			return
		}
		const edge = graph?.edges.find(
			(e) =>
				(e.sourceId === academic.id && e.targetId === node.id) ||
				(e.sourceId === node.id && e.targetId === academic.id),
		)
		if (!edge) return
		selectedEdge = { works: edge.works, coauthor: node.displayName, coauthorId: node.id }
		edgeDialogOpen = true
	}

	function shortLabel(name: string): string {
		return name.length > 24 ? `${name.slice(0, 22)}…` : name
	}

	function tooltipBadge(node: GraphNode): string | null {
		if (node.kind === "recommendation") return "Posible colaboración"
		if (node.kind === "focus") return "Este académico"
		return null
	}
</script>

<div class="flex h-full min-h-0 flex-col">
	{#if query.isPending}
		<div class="flex min-h-0 flex-1 items-center justify-center">
			<Loader class="size-5 animate-spin text-corp-gray" />
		</div>
	{:else if query.isError}
		<div class="flex min-h-0 flex-1 flex-col items-center justify-center text-center">
			<CircleAlert class="size-6 text-red-500" />
			<p class="mt-2 text-sm text-corp-gray">Error al cargar la red de colaboración.</p>
		</div>
	{:else}
		<div class="relative min-h-0 flex-1 overflow-hidden px-6 pb-6">
			<div class="absolute right-6 top-4 z-20">
				{#if controlsOpen}
					<div
						class="w-64 rounded-xl border border-corp-gray/20 bg-white p-4 shadow-lg"
						role="dialog"
						aria-label="Ajustes del grafo"
					>
						<div class="mb-3 flex items-center justify-between">
							<p
								class="text-xs font-semibold tracking-widest uppercase text-corp-blue"
							>
								Ajustes del grafo
							</p>
							<button
								type="button"
								title="Cerrar ajustes"
								class="flex size-6 items-center justify-center rounded-md text-corp-gray transition-colors hover:bg-corp-gray/10 hover:text-[#1A1A1A]"
								onclick={() => (controlsOpen = false)}
							>
								<X class="size-3.5" />
							</button>
						</div>

						<div class="space-y-2.5">
							<div class="flex items-center justify-between gap-3">
								<span class="flex items-center gap-2 text-sm text-[#1A1A1A]">
									<span class="size-2.5 rounded-full bg-corp-blue"></span>
									Colaboraciones
								</span>
								<Switch.Root bind:checked={showCollaborations} class="shrink-0">
									{#snippet children({ checked })}
										<span
											class="flex h-5 w-9 items-center rounded-full p-0.5 transition-colors {checked
												? 'bg-corp-blue'
												: 'bg-corp-gray/30'}"
										>
											<span
												class="size-4 rounded-full bg-white transition-transform {checked
													? 'translate-x-4'
													: ''}"
											></span>
										</span>
									{/snippet}
								</Switch.Root>
							</div>
							<div class="flex items-center justify-between gap-3">
								<span class="flex items-center gap-2 text-sm text-[#1A1A1A]">
									<span class="size-2.5 rounded-full bg-green-500"></span>
									Sugerencias
								</span>
								<Switch.Root bind:checked={showSuggestions} class="shrink-0">
									{#snippet children({ checked })}
										<span
											class="flex h-5 w-9 items-center rounded-full p-0.5 transition-colors {checked
												? 'bg-corp-blue'
												: 'bg-corp-gray/30'}"
										>
											<span
												class="size-4 rounded-full bg-white transition-transform {checked
													? 'translate-x-4'
													: ''}"
											></span>
										</span>
									{/snippet}
								</Switch.Root>
							</div>
							<div class="flex items-center justify-between gap-3">
								<span class="text-sm text-[#1A1A1A]">Peso en aristas</span>
								<Switch.Root bind:checked={showWeights} class="shrink-0">
									{#snippet children({ checked })}
										<span
											class="flex h-5 w-9 items-center rounded-full p-0.5 transition-colors {checked
												? 'bg-corp-blue'
												: 'bg-corp-gray/30'}"
										>
											<span
												class="size-4 rounded-full bg-white transition-transform {checked
													? 'translate-x-4'
													: ''}"
											></span>
										</span>
									{/snippet}
								</Switch.Root>
							</div>
						</div>

						<div class="my-3 border-t border-corp-gray/10"></div>

						<div class="space-y-3">
							<div class="flex items-center justify-between gap-3">
								<label
									for="graph-link-distance"
									class="text-xs font-medium uppercase tracking-wide text-corp-gray"
									>Separación</label
								>
								<div class="flex items-center gap-2">
									<input
										id="graph-link-distance"
										type="range"
										min="80"
										max="200"
										step="5"
										bind:value={linkDistance}
										class="w-24 accent-corp-blue"
									/>
									<span class="w-8 text-xs text-corp-gray tabular-nums"
										>{linkDistance}</span
									>
								</div>
							</div>
							<div class="flex items-center justify-between gap-3">
								<label
									for="graph-zoom"
									class="text-xs font-medium uppercase tracking-wide text-corp-gray"
									>Zoom</label
								>
								<div class="flex items-center gap-2">
									<input
										id="graph-zoom"
										type="range"
										min="50"
										max="400"
										step="5"
										value={zoomPercent}
										oninput={(e) =>
											setZoomPercent(Number(e.currentTarget.value))}
										class="w-24 accent-corp-blue"
									/>
									<span class="w-9 text-xs text-corp-gray tabular-nums"
										>{zoomPercent}%</span
									>
								</div>
							</div>
						</div>

						<div class="my-3 border-t border-corp-gray/10"></div>

						<div class="space-y-3">
							<div class="flex items-center justify-between gap-3">
								<label
									for="graph-score-threshold"
									class="text-xs font-medium uppercase tracking-wide text-corp-gray"
									>Porcentaje de coincidencia</label
								>
								<div class="flex items-center gap-2">
									<input
										id="graph-score-threshold"
										type="range"
										min="10"
										max="100"
										step="10"
										bind:value={scoreThreshold}
										onchange={() => applyScoreThreshold(scoreThreshold)}
										class="w-20 accent-corp-blue"
									/>
									<span class="w-7 text-xs text-corp-gray tabular-nums"
										>{scoreThreshold}%</span
									>
								</div>
							</div>
							<div class="flex items-center justify-between gap-3">
								<label
									for="graph-min-coincidences"
									class="text-xs font-medium uppercase tracking-wide text-corp-gray"
									>Coincidencias por publicación</label
								>
								<div class="flex items-center gap-2">
									<input
										id="graph-min-coincidences"
										type="range"
										min="1"
										max="10"
										step="1"
										bind:value={minCoincidences}
										onchange={() => applyMinCoincidences(minCoincidences)}
										class="w-20 accent-corp-blue"
									/>
									<span class="w-7 text-xs text-corp-gray tabular-nums"
										>{minCoincidences}</span
									>
								</div>
							</div>
						</div>
					</div>
				{:else}
					<button
						type="button"
						title="Ajustes del grafo"
						class="flex size-8 items-center justify-center rounded-lg border border-corp-gray/20 bg-white text-corp-blue transition-colors hover:bg-corp-gray/5 active:scale-[0.96]"
						onclick={() => (controlsOpen = true)}
					>
						<SlidersHorizontal class="size-4" />
					</button>
				{/if}
			</div>

			{#if isEmptyGraph}
				<div class="flex h-full flex-col items-center justify-center text-center">
					<div
						class="mb-3 flex size-12 items-center justify-center rounded-full bg-corp-blue/5"
					>
						<Network class="size-5 text-corp-blue/60" />
					</div>
					<p class="text-sm text-[#1A1A1A]">
						No hay colaboraciones internas registradas.
					</p>
					<p class="mt-1 max-w-sm text-xs text-corp-gray">
						Este académico no comparte publicaciones con otros académicos de la
						facultad.
					</p>
				</div>
			{:else}
				<Chart
					transform={{
						mode: "canvas",
						scaleExtent: [0.5, 4],
						initialScale: 1,
						scrollMode: "scale",
						translateExtent: [
							[-1500, -1500],
							[1500, 1500],
						],
					}}
					bind:context={chartContext}
				>
					{#snippet children({ context })}
						<Layer>
							<ForceSimulation
								forces={{
									link: forceLink<GraphNode, GraphLink>(graphLinks)
										.id((d) => d.id)
										.distance(linkDistance)
										.strength(0.35),
									charge: forceManyBody<GraphNode>()
										.strength(-220)
										.distanceMax(500),
									collide: forceCollide<GraphNode>().radius(
										(d) => nodeRadius(d) + 6,
									),
									center: forceCenter(context.width / 2, context.height / 2),
								}}
								data={{ nodes: graphNodes, links: graphLinks }}
								static
								cloneNodes
							>
								{#snippet children({ nodes, linkPositions })}
									{#each graphLinks as link, i (i)}
										{#if link.kind === "recommendation" && showSuggestions}
											<Link
												data={link}
												{...linkPositions[i]}
												type="straight"
												stroke="transparent"
												stroke-width={14}
												fill="none"
												pointer-events="stroke"
												class="cursor-pointer"
												onclick={() => openEdge(link)}
											/>
											<Link
												data={link}
												{...linkPositions[i]}
												type="straight"
												stroke-width={edgeWidth(link)}
												opacity={edgeOpacity(link)}
												class={edgeClass(link)}
												stroke-linecap="round"
												pointer-events="none"
											/>
											{#if showWeights}
												{@const mid = linkMidpoint(link)}
												<text
													x={mid.x}
													y={mid.y}
													text-anchor="middle"
													class="fill-green-600 text-[10px] font-semibold"
													stroke="white"
													stroke-width="3"
													paint-order="stroke"
												>
													{link.weight}
												</text>
											{/if}
										{/if}
									{/each}

									{#each graphLinks as link, i (i)}
										{#if link.kind === "coauthor" && showCollaborations}
											<Link
												data={link}
												{...linkPositions[i]}
												type="straight"
												stroke="transparent"
												stroke-width={14}
												fill="none"
												pointer-events="stroke"
												class="cursor-pointer"
												onclick={() => openEdge(link)}
											/>
											<Link
												data={link}
												{...linkPositions[i]}
												type="straight"
												stroke-width={edgeWidth(link)}
												opacity={edgeOpacity(link)}
												class={edgeClass(link)}
												stroke-linecap="round"
												pointer-events="none"
											/>
											{#if showWeights}
												{@const mid = linkMidpoint(link)}
												<text
													x={mid.x}
													y={mid.y}
													text-anchor="middle"
													class="fill-corp-blue text-[10px] font-semibold"
													stroke="white"
													stroke-width="3"
													paint-order="stroke"
												>
													{link.weight}
												</text>
											{/if}
										{/if}
									{/each}

									{#each nodes as node (node.id)}
										{#if (node.kind === "coauthor" && showCollaborations) || (node.kind === "recommendation" && showSuggestions) || node.kind === "focus"}
											<Circle
												cx={node.x}
												cy={node.y}
												r={nodeRadius(node)}
												fill={nodeFill(node)}
												stroke={nodeStroke(node)}
												stroke-width={node.kind === "recommendation"
													? 2
													: 2}
												opacity={nodeOpacity(node)}
												class="cursor-pointer"
												onclick={() => openNode(node)}
												onpointermove={(e) => {
													hoveredId = node.id
													context.tooltip.show(e, node)
												}}
												onpointerleave={() => {
													hoveredId = null
													context.tooltip.hide()
												}}
											/>
											<text
												x={node.x ?? 0}
												y={(node.y ?? 0) + nodeRadius(node) + 14}
												text-anchor="middle"
												class="fill-corp-gray text-[11px] font-medium"
												opacity={nodeOpacity(node)}
											>
												{shortLabel(node.displayName)}
											</text>
										{/if}
									{/each}
								{/snippet}
							</ForceSimulation>
						</Layer>

						<Tooltip.Root variant="none">
							{#if context.tooltip.data}
								<div
									class="pointer-events-none rounded-lg border border-corp-gray/20 bg-white px-3 py-2 shadow-lg"
								>
									{#if tooltipBadge(context.tooltip.data)}
										<span
											class={`mb-1 inline-block rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide ${
												context.tooltip.data.kind === "recommendation"
													? "bg-green-600/10 text-green-600"
													: "bg-corp-blue/10 text-corp-blue"
											}`}
										>
											{tooltipBadge(context.tooltip.data)}
										</span>
									{/if}
									<p class="text-[13px] font-semibold text-[#1A1A1A]">
										{context.tooltip.data.displayName}
									</p>
									<p class="mt-0.5 text-xs text-corp-gray">
										{context.tooltip.data.department}
									</p>
									<p class="mt-0.5 text-xs text-corp-gray">
										{context.tooltip.data.totalWorks} publicaciones
									</p>
									{#if context.tooltip.data.kind === "recommendation"}
										<p class="mt-0.5 text-xs font-medium text-green-600">
											{context.tooltip.data.weight} coincidencias
										</p>
									{/if}
								</div>
							{/if}
						</Tooltip.Root>
					{/snippet}
				</Chart>
			{/if}
		</div>

		{#if !isEmptyGraph && (showCollaborations || showSuggestions)}
			<div class="shrink-0">
				<div
					class="flex items-center justify-center gap-6 px-6 pb-4 text-xs text-corp-gray"
				>
					{#if showCollaborations && (graph?.edges.length ?? 0) > 0}
						<span class="flex items-center gap-1.5">
							<span class="size-2.5 rounded-full bg-corp-blue"></span>
							Coautoría
						</span>
					{/if}
					{#if showSuggestions}
						<span class="flex items-center gap-1.5">
							<span class="size-2.5 rounded-full bg-green-500"></span>
							Posible colaboración
						</span>
					{/if}
				</div>
			</div>
		{/if}
	{/if}
</div>

<EdgeWorksDialog
	bind:open={edgeDialogOpen}
	works={selectedEdge?.works ?? null}
	coauthor={selectedEdge?.coauthor ?? null}
	coauthorId={selectedEdge?.coauthorId ?? null}
/>

<RecommendationDetailDialog
	bind:open={recommendationDialogOpen}
	recommendation={selectedRecommendation}
	{focusName}
	focusDepartment={academic.department}
/>
