use crate::{
	research::{AuthorshipPosition, Source, SourceId, Work, WorkType, WorksError},
	shared::AppResult,
};

use std::{str::FromStr, time::Duration};

use chrono::NaiveDate;
use html_escape::decode_html_entities;
use papers_openalex::{GetParams, OpenAlexClient as OaClient, OpenAlexError, Work as OaWork};
use serde::Deserialize;
use sword::prelude::*;

const SELECT_FIELDS: &[&str] = &[
	"id",
	"doi",
	"title",
	"display_name",
	"publication_year",
	"publication_date",
	"language",
	"type",
	"primary_location",
	"authorships",
	"primary_topic",
	"topics",
	"keywords",
	"abstract_inverted_index",
	"updated_date",
	"created_date",
];

pub(crate) const API_DELAY: Duration = Duration::from_millis(50);

fn repair_mojibake(s: &str) -> String {
	if s.chars().any(|c| c > '\u{00FF}') {
		return s.to_string();
	}

	let bytes: Vec<u8> = s.chars().map(|c| c as u8).collect();

	match String::from_utf8(bytes) {
		Ok(repaired) if repaired != s => repaired,
		_ => s.to_string(),
	}
}

#[derive(Debug, Clone, Deserialize)]
#[config(key = "openalex")]
pub struct OpenAlexConfig {
	pub api_key: String,
}

#[injectable(provider)]
pub struct OpenAlexClient {
	inner: OaClient,
}

impl OpenAlexClient {
	pub fn new(config: OpenAlexConfig) -> Self {
		Self {
			inner: OaClient::with_api_key(&config.api_key),
		}
	}

	pub async fn get_work_by_doi(&self, doi: &str) -> AppResult<Option<OaWork>> {
		let params = GetParams::builder().select(SELECT_FIELDS.join(",")).build();

		let url = format!("https://doi.org/{doi}");

		match self.inner.get_work(&url, &params).await {
			Ok(work) => Ok(Some(work)),
			Err(e) if matches!(&e, OpenAlexError::Api { status: 404, .. }) => Ok(None),
			Err(e) => Err(WorksError::OpenAlexError(e).into()),
		}
	}
}

#[derive(Debug, Clone)]
pub struct OaAuthorshipData {
	pub orcid: String,
	pub display_name: String,
	pub position: AuthorshipPosition,
	pub is_corresponding: bool,
	pub affiliations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OaTopicRef {
	pub openalex_id: Option<String>,
	pub score: f64,
}

#[derive(Debug, Clone)]
pub struct OaKeywordRef {
	pub openalex_id: Option<String>,
	pub name: String,
	pub score: f64,
}

pub trait OpenAlexWorkExt {
	fn title(&self) -> String;
	fn publication_date(&self) -> Option<NaiveDate>;
	fn ty(&self) -> WorkType;
	fn language(&self) -> String;
	fn is_accepted_and_published(&self) -> (bool, bool);
	fn source(&self) -> Option<Source>;
	fn source_issns(&self) -> Option<Vec<String>>;
	fn to_work(&self, source_id: Option<SourceId>) -> Work;
	fn apply_to_work(&self, work: &mut Work, source_id: Option<SourceId>);
	fn authorships(&self) -> Vec<OaAuthorshipData>;
	fn topic_refs(&self) -> Vec<OaTopicRef>;
	fn keyword_refs(&self) -> Vec<OaKeywordRef>;
	fn is_indexable_ty(&self) -> bool;
}

impl OpenAlexWorkExt for OaWork {
	fn title(&self) -> String {
		self.title
			.clone()
			.or_else(|| self.display_name.clone())
			.unwrap_or_default()
	}

	fn publication_date(&self) -> Option<NaiveDate> {
		self.publication_date
			.as_deref()
			.and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
	}

	fn ty(&self) -> WorkType {
		self.r#type
			.as_deref()
			.and_then(|s| WorkType::from_str(s).ok())
			.unwrap_or(WorkType::Other)
	}

	fn language(&self) -> String {
		self.language.clone().unwrap_or_else(|| "en".to_string())
	}

	fn is_accepted_and_published(&self) -> (bool, bool) {
		self.primary_location
			.as_ref()
			.map(|loc| {
				(
					loc.is_accepted.unwrap_or(false),
					loc.is_published.unwrap_or(false),
				)
			})
			.unwrap_or((false, false))
	}

