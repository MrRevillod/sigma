import type { RecommendationSharedItemDTO, RecommendationWorkDTO } from "./dtos"

export interface Interest {
	type: RecommendationSharedItemDTO["type"]
	id: string
	name: string
	bestScore: number
	focusWorks: RecommendationWorkDTO[]
	candidateWorks: RecommendationWorkDTO[]
}
