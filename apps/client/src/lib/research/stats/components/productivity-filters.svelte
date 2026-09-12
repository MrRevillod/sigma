<script lang="ts">
	import Select from "$shared/components/ui/select.svelte"

	import type { ProductivityJceScope } from "../dtos"
	import {
		degreeItems,
		indexationItems,
		jceScopeItems,
		monthItems,
		precisionItems,
	} from "../productivity-labels"

	let {
		degree = $bindable("all"),
		jceScope = $bindable<ProductivityJceScope>("doctor"),
		month = $bindable("1"),
		indexation = $bindable("all"),
		precision = $bindable("3"),
	} = $props()
</script>

<div class="grid w-full grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-5">
	<div class="space-y-1">
		<label
			for="productivity-degree"
			title="Incluye las publicaciones de todos los autores, o solo las de quienes tienen grado de doctor o magíster"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Publicaciones por grado
		</label>
		<Select
			id="productivity-degree"
			items={degreeItems}
			bind:value={degree}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
	<div class="space-y-1">
		<label
			for="productivity-jce-scope"
			title="Define quiénes aportan JCE al indicador: solo doctores, o todos los académicos del alcance"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Académicos para la JCE
		</label>
		<Select
			id="productivity-jce-scope"
			items={jceScopeItems}
			bind:value={jceScope}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
	<div class="space-y-1">
		<label
			for="productivity-month"
			title="Define cómo se agrupan las publicaciones en años académicos: las publicadas antes de este mes se cuentan en el año anterior"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Mes de corte del año
		</label>
		<Select
			id="productivity-month"
			items={monthItems}
			bind:value={month}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
	<div class="space-y-1">
		<label
			for="productivity-indexation"
			title="Muestra solo las publicaciones indexadas en WoS, en Scopus, o ambas"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Indexación (WoS / Scopus)
		</label>
		<Select
			id="productivity-indexation"
			items={indexationItems}
			bind:value={indexation}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
	<div class="space-y-1">
		<label
			for="productivity-precision"
			title="Cantidad de decimales mostrada en los valores del gráfico, sin redondear"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Decimales en el eje Y
		</label>
		<Select
			id="productivity-precision"
			items={precisionItems}
			bind:value={precision}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
</div>
