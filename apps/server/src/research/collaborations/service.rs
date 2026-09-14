use crate::{
	academic::AcademicId,
	research::{
		AcademicKeywordRow, AcademicLineRow, AcademicTopicRow, CollaborationEdge,
		CollaborationGraph, CollaborationNode, CollaborationRecommendation, CollaborationsQuery,
		CollaborationsRepository, KeywordId, MatchWorkRef, RecommendationCandidateRow,
		RecommendationMatchType, RecommendationSharedItem, RecommendationWork,
		RecommendationWorkSide, ResearchLineId, TopicId, WorkId, WorkRef,
	},
	shared::AppResult,
};

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct CollaborationsService {
	repository: Arc<CollaborationsRepository>,
}

impl CollaborationsService {
	pub async fn get_collaborations(
		&self,
		academic_id: AcademicId,
		query: CollaborationsQuery,
	) -> AppResult<CollaborationGraph> {
		let score_threshold = query.score_threshold.unwrap_or(0.6);
		let min_coincidences = query.min_coincidences.unwrap_or(2) as usize;

		let (nodes, edges, candidates, topics, keywords, lines) = tokio::join!(
			self.repository.find_nodes(&academic_id),
			self.repository.find_edges(&academic_id),
			self.repository.find_recommendation_candidates(&academic_id),
			self.repository.find_academic_topics(score_threshold),
			self.repository.find_academic_keywords(score_threshold),
			self.repository.find_academic_lines(score_threshold),
		);

		let nodes = nodes?;
		let edges = edges?;
		let candidates = candidates?;
		let topics = topics?;
		let keywords = keywords?;
		let lines = lines?;

		let work_ids = edges
			.iter()
			.flat_map(|e| e.work_ids.iter().copied())
			.collect::<Vec<WorkId>>();

		let works = self.repository.find_works(&work_ids).await?;
		let works_by_id: HashMap<WorkId, WorkRef> = works.into_iter().map(|w| (w.id, w)).collect();

		let nodes = nodes
			.into_iter()
			.map(|n| CollaborationNode {
				id: n.id,
				name: format!("{} {} {}", n.names, n.paternal_surname, n.maternal_surname),
				department: n.department,
				total_works: n.total_works,
			})
			.collect();

		let edges: Vec<CollaborationEdge> = edges
			.into_iter()
			.map(|e| CollaborationEdge {
				source_id: e.source_id,
				target_id: e.target_id,
				weight: e.weight,
				works: e
					.work_ids
					.iter()
					.filter_map(|id| works_by_id.get(id).cloned())
					.collect(),
			})
			.collect();

		let recommendations = self.build_recommendations(
			&academic_id,
			&edges,
			candidates,
			topics,
			keywords,
			lines,
			min_coincidences,
		);

		Ok(CollaborationGraph {
			academic_id,
			nodes,
			edges,
			recommendations,
		})
	}

