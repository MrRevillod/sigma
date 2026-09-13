<script lang="ts">
	import type { ResetPasswordDTO } from "$auth/dtos"

	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { useMutation } from "$shared/http/tanstack"
	import { createForm, Field, Form } from "@formisch/svelte"
	import { CircleAlert, Eye, EyeOff } from "@lucide/svelte"

	import { authService } from "$auth/service"
	import { resetPasswordDTOSchema } from "$auth/dtos"

	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormHeader from "$shared/components/ui/form/header.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	const token = page.url.searchParams.get("token") ?? ""

	const form = createForm({
		schema: resetPasswordDTOSchema,
		initialInput: { token, password: "", confirmPassword: "" },
	})

	let showPassword = $state(false)

	const resetMutation = useMutation(() => ({
		mutationFn: (payload: ResetPasswordDTO) => authService.resetPassword(payload),
		onSuccess: async () => {
			toast.success("Contraseña actualizada. Inicia sesión con tu nueva contraseña.")
			await goto("/login")
		},
		onError: (error) => toast.error(error.message ?? "El enlace es inválido o ha expirado"),
	}))
</script>

<div class="flex h-full items-center justify-center px-4 py-8">
	<section class="w-full max-w-md rounded-xl border border-corp-gray/20 bg-white px-6 py-8">
		{#if !token || resetMutation.isError}
			<div class="flex flex-col items-center text-center">
				<CircleAlert class="size-10 text-red-500" />
				<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">
					Enlace inválido o expirado
				</h1>
				<p class="mt-2 text-sm text-corp-gray">
					El enlace de recuperación ha expirado o ya fue utilizado. Solicita uno nuevo
					para continuar.
				</p>
				<a
					class="mt-6 text-sm font-medium text-corp-blue underline decoration-corp-blue/30 underline-offset-3 hover:decoration-corp-blue/60"
					href="/forgot-password"
				>
					Solicitar un nuevo enlace
				</a>
			</div>
		{:else}
			<Form of={form} onsubmit={(output) => resetMutation.mutate(output)}>
				<FormHeader
					of={form}
					heading="Nueva contraseña"
					description="Define una nueva contraseña para tu cuenta."
				/>

				<div class="grid gap-5">
					<Field of={form} path={["password"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type={showPassword ? "text" : "password"}
								label="Nueva contraseña"
								placeholder="••••••••"
							>
								{#snippet rightIcon()}
									<button
										type="button"
										class="flex items-center justify-center size-8 text-corp-gray/50 transition-colors hover:text-corp-gray"
										onclick={() => (showPassword = !showPassword)}
										tabindex={-1}
									>
										{#if showPassword}
											<EyeOff class="size-4" />
										{:else}
											<Eye class="size-4" />
										{/if}
									</button>
								{/snippet}
							</TextInput>
						{/snippet}
					</Field>

					<Field of={form} path={["confirmPassword"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type={showPassword ? "text" : "password"}
								label="Confirmar contraseña"
								placeholder="••••••••"
							/>
						{/snippet}
					</Field>
				</div>

				<FormFooter submitLabel="Cambiar contraseña" isPending={resetMutation.isPending} />
			</Form>
		{/if}
	</section>
</div>
