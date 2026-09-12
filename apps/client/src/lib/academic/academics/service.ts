import { http } from "$lib/shared/http/client"
import { Academic } from "./entity"

import type { WorkOverridesInput } from "$works/dtos"
import type {
	AcademicDTO,
	PublicAcademicDTO,
	CreateAcademicDTO,
	GetAcademicsParams,
	ImportResult,
	UpdateAcademicDTO,
	SelfUpdateDTO,
	SyncResultDTO,
} from "./dtos"

class AcademicsService {
	public list(params?: GetAcademicsParams): Promise<Academic[]> {
		const academics = http.request<AcademicDTO[]>({
			method: "GET",
			url: "/academics",
			params,
		})

		return academics.then((data) => data.map((dto) => Academic.fromDTO(dto)))
	}

	public get(id: string): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "GET",
			url: `/academics/${id}`,
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public listPublic(params?: GetAcademicsParams): Promise<Academic[]> {
		const academics = http.request<PublicAcademicDTO[]>({
			method: "GET",
			url: "/academics/public",
			params,
		})

		return academics.then((data) => data.map((dto) => Academic.fromPublicDTO(dto)))
	}

	public getPublic(id: string): Promise<Academic> {
		const academic = http.request<PublicAcademicDTO>({
			method: "GET",
			url: `/academics/public/${id}`,
		})

		return academic.then((dto) => Academic.fromPublicDTO(dto))
	}

	public create(data: CreateAcademicDTO): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "POST",
			url: "/academics",
			data,
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public update(id: string, data: UpdateAcademicDTO): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "PATCH",
			url: `/academics/${id}`,
			data,
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public unlink(id: string): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "POST",
			url: `/academics/${id}/unlink`,
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public import(file: File): Promise<ImportResult> {
		const formData = new FormData()
		formData.append("file", file)

		return http.request<ImportResult>({
			method: "POST",
			url: "/academics/import",
			data: formData,
			headers: {
				"Content-Type": "multipart/form-data",
			},
		})
	}

	public requestProfileUpdate(id: string, code: string): Promise<void> {
		return http.request<void>({
			method: "POST",
			url: `/academics/${id}/update-profile-request`,
			data: { code },
		})
	}

	public sendEditCodes(id: string): Promise<void> {
		return http.request<void>({
			method: "POST",
			url: `/academics/${id}/edit-codes/send`,
		})
	}

	public sendEditCodesMass(): Promise<number> {
		return http.request<number>({
			method: "POST",
			url: "/academics/edit-codes/mass",
		})
	}

	public validateOneTimeToken(token: string): Promise<PublicAcademicDTO> {
		const academic = http.request<PublicAcademicDTO>({
			method: "POST",
			url: "/academics/profile/update/validate",
			data: { token },
		})

		return academic
	}

	public updateByToken(token: string, data: SelfUpdateDTO): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "POST",
			url: "/academics/profile/update",
			data: { token, data },
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public syncWorksByToken(token: string): Promise<SyncResultDTO> {
		return http.request<SyncResultDTO>({
			method: "POST",
			url: "/academics/profile/update/sync-works",
			data: { token },
		})
	}

	public updateWorkOverridesByToken(
		token: string,
		workId: string,
		data: WorkOverridesInput,
	): Promise<void> {
		return http.request<void>({
			method: "PUT",
			url: `/academics/profile/update/works/${workId}/overrides`,
			data: { token, data },
		})
	}

	public clearWorkOverridesByToken(token: string, workId: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/academics/profile/update/works/${workId}/overrides`,
			data: { token },
		})
	}

	public updateAuthorshipAffiliationsByToken(
		token: string,
		workId: string,
		orcid: string,
		affiliations: string[],
	): Promise<void> {
		return http.request<void>({
			method: "PUT",
			url: `/academics/profile/update/works/${workId}/authorships/${orcid}/affiliations`,
			data: { token, affiliations },
		})
	}
}

export const academicService = new AcademicsService()
