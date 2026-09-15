//! Search filtering, scoring, definition folding, counting and keyset paging stay in the plan.

use crate::{
    projection,
    runtime::QueryRuntime,
    scoring::{self, ScoreKind},
    views,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    evidence::{FragmentKind, SymbolKind, path::PublicPath, relational::FactSource},
    search::{page::SearchKey, spec::SearchSpec},
    wire::data::{HitKind, ScoreFactor},
};

pub struct SearchOptions {
    pub include_api: bool,
    pub fragment_kinds: Vec<FragmentKind>,
    pub area: Option<PublicPath>,
    pub page_size: usize,
    pub after: Option<SearchKey>,
}

/// A final bounded result row, carrying the exact fact used to rank/cite it.
#[derive(Debug)]
pub struct RankedEvidence {
    pub key: SearchKey,
    pub hit: HitKind,
    pub path: Option<String>,
    pub symbol_kind: Option<SymbolKind>,
    pub signature: Option<String>,
    pub fragment_kind: Option<FragmentKind>,
    pub excerpt: String,
    pub also_at: Vec<String>,
    pub deprecated: bool,
    pub factors: Vec<ScoreFactor>,
    pub source: FactSource,
}

pub struct SearchPage {
    pub total: u64,
    pub rows: Vec<RankedEvidence>,
    pub has_more: bool,
}