	fn source(&self) -> Option<Source> {
		let source = self
			.primary_location
			.as_ref()
			.and_then(|l| l.source.as_ref())?;

		let name = source.display_name.clone().unwrap_or_default();
		let openalex_id = source.id.clone().unwrap_or_default();
		let ty = source.r#type.clone().unwrap_or("unknown".to_string());

		Some(
			Source::builder()
				.openalex_id(openalex_id)
				.name(name)
				.ty(ty)
				.build(),
		)
	}

	fn source_issns(&self) -> Option<Vec<String>> {
		let source = self
			.primary_location
			.as_ref()
			.and_then(|l| l.source.as_ref())?;

		let issn_l = source.issn_l.as_deref().and_then(Source::normalize_issn);

		let mut issn: Option<Vec<String>> = source.issn.as_ref().and_then(|vec| {
			let normalized: Vec<String> = vec
				.iter()
				.filter_map(|v| Source::normalize_issn(v))
				.collect();

			if normalized.is_empty() {
				None
			} else {
				Some(normalized)
			}
		});

		if let Some(ref l) = issn_l {
			let vec = issn.get_or_insert_with(Vec::new);

			if !vec.contains(l) {
				vec.push(l.clone());
			}
		}

		issn
	}

	fn to_work(&self, source_id: Option<SourceId>) -> Work {
		let (is_accepted, is_published) = self.is_accepted_and_published();

		Work::builder()
			.openalex_id(self.id.clone())
			.title(self.title())
			.maybe_abstract_text(self.abstract_text.clone())
			.maybe_doi(self.doi.clone())
			.maybe_publication_date(self.publication_date())
			.maybe_publication_year(self.publication_year.map(|y| y as i16))
			.ty(self.ty())
			.lang(self.language())
			.is_accepted(is_accepted)
			.is_published(is_published)
			.maybe_source_id(source_id)
			.updated_at(chrono::Utc::now())
			.build()
	}

	fn apply_to_work(&self, work: &mut Work, source_id: Option<SourceId>) {
		let (is_accepted, is_published) = self.is_accepted_and_published();

		work.title = self.title();
		work.abstract_text = self.abstract_text.clone();
		work.doi = self.doi.clone();
		work.publication_date = self.publication_date();
		work.publication_year = self.publication_year.map(|y| y as i16);
		work.ty = self.ty();
		work.lang = self.language();
		work.is_accepted = is_accepted;
		work.is_published = is_published;
		work.source_id = source_id;
		work.updated_at = chrono::Utc::now();
	}

	fn authorships(&self) -> Vec<OaAuthorshipData> {
		let Some(auths) = &self.authorships else {
			return Vec::new();
		};

		auths
			.iter()
			.filter_map(|auth| {
				let orcid = auth.author.as_ref().and_then(|a| a.orcid.clone())?;
				let display_name = auth
					.author
					.as_ref()
					.and_then(|a| a.display_name.clone())
					.unwrap_or_else(|| "Unknown".into());

				let position = match auth.author_position.as_deref() {
					Some("first") => AuthorshipPosition::First,
					Some("last") => AuthorshipPosition::Last,
					_ => AuthorshipPosition::Middle,
				};

				let mut seen = std::collections::HashSet::new();
				let affiliations: Vec<String> = auth
					.raw_affiliation_strings
					.clone()
					.unwrap_or_default()
					.into_iter()
					.map(|s| repair_mojibake(&decode_html_entities(&s)))
					.filter(|s| seen.insert(s.clone()))
					.collect();

				Some(OaAuthorshipData {
					orcid,
					display_name,
					position,
					is_corresponding: auth.is_corresponding.unwrap_or(false),
					affiliations,
				})
			})
			.collect()
	}

	fn topic_refs(&self) -> Vec<OaTopicRef> {
		self.topics
			.as_ref()
			.map(|topics| {
				topics
					.iter()
					.map(|t| OaTopicRef {
						openalex_id: t.id.clone(),
						score: t.score.unwrap_or(0.0),
					})
					.collect()
			})
			.unwrap_or_default()
	}

	fn keyword_refs(&self) -> Vec<OaKeywordRef> {
		self.keywords
			.as_ref()
			.map(|kws| {
				kws.iter()
					.map(|k| OaKeywordRef {
						openalex_id: k.id.clone(),
						name: k.display_name.clone().unwrap_or_else(|| "Unknown".into()),
						score: k.score.unwrap_or(0.0),
					})
					.collect()
			})
			.unwrap_or_default()
	}

	fn is_indexable_ty(&self) -> bool {
		matches!(self.ty(), WorkType::Article | WorkType::ConferencePaper)
	}
}
