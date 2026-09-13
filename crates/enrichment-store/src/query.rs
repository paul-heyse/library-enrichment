//! Structured retrieval over a published snapshot with DataFusion (blueprint §1.1, §7.1).
//!
//! A [`SnapshotReader`] registers the snapshot's Parquet tables in a `SessionContext` and
//! answers the questions the research tools ask: the faceted overview, exact and prefix
//! symbol lookups, candidate rows for lexical search, and the fragments and edges around one
//! symbol. Every filter is a bound parameter or a DataFrame expression, never interpolated
//! text, so a caller-supplied string is only ever data.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use datafusion::common::ScalarValue;
use datafusion::error::DataFusionError;
use datafusion::prelude::{ParquetReadOptions, SessionConfig, SessionContext, col, lit};
use enrichment_core::evidence::{
    EvidenceFragment, FragmentKind, Relationship, SnapshotManifest, Symbol, SymbolKind,
};
use enrichment_core::identity::SnapshotId;
use serde::{Deserialize, Serialize};

use crate::paths::StatePaths;
use crate::snapshot;
use crate::tables;

/// One open snapshot.
pub struct SnapshotReader {
    ctx: SessionContext,
    dir: PathBuf,
    manifest: SnapshotManifest,
}

/// Why a snapshot could not be read.
#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    /// The snapshot is not published.
    #[error("snapshot {0} is not published")]
    NotPublished(String),
    /// Filesystem failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Query engine failure.
    #[error("query failed: {0}")]
    DataFusion(#[from] DataFusionError),
}

/// A namespace facet in an overview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamespaceFacet {
    /// The module path.
    pub path: String,
    /// First paragraph of the module docs.
    pub doc_summary: Option<String>,
    /// Distinct definitions directly under this module, by kind.
    pub counts_by_kind: BTreeMap<String, u64>,
    /// A bounded sample of direct children (path, kind, summary), definitions counted once.
    pub children: Vec<OverviewChild>,
    /// Children beyond the sample.
    pub truncated_children: u64,
}

/// One child in a namespace sample.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewChild {
    /// Public path.
    pub path: String,
    /// Kind.
    pub kind: SymbolKind,
    /// Summary, when documented.
    pub doc_summary: Option<String>,
    /// Whether this path re-exports a definition elsewhere.
    pub is_reexport: bool,
    /// Whether deprecated.
    pub deprecated: bool,
}

/// The faceted overview of a snapshot (§7.1: a tree and facets, not a symbol dump).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Overview {
    /// Distinct definitions by kind, crate-wide.
    pub definitions_by_kind: BTreeMap<String, u64>,
    /// Namespaces, root first.
    pub namespaces: Vec<NamespaceFacet>,
    /// Feature definitions: name and what it enables.
    pub features: BTreeMap<String, String>,
    /// Documentation headings from README sections, in order.
    pub documentation_headings: Vec<String>,
    /// Changelog headings, in order.
    pub release_note_headings: Vec<String>,
    /// Example names.
    pub examples: Vec<String>,
    /// Re-export count and unresolved count, from the manifest.
    pub reexports: u64,
    /// Re-exports whose target is outside this crate.
    pub unresolved_reexports: u64,
}

impl SnapshotReader {
    /// Open a published snapshot.
    ///
    /// # Errors
    ///
    /// Fails if the snapshot is not published or a table cannot be registered.
    pub async fn open(paths: &StatePaths, id: &SnapshotId) -> Result<Self, QueryError> {
        let manifest = snapshot::read_manifest(paths, id)?
            .ok_or_else(|| QueryError::NotPublished(id.to_string()))?;
        let dir = snapshot::snapshot_dir(paths, id);
        // DataFusion rewrites Utf8 columns to Utf8View when scanning Parquet unless told not
        // to; the decoders in `tables` read the schema the writer produced, and a view array
        // would decode as no rows at all. Measured by `symbols_round_trip_through_datafusion`.
        let config = SessionConfig::new().set_bool(
            "datafusion.execution.parquet.schema_force_view_types",
            false,
        );
        let ctx = SessionContext::new_with_config(config);
        for (name, file) in [
            ("symbols", tables::SYMBOLS_FILE),
            ("relationships", tables::RELATIONSHIPS_FILE),
            ("fragments", tables::FRAGMENTS_FILE),
        ] {
            let path = dir.join(file);
            ctx.register_parquet(
                name,
                path.to_string_lossy().as_ref(),
                ParquetReadOptions::default(),
            )
            .await?;
        }
        Ok(Self { ctx, dir, manifest })
    }

    /// The manifest.
    #[must_use]
    pub fn manifest(&self) -> &SnapshotManifest {
        &self.manifest
    }

