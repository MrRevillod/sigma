<script lang="ts">
	import { FIRST_PAGE } from "$manual/sections"
</script>

<div class="mx-auto max-w-3xl">
	<h1 class="text-2xl font-semibold text-corp-ink">Página no encontrada</h1>
	<p class="mt-2 text-sm text-corp-gray">
		La página que buscas no existe o fue movida. Vuelve al inicio del manual.
	</p>
	<a
		href={FIRST_PAGE.href}
		class="mt-4 inline-flex items-center rounded-lg bg-corp-blue/10 px-3 py-2 text-sm font-medium text-corp-blue transition-colors hover:bg-corp-blue/15"
	>
		Ir al inicio del manual
	</a>
</div>
