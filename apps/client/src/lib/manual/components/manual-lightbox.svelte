<script lang="ts">
	import { RotateCcw, X, ZoomIn, ZoomOut } from "@lucide/svelte"

	let {
		src,
		alt = "",
		onclose,
	}: {
		src: string
		alt?: string
		onclose: () => void
	} = $props()

	const MIN_SCALE = 1
	const MAX_SCALE = 6
	const STEP = 0.5
	const WHEEL_FACTOR = 1.15

	let scale = $state(1)
	let offsetX = $state(0)
	let offsetY = $state(0)
	let dragging = $state(false)
	let imageEl = $state<HTMLImageElement | null>(null)
	let closeButtonEl = $state<HTMLButtonElement | null>(null)

	let startX = 0
	let startY = 0
	let startOffsetX = 0
	let startOffsetY = 0

	const clampScale = (value: number) => Math.min(MAX_SCALE, Math.max(MIN_SCALE, value))

	function clampOffset(value: number, axis: "x" | "y") {
		const viewport = axis === "x" ? window.innerWidth : window.innerHeight
		const limit = (viewport / 2) * (scale - 1) + viewport * 0.1
		return Math.min(limit, Math.max(-limit, value))
	}

	function zoomTo(next: number, pointerX = 0, pointerY = 0) {
		const target = clampScale(next)
		const ratio = target / scale
		offsetX = clampOffset(pointerX - (pointerX - offsetX) * ratio, "x")
		offsetY = clampOffset(pointerY - (pointerY - offsetY) * ratio, "y")
		scale = target
	}

	function pointerFromCenter(event: { clientX: number; clientY: number }) {
		return {
			x: event.clientX - window.innerWidth / 2,
			y: event.clientY - window.innerHeight / 2,
		}
	}

	function reset() {
		scale = 1
		offsetX = 0
		offsetY = 0
	}

	function handleWheel(event: WheelEvent) {
		event.preventDefault()
		const { x, y } = pointerFromCenter(event)
		zoomTo(scale * (event.deltaY < 0 ? WHEEL_FACTOR : 1 / WHEEL_FACTOR), x, y)
	}

	function handleDoubleClick(event: MouseEvent) {
		if (scale > 1) {
			reset()
			return
		}
		const { x, y } = pointerFromCenter(event)
		zoomTo(2.5, x, y)
	}

	function handlePointerDown(event: PointerEvent) {
		if (scale <= 1) return
		dragging = true
		startX = event.clientX
		startY = event.clientY
		startOffsetX = offsetX
		startOffsetY = offsetY
		imageEl?.setPointerCapture(event.pointerId)
	}

	function handlePointerMove(event: PointerEvent) {
		if (!dragging) return
		offsetX = clampOffset(startOffsetX + (event.clientX - startX), "x")
		offsetY = clampOffset(startOffsetY + (event.clientY - startY), "y")
	}

	function handlePointerUp(event: PointerEvent) {
		if (!dragging) return
		dragging = false
		if (imageEl?.hasPointerCapture(event.pointerId)) {
			imageEl.releasePointerCapture(event.pointerId)
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Escape") {
			onclose()
		} else if (event.key === "+" || event.key === "=") {
			zoomTo(scale + STEP)
		} else if (event.key === "-") {
			zoomTo(scale - STEP)
		} else if (event.key === "0") {
			reset()
		}
	}

	$effect(() => {
		const el = imageEl
		if (!el) return

		el.addEventListener("wheel", handleWheel, { passive: false })
		el.addEventListener("dblclick", handleDoubleClick)
		el.addEventListener("pointerdown", handlePointerDown)
		el.addEventListener("pointermove", handlePointerMove)
		el.addEventListener("pointerup", handlePointerUp)
		el.addEventListener("pointercancel", handlePointerUp)

		return () => {
			el.removeEventListener("wheel", handleWheel)
			el.removeEventListener("dblclick", handleDoubleClick)
			el.removeEventListener("pointerdown", handlePointerDown)
			el.removeEventListener("pointermove", handlePointerMove)
			el.removeEventListener("pointerup", handlePointerUp)
			el.removeEventListener("pointercancel", handlePointerUp)
		}
	})

	$effect(() => {
		const previousOverflow = document.body.style.overflow
		document.body.style.overflow = "hidden"
		closeButtonEl?.focus()
		return () => {
			document.body.style.overflow = previousOverflow
		}
	})
</script>

<svelte:window onkeydown={handleKeydown} />

<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-black/85 backdrop-blur-sm"
	role="dialog"
	aria-modal="true"
	aria-label={alt || "Imagen ampliada"}
>
	<button
		type="button"
		aria-label="Cerrar imagen"
		class="absolute inset-0 h-full w-full cursor-zoom-out"
		onclick={onclose}
	></button>

	<img
		bind:this={imageEl}
		{src}
		{alt}
		draggable="false"
		style={`transform: translate(${offsetX}px, ${offsetY}px) scale(${scale})`}
		class="relative z-10 max-h-[85vh] max-w-[90vw] touch-none select-none rounded-lg bg-white/5 shadow-2xl {dragging
			? ''
			: 'transition-transform duration-150 ease-out'} {scale > 1
			? dragging
				? 'cursor-grabbing'
				: 'cursor-grab'
			: 'cursor-zoom-in'}"
	/>

	<div class="absolute right-4 top-4 z-20 flex items-center gap-2">
		<button
			type="button"
			aria-label="Alejar"
			class="flex size-9 items-center justify-center rounded-full bg-white/10 text-white transition-colors hover:bg-white/20 disabled:opacity-40"
			disabled={scale <= MIN_SCALE}
			onclick={() => zoomTo(scale - STEP)}
		>
			<ZoomOut class="size-4" />
		</button>
		<button
			type="button"
			aria-label="Acercar"
			class="flex size-9 items-center justify-center rounded-full bg-white/10 text-white transition-colors hover:bg-white/20 disabled:opacity-40"
			disabled={scale >= MAX_SCALE}
			onclick={() => zoomTo(scale + STEP)}
		>
			<ZoomIn class="size-4" />
		</button>
		<button
			type="button"
			aria-label="Restablecer zoom"
			class="flex size-9 items-center justify-center rounded-full bg-white/10 text-white transition-colors hover:bg-white/20 disabled:opacity-40"
			disabled={scale <= MIN_SCALE && offsetX === 0 && offsetY === 0}
			onclick={reset}
		>
			<RotateCcw class="size-4" />
		</button>
		<button
			bind:this={closeButtonEl}
			type="button"
			aria-label="Cerrar"
			class="flex size-9 items-center justify-center rounded-full bg-white/10 text-white transition-colors hover:bg-white/20"
			onclick={onclose}
		>
			<X class="size-4" />
		</button>
	</div>

	<p
		class="pointer-events-none absolute bottom-4 left-1/2 z-20 -translate-x-1/2 rounded-full bg-black/50 px-3 py-1 text-xs text-white/80"
	>
		Rueda o doble clic para ampliar · Arrastra para mover · Esc para cerrar
	</p>
</div>
