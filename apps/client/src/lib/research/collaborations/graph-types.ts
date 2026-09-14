import type { SimulationLinkDatum, SimulationNodeDatum } from "d3-force"

import type { CollaborationNodeDTO, CollaborationWorkRefDTO } from "$collaborations/dtos"

export type NodeKind = "focus" | "coauthor" | "recommendation"

export type GraphNode = CollaborationNodeDTO &
	SimulationNodeDatum & {
		displayName: string
		kind: NodeKind
		weight: number
	}

export type LinkKind = "coauthor" | "recommendation"

export type GraphLink = {
	source: string
	target: string
	weight: number
	kind: LinkKind
	works: CollaborationWorkRefDTO[]
} & SimulationLinkDatum<GraphNode>
