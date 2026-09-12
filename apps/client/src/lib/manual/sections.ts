import type { Component } from "svelte"

import {
	BookOpen,
	ChartBar,
	Compass,
	GraduationCap,
	KeyRound,
	NotebookText,
	Settings,
} from "@lucide/svelte"

import introduccion from "./content/01-introduccion.md?raw"
import acceso from "./content/02-acceso.md?raw"
import navegacion from "./content/03-navegacion.md?raw"

import adminGeneral from "./content/04-administracion/01-configuracion-general.md?raw"
import adminCatalogs from "./content/04-administracion/02-categorias-opciones-cargos.md?raw"
import adminResearchLines from "./content/04-administracion/03-lineas-investigacion.md?raw"
import adminUsers from "./content/04-administracion/04-usuarios.md?raw"

import academicList from "./content/05-academicos/01-listado.md?raw"
import academicCreate from "./content/05-academicos/02-crear.md?raw"
import academicImport from "./content/05-academicos/03-importar-csv.md?raw"
import academicEdit from "./content/05-academicos/04-editar.md?raw"
import academicProfile from "./content/05-academicos/05-ficha.md?raw"
import academicUnlink from "./content/05-academicos/07-desvincular.md?raw"

import worksCatalog from "./content/06-publicaciones/01-catalogo.md?raw"
import worksDetail from "./content/06-publicaciones/02-detalle.md?raw"
import worksEdit from "./content/06-publicaciones/03-edicion.md?raw"
import worksSync from "./content/06-publicaciones/04-sincronizacion.md?raw"

import statsGeneral from "./content/07-estadisticas/01-general.md?raw"
import statsJce from "./content/07-estadisticas/02-jce.md?raw"

export interface ManualPage {
	slug: string
	title: string
	description: string
	body: string
}

export interface ManualChapter {
	slug: string
	title: string
	icon: Component
	page?: ManualPage
	children?: ManualPage[]
}

export const MANUAL_BASE_PATH = "/admin/manual"

export const MANUAL_CHAPTERS: ManualChapter[] = [
	{
		slug: "introduccion",
		title: "Introducción",
		icon: BookOpen,
		page: {
			slug: "introduccion",
			title: "Introducción",
			description: "Qué es el sistema, a quién está dirigido y cómo usar este manual.",
			body: introduccion,
		},
	},
	{
		slug: "acceso",
		title: "Acceso y sesión",
		icon: KeyRound,
		page: {
			slug: "acceso",
			title: "Acceso y sesión",
			description: "Ingresar, recuperar la contraseña y cerrar sesión.",
			body: acceso,
		},
	},
	{
		slug: "navegacion",
		title: "Navegación",
		icon: Compass,
		page: {
			slug: "navegacion",
			title: "Navegación",
			description: "Encabezado global y panel de Administración.",
			body: navegacion,
		},
	},
	{
		slug: "administracion",
		title: "Administración",
		icon: Settings,
		children: [
			{
				slug: "configuracion-general",
				title: "Configuración general",
				description: "Jornada completa equivalente y códigos de edición masivos.",
				body: adminGeneral,
			},
			{
				slug: "categorias-opciones-cargos",
				title: "Categorías, opciones y cargos",
				description: "Catálogos que clasifican y describen a los académicos.",
				body: adminCatalogs,
			},
			{
				slug: "lineas-investigacion",
				title: "Líneas de investigación",
				description: "Asignar áreas de conocimiento a las líneas.",
				body: adminResearchLines,
			},
			{
				slug: "usuarios",
				title: "Usuarios",
				description: "Crear, editar y eliminar cuentas de acceso.",
				body: adminUsers,
			},
		],
	},
	{
		slug: "academicos",
		title: "Académicos",
		icon: GraduationCap,
		children: [
			{
				slug: "listado",
				title: "Listado y búsqueda",
				description: "Filtros, columnas y paginación del listado.",
				body: academicList,
			},
			{
				slug: "ficha",
				title: "Ficha del académico",
				description: "Tarjeta lateral y pestañas de la ficha.",
				body: academicProfile,
			},
			{
				slug: "crear",
				title: "Crear un académico",
				description: "Campos y reglas del formulario de creación.",
				body: academicCreate,
			},
			{
				slug: "importar-csv",
				title: "Importar desde CSV",
				description: "Carga masiva: columnas, valores y resultado.",
				body: academicImport,
			},
			{
				slug: "editar",
				title: "Editar un académico",
				description: "Datos editables, grados académicos y edición delegada.",
				body: academicEdit,
			},
			{
				slug: "desvincular",
				title: "Desvincular",
				description: "Retirar a un académico y efectos en publicaciones y JCE.",
				body: academicUnlink,
			},
		],
	},
	{
		slug: "publicaciones",
		title: "Publicaciones",
		icon: NotebookText,
		children: [
			{
				slug: "catalogo",
				title: "Catálogo",
				description: "Filtros, columnas y comportamiento del catálogo.",
				body: worksCatalog,
			},
			{
				slug: "detalle",
				title: "Detalle de una publicación",
				description: "Qué información muestra cada publicación.",
				body: worksDetail,
			},
			{
				slug: "edicion",
				title: "Edición y ajustes",
				description: "Ajustes manuales que sobrescriben lo importado.",
				body: worksEdit,
			},
			{
				slug: "sincronizacion",
				title: "Sincronización con ORCID",
				description: "Cómo se importan las publicaciones y qué se considera.",
				body: worksSync,
			},
		],
	},
	{
		slug: "estadisticas",
		title: "Estadísticas",
		icon: ChartBar,
		children: [
			{
				slug: "general",
				title: "General",
				description: "Indicadores, tendencias y rankings de publicaciones.",
				body: statsGeneral,
			},
			{
				slug: "jce",
				title: "Productividad por JCE",
				description: "Gráficos y filtros del indicador de productividad.",
				body: statsJce,
			},
		],
	},
]

export interface FlatPage {
	chapter: ManualChapter
	page: ManualPage
	href: string
}

export const MANUAL_FLAT_PAGES: FlatPage[] = MANUAL_CHAPTERS.flatMap((chapter) => {
	if (chapter.page) {
		return [{ chapter, page: chapter.page, href: `${MANUAL_BASE_PATH}/${chapter.slug}` }]
	}

	return (chapter.children ?? []).map((page) => ({
		chapter,
		page,
		href: `${MANUAL_BASE_PATH}/${chapter.slug}/${page.slug}`,
	}))
})

export function getChapter(slug: string | undefined): ManualChapter | undefined {
	return MANUAL_CHAPTERS.find((chapter) => chapter.slug === slug)
}

export function findPage(sectionSlug: string | undefined, pageSlug?: string): FlatPage | undefined {
	if (!sectionSlug) return undefined

	return MANUAL_FLAT_PAGES.find(
		(entry) =>
			entry.chapter.slug === sectionSlug &&
			(entry.chapter.page ? pageSlug === undefined : entry.page.slug === pageSlug),
	)
}

export function getFirstChildHref(chapter: ManualChapter): string {
	return chapter.children?.[0]
		? `${MANUAL_BASE_PATH}/${chapter.slug}/${chapter.children[0].slug}`
		: `${MANUAL_BASE_PATH}/${chapter.slug}`
}

export const FIRST_PAGE: FlatPage = MANUAL_FLAT_PAGES[0]
