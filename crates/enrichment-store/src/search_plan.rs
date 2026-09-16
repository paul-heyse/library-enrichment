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
    let mut symbols = session.table("snapshot.domain.api_surface").await?.filter(
        lit(options.include_api)
            .and(col("observation_id").is_not_null())
            .and(scoring::eligibility(spec)?),
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
    symbols = symbols
        .with_column(
            "ranking",
            scoring::ranking(
                ScoreKind::Symbol,
                spec,
                vec![
                    col("path"),
                    col("name"),
                    col("signature"),
                    col("doc_summary"),
                    col("docs"),
                    col("is_reexport"),
                ],
            )?,
        )?
        .filter(col("ranking").is_not_null())?
        .with_column("rank_score", col("ranking").field("score"))?;
    crate::native_catalog::work(session, "eligible_symbols", symbols.into_view())?;
    crate::native_catalog::work(session, "matching_aliases", session.sql("SELECT definition_id, array_agg(DISTINCT path ORDER BY path) AS paths FROM eligible_symbols GROUP BY definition_id").await?.into_view())?;
    crate::native_catalog::work(session, "folded_symbols", session.sql(r"
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
    let mut fragments = session.table("snapshot.domain.fragment_surface").await?;
    if let Some(area) = &options.area {
        let members = session
            .table("snapshot.domain.fragment_paths")
            .await?
            .filter(views::ecosystem(area.ecosystem()).and(views::namespace("components", area)))?
            .select(vec![col("fragment_id")])?
            .distinct()?;
        crate::native_catalog::work(session, "scoped_fragment_ids", members.into_view())?;
        let scoped = session.sql("SELECT f.* FROM snapshot.domain.fragment_surface f LEFT SEMI JOIN scoped_fragment_ids m ON f.fragment_id = m.fragment_id").await?;
        fragments = if area.components().len() == 1 {
            scoped.union(
                session
                    .table("snapshot.domain.fragment_surface")
                    .await?
                    .filter(
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
        .filter(scoring::fragment_eligibility(spec)?)?;
    fragments = fragments.select(
        [
            "fragment_id",
            "kind",
            "label",
            "text",
            "definition_id",
            "source",
        ]
        .into_iter()
        .map(col),
    )?;
    fragments = fragments
        .with_column(
            "ranking",
            scoring::ranking(ScoreKind::Fragment, spec, vec![col("label"), col("text")])?,
        )?
        .filter(col("ranking").is_not_null())?
        .with_column("rank_score", col("ranking").field("score"))?;
    crate::native_catalog::work(session, "eligible_fragments", fragments.into_view())?;
    // Fold only qualified aliases of the same definition. Full source equality retains
    // producer, artifact, locator and epistemic distinctions even when text is identical.
    crate::native_catalog::work(
        session,
        "fragment_aliases",
        session
            .sql(
                r"
        SELECT coalesce(definition_id, fragment_id) AS fold_key, kind, text, source,
            array_agg(DISTINCT label ORDER BY label) AS labels
        FROM eligible_fragments GROUP BY coalesce(definition_id, fragment_id), kind, text, source
    ",
            )
            .await?
            .into_view(),
    )?;
    crate::native_catalog::work(
        session,
        "folded_fragments",
        session
            .sql(
                r"
        SELECT * FROM (
            SELECT f.*, row_number() OVER (
                PARTITION BY coalesce(definition_id, fragment_id), kind, text, source
                ORDER BY rank_score DESC, label ASC, fragment_id ASC
            ) AS position FROM eligible_fragments f
        ) WHERE position = 1
    ",
            )
            .await?
            .into_view(),
    )?;
    let api = session.sql(r"
        SELECT CAST(0 AS INTEGER UNSIGNED) AS hit_order, s.observation_id || '/' || s.symbol_id AS candidate_id,
            s.observation_id AS fact_id, s.path AS label, s.path, coalesce(s.declared_kind, s.kind) AS symbol_kind, s.signature,
            CAST(NULL AS VARCHAR) AS fragment_kind, coalesce(s.signature, s.doc_summary, s.docs, '') AS excerpt,
            array_remove(a.paths, s.path) AS also_at, s.is_deprecated AS deprecated, s.ranking, s.rank_score
        FROM folded_symbols s JOIN matching_aliases a ON s.definition_id = a.definition_id
    ").await?;
    let fragments = session.sql(r"
        SELECT CAST(1 AS INTEGER UNSIGNED) AS hit_order, f.fragment_id AS candidate_id, f.fragment_id AS fact_id, f.label, CAST(NULL AS VARCHAR) AS path, CAST(NULL AS VARCHAR) AS symbol_kind, CAST(NULL AS VARCHAR) AS signature,
            f.kind AS fragment_kind, f.text AS excerpt, array_remove(a.labels, f.label) AS also_at, false AS deprecated, f.ranking, f.rank_score
        FROM folded_fragments f JOIN fragment_aliases a
          ON coalesce(f.definition_id, f.fragment_id) = a.fold_key AND f.kind = a.kind
          AND f.text = a.text AND f.source IS NOT DISTINCT FROM a.source
    ").await?;
    crate::native_catalog::work(
        session,
        "search_api_candidates",
        crate::provider::derived(api, "search_candidates")?.into_view(),
    )?;
    crate::native_catalog::work(
        session,
        "search_fragment_candidates",
        crate::provider::derived(fragments, "search_candidates")?.into_view(),
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
    // Exact counts and pages consume the same native ranking once. Keep long text and
    // signatures in their admitted source relations until the small page is selected.
    let index = folded(session, spec, options).await?.select(
        [
            "hit_order",
            "candidate_id",
            "fact_id",
            "label",
            "path",
            "symbol_kind",
            "fragment_kind",
            "also_at",
            "deprecated",
            "ranking",
            "rank_score",
        ]
        .into_iter()
        .map(col),
    )?;
    let completed = crate::operation_index::materialize(
        runtime,
        index,
        crate::preparation::QueryFamily::SearchIndex,
    )
    .await?;
    let total = completed.rows;
    completed.register(session, "search_candidates")?;
    let folded = session.table("search_candidates").await?;
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
    crate::native_catalog::work(session, "search_page", sorted.into_view())?;
    let hydrated = session
        .sql(
            r"
        SELECT p.*,
            CASE p.hit_order WHEN 0 THEN a.payload.signature ELSE NULL END AS signature,
            CASE p.hit_order WHEN 0 THEN coalesce(a.payload.signature, a.payload.doc_summary, a.docs, '') ELSE f.text END AS excerpt,
            CASE p.hit_order WHEN 0 THEN a.source ELSE f.source END AS source
        FROM search_page p
        LEFT JOIN snapshot.evidence.api_observations a ON p.fact_id = a.observation_id AND p.hit_order = 0
        LEFT JOIN snapshot.evidence.fragments f ON p.fact_id = f.fragment_id AND p.hit_order = 1
        ORDER BY p.rank_score DESC, p.hit_order ASC, p.label ASC, p.candidate_id ASC
    ",
        )
        .await?;
    // CASE derives a new field and does not copy top-level metadata. Declare the role of
    // this selected qualified source explicitly; row provenance still comes from the joins.
    let source = crate::admission::Relation::ApiObservations.schema()?;
    let hydrated = hydrated.with_column(
        "source",
        col("source").alias_with_metadata(
            "source",
            Some(datafusion::common::metadata::FieldMetadata::from(
                source.field_with_name("source")?.metadata().clone(),
            )),
        ),
    )?;
    let output = runtime
        .execute_family(hydrated, Some(crate::preparation::QueryFamily::Search))
        .await?;
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