    /// The snapshot directory.
    #[must_use]
    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    async fn symbols_where(
        &self,
        sql: &str,
        params: Vec<ScalarValue>,
    ) -> Result<Vec<Symbol>, QueryError> {
        let df = self.ctx.sql(sql).await?.with_param_values(params)?;
        let batches = df.collect().await?;
        Ok(batches
            .iter()
            .flat_map(tables::symbols_from_batch)
            .collect())
    }

    /// Every symbol, in stored order. Small crates fit comfortably; callers page above this.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn all_symbols(&self) -> Result<Vec<Symbol>, QueryError> {
        self.symbols_where("SELECT * FROM symbols", Vec::new())
            .await
    }

    /// Symbols at exactly this path (a path may carry several kinds, e.g. a struct and its
    /// constructor macro; trait-impl methods share a path across traits).
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn symbols_at(&self, path: &str) -> Result<Vec<Symbol>, QueryError> {
        self.symbols_where(
            "SELECT * FROM symbols WHERE path = $1",
            vec![ScalarValue::from(path)],
        )
        .await
    }

    /// Symbols whose path ends with `::<suffix>` or equals it, for unqualified lookups.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn symbols_ending_with(&self, suffix: &str) -> Result<Vec<Symbol>, QueryError> {
        let pattern = format!("%::{}", escape_like(suffix));
        self.symbols_where(
            "SELECT * FROM symbols WHERE path = $1 OR path LIKE $2 ESCAPE '\\'",
            vec![
                ScalarValue::from(suffix),
                ScalarValue::from(pattern.as_str()),
            ],
        )
        .await
    }

    /// Symbols whose path, signature or docs contain any of the tokens, case-insensitively.
    /// Ranking happens in the core; this only narrows.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn symbols_matching_any(&self, tokens: &[String]) -> Result<Vec<Symbol>, QueryError> {
        if tokens.is_empty() {
            return Ok(Vec::new());
        }
        let df = self.ctx.table("symbols").await?;
        let mut predicate = None;
        for token in tokens {
            let pattern = format!("%{}%", escape_like(&token.to_lowercase()));
            let clause = datafusion::functions::string::expr_fn::lower(col("path"))
                .like(lit(pattern.clone()))
                .or(
                    datafusion::functions::string::expr_fn::lower(col("signature"))
                        .like(lit(pattern.clone())),
                )
                .or(datafusion::functions::string::expr_fn::lower(col("docs")).like(lit(pattern)));
            predicate = Some(match predicate {
                None => clause,
                Some(p) => datafusion::prelude::Expr::or(p, clause),
            });
        }
        let Some(predicate) = predicate else {
            return Ok(Vec::new());
        };
        let batches = df.filter(predicate)?.collect().await?;
        Ok(batches
            .iter()
            .flat_map(tables::symbols_from_batch)
            .collect())
    }

    /// Fragments about one subject.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn fragments_for(&self, subject: &str) -> Result<Vec<EvidenceFragment>, QueryError> {
        let df = self
            .ctx
            .sql("SELECT * FROM fragments WHERE subject = $1")
            .await?
            .with_param_values(vec![ScalarValue::from(subject)])?;
        let batches = df.collect().await?;
        Ok(batches
            .iter()
            .flat_map(tables::fragments_from_batch)
            .collect())
    }

    /// Fragments of one kind, in stored order.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn fragments_of_kind(
        &self,
        kind: FragmentKind,
    ) -> Result<Vec<EvidenceFragment>, QueryError> {
        let df = self
            .ctx
            .sql("SELECT * FROM fragments WHERE kind = $1")
            .await?
            .with_param_values(vec![ScalarValue::from(kind.as_str())])?;
        let batches = df.collect().await?;
        Ok(batches
            .iter()
            .flat_map(tables::fragments_from_batch)
            .collect())
    }

    /// Fragments whose text contains any token, case-insensitively, optionally limited to kinds.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn fragments_matching_any(
        &self,
        tokens: &[String],
        kinds: Option<&[FragmentKind]>,
    ) -> Result<Vec<EvidenceFragment>, QueryError> {
        if tokens.is_empty() {
            return Ok(Vec::new());
        }
        let df = self.ctx.table("fragments").await?;
        let mut predicate: Option<datafusion::prelude::Expr> = None;
        for token in tokens {
            let pattern = format!("%{}%", escape_like(&token.to_lowercase()));
            let clause = datafusion::functions::string::expr_fn::lower(col("text"))
                .like(lit(pattern.clone()))
                .or(
                    datafusion::functions::string::expr_fn::lower(col("subject"))
                        .like(lit(pattern)),
                );
            predicate = Some(match predicate {
                None => clause,
                Some(p) => p.or(clause),
            });
        }
        let mut predicate = predicate.expect("tokens is non-empty");
        if let Some(kinds) = kinds {
            let list: Vec<datafusion::prelude::Expr> =
                kinds.iter().map(|k| lit(k.as_str())).collect();
            predicate = predicate.and(col("kind").in_list(list, false));
        }
        let batches = df.filter(predicate)?.collect().await?;
        Ok(batches
            .iter()
            .flat_map(tables::fragments_from_batch)
            .collect())
    }

    /// Edges touching one symbol, as source or target.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn relationships_for(
        &self,
        symbol_id: &str,
    ) -> Result<Vec<Relationship>, QueryError> {
        let df = self
            .ctx
            .sql("SELECT * FROM relationships WHERE source_id = $1 OR target_id = $1")
            .await?
            .with_param_values(vec![ScalarValue::from(symbol_id)])?;
        let batches = df.collect().await?;
        Ok(batches
            .iter()
            .flat_map(tables::relationships_from_batch)
            .collect())
    }

    /// The faceted overview (§7.1), optionally narrowed to one module subtree.
    ///
    /// # Errors
    ///
    /// Fails on a query error.
    pub async fn overview(
        &self,
        area: Option<&str>,
        per_namespace: usize,
    ) -> Result<Overview, QueryError> {
        let symbols = self.all_symbols().await?;
        let in_area = |s: &Symbol| match area {
            None => true,
            Some(area) => s.path == area || s.path.starts_with(&format!("{area}::")),
        };

        let mut definitions_by_kind: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for s in symbols.iter().filter(|s| in_area(s)) {
            definitions_by_kind
                .entry(s.kind.as_str().to_owned())
                .or_default()
                .insert(s.definition_id.clone());
        }

        let mut namespaces = Vec::new();
        let modules: Vec<&Symbol> = symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Module && in_area(s))
            .collect();
        let root_path = self.manifest.crate_name.clone();
        let mut namespace_paths: Vec<(String, Option<String>)> = Vec::new();
        if area.is_none_or(|a| a == root_path) {
            namespace_paths.push((root_path.clone(), None));
        }
        for m in &modules {
            namespace_paths.push((m.path.clone(), m.doc_summary.clone()));
        }
        namespace_paths.sort();
        namespace_paths.dedup_by(|a, b| a.0 == b.0);

        for (ns, doc_summary) in namespace_paths {
            let mut seen_definitions = BTreeSet::new();
            let mut counts_by_kind: BTreeMap<String, u64> = BTreeMap::new();
            let mut children = Vec::new();
            let mut truncated = 0u64;
            for s in symbols
                .iter()
                .filter(|s| s.parent_path.as_deref() == Some(ns.as_str()))
                .filter(|s| {
                    !matches!(
                        s.kind,
                        SymbolKind::Method
                            | SymbolKind::StructField
                            | SymbolKind::Variant
                            | SymbolKind::AssocConst
                            | SymbolKind::AssocType
                    )
                })
            {
                if !seen_definitions.insert(s.definition_id.clone()) {
                    continue;
                }
                *counts_by_kind
                    .entry(s.kind.as_str().to_owned())
                    .or_default() += 1;
                if children.len() < per_namespace {
                    children.push(OverviewChild {
                        path: s.path.clone(),
                        kind: s.kind,
                        doc_summary: s.doc_summary.clone(),
                        is_reexport: s.is_reexport,
                        deprecated: s.deprecated.is_some(),
                    });
                } else {
                    truncated += 1;
                }
            }
            namespaces.push(NamespaceFacet {
                path: ns,
                doc_summary,
                counts_by_kind,
                children,
                truncated_children: truncated,
            });
        }

        let mut features = BTreeMap::new();
        for f in self
            .fragments_of_kind(FragmentKind::FeatureDefinition)
            .await?
        {
            features.insert(f.subject, f.text);
        }
        let documentation_headings = self
            .fragments_of_kind(FragmentKind::ReadmeSection)
            .await?
            .into_iter()
            .map(|f| f.subject)
            .collect();
        let release_note_headings = self
            .fragments_of_kind(FragmentKind::ChangelogSection)
            .await?
            .into_iter()
            .map(|f| f.subject)
            .collect();
        let examples = self
            .fragments_of_kind(FragmentKind::Example)
            .await?
            .into_iter()
            .map(|f| f.subject)
            .collect();

        Ok(Overview {
            definitions_by_kind: definitions_by_kind
                .into_iter()
                .map(|(k, v)| (k, v.len() as u64))
                .collect(),
            namespaces,
            features,
            documentation_headings,
            release_note_headings,
            examples,
            reexports: self.manifest.counts.reexports,
            unresolved_reexports: self.manifest.counts.unresolved_reexports,
        })
    }
}

/// Escape `%`, `_` and `\` for a `LIKE ... ESCAPE '\'` pattern.
#[must_use]
pub fn escape_like(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '%' | '_' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn like_patterns_are_escaped() {
        assert_eq!(escape_like("a_b%c\\d"), "a\\_b\\%c\\\\d");
        assert_eq!(escape_like("plain"), "plain");
    }
}
