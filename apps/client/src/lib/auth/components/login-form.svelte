<script lang="ts">
	import type { LoginDTO } from "$auth/dtos"
	import type { ApiResponse } from "$shared/http/response"

	import { toast } from "svelte-sonner"
	import { useMutation } from "$shared/http/tanstack"
	import { Eye, EyeOff } from "@lucide/svelte"
	import { createForm, Field, Form } from "@formisch/svelte"

	import { authStore } from "$auth/store.svelte"
	import { authService } from "$auth/service"
	import { loginDTOSchema } from "$auth/dtos"

	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormHeader from "$shared/components/ui/form/header.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	interface Props {
		onSuccess?: () => void
	}

	let { onSuccess }: Props = $props()

	const form = createForm({ schema: loginDTOSchema })

	let showPassword = $state(false)

	const loginMutation = useMutation(() => ({
		mutationFn: (payload: LoginDTO) => authService.login(payload),
		onSuccess: async (user) => {
			authStore.setSession(user)
			onSuccess?.()
		},
		onError: (error: ApiResponse) => {
			toast.error(error.message ?? "Error al iniciar sesión")
		},
	}))

	const loading = $derived(loginMutation.isPending)
</script>

<section class="w-full max-w-md rounded-xl border border-corp-gray/20 bg-white px-6 py-8">
	<Form of={form} onsubmit={(output) => loginMutation.mutate(output)}>
		<FormHeader
			of={form}
			heading="Iniciar sesión"
			description="Ingresa a la web de Administración de la Plataforma."
		/>

		<div class="grid gap-5">
			<Field of={form} path={["email"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Correo electrónico"
						placeholder="jdoe@domain.com"
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["password"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type={showPassword ? "text" : "password"}
						label="Contraseña"
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
		</div>

		<FormFooter submitLabel="Ingresar" isPending={loading}>
			<p class="mr-auto text-sm text-corp-gray">
				<a
					class="font-medium text-corp-blue underline decoration-corp-blue/30 underline-offset-3 hover:decoration-corp-blue/60"
					href="/forgot-password"
				>
					¿Olvidaste tu contraseña?
				</a>
			</p>
		</FormFooter>
	</Form>
</section>
