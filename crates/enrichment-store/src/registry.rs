//! Native registry selection. Rust decodes source facts and the one selected effect descriptor.
use crate::runtime::QueryRuntime;
use datafusion::{
    common::ScalarValue,
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::SessionContext,
};
use enrichment_core::registry::{IndexEntry, SelectionError, UpstreamCheck};

pub struct RustIndex {
    session: SessionContext,
    runtime: QueryRuntime,
}

/// One bounded acquisition descriptor; its source line belongs to the original index bytes.
#[derive(Debug)]
pub struct Selected {
    pub entry: IndexEntry,
    pub source_line: u64,
    pub upstream: UpstreamCheck,
}

const FACTS: &str = r#"
WITH typed AS (
    SELECT *, semver_precedence_key_v1(release['vers']) AS precedence,
           contains(split_part(release['vers'], '+', 1), '-') AS prerelease
    FROM registry_facts
), valid AS (SELECT * FROM typed WHERE precedence IS NOT NULL)
"#;

impl RustIndex {
    pub fn new(runtime: &QueryRuntime, facts: DataFrame) -> Result<Self> {
        enrichment_core::native_schema::check_input(
            facts.schema().as_arrow(),
            &enrichment_core::registry::facts::schema(),
        )?;
        let session = runtime.session();
        crate::native_catalog::work(&session, "registry_facts", facts.into_view())?;
        Ok(Self {
            session,
            runtime: runtime.clone(),
        })
    }

    /// Exact requests stay exact; the independently selected newest release never upgrades one.
    pub async fn select(
        &self,
        requested: Option<&str>,
        allow_prerelease: bool,
        allow_yanked: bool,
    ) -> Result<std::result::Result<Selected, SelectionError>> {
        enrichment_core::native_struct! {
        struct Decision {
            status: String => enrichment_core::native_union::Rule::Text,
            entry: Option<IndexEntry> => enrichment_core::native_union::Rule::Text,
            source_line: Option<u64> => enrichment_core::native_union::Rule::Text,
            newest_stable: Option<String> => enrichment_core::native_union::Rule::Text,
            newest_any: Option<String> => enrichment_core::native_union::Rule::Text,
            resolved_is_newest_stable: bool => enrichment_core::native_union::Rule::Text,
            published_versions: usize => enrichment_core::native_union::Rule::Text,
        }
        }
        let sql = format!(
            r#"{FACTS}, picked AS (
            SELECT * FROM valid
            WHERE (CAST($1 AS VARCHAR) IS NOT NULL AND release['vers'] = $1)
               OR (CAST($1 AS VARCHAR) IS NULL AND ($2 OR NOT prerelease)
                   AND ($3 OR NOT release['yanked']))
            ORDER BY precedence DESC, source_line DESC LIMIT 1
        ), stable AS (
            SELECT release['vers'] AS vers FROM valid
            WHERE NOT prerelease AND NOT release['yanked']
            ORDER BY precedence DESC, source_line DESC LIMIT 1
        ), newest AS (
            SELECT release['vers'] AS vers FROM valid WHERE NOT release['yanked']
            ORDER BY precedence DESC, source_line DESC LIMIT 1
        ), totals AS (SELECT CAST(count(*) AS BIGINT UNSIGNED) AS published_versions FROM registry_facts)
        SELECT CASE WHEN totals.published_versions = 0 THEN 'no_versions'
          WHEN CAST($1 AS VARCHAR) IS NOT NULL AND semver_precedence_key_v1($1) IS NULL THEN 'invalid'
          WHEN picked.source_line IS NULL AND CAST($1 AS VARCHAR) IS NOT NULL THEN 'not_found'
          WHEN picked.source_line IS NULL THEN 'no_eligible'
          WHEN picked.release['yanked'] AND NOT $3 THEN 'yanked'
          ELSE 'selected' END AS status,
          picked.release AS entry, picked.source_line, stable.vers AS newest_stable,
          newest.vers AS newest_any,
          ((picked.release['vers'] IS NOT DISTINCT FROM stable.vers) AND stable.vers IS NOT NULL) AS resolved_is_newest_stable,
          totals.published_versions
        FROM totals LEFT JOIN picked ON true LEFT JOIN stable ON true LEFT JOIN newest ON true"#
        );
        let query = self.session.sql(&sql).await?.with_param_values(vec![
            ScalarValue::Utf8(requested.map(str::to_owned)),
            ScalarValue::Boolean(Some(allow_prerelease)),
            ScalarValue::Boolean(Some(allow_yanked)),
        ])?;
        let mut rows: Vec<Decision> = rows(&self.runtime, query, 1).await?;
        let decision = rows
            .pop()
            .ok_or_else(|| DataFusionError::Internal("missing registry decision".into()))?;
        let requested = requested.unwrap_or_default().to_owned();
        Ok(match decision.status.as_str() {
            "no_versions" => Err(SelectionError::NoVersions),
            "invalid" => Err(SelectionError::InvalidVersion { requested }),
            "not_found" => Err(SelectionError::VersionNotFound {
                nearest: self.neighbours(&requested).await?,
                requested,
            }),
            "no_eligible" => Err(SelectionError::NoEligibleVersion),
            "yanked" => Err(SelectionError::Yanked { requested }),
            "selected" => Ok(Selected {
                entry: decision
                    .entry
                    .ok_or_else(|| DataFusionError::Internal("selected release is null".into()))?,
                source_line: decision
                    .source_line
                    .ok_or_else(|| DataFusionError::Internal("selected source is null".into()))?,
                upstream: UpstreamCheck {
                    newest_stable: decision.newest_stable,
                    newest_any: decision.newest_any,
                    resolved_is_newest_stable: decision.resolved_is_newest_stable,
                    published_versions: decision.published_versions,
                },
            }),
            _ => {
                return Err(DataFusionError::Internal(
                    "unknown registry decision".into(),
                ));
            }
        })
    }

    async fn neighbours(&self, requested: &str) -> Result<Vec<String>> {
        enrichment_core::native_struct! {
        struct Neighbour {
            vers: String => enrichment_core::native_union::Rule::Text,
        }
        }
        let sql = format!(
            r#"{FACTS}, below AS (
            SELECT * FROM valid WHERE precedence < semver_precedence_key_v1($1)
            ORDER BY precedence DESC, source_line DESC LIMIT 3
        ), above AS (
            SELECT * FROM valid WHERE precedence >= semver_precedence_key_v1($1)
            ORDER BY precedence ASC, source_line ASC LIMIT 3
        ), neighbours AS (SELECT * FROM below UNION ALL SELECT * FROM above)
        SELECT release['vers'] AS vers FROM neighbours
        ORDER BY precedence ASC, source_line ASC LIMIT 5"#
        );
        let rows: Vec<Neighbour> = rows(
            &self.runtime,
            self.session
                .sql(&sql)
                .await?
                .with_param_values(vec![ScalarValue::Utf8(Some(requested.into()))])?,
            5,
        )
        .await?;
        Ok(rows.into_iter().map(|r| r.vers).collect())
    }
}

pub(crate) async fn rows<T: enrichment_core::native_union::NativeStruct>(
    runtime: &QueryRuntime,
    frame: datafusion::dataframe::DataFrame,
    bound: usize,
) -> Result<Vec<T>> {
    // Project the declared descriptor from a richer native decision relation.
    let selected = frame.select(
        T::fields()
            .iter()
            .map(|field| datafusion::prelude::col(field.name()))
            .collect::<Vec<_>>(),
    )?;
    runtime.records(selected, bound).await
}
