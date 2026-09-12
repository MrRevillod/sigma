<script lang="ts">
	import type { Academic } from "$academics/entity"
	import { ChevronLeft, Pencil, ExternalLink, Send, UserMinus } from "@lucide/svelte"
	import { FullName } from "$shared/value-objects/full-name.value"

	interface Props {
		academic: Academic
		readonly?: boolean
		onEdit?: () => void
		onRequestEdit?: () => void
		onSendCodes?: () => void
		onUnlink?: () => void
	}

	let {
		academic,
		readonly = false,
		onEdit,
		onRequestEdit,
		onSendCodes,
		onUnlink,
	}: Props = $props()

	const nameObj = $derived(
		FullName.of(academic.names, academic.paternalSurname, academic.maternalSurname),
	)

	const initials = $derived(nameObj.initials())
	const fullName = $derived(nameObj.format())
</script>

<aside class="relative overflow-hidden rounded-xl bg-corp-blue text-white">
	<button
		class="absolute left-3 top-3 z-10 flex size-8 items-center justify-center rounded-full bg-white text-corp-blue shadow-sm active:scale-95"
		onclick={() => window.history.back()}
	>
		<ChevronLeft class="size-4" />
	</button>
	{#if !readonly && onEdit}
		<button
			class="absolute right-3 top-3 z-10 flex size-8 items-center justify-center rounded-full bg-white text-corp-blue shadow-sm active:scale-95"
			onclick={onEdit}
		>
			<Pencil class="size-4" />
		</button>
	{/if}
	<div class="p-6 pb-4 text-center">
		<div
			class="mx-auto mb-4 flex size-24 items-center justify-center rounded-full bg-white/10 text-2xl font-bold tracking-widest text-white ring-2 ring-white/15"
		>
			{initials}
		</div>
		<h1 class="text-lg font-semibold leading-snug">{fullName}</h1>
		<p class="mt-1.5 text-sm text-white/60">{academic.email}</p>
		{#if academic.isUnlinked}
			<span
				class="mt-3 inline-flex items-center gap-1 rounded-full bg-white/15 px-2.5 py-1 text-[11px] font-semibold tracking-wide uppercase text-white/80"
			>
				Desvinculado{#if academic.leftAt}· {academic.leftAt.toDisplayDate()}{/if}
			</span>
		{/if}
	</div>

	<div class="border-t border-white/10 px-6 py-4">
		<div class="space-y-4">
			<div>
				<p class="text-[11px] font-medium tracking-wide uppercase text-white/50">
					País de Nacionalidad
				</p>
				<p class="mt-1 text-sm font-semibold text-white">
					{academic.nationality.toDisplay()}
				</p>
			</div>
			<div>
				<p class="text-[11px] font-medium tracking-wide uppercase text-white/50">
					Ciudad de Residencia
				</p>
				<p class="mt-1 text-sm font-semibold text-white">{academic.city}</p>
			</div>
			<div>
				<p class="text-[11px] font-medium tracking-wide uppercase text-white/50">
					Fecha de Nacimiento
				</p>
				<p class="mt-1 text-sm font-semibold text-white">
					{academic.birthDate.toDisplayDate()}
				</p>
			</div>
			<div>
				<p class="text-[11px] font-medium tracking-wide uppercase text-white/50">Sexo</p>
				<p class="mt-1 text-sm font-semibold text-white">
					{academic.sex.toDisplay()}
				</p>
			</div>
			{#if academic.orcid}
				<a
					href={academic.orcid}
					target="_blank"
					rel="noopener"
					class="flex items-center gap-3 text-sm transition-colors hover:text-white/80"
				>
					<ExternalLink class="size-4 shrink-0 text-corp-yellow" />
					<div class="min-w-0">
						<p class="truncate text-white/90">{academic.orcid}</p>
						<p class="text-xs text-white/60">ORCID</p>
					</div>
				</a>
			{/if}
		</div>
	</div>

	{#if onRequestEdit}
		<div class="border-t border-white/10 px-6 py-4">
			<button
				class="flex w-full items-center justify-center gap-2 rounded-lg bg-white/10 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-white/15 active:scale-[0.97]"
				onclick={onRequestEdit}
			>
				Solicitar edición de perfil
			</button>
		</div>
	{/if}

	{#if onSendCodes}
		<div class="border-t border-white/10 px-6 py-4">
			<button
				class="flex w-full items-center justify-center gap-2 rounded-lg bg-white/10 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-white/15 active:scale-[0.97]"
				onclick={onSendCodes}
			>
				<Send class="size-4" />
				Enviar códigos de edición
			</button>
		</div>
	{/if}

	{#if onUnlink}
		<div class="border-t border-white/10 px-6 py-4">
			<button
				class="flex w-full items-center justify-center gap-2 rounded-lg bg-white/10 px-3 py-2 text-sm font-medium text-red-100 transition-colors hover:bg-red-500/25 active:scale-[0.97]"
				onclick={onUnlink}
			>
				<UserMinus class="size-4" />
				Desvincular académico
			</button>
		</div>
	{/if}
</aside>
