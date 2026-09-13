<script lang="ts">
	import type { ForgotPasswordDTO } from "$auth/dtos"

	import { toast } from "svelte-sonner"
	import { useMutation } from "$shared/http/tanstack"
	import { createForm, Field, Form } from "@formisch/svelte"
	import { CircleCheck } from "@lucide/svelte"

	import { authService } from "$auth/service"
	import { forgotPasswordDTOSchema } from "$auth/dtos"

	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormHeader from "$shared/components/ui/form/header.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	const form = createForm({ schema: forgotPasswordDTOSchema })

	const forgotMutation = useMutation(() => ({
		mutationFn: (payload: ForgotPasswordDTO) => authService.forgotPassword(payload),
		onError: (error) => toast.error(error.message ?? "Error al enviar la solicitud"),
	}))
</script>

<div class="flex h-full items-center justify-center px-4 py-8">
	<section class="w-full max-w-md rounded-xl border border-corp-gray/20 bg-white px-6 py-8">
		{#if forgotMutation.isSuccess}
			<div class="flex flex-col items-center text-center">
				<CircleCheck class="size-10 text-green-500" />
				<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">Revisa tu correo</h1>
				<p class="mt-2 text-sm text-corp-gray">
					Si el correo está registrado, recibirás un enlace para restablecer tu
					contraseña. Revisa también la carpeta de spam o correo no deseado.
				</p>
				<a
					class="mt-6 text-sm font-medium text-corp-blue underline decoration-corp-blue/30 underline-offset-3 hover:decoration-corp-blue/60"
					href="/login"
				>
					Volver a iniciar sesión
				</a>
			</div>
		{:else}
			<Form of={form} onsubmit={(output) => forgotMutation.mutate(output)}>
				<FormHeader
					of={form}
					heading="Recuperar contraseña"
					description="Ingresa tu correo y te enviaremos un enlace para restablecerla."
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
				</div>

				<FormFooter submitLabel="Enviar enlace" isPending={forgotMutation.isPending}>
					<p class="mr-auto text-sm text-corp-gray">
						<a
							class="font-medium text-corp-blue underline decoration-corp-blue/30 underline-offset-3 hover:decoration-corp-blue/60"
							href="/login"
						>
							Volver a iniciar sesión
						</a>
					</p>
				</FormFooter>
			</Form>
		{/if}
	</section>
</div>
