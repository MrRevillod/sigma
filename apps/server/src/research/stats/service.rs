use crate::academic::{AcademicId, DegreeKind};
use crate::config::ConfigService;
use crate::research::*;
use crate::shared::AppResult;
use crate::university::DepartmentId;

use std::collections::BTreeMap;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct StatsService {
	stats: Arc<StatsRepository>,
	config: Arc<ConfigService>,
}

impl StatsService {
	pub async fn get_works_stats(&self, query: WorksStatsQuery) -> AppResult<WorksStatsResponse> {
		let (summary, by_journal_kind, by_department, by_research_line, top_publishers) = tokio::join!(
			self.stats.faculty_summary(&query),
			self.stats.stats_by_journal_kind(&query),
			self.stats.stats_by_department(&query),
			self.stats.stats_by_research_line(&query),
			self.stats
				.top_publishers_faculty(&query, query.limit.unwrap_or(10)),
		);

		let summary = summary?;

		Ok(WorksStatsResponse {
			faculty_summary: FacultySummary {
				total_works: summary.total.unwrap_or(0),
				wos_count: summary.wos.unwrap_or(0),
				scopus_count: summary.scopus.unwrap_or(0),
			},
			by_journal_kind: Self::build_journal_kind_series(by_journal_kind?),
			by_department: Self::build_department_scopes(by_department?),
			by_research_line: Self::build_research_line_scopes(by_research_line?),
			top_publishers: Self::build_top_publishers(top_publishers?),
		})
	}

	pub async fn get_department_detail(
		&self,
		id: DepartmentId,
		query: DepartmentDetailQuery,
	) -> AppResult<DepartmentDetailResponse> {
		let (summary, publishers, trend) = tokio::join!(
			self.stats.department_summary(&id, &query),
			self.stats
				.top_publishers(&id, &query, query.limit.unwrap_or(10)),
			self.stats.department_journal_kind_trend(&id, &query),
		);

		let summary = summary.map_err(|_| StatsError::DepartmentNotFound(id))?;
		let publishers = publishers?;
		let trend = trend?;

		Ok(DepartmentDetailResponse {
			department: summary.department,
			total_works: summary.total.unwrap_or(0),
			scopus_count: summary.scopus.unwrap_or(0),
			wos_count: summary.wos.unwrap_or(0),
			teaching_count: summary.teaching.unwrap_or(0),
			research_count: summary.research.unwrap_or(0),
			by_journal_kind: Self::build_journal_kind_series(trend),
			top_publishers: Self::build_top_publishers(publishers),
		})
	}

	pub async fn get_research_line_stats(
		&self,
		id: crate::research::ResearchLineId,
		query: ResearchLineStatsQuery,
	) -> AppResult<ResearchLineStatsResponse> {
		let (summary, trend, by_department, top_publishers) = tokio::join!(
			self.stats.research_line_summary(&id, &query),
			self.stats.research_line_journal_kind_trend(&id, &query),
			self.stats
				.research_line_department_distribution(&id, &query),
			self.stats
				.research_line_top_publishers(&id, &query, query.limit.unwrap_or(10)),
		);

		let summary = summary?;
		let by_department = by_department?;
		let top_publishers = top_publishers?;

		let scopes = by_department
			.into_iter()
			.map(|r| ScopeTotal {
				id: Some(r.department_id.to_string()),
				name: r.department,
				total: r.count.unwrap_or(0),
			})
			.collect();

		Ok(ResearchLineStatsResponse {
			name: summary.name,
			total_works: summary.total.unwrap_or(0),
			wos_count: summary.wos.unwrap_or(0),
			scopus_count: summary.scopus.unwrap_or(0),
			by_journal_kind: Self::build_journal_kind_series(trend?),
			by_department: scopes,
			top_publishers: Self::build_top_publishers(top_publishers),
		})
	}

