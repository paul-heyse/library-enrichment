//! A bounded Arrow dependency frontier. DataFusion selects every expansion/acquisition step.
//! Rust retains immutable fact batches and performs only the selected physical acquisition.
use crate::{control_jobs::encode, registry::rows, runtime::QueryRuntime};
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{SessionContext, col},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub filename: String,
    pub url: String,
    pub sha256: String,
    pub requirements: Vec<String>,
    pub extras: Vec<String>,
    pub downloaded_bytes: u64,
}

pub enum Work {
    Expand {
        name: String,
        requirements: Vec<String>,
        extras: Vec<String>,
    },
    Acquire {
        name: String,
        specifiers: String,
        extras: Vec<String>,
    },
}

pub struct Frontier {
    runtime: QueryRuntime,
    packages: Vec<RecordBatch>,
    expansions: Vec<RecordBatch>,
}

fn text(name: &str) -> Field {
    Field::new(name, DataType::Utf8, true)
}
fn strings(name: &str) -> Field {
    Field::new(name, DataType::List(Arc::new(text("item"))), true)
}
fn schema(fields: Vec<Field>) -> SchemaRef {
    Arc::new(Schema::new(
        fields
            .into_iter()
            .map(|f| f.with_nullable(true))
            .collect::<Vec<_>>(),
    ))
}
fn package_schema() -> SchemaRef {
    schema(vec![
        text("name"),
        text("version"),
        text("filename"),
        text("url"),
        text("sha256"),
        strings("requirements"),
        strings("extras"),
        Field::new("downloaded_bytes", DataType::UInt64, false),
    ])
}
pub(crate) fn demand_schema() -> SchemaRef {
    schema(vec![text("name"), text("specifiers"), strings("extras")])
}
fn expansion_schema() -> SchemaRef {
    schema(vec![
        text("name"),
        Field::new("sequence", DataType::UInt64, false),
        strings("extras"),
        Field::new(
            "demands",
            DataType::List(Arc::new(Field::new(
                "item",
                DataType::Struct(demand_schema().fields().clone()),
                true,
            ))),
            false,
        ),
    ])
}
fn register(
    session: &SessionContext,
    name: &str,
    schema: SchemaRef,
    batches: &[RecordBatch],
) -> Result<()> {
    session.register_table(
        name,
        Arc::new(datafusion::datasource::MemTable::try_new(
            schema,
            vec![batches.to_vec()],
        )?),
    )?;
    Ok(())
}

const VIEWS: &str = r#"
WITH latest AS (
 SELECT *, row_number() OVER (PARTITION BY name ORDER BY sequence DESC) AS position FROM expansion_facts
), expansions AS (SELECT * EXCLUDE(position) FROM latest WHERE position=1),
demands AS (SELECT unnest(demands) AS demand FROM expansions),
extra_facts AS (
 SELECT name,unnest(extras) AS extra FROM package_facts
 UNION ALL SELECT demand.name,unnest(demand.extras) AS extra FROM demands
), extras AS (SELECT name,array_sort(array_agg(DISTINCT extra)) AS extras FROM extra_facts GROUP BY name),
packages AS (SELECT p.* EXCLUDE(extras),coalesce(e.extras,[]) AS extras FROM package_facts p LEFT JOIN extras e ON p.name=e.name)
"#;

