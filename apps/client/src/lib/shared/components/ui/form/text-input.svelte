<script lang="ts">
	import type { Snippet } from "svelte"
	import type { FieldElementProps } from "@formisch/svelte"
	import type { HTMLInputAttributes } from "svelte/elements"

	import InputLabel from "./label.svelte"
	import InputErrors from "./errors.svelte"

	interface Props extends FieldElementProps {
		class?: string
		type: "text" | "email" | "tel" | "password" | "url" | "number" | "date"
		label?: string
		hint?: string
		placeholder?: string
		required?: boolean
		autocomplete?: HTMLInputAttributes["autocomplete"]
		min?: number
		max?: number
		step?: number
		input: unknown
		errors: [string, ...string[]] | null
		rightIcon?: Snippet
	}

	let {
		class: className,
		label,
		hint,
		name,
		required,
		input,
		errors,
		rightIcon,
		...fieldProps
	}: Props = $props()

	let value: string | number | undefined = $state()

	$effect(() => {
		if (typeof input === "string" || typeof input === "number") {
			value = input
		}
	})
</script>

<div class={["grid gap-1.5", className]}>
	<InputLabel {name} {label} {hint} {required} />
	<div class="relative">
		<input
			{...fieldProps}
			id={name}
			{name}
			class={[
				"h-10 w-full rounded-lg border bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50",
				errors ? "border-red-500" : "border-corp-gray/20 focus:border-corp-blue/50",
			]}
			{value}
			{required}
			aria-invalid={!!errors}
			aria-errormessage={`${name}-error`}
		/>
		{#if rightIcon}
			<div class="absolute inset-y-0 right-2.5 flex items-center">
				{@render rightIcon()}
			</div>
		{/if}
	</div>
	<InputErrors {name} {errors} />
</div>

<style>
	input[type="date"] {
		color-scheme: light;
		appearance: none;
		-webkit-appearance: none;
		-moz-appearance: textfield;
	}

	input[type="date"]::-webkit-calendar-picker-indicator {
		opacity: 0.45;
		filter: invert(0);
		cursor: pointer;
	}

	input[type="date"]::-webkit-calendar-picker-indicator:hover {
		opacity: 0.75;
	}

	input[type="date"]::-webkit-datetime-edit-text {
		opacity: 0.5;
	}

	input[type="date"]:user-invalid::-webkit-datetime-edit {
		color: #dc2626;
	}
</style>