	pub async fn get_academic_stats(
		&self,
		academic_id: AcademicId,
		query: AcademicStatsQuery,
	) -> AppResult<AcademicStatsResponse> {
		let (lines, trend, contribution) = tokio::join!(
			self.stats.academic_line_distribution(&academic_id, &query),
			self.stats.academic_journal_kind_trend(&academic_id, &query),
			self.stats.academic_contribution(&academic_id, &query),
		);

		let lines = lines?;
		let by_research_line = lines
			.iter()
			.map(|r| ResearchLineStat {
				research_line_id: r.research_line_id.to_string(),
				name: r.name.clone(),
				count: r.count.unwrap_or(0),
			})
			.collect::<Vec<_>>();

		let total = by_research_line.iter().map(|s| s.count).sum::<i64>();
		let dominant = lines.iter().max_by_key(|r| r.count.unwrap_or(0));

		let dominant_research_line_id = if total == 0 {
			None
		} else {
			dominant.map(|r| r.research_line_id.to_string())
		};

		let dominant_line_works = dominant.map(|r| r.count.unwrap_or(0)).unwrap_or(0);
		let line_total_works = match dominant {
			Some(r) => {
				self.stats
					.works_in_research_line(&r.research_line_id, &query)
					.await?
			}
			None => 0,
		};

		let contribution = contribution?;

		Ok(AcademicStatsResponse {
			by_research_line,
			dominant_research_line_id,
			by_journal_kind: Self::build_journal_kind_series(trend?),
			contribution: AcademicContribution {
				academic_works: contribution.academic_works.unwrap_or(0),
				faculty_works: contribution.faculty_works.unwrap_or(0),
				department_works: contribution.department_works.unwrap_or(0),
				department_name: contribution.department_name,
				dominant_line_works,
				line_total_works,
			},
		})
	}

	pub async fn get_productivity(
		&self,
		query: ProductivityQuery,
	) -> AppResult<ProductivityResponse> {
		let month = query.month.unwrap_or(1);
		let year_from = query.year_from.unwrap_or(1900);
		let year_to = query.year_to.unwrap_or(2100);

		let degree = match query.degree {
			Some(ProductivityDegree::All) | None => None,
			Some(ProductivityDegree::Magister) => Some(DegreeKind::Magister),
			Some(ProductivityDegree::Doctor) => Some(DegreeKind::Doctor),
		};

		let jce_kind = match query.jce_scope {
			Some(ProductivityJceScope::All) => None,
			Some(ProductivityJceScope::Doctor) | None => Some(DegreeKind::Doctor),
		};

		let (jce, academic_count) = match query.scope {
			Some(ProductivityScope::Faculty) | None => {
				let (jce, count) = tokio::join!(
					self.stats.sum_jce(None, jce_kind),
					self.stats.count_jce(None, jce_kind),
				);
				(jce?, count?)
			}
			Some(ProductivityScope::Department) => {
				let Some(department_id) = query.department_id else {
					return Err(StatsError::InvalidScopeParams)?;
				};

				let (jce, count) = tokio::join!(
					self.stats.sum_jce(Some(department_id), jce_kind),
					self.stats.count_jce(Some(department_id), jce_kind),
				);
				(jce?, count?)
			}
			Some(ProductivityScope::ResearchLine) => {
				let Some(research_line_id) = query.research_line_id else {
					return Err(StatsError::InvalidScopeParams)?;
				};

				let (jce, count) = tokio::join!(
					self.stats
						.sum_jce_dominant_line(&research_line_id, jce_kind),
					self.stats
						.count_jce_dominant_line(&research_line_id, jce_kind),
				);
				(jce?, count?)
			}
		};

		let rows = self
			.stats
			.productivity_numerator(&query, month, year_from, year_to, degree)
			.await?;

		let jce_max = self.config.jce_max().await?;
		let jce = if jce_max > 0.0 { jce / jce_max } else { 0.0 };

		let factor = if jce > 0.0 { 1.0 / jce } else { 0.0 };

		let mut total = Vec::new();
		let mut wos = Vec::new();
		let mut scopus = Vec::new();

		for r in &rows {
			total.push(ProductivityYearValue {
				year: r.period,
				value: r.total.unwrap_or(0) as f64 * factor,
				pubs: r.total.unwrap_or(0),
			});
			wos.push(ProductivityYearValue {
				year: r.period,
				value: r.wos.unwrap_or(0) as f64 * factor,
				pubs: r.wos.unwrap_or(0),
			});
			scopus.push(ProductivityYearValue {
				year: r.period,
				value: r.scopus.unwrap_or(0) as f64 * factor,
				pubs: r.scopus.unwrap_or(0),
			});
		}

		Ok(ProductivityResponse {
			jce,
			academic_count,
			trend: vec![
				ProductivitySeries {
					key: "total".into(),
					values: total,
				},
				ProductivitySeries {
					key: "wos".into(),
					values: wos,
				},
				ProductivitySeries {
					key: "scopus".into(),
					values: scopus,
				},
			],
		})
	}