	#[allow(clippy::too_many_arguments)]
	fn build_recommendations(
		&self,
		academic_id: &AcademicId,
		edges: &[CollaborationEdge],
		candidates: Vec<RecommendationCandidateRow>,
		topics: Vec<AcademicTopicRow>,
		keywords: Vec<AcademicKeywordRow>,
		lines: Vec<AcademicLineRow>,
		min_coincidences: usize,
	) -> Vec<CollaborationRecommendation> {
		let mut topics_by_academic: HashMap<AcademicId, HashMap<TopicId, Vec<MatchWorkRef>>> =
			HashMap::new();
		let mut topic_names: HashMap<(AcademicId, TopicId), String> = HashMap::new();
		for t in topics {
			topics_by_academic
				.entry(t.academic_id)
				.or_default()
				.entry(t.topic_id)
				.or_default()
				.push(MatchWorkRef {
					work_id: t.work_id,
					title: t.work_title,
					publication_year: t.publication_year,
					score: t.score,
				});
			topic_names.insert((t.academic_id, t.topic_id), t.topic_name);
		}

		let mut keywords_by_academic: HashMap<AcademicId, HashMap<KeywordId, Vec<MatchWorkRef>>> =
			HashMap::new();
		let mut keyword_names: HashMap<(AcademicId, KeywordId), String> = HashMap::new();
		for k in keywords {
			keywords_by_academic
				.entry(k.academic_id)
				.or_default()
				.entry(k.keyword_id)
				.or_default()
				.push(MatchWorkRef {
					work_id: k.work_id,
					title: k.work_title,
					publication_year: k.publication_year,
					score: k.score,
				});
			keyword_names.insert((k.academic_id, k.keyword_id), k.keyword_name);
		}

		let mut lines_by_academic: HashMap<AcademicId, HashSet<ResearchLineId>> = HashMap::new();
		for l in lines {
			lines_by_academic
				.entry(l.academic_id)
				.or_default()
				.insert(l.research_line_id);
		}

		let focus_topics = topics_by_academic.get(academic_id);
		let focus_keywords = keywords_by_academic.get(academic_id);
		let focus_lines = lines_by_academic.get(academic_id);

		if focus_topics.is_none() && focus_keywords.is_none() {
			return Vec::new();
		}

		let existing_coauthors: HashSet<AcademicId> = edges
			.iter()
			.flat_map(|e| [e.source_id, e.target_id])
			.filter(|id| id != academic_id)
			.collect();

		let mut recommendations = Vec::new();

		for candidate in candidates {
			if existing_coauthors.contains(&candidate.id) {
				continue;
			}

			if !self.candidate_shares_line(&candidate.id, focus_lines, &lines_by_academic) {
				continue;
			}

			let mut focus_shared: HashMap<WorkId, Vec<RecommendationSharedItem>> = HashMap::new();
			let mut cand_shared: HashMap<WorkId, Vec<RecommendationSharedItem>> = HashMap::new();
			let mut focus_meta: HashMap<WorkId, MatchWorkRef> = HashMap::new();
			let mut cand_meta: HashMap<WorkId, MatchWorkRef> = HashMap::new();

			if let (Some(focus_topics), Some(cand_topics)) =
				(focus_topics, topics_by_academic.get(&candidate.id))
			{
				for topic_id in cand_topics
					.keys()
					.filter(|id| focus_topics.contains_key(id))
				{
					let name = topic_names[&(candidate.id, *topic_id)].clone();
					for w in &focus_topics[topic_id] {
						focus_meta.entry(w.work_id).or_insert_with(|| w.clone());
						focus_shared
							.entry(w.work_id)
							.or_default()
							.push(RecommendationSharedItem {
								r#type: RecommendationMatchType::Topic,
								id: **topic_id,
								name: name.clone(),
								score: w.score,
							});
					}
					for w in &cand_topics[topic_id] {
						cand_meta.entry(w.work_id).or_insert_with(|| w.clone());
						cand_shared
							.entry(w.work_id)
							.or_default()
							.push(RecommendationSharedItem {
								r#type: RecommendationMatchType::Topic,
								id: **topic_id,
								name: name.clone(),
								score: w.score,
							});
					}
				}
			}
			if let (Some(focus_keywords), Some(cand_keywords)) =
				(focus_keywords, keywords_by_academic.get(&candidate.id))
			{
				for keyword_id in cand_keywords
					.keys()
					.filter(|id| focus_keywords.contains_key(id))
				{
					let name = keyword_names[&(candidate.id, *keyword_id)].clone();
					for w in &focus_keywords[keyword_id] {
						focus_meta.entry(w.work_id).or_insert_with(|| w.clone());
						focus_shared
							.entry(w.work_id)
							.or_default()
							.push(RecommendationSharedItem {
								r#type: RecommendationMatchType::Keyword,
								id: **keyword_id,
								name: name.clone(),
								score: w.score,
							});
					}
					for w in &cand_keywords[keyword_id] {
						cand_meta.entry(w.work_id).or_insert_with(|| w.clone());
						cand_shared
							.entry(w.work_id)
							.or_default()
							.push(RecommendationSharedItem {
								r#type: RecommendationMatchType::Keyword,
								id: **keyword_id,
								name: name.clone(),
								score: w.score,
							});
					}
				}
			}

			let mut works = Vec::new();
			for (work_id, shared) in cand_shared {
				if shared.len() >= min_coincidences {
					let meta = &cand_meta[&work_id];
					works.push(RecommendationWork {
						work_id,
						title: meta.title.clone(),
						publication_year: meta.publication_year,
						side: RecommendationWorkSide::Candidate,
						shared,
					});
				}
			}

			if works.is_empty() {
				continue;
			}

			let mut effective_items: HashSet<(RecommendationMatchType, Uuid)> = HashSet::new();
			for work in &works {
				for item in &work.shared {
					effective_items.insert((item.r#type.clone(), item.id));
				}
			}

			let mut focus_works = Vec::new();
			for (work_id, shared) in focus_shared {
				let filtered: Vec<RecommendationSharedItem> = shared
					.into_iter()
					.filter(|i| effective_items.contains(&(i.r#type.clone(), i.id)))
					.collect();
				if filtered.is_empty() {
					continue;
				}
				let meta = &focus_meta[&work_id];
				focus_works.push(RecommendationWork {
					work_id,
					title: meta.title.clone(),
					publication_year: meta.publication_year,
					side: RecommendationWorkSide::Focus,
					shared: filtered,
				});
			}

			recommendations.push(CollaborationRecommendation {
				academic_id: candidate.id,
				name: format!(
					"{} {} {}",
					candidate.names, candidate.paternal_surname, candidate.maternal_surname
				),
				names: candidate.names,
				paternal_surname: candidate.paternal_surname,
				maternal_surname: candidate.maternal_surname,
				department: candidate.department,
				total_works: candidate.total_works,
				weight: works.len() as i64,
				works,
				focus_works,
			});
		}

		recommendations.sort_by_key(|a| std::cmp::Reverse(a.weight));
		recommendations.truncate(10);

		recommendations
	}

	fn candidate_shares_line(
		&self,
		candidate_id: &AcademicId,
		focus: Option<&HashSet<ResearchLineId>>,
		by_academic: &HashMap<AcademicId, HashSet<ResearchLineId>>,
	) -> bool {
		let Some(focus) = focus else {
			return false;
		};
		let Some(cand_lines) = by_academic.get(candidate_id) else {
			return false;
		};
		!cand_lines.is_disjoint(focus)
	}
}
