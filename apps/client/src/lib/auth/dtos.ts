import * as v from "valibot"

import { validatePassword } from "$lib/users/dtos"

export const loginDTOSchema = v.object({
	email: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(1, "El email es obligatorio."),
		v.maxLength(255, "El email no puede tener más de 255 caracteres."),
	),
	password: v.pipe(
		v.string(),
		v.minLength(1, "La contraseña es obligatoria."),
		v.maxLength(255, "La contraseña no puede tener más de 255 caracteres."),
	),
})

export type LoginDTOSchema = typeof loginDTOSchema
export type LoginDTO = v.InferInput<typeof loginDTOSchema>

export const forgotPasswordDTOSchema = v.object({
	email: v.pipe(
		v.string(),
		v.trim(),
		v.email("El correo electrónico no es válido."),
		v.maxLength(255, "El email no puede tener más de 255 caracteres."),
	),
})

export type ForgotPasswordDTOSchema = typeof forgotPasswordDTOSchema
export type ForgotPasswordDTO = v.InferInput<typeof forgotPasswordDTOSchema>

export const resetPasswordDTOSchema = v.pipe(
	v.object({
		token: v.pipe(v.string(), v.minLength(1, "El token es obligatorio.")),
		password: v.pipe(
			v.string(),
			v.check(
				(pw) => !validatePassword(pw),
				"Mínimo 8 caracteres, una mayúscula, una minúscula, un número y un carácter especial.",
			),
		),
		confirmPassword: v.string(),
	}),
	v.forward(
		v.check(
			(input) => input.password === input.confirmPassword,
			"Las contraseñas no coinciden.",
		),
		["confirmPassword"],
	),
)

export type ResetPasswordDTOSchema = typeof resetPasswordDTOSchema
export type ResetPasswordDTO = v.InferInput<typeof resetPasswordDTOSchema>
