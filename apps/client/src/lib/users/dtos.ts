import * as v from "valibot"

// Entity View DTOs ----------------------------------------

export type UserRoleDTO = "admin"

export interface UserDTO {
	id: string
	name: string
	email: string
	role: string
}

// Create User DTOs ----------------------------------------

export const createUserDTOSchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre es obligatorio"),
		v.maxLength(255, "El nombre no puede tener más de 255 caracteres"),
	),
	email: v.pipe(v.string(), v.email("El correo electrónico no es válido")),
	password: v.pipe(
		v.string(),
		v.check(
			(pw) => !validatePassword(pw),
			"Mínimo 8 caracteres, una mayúscula, una minúscula, un número y un carácter especial",
		),
	),
	role: v.picklist(["admin"] as const),
})

export type CreateUserDTOSchema = typeof createUserDTOSchema
export type CreateUserDTO = v.InferInput<typeof createUserDTOSchema>

// Update User DTOs ----------------------------------------

export const updateUserDTOSchema = v.object({
	name: v.optional(
		v.pipe(
			v.string(),
			v.minLength(1, "El nombre es obligatorio"),
			v.maxLength(255, "El nombre no puede tener más de 255 caracteres"),
		),
	),
	email: v.optional(v.pipe(v.string(), v.email("El correo electrónico no es válido"))),
	password: v.optional(
		v.pipe(
			v.string(),
			v.check(
				(pw) => !validatePassword(pw),
				"Mínimo 8 caracteres, una mayúscula, una minúscula, un número y un carácter especial",
			),
		),
	),
	role: v.optional(v.picklist(["admin"] as const)),
})

export type UpdateUserDTOSchema = typeof updateUserDTOSchema
export type UpdateUserDTO = v.InferInput<typeof updateUserDTOSchema>

// Password validation ----------------------------------------

export const validatePassword = (pw: string) => {
	const missing: string[] = []

	if (pw.length < 8) missing.push("al menos 8 caracteres")
	if (!/[a-z]/.test(pw)) missing.push("una minúscula")
	if (!/[A-Z]/.test(pw)) missing.push("una mayúscula")
	if (!/[0-9]/.test(pw)) missing.push("un número")
	if (!/[^a-zA-Z0-9]/.test(pw)) missing.push("un carácter especial")
	if (pw.length > 255) missing.push("máximo 255 caracteres")

	if (missing.length > 0) {
		return `La contraseña debe tener ${missing.join(", ")}`
	}

	return null
}
