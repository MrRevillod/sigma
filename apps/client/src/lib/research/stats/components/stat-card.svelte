<script lang="ts">
	type Variant = "neutral" | "blue" | "gold" | "muted"

	interface Props {
		label: string
		value: number
		variant?: Variant
	}

	let { label, value, variant = "neutral" }: Props = $props()

	const box: Record<Variant, string> = {
		neutral: "rounded-xl border border-corp-gray/20 bg-white p-5",
		blue: "rounded-xl border border-corp-blue/30 bg-corp-blue/5 p-5",
		gold: "rounded-xl border border-corp-yellow/30 bg-corp-yellow/5 p-5",
		muted: "rounded-xl border border-corp-gray/20 bg-white p-5",
	}
	const valueClass: Record<Variant, string> = {
		neutral: "text-corp-ink",
		blue: "text-corp-blue",
		gold: "text-corp-gold",
		muted: "text-corp-gray",
	}
</script>

<div class={box[variant]}>
	<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">{label}</p>
	<p class="mt-2 text-[28px] font-semibold leading-none {valueClass[variant]} tabular-nums">
		{value}
	</p>
</div>
