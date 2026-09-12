<script lang="ts">
	import { Gauge, Sigma, SlidersHorizontal } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"

	interface Props {
		open: boolean
	}

	let { open = $bindable(false) }: Props = $props()
</script>

<Dialog
	bind:open
	title="Productividad por jornada completa (JCE)"
	description="Qué muestra este gráfico y cómo se calcula."
	class="max-w-2xl"
>
	<div class="space-y-5">
		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Gauge class="size-4 text-corp-blue/60" />
				Qué muestra este gráfico
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				Relaciona las publicaciones de cada año con la jornada completa equivalente (JCE) de
				los académicos del alcance elegido (facultad, departamento o línea de
				investigación). Cada punto indica cuántas publicaciones se producen por cada JCE.
			</p>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Sigma class="size-4 text-corp-blue/60" />
				Jornada Completa Equivalente (JCE)
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				La JCE mide la jornada completa: se suman las horas de jornada de los académicos
				considerados y se dividen por la jornada máxima configurada, de modo que el
				resultado son jornadas completas equivalentes (por ejemplo, 42,5 h = 1 JCE). Este
				valor es el denominador del indicador y se calcula año a año según los académicos
				vinculados ese año. El filtro
				<span class="font-medium text-corp-ink">Académicos para la JCE</span>
				decide quiénes aportan:
			</p>
			<ul class="mt-2 space-y-0.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Solo doctores</span> — solo los académicos
						con grado de doctor del alcance.
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink"
							>Todos los académicos del alcance</span
						>
						— doctores, magísteres y profesionales.
					</p>
				</li>
			</ul>
			<p class="mt-3 text-sm leading-relaxed text-corp-gray">
				Un académico desvinculado deja de aportar JCE desde su año de desvinculación. Sus
				publicaciones del período en que estuvo vinculado se siguen contando.
			</p>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<SlidersHorizontal class="size-4 text-corp-blue/60" />
				Los filtros
			</h3>
			<ul class="mt-3 space-y-0.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Publicaciones por grado</span> — qué publicaciones
						se cuentan en el indicador: todas, o solo las de autores con grado de doctor o
						magíster.
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Académicos para la JCE</span> — quiénes
						aportan JCE al cálculo: solo doctores o todos los académicos del alcance.
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Mes de corte del año</span> — define cómo
						se agrupan las publicaciones en años académicos: las publicadas antes de este
						mes se cuentan en el año anterior (un año va de un mes de corte al equivalente
						del año siguiente).
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Indexación</span> — qué publicaciones
						se muestran: ambas indexaciones, solo WoS o solo Scopus.
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Decimales en el eje Y</span> — cuántos
						decimales se muestran en los valores del gráfico, sin redondear.
					</p>
				</li>
			</ul>
		</section>
	</div>
</Dialog>
