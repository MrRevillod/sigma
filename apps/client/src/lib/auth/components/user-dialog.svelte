<script lang="ts">
	import type { User } from "$users/entity"
	import type { CreateUserDTO, UpdateUserDTO } from "$users/dtos"

	import { Eye, EyeOff } from "@lucide/svelte"
	import { toast } from "svelte-sonner"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	import { usersService } from "$users/service"
	import { useMutation, queryClient } from "$shared/http/tanstack"
	import { createUserDTOSchema, updateUserDTOSchema } from "$users/dtos"

	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import TextInput from "$lib/shared/components/ui/form/text-input.svelte"

	interface Props {
		user?: User | null
		open: boolean
		onClose: () => void
		onDelete?: (u: User) => void
	}

	let { user = null, open = $bindable(), onClose, onDelete }: Props = $props()

	let showPassword = $state(false)

	const schema = $derived(user ? updateUserDTOSchema : createUserDTOSchema)
	const form = $derived.by(() => createForm({ schema }))

	$effect(() => {
		if (!open) return
		if (user) {
			reset(form, {
				initialInput: {
					name: user.name,
					email: user.email,
					role: user.role.code as "admin",
				},
			})
		} else {
			reset(form, {
				initialInput: {
					name: "",
					email: "",
					password: "",
					role: "admin" as const,
				},
			})
		}
	})

	const createUserMut = useMutation(() => ({
		mutationFn: (output: CreateUserDTO) => usersService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["users"] })
			toast.success("Usuario creado")
			open = false
		},
		onError: () => toast.error("Error al crear el usuario"),
	}))

	const updateUserMut = useMutation(() => ({
		mutationFn: ({ id, data }: { id: string; data: UpdateUserDTO }) =>
			usersService.update(id, data),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["users"] })
			toast.success("Usuario actualizado")
			open = false
		},
		onError: () => toast.error("Error al actualizar el usuario"),
	}))

	function handleCreate(output: CreateUserDTO) {
		createUserMut.mutate(output)
	}

	function handleUpdate(output: UpdateUserDTO) {
		if (!user) return
		const data: UpdateUserDTO = {
			name: output.name,
			email: output.email,
		}
		if (output.password) data.password = output.password
		updateUserMut.mutate({ id: user.id, data })
	}

	const pending = $derived(createUserMut.isPending || updateUserMut.isPending)
</script>

<Dialog bind:open title={user ? "Editar usuario" : "Nuevo usuario"} class="max-w-xl">
	<Form
		of={form}
		onsubmit={(output) =>
			user ? handleUpdate(output as UpdateUserDTO) : handleCreate(output as CreateUserDTO)}
	>
		<div class="grid gap-4">
			<Field of={form} path={["name"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Nombre"
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["email"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="email"
						label="Email"
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["password"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input ?? ""}
						errors={field.errors}
						type={showPassword ? "text" : "password"}
						label={user ? "Contraseña (dejar vacío para mantener)" : "Contraseña"}
						placeholder={user ? "Sin cambios" : ""}
						autocomplete="current-password"
					>
						{#snippet rightIcon()}
							<button
								type="button"
								class="flex size-8 items-center justify-center text-corp-gray transition-colors hover:text-corp-ink"
								aria-label={showPassword
									? "Ocultar contraseña"
									: "Mostrar contraseña"}
								aria-pressed={showPassword}
								onclick={() => (showPassword = !showPassword)}
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

		<div class="flex items-center justify-between gap-2 border-t border-corp-gray/20 pt-4 mt-4">
			{#if user && onDelete}
				<button
					type="button"
					class="rounded-lg px-3 py-2 text-sm font-medium text-red-600 transition-colors hover:bg-red-50"
					onclick={() => onDelete(user)}
				>
					Eliminar usuario
				</button>
			{:else}
				<span></span>
			{/if}
			<div class="flex gap-2">
				<button
					type="button"
					class="inline-flex h-10 items-center justify-center gap-1.5 rounded-lg border border-corp-gray/20 bg-white px-4 py-2 text-sm font-medium text-corp-gray outline-none transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A] focus-visible:ring-2 focus-visible:ring-corp-gray/20 active:scale-[0.96] disabled:pointer-events-none disabled:opacity-50"
					onclick={onClose}
				>
					Cancelar
				</button>
				<button
					type="submit"
					disabled={pending}
					class="inline-flex h-10 items-center justify-center gap-1.5 rounded-lg bg-corp-blue px-4 py-2 text-sm font-medium text-white outline-none transition-colors hover:bg-corp-blue/90 focus-visible:ring-2 focus-visible:ring-corp-blue/30 active:scale-[0.96] disabled:pointer-events-none disabled:opacity-50"
				>
					{pending ? "Guardando..." : "Guardar"}
				</button>
			</div>
		</div>
	</Form>
</Dialog>