impl Frontier {
    pub fn new(runtime: &QueryRuntime, root: Package) -> Result<Self> {
        let mut this = Self {
            runtime: runtime.clone(),
            packages: Vec::new(),
            expansions: Vec::new(),
        };
        this.acquired(root)?;
        Ok(this)
    }
    pub fn acquired(&mut self, package: Package) -> Result<()> {
        enrichment_core::canonical::serialized_size(&package, 1024 * 1024)?;
        self.packages.push(encode(package_schema(), &[package])?);
        self.input_budget()
    }
    fn input_budget(&self) -> Result<()> {
        let bytes: usize = self
            .packages
            .iter()
            .chain(&self.expansions)
            .map(RecordBatch::get_array_memory_size)
            .sum();
        if bytes > 64 * 1024 * 1024 {
            return Err(DataFusionError::ResourcesExhausted(
                "dependency Arrow input exceeds 64 MiB".into(),
            ));
        }
        Ok(())
    }
    fn session(&self) -> Result<SessionContext> {
        let session = self.runtime.session();
        register(&session, "package_facts", package_schema(), &self.packages)?;
        register(
            &session,
            "expansion_facts",
            expansion_schema(),
            &self.expansions,
        )?;
        Ok(session)
    }
    pub async fn next(&self) -> Result<Option<Work>> {
        let session = self.session()?;
        self.runtime
            .require_empty(
                session
                    .sql("SELECT name FROM package_facts GROUP BY name HAVING count(*)>1 LIMIT 1")
                    .await?,
                "dependency_selection_conflict",
                "dependency_frontier",
            )
            .await?;
        self.runtime.require_empty(session.sql("SELECT 'dependency_packages' AS witness FROM package_facts HAVING count(*)>256 OR sum(downloaded_bytes)>536870912 UNION ALL SELECT 'dependency_expansions' FROM expansion_facts HAVING count(*)>1024").await?, "dependency_resource_limits", "dependency_frontier").await?;
        self.runtime.require_empty(session.sql(&format!("{VIEWS} SELECT p.name FROM packages p JOIN demands d ON p.name=d.demand.name WHERE NOT pep440_matches_v1(d.demand.specifiers,p.version) LIMIT 1")).await?, "dependency_selected_version_conflict", "dependency_frontier").await?;
        #[derive(Deserialize)]
        struct Action {
            action: String,
            name: String,
            requirements: Option<Vec<String>>,
            extras: Vec<String>,
            specifiers: Option<String>,
        }
        let frame = session.sql(&format!(r#"{VIEWS}, work AS (
           SELECT 0 AS priority,'expand' AS action,p.name,p.requirements,p.extras,CAST(NULL AS VARCHAR) AS specifiers
           FROM packages p LEFT JOIN expansions e ON p.name=e.name WHERE e.name IS NULL OR p.extras<>e.extras
           UNION ALL
           SELECT 1,'acquire',d.demand.name,NULL,coalesce(e.extras,[]),coalesce(string_agg(DISTINCT nullif(d.demand.specifiers,''),','),'')
           FROM demands d LEFT ANTI JOIN packages p ON p.name=d.demand.name LEFT JOIN extras e ON e.name=d.demand.name
           GROUP BY d.demand.name,e.extras
        ) SELECT action,name,requirements,extras,specifiers FROM work ORDER BY priority,name LIMIT 1"#)).await?;
        let action: Option<Action> = rows(&self.runtime, frame, 1).await?.pop();
        action
            .map(|a| {
                Ok(match a.action.as_str() {
                    "expand" => Work::Expand {
                        name: a.name,
                        requirements: a
                            .requirements
                            .ok_or_else(|| invalid("missing expansion metadata"))?,
                        extras: a.extras,
                    },
                    "acquire" => Work::Acquire {
                        name: a.name,
                        specifiers: a
                            .specifiers
                            .ok_or_else(|| invalid("missing acquisition specifiers"))?,
                        extras: a.extras,
                    },
                    _ => return Err(invalid("unknown dependency command")),
                })
            })
            .transpose()
    }
    /// The active requirement relation is already selected by native marker expressions.
    pub async fn expanded(
        &mut self,
        name: &str,
        extras: &[String],
        demands: DataFrame,
    ) -> Result<()> {
        let session = self.session()?;
        crate::native_catalog::work(&session, "active_demands", demands.into_view())?;
        #[derive(Serialize)]
        struct ExpansionInput<'a> {
            name: &'a str,
            extras: &'a [String],
        }
        let input = encode(
            schema(vec![text("name"), strings("extras")]),
            &[ExpansionInput { name, extras }],
        )?;
        register(&session, "expansion_input", input.schema(), &[input])?;
        let frame = session.sql("WITH demand_set AS (SELECT coalesce(array_agg(named_struct('name',name,'specifiers',specifiers,'extras',extras) ORDER BY name,specifiers),[]) AS demands FROM active_demands), sequence AS (SELECT coalesce(max(sequence),CAST(0 AS BIGINT UNSIGNED))+CAST(1 AS BIGINT UNSIGNED) AS sequence FROM expansion_facts) SELECT i.name,s.sequence,i.extras,d.demands FROM expansion_input i CROSS JOIN sequence s CROSS JOIN demand_set d").await?;
        let frame = crate::native_delta::project(frame, &expansion_schema())?;
        self.expansions
            .extend(self.runtime.execute(frame).await?.batches);
        self.input_budget()
    }
    pub async fn finish(&self) -> Result<(Vec<u8>, String)> {
        if self.next().await?.is_some() {
            return Err(invalid("dependency frontier still has work"));
        }
        let session = self.session()?;
        let packages: Vec<Package> = rows(
            &self.runtime,
            session
                .sql(&format!("{VIEWS} SELECT * FROM packages ORDER BY name"))
                .await?,
            256,
        )
        .await?;
        #[derive(Deserialize)]
        struct Requirements {
            requirements: String,
        }
        let requirements: Requirements = rows(&self.runtime, session.sql("SELECT coalesce(string_agg(concat(name,'==',version,' --hash=sha256:',sha256,chr(10)),'' ORDER BY name),'') AS requirements FROM package_facts").await?, 1).await?.pop().ok_or_else(|| invalid("missing requirements encoding"))?;
        // Final bounded protocol encoding; selection, ordering and requirement composition ran natively.
        let lock = enrichment_core::canonical::to_canonical_string(&serde_json::json!({"resolver":"native-registry-closure/1","python":"3.14.7","platform":"linux-x86_64","packages":packages,"limitations":["Conservative pure-wheel selection; conflicts with retained choices are unresolved, never reported as a solved environment."]})).into_bytes();
        Ok((lock, requirements.requirements))
    }
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

/// Mechanical parsed-requirement ingress, joined by ordinal to native marker results.
pub(crate) fn requirement_facts(
    requirements: &[enrichment_core::producer::python::requirements::Requirement],
) -> Result<RecordBatch> {
    #[derive(Serialize)]
    struct Fact<'a> {
        index: u64,
        name: &'a str,
        specifiers: String,
        extras: &'a [String],
    }
    let mut fields = demand_schema()
        .fields()
        .iter()
        .map(|f| f.as_ref().clone())
        .collect::<Vec<_>>();
    fields.push(Field::new("index", DataType::UInt64, false));
    encode(
        schema(fields),
        &requirements
            .iter()
            .enumerate()
            .map(|(index, r)| Fact {
                index: index as u64,
                name: &r.name,
                specifiers: r.versions.to_string(),
                extras: &r.extras,
            })
            .collect::<Vec<_>>(),
    )
}

pub async fn active_demands(
    runtime: &QueryRuntime,
    requirements: &[enrichment_core::producer::python::requirements::Requirement],
    environment: &enrichment_core::producer::python::requirements::MarkerEnvironment,
    extras: &[String],
) -> Result<DataFrame> {
    let selected =
        crate::python_registry::active_requirement_plan(runtime, requirements, environment, extras)
            .await?;
    let session = runtime.session();
    let input = crate::native_catalog::batch(
        &session,
        "requirement_facts",
        requirement_facts(requirements)?,
    )?;
    input
        .join(
            selected,
            datafusion::logical_expr::JoinType::LeftSemi,
            &["index"],
            &["index"],
            None,
        )?
        .select(vec![col("name"), col("specifiers"), col("extras")])
}
