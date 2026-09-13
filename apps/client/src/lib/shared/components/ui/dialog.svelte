<script lang="ts">
	import type { Snippet } from "svelte"

	import { X } from "@lucide/svelte"
	import { Dialog as DialogPrimitive, type WithoutChild } from "bits-ui"

	type Props = DialogPrimitive.RootProps & {
		title: string
		description?: string
		class?: string
		contentProps?: WithoutChild<DialogPrimitive.ContentProps>
		children: Snippet
		actions?: Snippet
	}

	let {
		open = $bindable(false),
		title,
		description,
		class: className = "max-w-lg",
		contentProps = {},
		children,
		actions,
		...restProps
	}: Props = $props()
</script>

<DialogPrimitive.Root bind:open {...restProps}>
	<DialogPrimitive.Portal>
		<DialogPrimitive.Overlay
			class="fixed inset-0 z-40 bg-black/20 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0"
		/>
		<DialogPrimitive.Content
			{...contentProps}
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed left-1/2 top-1/2 z-50 max-h-[calc(100dvh_-_2rem)] w-full -translate-x-1/2 -translate-y-1/2 overflow-y-auto overscroll-contain rounded-xl border border-corp-gray/20 bg-white p-6 shadow-lg {className}"
		>
			<div class="mb-4 flex items-start justify-between">
				<div>
					<DialogPrimitive.Title class="text-lg font-semibold text-[#1A1A1A]">
						{title}
					</DialogPrimitive.Title>
					{#if description}
						<DialogPrimitive.Description class="mt-1 text-sm text-corp-gray">
							{description}
						</DialogPrimitive.Description>
					{/if}
				</div>
				<div class="flex items-center gap-1">
					{#if actions}
						{@render actions()}
					{/if}
					<DialogPrimitive.Close
						class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A]"
					>
						<X class="size-4" />
					</DialogPrimitive.Close>
				</div>
			</div>
			{@render children?.()}
		</DialogPrimitive.Content>
	</DialogPrimitive.Portal>
</DialogPrimitive.Root>