/// Build the canonical folded relation in one already scoped domain session.
/// # Errors
/// Invalid query shapes or unavailable native operators fail explicitly.
pub async fn folded(
    session: &SessionContext,
    spec: &SearchSpec,
    options: &SearchOptions,
) -> Result<DataFrame> {
    let mut symbols = session.table("api_surface").await?.filter(
        lit(options.include_api)
            .and(col("observation_id").is_not_null())
            .and(scoring::eligibility(&spec.symbol_clauses)),
    )?;
    if let Some(area) = &options.area {
        symbols = symbols
            .filter(views::ecosystem(area.ecosystem()).and(views::namespace("components", area)))?;
    }
    symbols = symbols.select(
        [
            "symbol_id",
            "definition_id",
            "path",
            "name",
            "kind",
            "declared_kind",
            "signature",
            "doc_summary",
            "docs",
            "observation_id",
            "is_reexport",
            "is_deprecated",
        ]
        .into_iter()
        .map(col),
    )?;
    let scorer = scoring::function(ScoreKind::Symbol, spec.clone());
    symbols = symbols
        .with_column(
            "ranking",
            scorer.call(vec![
                col("path"),
                col("name"),
                col("signature"),
                col("doc_summary"),
                col("docs"),
                col("is_reexport"),
            ]),
        )?
        .filter(col("ranking").is_not_null())?
        .with_column("rank_score", col("ranking").field("score"))?;
    session.register_table("eligible_symbols", symbols.into_view())?;
    session.register_table("matching_aliases", session.sql("SELECT definition_id, array_agg(DISTINCT path ORDER BY path) AS paths FROM eligible_symbols GROUP BY definition_id").await?.into_view())?;
    session.register_table("folded_symbols", session.sql(r"
        SELECT * FROM (
            SELECT e.*, row_number() OVER (PARTITION BY definition_id ORDER BY rank_score DESC, is_reexport ASC, path ASC, observation_id ASC, symbol_id ASC) AS position
            FROM eligible_symbols e
        ) WHERE position = 1
    ").await?.into_view())?;

    let kinds = options
        .fragment_kinds
        .iter()
        .map(|k| lit(k.as_str()))
        .collect::<Vec<_>>();
    let mut fragments = session.table("fragment_surface").await?;
    if let Some(area) = &options.area {
        let members = session
            .table("fragment_paths")
            .await?
            .filter(views::ecosystem(area.ecosystem()).and(views::namespace("components", area)))?
            .select(vec![col("fragment_id")])?
            .distinct()?;
        session.register_table("scoped_fragment_ids", members.into_view())?;
        let scoped = session.sql("SELECT f.* FROM fragment_surface f LEFT SEMI JOIN scoped_fragment_ids m ON f.fragment_id = m.fragment_id").await?;
        fragments = if area.components().len() == 1 {
            scoped.union(
                session.table("fragment_surface").await?.filter(
                    col("subject_ref").field("kind").in_list(
                        ["library", "feature", "document", "example"]
                            .into_iter()
                            .map(lit)
                            .collect(),
                        false,
                    ),
                )?,
            )?
        } else {
            scoped
        };
    }
    fragments = fragments
        .filter(if kinds.is_empty() {
            lit(false)
        } else {
            col("kind").in_list(kinds, false)
        })?
        .filter(scoring::fragment_eligibility(&spec.fragment_clauses))?;
    fragments = fragments.select(
        ["fragment_id", "kind", "label", "text"]
            .into_iter()
            .map(col),
    )?;
    fragments = fragments
        .with_column(
            "ranking",
            scoring::function(ScoreKind::Fragment, spec.clone())
                .call(vec![col("label"), col("text")]),
        )?
        .filter(col("ranking").is_not_null())?
        .with_column("rank_score", col("ranking").field("score"))?;
    session.register_table("eligible_fragments", fragments.into_view())?;
    let api = session.sql(r"
        SELECT CAST(0 AS INTEGER UNSIGNED) AS hit_order, s.observation_id || '/' || s.symbol_id AS candidate_id,
            s.observation_id AS fact_id, s.path AS label, s.path, coalesce(s.declared_kind, s.kind) AS symbol_kind, s.signature,
            CAST(NULL AS VARCHAR) AS fragment_kind, coalesce(s.signature, s.doc_summary, s.docs, '') AS excerpt,
            array_remove(a.paths, s.path) AS also_at, s.is_deprecated AS deprecated, s.ranking, s.rank_score
        FROM folded_symbols s JOIN matching_aliases a ON s.definition_id = a.definition_id
    ").await?;
    let fragments = session.sql(r"
        SELECT CAST(1 AS INTEGER UNSIGNED) AS hit_order, f.fragment_id AS candidate_id, f.fragment_id AS fact_id, f.label, CAST(NULL AS VARCHAR) AS path, CAST(NULL AS VARCHAR) AS symbol_kind, CAST(NULL AS VARCHAR) AS signature,
            f.kind AS fragment_kind, f.text AS excerpt, arrow_cast(make_array(), 'List(Utf8)') AS also_at, false AS deprecated, f.ranking, f.rank_score
        FROM eligible_fragments f
    ").await?;
    session.register_table(
        "search_api_candidates",
        std::sync::Arc::new(crate::provider::DerivedRelation::new(
            api.into_view(),
            "search_candidates",
        )),
    )?;
    session.register_table(
        "search_fragment_candidates",
        std::sync::Arc::new(crate::provider::DerivedRelation::new(
            fragments.into_view(),
            "search_candidates",
        )),
    )?;
    let result = session
        .table("search_api_candidates")
        .await?
        .union(session.table("search_fragment_candidates").await?)?;
    Ok(result)
}

/// Execute count and limited page against exactly the same folded relation.
/// # Errors
/// Resource exhaustion, invalid keys or decoding failures do not become empty search results.
pub async fn page(
    session: &SessionContext,
    runtime: &QueryRuntime,
    spec: &SearchSpec,
    options: &SearchOptions,
) -> Result<SearchPage> {
    if options.page_size == 0 || options.page_size > 1024 {
        return Err(DataFusionError::ResourcesExhausted(
            "search page item budget is outside 1..=1024".into(),
        ));
    }
    let folded = folded(session, spec, options).await?;
    session.register_table("search_candidates", folded.clone().into_view())?;
    let count = runtime
        .execute(
            session
                .sql("SELECT CAST(count(*) AS BIGINT UNSIGNED) AS count FROM search_candidates")
                .await?,
        )
        .await?;
    let total = projection::search::count(&count.batches)?;
    let filtered = if let Some(key) = &options.after {
        folded.filter(after(key))?
    } else {
        folded
    };
    let sorted = filtered
        .sort(vec![
            col("rank_score").sort(false, false),
            col("hit_order").sort(true, false),
            col("label").sort(true, false),
            col("candidate_id").sort(true, false),
        ])?
        .limit(0, Some(options.page_size + 1))?;
    session.register_table("search_page", sorted.into_view())?;
    let hydrated = session
        .sql(
            r"
        SELECT p.*, CASE p.hit_order WHEN 0 THEN a.source ELSE f.source END AS source
        FROM search_page p
        LEFT JOIN api_observations a ON p.fact_id = a.observation_id AND p.hit_order = 0
        LEFT JOIN fragments f ON p.fact_id = f.fragment_id AND p.hit_order = 1
        ORDER BY p.rank_score DESC, p.hit_order ASC, p.label ASC, p.candidate_id ASC
    ",
        )
        .await?;
    let output = runtime.execute(hydrated).await?;
    let mut rows = output
        .batches
        .iter()
        .map(projection::search::rows)
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let has_more = rows.len() > options.page_size;
    rows.truncate(options.page_size);
    Ok(SearchPage {
        total,
        rows,
        has_more,
    })
}

fn after(key: &SearchKey) -> datafusion::logical_expr::Expr {
    let score = col("rank_score");
    let same_score = score.clone().eq(lit(key.score));
    let same_hit = col("hit_order").eq(lit(key.hit_order));
    let same_subject = col("label").eq(lit(&key.subject));
    score
        .lt(lit(key.score))
        .or(same_score
            .clone()
            .and(col("hit_order").gt(lit(key.hit_order))))
        .or(same_score
            .clone()
            .and(same_hit.clone())
            .and(col("label").gt(lit(&key.subject))))
        .or(same_score
            .and(same_hit)
            .and(same_subject)
            .and(col("candidate_id").gt(lit(&key.candidate_id))))
}
