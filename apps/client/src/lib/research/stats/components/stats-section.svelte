<script lang="ts">
	import type { Component, Snippet } from "svelte"
	import { expoOut } from "svelte/easing"
	import { slide } from "svelte/transition"

	import { ChevronDown } from "@lucide/svelte"

	interface Props {
		title: string
		description?: string
		icon: Component
		open: boolean
		ontoggle: () => void
		first?: boolean
		action?: Snippet
		barAction?: Snippet
		children: Snippet
	}

	let {
		title,
		description,
		icon: Icon,
		open,
		ontoggle,
		first = false,
		action,
		barAction,
		children,
	}: Props = $props()
</script>

<section class={first ? "" : "border-t border-corp-gray/20"}>
	<div class="flex items-center transition-colors hover:bg-corp-gray/3">
		<button
			type="button"
			class="flex min-w-0 flex-1 items-center gap-2 px-5 py-4 text-left"
			onclick={ontoggle}
			aria-expanded={open}
		>
			<Icon class="size-4 shrink-0 text-corp-blue" />
			<div class="min-w-0">
				<h2 class="truncate text-sm font-semibold tracking-wide uppercase text-corp-blue">
					{title}
				</h2>
				{#if open && description}
					<span
						class="mt-0.5 block truncate text-xs font-normal normal-case text-corp-gray"
					>
						{description}
					</span>
				{/if}
			</div>
		</button>

		{#if open && action}
			<div class="shrink-0 pr-1">{@render action()}</div>
		{/if}

		{#if barAction}
			<div class="shrink-0 pr-1">{@render barAction()}</div>
		{/if}

		<button
			type="button"
			class="flex items-center self-stretch px-4"
			onclick={ontoggle}
			aria-expanded={open}
		>
			<ChevronDown
				class={`size-4 text-corp-gray transition-transform ${open ? "rotate-180" : ""}`}
			/>
		</button>
	</div>

	{#if open}
		<div
			class="border-t border-corp-gray/10 p-6"
			transition:slide={{ duration: 250, easing: expoOut }}
		>
			{@render children?.()}
		</div>
	{/if}
</section>
