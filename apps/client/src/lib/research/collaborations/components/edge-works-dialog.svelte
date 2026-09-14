<script lang="ts">
	import type { CollaborationWorkRefDTO } from "$collaborations/dtos"

	import { FileText } from "@lucide/svelte"
	import { goto } from "$app/navigation"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import HtmlRenderer from "$shared/components/ui/html-renderer.svelte"
	import { authStore } from "$lib/auth/store.svelte"

	interface Props {
		open: boolean
		works: CollaborationWorkRefDTO[] | null
		coauthor: string | null
		coauthorId: string | null
	}

	let { open = $bindable(false), works, coauthor, coauthorId }: Props = $props()

	const profileHref = $derived(
		coauthorId
			? `${authStore.isAuthenticated ? "/academics" : "/public/academics"}/${coauthorId}`
			: "#",
	)
</script>

<Dialog
	bind:open
	title="Publicaciones compartidas"
	description={coauthor ? `Colaboración con ${coauthor}` : undefined}
	class="max-w-xl"
>
	{#if !works || works.length === 0}
		<p class="py-8 text-center text-sm text-corp-gray">No hay publicaciones para mostrar.</p>
	{:else}
		<ul class="divide-y divide-corp-gray/10">
			{#each works as work (work.id)}
				<li>
					<button
						type="button"
						class="group flex w-full items-start gap-3 px-1 py-3 text-left transition-colors"
						onclick={() => goto(`/works/${work.id}`)}
					>
						<span
							class="mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-lg bg-corp-blue/5"
						>
							<FileText class="size-4 text-corp-blue/60" />
						</span>
						<span class="min-w-0 flex-1">
							<span
								class="block text-[14px] font-medium leading-snug text-[#1A1A1A] group-hover:text-corp-blue"
							>
								<HtmlRenderer html={work.title} />
							</span>
							{#if work.publicationYear}
								<span class="mt-0.5 block text-xs text-corp-gray tabular-nums">
									{work.publicationYear}
								</span>
							{/if}
						</span>
					</button>
				</li>
			{/each}
		</ul>
	{/if}

	{#if coauthor && coauthorId}
		<div class="mt-2 flex items-center justify-end border-t border-corp-gray/10 pt-3">
			<a href={profileHref} class="text-xs font-semibold text-corp-blue hover:underline">
				Ir al perfil académico de {coauthor} →
			</a>
		</div>
	{/if}
</Dialog>
