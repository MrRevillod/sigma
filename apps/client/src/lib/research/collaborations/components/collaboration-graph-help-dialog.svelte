<script lang="ts">
	import { CircleDot, Hash, Layers, Scale, Sparkles, Tag } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"

	interface Props {
		open: boolean
	}

	let { open = $bindable(false) }: Props = $props()
	let activeTab = $state<"concepts" | "filters">("concepts")
</script>

<Dialog
	bind:open
	title="Cómo leer esta red"
	description="Significado de los conceptos y filtros del grafo."
	class="max-w-3xl"
>
	<div class="mb-4 flex gap-1 border-b border-corp-gray/10">
		<button
			type="button"
			onclick={() => (activeTab = "concepts")}
			class="px-3 py-2.5 text-sm font-semibold transition-colors {activeTab === 'concepts'
				? 'border-b-2 border-corp-blue text-corp-blue'
				: 'text-corp-gray hover:text-[#1A1A1A]'}"
		>
			Conceptos
		</button>
		<button
			type="button"
			onclick={() => (activeTab = "filters")}
			class="px-3 py-2.5 text-sm font-semibold transition-colors {activeTab === 'filters'
				? 'border-b-2 border-corp-blue text-corp-blue'
				: 'text-corp-gray hover:text-[#1A1A1A]'}"
		>
			Filtros
		</button>
	</div>

	<div class="space-y-5">
		{#if activeTab === "concepts"}
			<section>
				<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
					<CircleDot class="size-4 text-corp-blue/60" />
					Nodos
				</h3>
				<ul class="mt-2 space-y-1.5 text-sm text-corp-gray">
					<li class="flex items-start gap-2">
						<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"></span>
						<span>
							<span class="font-medium text-[#1A1A1A]">Este académico</span> — el foco de
							la vista.
						</span>
					</li>
					<li class="flex items-start gap-2">
						<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-[#D8E6EF]"></span>
						<span>
							<span class="font-medium text-[#1A1A1A]">Coautor</span> — alguien con quien
							comparte publicaciones. El tamaño del nodo refleja su cantidad de publicaciones.
						</span>
					</li>
					<li class="flex items-start gap-2">
						<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-green-500"></span>
						<span>
							<span class="font-medium text-[#1A1A1A]">Posible colaboración</span> — un
							académico sin publicaciones en común que comparte intereses de investigación.
						</span>
					</li>
				</ul>
			</section>

			<section>
				<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
					<Hash class="size-4 text-corp-blue/60" />
					Conexiones y pesos
				</h3>
				<ul class="mt-2 space-y-1.5 text-sm text-corp-gray">
					<li class="flex items-start gap-2">
						<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"></span>
						<span>
							<span class="font-medium text-[#1A1A1A]">Coautoría</span> — publicaciones
							compartidas entre ambos académicos. El grosor de la línea indica cuántas publicaciones
							comparten.
						</span>
					</li>
					<li class="flex items-start gap-2">
						<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-green-500"></span>
						<span>
							<span class="font-medium text-[#1A1A1A]">Posible colaboración</span> — línea
							que conecta académicos con intereses compartidos. El grosor indica el número
							de publicaciones que cumplen el mínimo de coincidencias.
						</span>
					</li>
					<li class="flex items-start gap-2">
						<Scale class="mt-0.5 size-4 shrink-0 text-corp-gray" />
						<span>
							Con
							<span class="font-medium text-[#1A1A1A]">Peso en aristas</span> habilitado,
							se muestra el número exacto sobre cada conexión.
						</span>
					</li>
				</ul>
			</section>

			<section>
				<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
					<Tag class="size-4 text-corp-blue/60" />
					Conceptos clave
				</h3>
				<ul class="mt-2 space-y-1.5 text-sm text-corp-gray">
					<li class="flex items-start gap-2">
						<span
							class="mt-1 rounded-full bg-green-600/10 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-green-600"
						>
							topic
						</span>
						<span>
							Área temática (taxonomía OpenAlex) asignada a una publicación, agrupada
							en subfield y línea de investigación. Si dos académicos comparten un
							topic con porcentaje suficiente, cuenta como coincidencia.
						</span>
					</li>
					<li class="flex items-start gap-2">
						<span
							class="mt-1 rounded-full bg-corp-gray/10 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-corp-gray"
						>
							keyword
						</span>
						<span>
							Palabra clave extraída de la publicación. Cuenta como coincidencia con
							el mismo umbral de porcentaje que los topics.
						</span>
					</li>
				</ul>
			</section>

			<div class="flex items-start gap-2 rounded-lg bg-corp-blue/5 px-3 py-2.5">
				<Sparkles class="mt-0.5 size-4 shrink-0 text-corp-blue" />
				<p class="text-xs text-corp-gray">
					Haz clic en una línea de coautoría para ver las publicaciones compartidas, o en
					una posible colaboración para explorar los tópicos y keywords en común.
				</p>
			</div>
		{:else}
			<section>
				<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
					<Layers class="size-4 text-corp-blue/60" />
					Controles disponibles
				</h3>
				<ul class="mt-2 space-y-2 text-sm text-corp-gray">
					<li>
						<span class="font-medium text-[#1A1A1A]">Mostrar colaboraciones</span>
						<p class="mt-1 text-xs text-corp-gray">
							Oculta o muestra las coautorías (líneas azules).
						</p>
					</li>
					<li>
						<span class="font-medium text-[#1A1A1A]">Mostrar sugerencias</span>
						<p class="mt-1 text-xs text-corp-gray">
							Oculta o muestra los posibles colaboradores (líneas verdes).
						</p>
					</li>
					<li>
						<span class="font-medium text-[#1A1A1A]">Peso en aristas</span>
						<p class="mt-1 text-xs text-corp-gray">
							Muestra números sobre cada conexión para ver cuántas publicaciones
							comparten.
						</p>
					</li>
					<li>
						<span class="font-medium text-[#1A1A1A]">Separación</span>
						<p class="mt-1 text-xs text-corp-gray">
							Ajusta la distancia entre nodos. Valores mayores crean más espacio.
						</p>
					</li>
					<li>
						<span class="font-medium text-[#1A1A1A]">Zoom</span>
						<p class="mt-1 text-xs text-corp-gray">
							Acerca o aleja la vista para explorar diferentes partes del grafo.
						</p>
					</li>
					<li>
						<span class="font-medium text-[#1A1A1A]">Porcentaje de coincidencia ≥</span>
						<p class="mt-1 text-xs text-corp-gray">
							Porcentaje mínimo para que un tópico o keyword cuente como coincidencia.
							Aplica a coautorías y sugerencias.
						</p>
					</li>
					<li>
						<span class="font-medium text-[#1A1A1A]"
							>Coincidencias por publicación ≥</span
						>
						<p class="mt-1 text-xs text-corp-gray">
							Mínimo de tópicos/keywords compartidos en una publicación para que
							cuente. El peso de una sugerencia es el número de publicaciones que
							cumplen esta condición.
						</p>
					</li>
				</ul>
			</section>

			<div class="flex items-start gap-2 rounded-lg bg-corp-blue/5 px-3 py-2.5">
				<Sparkles class="mt-0.5 size-4 shrink-0 text-corp-blue" />
				<p class="text-xs text-corp-gray">
					Usa los filtros para enfocarte en los tipos de conexiones que te interesan. Los
					cambios se aplican en tiempo real al grafo.
				</p>
			</div>
		{/if}
	</div>
</Dialog>