	fn build_journal_kind_series(rows: Vec<JournalKindRow>) -> Vec<TimeSeriesStat> {
		let mut scopus_vals = Vec::new();
		let mut wos_vals = Vec::new();

		for r in &rows {
			scopus_vals.push(YearValue {
				year: r.year,
				value: r.scopus.unwrap_or(0),
			});
			wos_vals.push(YearValue {
				year: r.year,
				value: r.wos.unwrap_or(0),
			});
		}

		vec![
			TimeSeriesStat {
				id: None,
				key: "scopus".into(),
				values: scopus_vals,
			},
			TimeSeriesStat {
				id: None,
				key: "wos".into(),
				values: wos_vals,
			},
		]
	}

	fn build_department_scopes(rows: Vec<DepartmentRow>) -> Vec<ScopeSeries> {
		let mut map: BTreeMap<String, (String, i64, Vec<YearValue>, Vec<YearValue>)> =
			BTreeMap::new();

		for r in &rows {
			let entry = map
				.entry(r.department_id.to_string())
				.or_insert_with(|| (r.department.clone(), 0, Vec::new(), Vec::new()));
			entry.0 = r.department.clone();
			entry.1 += r.count.unwrap_or(0);
			entry.2.push(YearValue {
				year: r.year,
				value: r.wos.unwrap_or(0),
			});
			entry.3.push(YearValue {
				year: r.year,
				value: r.scopus.unwrap_or(0),
			});
		}

		let mut scopes = map
			.into_iter()
			.map(|(id, (name, total, wos, scopus))| ScopeSeries {
				id: Some(id),
				name,
				total,
				wos,
				scopus,
			})
			.collect::<Vec<_>>();

		scopes.sort_by(|a, b| a.name.cmp(&b.name));
		scopes
	}

	fn build_research_line_scopes(rows: Vec<ResearchLineDistributionRow>) -> Vec<ScopeSeries> {
		let mut map: BTreeMap<String, (String, i64, Vec<YearValue>, Vec<YearValue>)> =
			BTreeMap::new();

		for r in &rows {
			let entry = map
				.entry(r.research_line_id.to_string())
				.or_insert_with(|| (r.name.clone(), 0, Vec::new(), Vec::new()));
			entry.0 = r.name.clone();
			entry.1 += r.count.unwrap_or(0);
			entry.2.push(YearValue {
				year: r.year,
				value: r.wos.unwrap_or(0),
			});
			entry.3.push(YearValue {
				year: r.year,
				value: r.scopus.unwrap_or(0),
			});
		}

		let mut scopes = map
			.into_iter()
			.map(|(id, (name, total, wos, scopus))| ScopeSeries {
				id: Some(id),
				name,
				total,
				wos,
				scopus,
			})
			.collect::<Vec<_>>();

		scopes.sort_by_key(|a| std::cmp::Reverse(a.total));
		scopes
	}

	fn build_top_publishers(rows: Vec<TopPublisherRow>) -> Vec<TopPublisher> {
		rows.into_iter()
			.map(|r| TopPublisher {
				academic_id: r.academic_id.to_string(),
				name: r.name,
				total: r.total.unwrap_or(0),
				scopus: r.scopus.unwrap_or(0),
				wos: r.wos.unwrap_or(0),
				unindexed: r.unindexed.unwrap_or(0),
				option: r.option,
			})
			.collect()
	}
}
