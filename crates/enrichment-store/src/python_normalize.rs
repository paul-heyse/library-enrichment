//! Native Python evidence plans over exact Rust-owned Arrow worker facts.
use crate::{arrow_input, leases, runtime::QueryRuntime};
use arrow::{
    array::StringArray,
    datatypes::{DataType, Field, Schema},
    ipc::reader::StreamReader,
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::SessionContext,
};
use enrichment_core::producer::python::{WorkerFile, worker};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
    sync::Arc,
};

pub struct PythonFacts {
    pub(crate) session: SessionContext,
    runtime: QueryRuntime,
    depth: u32,
}
enrichment_core::native_struct! {
pub struct WorkerIdentity {
    griffe_version: String => enrichment_core::native_union::Rule::Text,
    worker_python: String => enrichment_core::native_union::Rule::Text,
    encoder_version: String => enrichment_core::native_union::Rule::Text,
    normalization_depth: u32 => enrichment_core::native_union::Rule::Text,
}
}
enrichment_core::native_struct! {
pub struct Summary {
    observations: u64 => enrichment_core::native_union::Rule::Text,
    unresolved: u64 => enrichment_core::native_union::Rule::Text,
    dynamic: bool => enrichment_core::native_union::Rule::Text,
}
}
enrichment_core::native_struct! {
pub struct AliasFailure {
    reason: String => enrichment_core::native_union::Rule::Text,
    count: u64 => enrichment_core::native_union::Rule::Text,
}
}
enrichment_core::native_struct! {
pub struct FileGap {
    file: String => enrichment_core::native_union::Rule::Text,
    detail: String => enrichment_core::native_union::Rule::Text,
}
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
/// Only Arrow framing and physical allocation limits are checked here. Membership,
/// variants, ordinals, provenance and successful file selection are native queries below.
fn validate_transport(path: &Path) -> Result<()> {
    let mut file = File::open(path)?;
    let bytes = file.metadata()?.len();
    if !(8..=worker::MAX_BYTES).contains(&bytes) {
        return Err(invalid("worker IPC byte bound"));
    }
    file.seek(SeekFrom::End(-8))?;
    let mut end = [0; 8];
    file.read_exact(&mut end)?;
    if end != [255, 255, 255, 255, 0, 0, 0, 0] {
        return Err(invalid("worker IPC lacks final EOS"));
    }
    file.rewind()?;
    let mut input = BufReader::new(file);
    let mut reader = StreamReader::try_new(&mut input, None)?;
    if reader.schema() != worker::schema() {
        return Err(invalid("worker Arrow schema mismatch"));
    }
    let mut rows = 0;
    for batch in &mut reader {
        let batch = batch?;
        if batch.num_rows() > worker::MAX_BATCH_ROWS
            || datafusion::common::utils::memory::get_record_batch_memory_size(&batch)
                > worker::MAX_BATCH_BYTES + 65536
        {
            return Err(invalid("worker Arrow batch bound"));
        }
        for array in batch.columns() {
            array.to_data().validate_full()?;
        }
        rows += batch.num_rows();
        if rows > worker::MAX_OBSERVATIONS + worker::MAX_FILES + 2 {
            return Err(invalid("worker Arrow row bound"));
        }
    }
    drop(reader);
    if input.stream_position()? != bytes {
        return Err(invalid("worker IPC trailing content"));
    }
    Ok(())
}

impl PythonFacts {
    pub async fn open(
        runtime: &QueryRuntime,
        directory: Arc<tempfile::TempDir>,
        files: &[WorkerFile],
    ) -> Result<Self> {
        if files.len() > worker::MAX_FILES {
            return Err(invalid("worker inventory bound"));
        }
        let path = directory.path().join("worker.arrow");
        let check = path.clone();
        runtime
            .blocking(move || validate_transport(&check))
            .await??;
        let session = runtime.session();
        let provider = arrow_input::provider(runtime, &path, worker::schema()).await?;
        let source = session.read_table(provider)?.into_view();
        session.register_table(
            "worker_facts",
            leases::staged_view(&source, &session, directory)?,
        )?;
        let inventory = RecordBatch::try_new(
            Arc::new(Schema::new(vec![
                Field::new("file", DataType::Utf8, false),
                Field::new("origin", DataType::Utf8, false),
            ])),
            vec![
                Arc::new(StringArray::from_iter_values(
                    files.iter().map(|f| f.file.as_str()),
                )),
                Arc::new(StringArray::from_iter_values(files.iter().map(
                    |f| match f.origin {
                        enrichment_core::producer::python::ObservationOrigin::Source => "source",
                        enrichment_core::producer::python::ObservationOrigin::Stub => "stub",
                    },
                ))),
            ],
        )?;
        session.register_table("expected_files", session.read_batch(inventory)?.into_view())?;
        let this = Self {
            session,
            runtime: runtime.clone(),
            depth: crate::native_policy::normalization_depth(&runtime.session().state())?,
        };
        this.admit().await?;
        this.view(
            "observations",
            r#"SELECT f.file, f.ordinal, f.observation AS o,
            f.observation['path'] AS path, f.observation['kind'] AS declared_kind,
            f.observation['alias_target'] AS target
            FROM worker_facts f JOIN worker_facts c ON f.file=c.file
            WHERE f.fact='observation' AND c.fact='file_complete'"#,
        )
        .await?;
        this.view(
            "path_groups",
            r#"SELECT path, min(declared_kind) AS declared_kind,
            min(target) AS target, count(*) AS declarations, count(target) AS targets,
            count(DISTINCT target) AS target_variants, count(DISTINCT declared_kind) AS kinds
            FROM observations GROUP BY path"#,
        )
        .await?;
        this.view(
            "alias_walk",
            &format!(
                r#"WITH RECURSIVE chain AS (
            SELECT path AS alias_path, min(target) AS target, CAST(1 AS BIGINT) AS depth,
                   make_array(path) AS visited
            FROM observations WHERE declared_kind='alias' GROUP BY path
            HAVING count(target)=count(*) AND count(DISTINCT target)=1
            UNION ALL
            SELECT c.alias_path, p.target, c.depth+1, array_append(c.visited,c.target)
            FROM chain c JOIN path_groups p ON c.target=p.path
            WHERE c.depth<{} AND p.targets=p.declarations AND p.target_variants=1
              AND NOT array_has(c.visited,c.target)
        ) SELECT * FROM chain"#,
                this.depth
            ),
        )
        .await?;
        this.view(
            "alias_resolutions",
            r#"SELECT c.alias_path, p.path AS definition_path, p.declared_kind AS kind
          FROM alias_walk c JOIN path_groups p ON c.target=p.path
          WHERE p.targets=0 AND p.kinds=1 AND NOT array_has(c.visited,c.target)"#,
        )
        .await?;
        this.view(
            "alias_failures",
            &format!(
                r#"WITH aliases AS (
            SELECT path,count(DISTINCT target) AS target_variants FROM observations
            WHERE declared_kind='alias' GROUP BY path
        ), stops AS (
            SELECT c.alias_path, CASE WHEN array_has(c.visited,c.target) THEN 'cycle'
              WHEN p.path IS NULL THEN 'missing_target'
              WHEN p.target_variants>1 OR p.targets<>p.declarations AND p.targets<>0
                OR p.targets=0 AND p.kinds<>1 THEN 'ambiguous_declarations'
              WHEN c.depth={} AND p.targets>0 THEN 'depth_limit' END AS reason
            FROM alias_walk c LEFT JOIN path_groups p ON c.target=p.path
        ) SELECT a.path AS alias_path,CASE WHEN a.target_variants<>1 THEN 'ambiguous_declarations'
            ELSE s.reason END AS reason
          FROM aliases a LEFT JOIN stops s ON a.path=s.alias_path AND s.reason IS NOT NULL
          LEFT ANTI JOIN alias_resolutions r ON a.path=r.alias_path"#,
                this.depth
            ),
        )
        .await?;
        this.reject(
            "SELECT alias_path AS witness_id FROM alias_failures WHERE reason IS NULL",
            "native_alias_disposition",
        )
        .await?;
        this.view("variants", r#"SELECT DISTINCT o.path, o.declared_kind,
            coalesce(r.definition_path,o.path) AS definition_path,
            coalesce(r.kind,CASE WHEN o.declared_kind='alias' THEN 'import' ELSE o.declared_kind END) AS kind,
            o.declared_kind='alias' AS is_reexport, r.definition_path IS NOT NULL AS resolved
            FROM observations o LEFT JOIN alias_resolutions r
            ON o.path=r.alias_path AND o.declared_kind='alias'"#).await?;
        this.view(
            "binding_memberships",
            r#"SELECT v.*,
            unnest(make_array(v.path,CASE WHEN v.resolved THEN v.definition_path
                   ELSE CAST(NULL AS VARCHAR) END)) AS observation_path FROM variants v"#,
        )
        .await?;
        this.view(
            "bound_observations",
            r#"SELECT v.path,v.declared_kind,v.definition_path,
            v.kind,v.is_reexport,v.resolved,o.file,o.ordinal,o.o FROM binding_memberships v
            JOIN observations o ON v.observation_path=o.path
            WHERE v.path<>o.path OR v.declared_kind=o.declared_kind"#,
        )
        .await?;
        Ok(this)
    }
    async fn view(&self, name: &str, sql: &str) -> Result<()> {
        self.session
            .register_table(name, self.session.sql(sql).await?.into_view())?;
        Ok(())
    }
    async fn reject(&self, sql: &str, rule: &str) -> Result<()> {
        self.runtime
            .require_empty(self.session.sql(sql).await?, rule, "python_worker")
            .await
    }
    async fn admit(&self) -> Result<()> {
        self.reject(
            "SELECT file AS witness_id FROM expected_files GROUP BY file HAVING count(*)<>1",
            "unique_worker_inventory",
        )
        .await?;
        self.reject(r#"SELECT fact AS witness_id FROM worker_facts WHERE fact IS NULL OR
            fact NOT IN ('producer','observation','file_complete','file_gap','complete') OR
            ((fact IN ('observation','file_complete','file_gap')) IS DISTINCT FROM (file IS NOT NULL)) OR
            ((fact='observation') IS DISTINCT FROM (observation IS NOT NULL)) OR
            ((fact='observation') IS DISTINCT FROM (ordinal IS NOT NULL)) OR
            ((fact='file_gap') IS DISTINCT FROM (detail IS NOT NULL)) OR
            ((fact IN ('file_complete','file_gap','complete')) IS DISTINCT FROM (count IS NOT NULL)) OR
            ((fact='producer') IS DISTINCT FROM (griffe_version IS NOT NULL)) OR
            ((fact='producer') IS DISTINCT FROM (worker_python IS NOT NULL)) OR
            ((fact='producer') IS DISTINCT FROM (encoder_version IS NOT NULL))"#, "worker_variant_contract").await?;
        self.reject(r#"SELECT 'receipt' AS witness_id FROM worker_facts
            HAVING count(*) FILTER (WHERE fact='producer')<>1 OR count(*) FILTER (WHERE fact='complete')<>1
            OR min(count) FILTER (WHERE fact='complete')<>count(*) FILTER (WHERE fact='observation')"#, "worker_terminal_receipt").await?;
        self.reject(&format!("SELECT fact AS witness_id FROM worker_facts WHERE fact='producer' AND (griffe_version<>'{}' OR encoder_version<>'{}' OR worker_python='')",worker::GRIFFE,worker::ENCODER), "worker_producer_identity").await?;
        self.reject(
            r#"SELECT f.file AS witness_id FROM worker_facts f LEFT ANTI JOIN expected_files e
            ON f.file=e.file WHERE f.file IS NOT NULL"#,
            "worker_input_membership",
        )
        .await?;
        self.reject(
            r#"SELECT e.file AS witness_id FROM expected_files e LEFT JOIN worker_facts f
            ON e.file=f.file AND f.fact IN ('file_complete','file_gap')
            GROUP BY e.file HAVING count(f.fact)<>1"#,
            "worker_complete_inventory",
        )
        .await?;
        self.reject(
            r#"SELECT file AS witness_id FROM worker_facts WHERE file IS NOT NULL GROUP BY file
            HAVING count(*) FILTER (WHERE fact='observation')<>max(count)
             OR count(DISTINCT ordinal)<>count(*) FILTER (WHERE fact='observation')
             OR (count(ordinal)>0 AND (min(ordinal)<>0 OR max(ordinal)+1<>count(ordinal)))"#,
            "worker_file_ordinals",
        )
        .await?;
        self.reject(r#"SELECT f.file AS witness_id FROM worker_facts f JOIN expected_files e ON f.file=e.file
            WHERE f.fact='observation' AND ((f.observation['file'] IS DISTINCT FROM f.file)
              OR (f.observation['origin'] IS DISTINCT FROM e.origin)
              OR f.observation['kind'] IS NULL OR f.observation['kind'] NOT IN ('module','class','function','attribute','alias')
              OR f.observation['path'] IS NULL OR f.observation['path']=''
              OR regexp_like(f.observation['path'],'[[:cntrl:]]|^\.|\.\.|\.$')
              OR octet_length(f.observation['path'])>4096 OR array_length(string_to_array(f.observation['path'],'.'))>64
              OR f.observation['line']=0
              OR ((f.observation['kind']='alias') IS DISTINCT FROM (f.observation['alias_target'] IS NOT NULL)))"#, "worker_observation_contract").await?;
        Ok(())
    }
    pub async fn identity(&self) -> Result<WorkerIdentity> {
        crate::registry::rows(&self.runtime, self.session.sql(&format!("SELECT griffe_version,worker_python,encoder_version,CAST({} AS INT UNSIGNED) AS normalization_depth FROM worker_facts WHERE fact='producer'",self.depth)).await?, 1).await?.pop().ok_or_else(|| invalid("worker identity missing"))
    }
    pub async fn summary(&self) -> Result<Summary> {
        crate::registry::rows(&self.runtime, self.session.sql(r#"WITH totals AS (SELECT CAST(count(*) AS BIGINT UNSIGNED) AS observations,
          count(*) FILTER (WHERE ends_with(path,'.__getattr__'))>0 AS dynamic FROM observations),
          unresolved AS (SELECT CAST(count(*) AS BIGINT UNSIGNED) AS unresolved FROM alias_failures)
          SELECT totals.observations, unresolved.unresolved, totals.dynamic FROM totals CROSS JOIN unresolved"#).await?, 1).await?.pop().ok_or_else(|| invalid("worker summary missing"))
    }
    pub async fn alias_failures(&self) -> Result<Vec<AliasFailure>> {
        crate::registry::rows(&self.runtime,self.session.sql("SELECT reason,CAST(count(*) AS BIGINT UNSIGNED) AS count FROM alias_failures GROUP BY reason ORDER BY reason").await?,4).await
    }
    pub async fn gaps(&self) -> Result<Vec<FileGap>> {
        crate::registry::rows(
            &self.runtime,
            self.session
                .sql("SELECT file,detail FROM worker_facts WHERE fact='file_gap' ORDER BY file")
                .await?,
            worker::MAX_FILES,
        )
        .await
    }
    pub async fn observations(&self) -> Result<DataFrame> {
        self.session.table("observations").await
    }
}

use crate::{admission::Relation, repository::EvidencePlans};
use datafusion::{
    common::ScalarValue,
    functions::core::expr_fn::get_field,
    logical_expr::Expr,
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::arrow_model::expressions::{
        child, null, record, variant, variants as union_variants,
    },
    evidence::{ingest::IngestContext, relational::FactSource},
    native_key::Key,
};

fn field_type(relation: Relation, name: &str) -> Result<DataType> {
    Ok(relation
        .schema()?
        .field_with_name(name)?
        .data_type()
        .clone())
}
fn symbol_key(package: &str, path: Expr, kind: Expr) -> Result<Expr> {
    Key::PublicBinding.bind(vec![
        lit(package),
        lit("python"),
        datafusion::functions_nested::expr_fn::string_to_array(
            path,
            lit("."),
            null(&DataType::Utf8)?,
        ),
        kind,
        null(&DataType::Utf8)?,
    ])
}
fn callable_expr(value: Expr, kind: &DataType) -> Result<Expr> {
    use enrichment_core::evidence::arrow_model::expressions::derive_record;
    let parameters = child(kind, "parameters")?;
    let DataType::List(item) = &parameters else {
        return datafusion::common::plan_err!("callable parameters require List");
    };
    let parameter = Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
        "parameter".into(),
        Some(item.clone()),
    ));
    let origin = datafusion::logical_expr::when(
        get_field(parameter.clone(), "kind").in_list(
            vec![lit("variadic positional"), lit("variadic keyword")],
            false,
        ),
        lit("implicit_variadic"),
    )
    .when(
        get_field(parameter.clone(), "reported_default").is_null(),
        lit("absent"),
    )
    .otherwise(lit("declared"))?;
    let transformed = enrichment_core::evidence::arrow_model::expressions::derive_record(
        parameter,
        item.data_type(),
        &[("default_origin", origin)],
    )?;
    let parameters = datafusion::functions_nested::expr_fn::array_transform(
        get_field(value.clone(), "parameters"),
        datafusion::logical_expr::expr_fn::lambda(vec!["parameter"], transformed),
    );
    derive_record(value, kind, &[("parameters", parameters)])
}

fn source_expr(source: &FactSource, overload: Option<Expr>) -> Result<Expr> {
    let kind = field_type(Relation::ApiObservations, "source")?;
    let locator = variant(
        &child(&kind, "locator")?,
        "python_declaration",
        &[
            ("file", get_field(col("o"), "file")),
            ("declaration", get_field(col("o"), "path")),
            ("line", get_field(col("o"), "line")),
            ("origin", get_field(col("o"), "origin")),
            (
                "overload",
                overload.unwrap_or(get_field(col("o"), "overload_ordinal")),
            ),
        ],
    )?;
    let encoded = enrichment_core::evidence::arrow_model::encode::source(&[source])?;
    let descriptor = lit(ScalarValue::try_from_array(&encoded, 0)?);
    let DataType::Struct(fields) = &kind else {
        return Err(invalid("source schema is not a struct"));
    };
    record(
        &kind,
        &fields
            .iter()
            .map(|f| {
                (
                    f.name().as_str(),
                    if f.name() == "locator" {
                        locator.clone()
                    } else {
                        get_field(descriptor.clone(), f.name())
                    },
                )
            })
            .collect::<Vec<_>>(),
    )
}
async fn keyed(frame: DataFrame, key: Key, name: &str, relation: Relation) -> Result<DataFrame> {
    crate::native_delta::project(
        frame.with_column(name, key.expression())?,
        relation.schema()?.as_ref(),
    )?
    .distinct()
}
impl PythonFacts {
    /// Construct evidence with native keys, joins, recursive alias closure, struct projections
    /// and list expansion. No declaration corpus is decoded back into Rust or Python objects.
    pub async fn evidence(
        &self,
        context: &IngestContext,
        source: &FactSource,
    ) -> Result<EvidencePlans> {
        let package = &context.symbol_package;
        let variants = self
            .session
            .table("variants")
            .await?
            .with_column("symbol_id", symbol_key(package, col("path"), col("kind"))?)?
            .with_column(
                "definition_id",
                Key::Definition.bind(vec![
                    lit(package),
                    col("definition_path"),
                    col("kind"),
                    null(&DataType::Utf8)?,
                ])?,
            )?;
        self.session
            .register_table("identified_variants", variants.clone().into_view())?;
        let definitions = crate::native_delta::project(
            variants.clone().select(vec![
                col("definition_id"),
                col("kind"),
                col("definition_path"),
                lit(package).alias("defined_in_package"),
                null(&DataType::Utf8)?.alias("qualifier"),
            ])?,
            Relation::Definitions.schema()?.as_ref(),
        )?
        .distinct()?;
        let paths = self
            .session
            .sql("SELECT *,string_to_array(path,'.') AS components FROM identified_variants")
            .await?;
        let paths = paths.with_column(
            "path_id",
            Key::PublicPath.bind(vec![lit("python"), col("components")])?,
        )?;
        self.session
            .register_table("identified_paths", paths.into_view())?;
        let bindings = self.session.sql("SELECT *,array_slice(components,1,CAST(array_length(components) AS BIGINT)-1) AS parent_components, array_element(components,-1) AS name FROM identified_paths").await?
            .with_column("parent_path_id",datafusion::logical_expr::when(
                datafusion::functions_nested::expr_fn::array_length(col("components")).gt(lit(1_u64)),
                Key::PublicPath.bind(vec![lit("python"),col("parent_components")])?
            ).otherwise(null(&DataType::Utf8)?)?)?
            .with_column("ecosystem",lit("python"))?
            .with_column("qualifier",null(&DataType::Utf8)?)?;
        let bindings =
            crate::native_delta::project(bindings, Relation::Symbols.schema()?.as_ref())?
                .distinct()?;
        let bound = self.session.sql("SELECT b.*, v.symbol_id FROM bound_observations b JOIN identified_variants v ON b.path=v.path AND b.declared_kind=v.declared_kind").await?;
        self.session
            .register_table("identified_observations", bound.clone().into_view())?;
        let subject = variant(
            &field_type(Relation::ApiObservations, "subject")?,
            "symbol",
            &[("symbol_id", col("symbol_id"))],
        )?;
        let payload_kind = field_type(Relation::ApiObservations, "payload")?;
        let python_kind = child(&payload_kind, "python")?;
        let callable_kind = child(&python_kind, "callable")?;
        let overload_kind = child(&python_kind, "overloads")?;
        let DataType::List(overload_item) = &overload_kind else {
            return datafusion::common::plan_err!("overloads require List");
        };
        let overload = Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
            "overload".into(),
            Some(overload_item.clone()),
        ));
        let overload_value = enrichment_core::evidence::arrow_model::expressions::derive_record(
            overload.clone(),
            overload_item.data_type(),
            &[(
                "callable",
                callable_expr(get_field(overload, "callable"), &callable_kind)?,
            )],
        )?;
        let python = record(
            &python_kind,
            &[
                (
                    "callable",
                    callable_expr(get_field(col("o"), "callable"), &callable_kind)?,
                ),
                (
                    "overloads",
                    datafusion::functions_nested::expr_fn::array_transform(
                        get_field(col("o"), "overloads"),
                        datafusion::logical_expr::expr_fn::lambda(vec!["overload"], overload_value),
                    ),
                ),
                ("alias_target", get_field(col("o"), "alias_target")),
                ("bases", get_field(col("o"), "bases")),
                ("publicness", get_field(col("o"), "publicness")),
            ],
        )?;
        let declared_kind = datafusion::logical_expr::when(
            get_field(col("o"), "kind").eq(lit("alias")),
            lit("import"),
        )
        .otherwise(get_field(col("o"), "kind"))?;
        let payload = record(
            &payload_kind,
            &[
                ("declared_kind", declared_kind),
                ("signature", get_field(col("o"), "signature")),
                (
                    "doc_summary",
                    datafusion::functions::unicode::expr_fn::left(
                        datafusion::functions::string::expr_fn::split_part(
                            get_field(col("o"), "docs"),
                            lit("\n"),
                            lit(1_i64),
                        ),
                        lit(240_i64),
                    ),
                ),
                (
                    "cfg_hints",
                    enrichment_core::evidence::arrow_model::expressions::literal(
                        &Vec::<String>::new(),
                    )?,
                ),
                ("python", python),
            ],
        )?;
        let observations = keyed(
            bound.clone().select(vec![
                subject.clone().alias("subject"),
                get_field(col("o"), "origin").alias("origin"),
                lit(&context.environment_id).alias("environment_id"),
                payload.alias("payload"),
                get_field(col("o"), "docs").alias("docs"),
                source_expr(source, None)?.alias("source"),
            ])?,
            Key::ApiObservation,
            "observation_id",
            Relation::ApiObservations,
        )
        .await?;
        let reexports = self.session.sql(r#"SELECT b.*, CASE WHEN resolved THEN 'symbol' ELSE 'unresolved' END AS target_kind,
            o['alias_target'] AS target_path, CASE WHEN resolved THEN 'resolved within distribution' ELSE 'unresolved or conflicting declaration target' END AS qualifier
            FROM identified_observations b WHERE declared_kind='alias' AND o['kind']='alias' AND o['alias_target'] IS NOT NULL"#).await?
            .with_column("target_symbol",datafusion::logical_expr::when(col("resolved"),symbol_key(package,col("definition_path"),col("kind"))?).otherwise(null(&DataType::Utf8)?)?)?;
        let target_type = field_type(Relation::Relationships, "target")?;
        let target = union_variants(
            &target_type,
            col("target_kind"),
            &[
                ("symbol", vec![("symbol_id", col("target_symbol"))]),
                ("unresolved", vec![("path", col("target_path"))]),
            ],
        )?;
        let relationships = reexports.select(vec![
            subject.clone().alias("subject"),
            target.alias("target"),
            lit("reexports").alias("relation"),
            col("qualifier"),
            source_expr(source, None)?.alias("source"),
        ])?;
        let bases = bound
            .select(vec![
                col("symbol_id"),
                col("o"),
                get_field(col("o"), "bases").alias("base"),
            ])?
            .unnest_columns_with_options(
                &["base"],
                datafusion::common::UnnestOptions::new().with_preserve_nulls(false),
            )?;
        let bases = bases.select(vec![
            subject.clone().alias("subject"),
            variant(
                &target_type,
                "unresolved",
                &[("path", get_field(col("base"), "rendering"))],
            )?
            .alias("target"),
            lit("inherits").alias("relation"),
            lit("declared base expression; not typechecker resolved").alias("qualifier"),
            source_expr(source, None)?.alias("source"),
        ])?;
        let relationships = keyed(
            relationships.union(bases)?,
            Key::Relationship,
            "relationship_id",
            Relation::Relationships,
        )
        .await?;
        let docs = self
            .session
            .table("identified_observations")
            .await?
            .filter(get_field(col("o"), "docs").is_not_null())?
            .select(vec![
                lit("doc_text").alias("kind"),
                subject.clone().alias("subject"),
                col("path").alias("display_subject"),
                get_field(col("o"), "docs").alias("text"),
                source_expr(source, None)?.alias("source"),
            ])?;
        // Ordinal and text travel in one generated native record. Typed lambda bindings
        // also avoid treating a struct lambda variable as a SQL table qualifier.
        let rendering_kind = DataType::Struct(
            vec![
                arrow::datatypes::Field::new("ordinal", DataType::UInt32, true),
                arrow::datatypes::Field::new("signature", DataType::Utf8, true),
            ]
            .into(),
        );
        let main = record(
            &rendering_kind,
            &[
                ("ordinal", get_field(col("o"), "overload_ordinal")),
                ("signature", get_field(col("o"), "signature")),
            ],
        )?;
        let overload = Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
            "rendered_overload".into(),
            Some(overload_item.clone()),
        ));
        let other = record(
            &rendering_kind,
            &[
                ("ordinal", get_field(overload.clone(), "ordinal")),
                ("signature", get_field(overload, "signature")),
            ],
        )?;
        let renderings = datafusion::functions_nested::expr_fn::array_concat(vec![
            datafusion::functions_nested::expr_fn::make_array(vec![main]),
            datafusion::functions_nested::expr_fn::array_transform(
                get_field(col("o"), "overloads"),
                datafusion::logical_expr::expr_fn::lambda(vec!["rendered_overload"], other),
            ),
        ]);
        let signatures = self
            .session
            .table("identified_observations")
            .await?
            .with_column("rendering", renderings)?
            .unnest_columns_with_options(
                &["rendering"],
                datafusion::common::UnnestOptions::new().with_preserve_nulls(false),
            )?
            .filter(get_field(col("rendering"), "signature").is_not_null())?
            .select(vec![
                lit("api_signature").alias("kind"),
                subject.alias("subject"),
                col("path").alias("display_subject"),
                get_field(col("rendering"), "signature").alias("text"),
                source_expr(source, Some(get_field(col("rendering"), "ordinal")))?.alias("source"),
            ])?;
        let docs = crate::native_delta::project(docs, Key::TextFragment.schema().as_ref())?;
        let signatures =
            crate::native_delta::project(signatures, Key::TextFragment.schema().as_ref())?;
        let fragments = keyed(
            docs.union(signatures)?,
            Key::TextFragment,
            "fragment_id",
            Relation::Fragments,
        )
        .await?;
        Ok(EvidencePlans::from([
            (Relation::Definitions, definitions),
            (Relation::Symbols, bindings),
            (Relation::ApiObservations, observations),
            (Relation::Relationships, relationships),
            (Relation::Fragments, fragments),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::{Artifact, ArtifactKind, relational::Locator},
        identity::Ecosystem,
        producer::python::{ObservationOrigin, WorkerRequest},
        wire::{EvidenceClass, SourceVersionMatch},
    };
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    #[tokio::test]
    async fn actual_python_worker_to_native_evidence() {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("source");
        std::fs::create_dir(&source).unwrap();
        std::fs::write(source.join("a.py"), "from b import Thing as Public\nfrom b import Loop as Loop\nfrom c import Choice as Ambiguous\nclass Base: ...\nclass Thing(Base):\n    def f(self, x: str):\n        \"\"\"λ documentation.\"\"\"\n        return x\n").unwrap();
        std::fs::write(
            source.join("b.pyi"),
            "from a import Thing as Thing\nfrom a import Loop as Loop\n",
        )
        .unwrap();
        std::fs::write(
            source.join("a.pyi"),
            "class Thing:\n    def f(self, x: bytes) -> bytes: ...\n",
        )
        .unwrap();
        std::fs::write(source.join("c.py"), "class Choice: ...\n").unwrap();
        std::fs::write(source.join("c.pyi"), "def Choice() -> str: ...\n").unwrap();
        std::fs::write(source.join("bad.py"), "def incomplete(:\n").unwrap();
        let files = vec![
            WorkerFile {
                file: "a.py".into(),
                module: "a".into(),
                origin: ObservationOrigin::Source,
            },
            WorkerFile {
                file: "b.pyi".into(),
                module: "b".into(),
                origin: ObservationOrigin::Stub,
            },
            WorkerFile {
                file: "a.pyi".into(),
                module: "a".into(),
                origin: ObservationOrigin::Stub,
            },
            WorkerFile {
                file: "c.py".into(),
                module: "c".into(),
                origin: ObservationOrigin::Source,
            },
            WorkerFile {
                file: "c.pyi".into(),
                module: "c".into(),
                origin: ObservationOrigin::Stub,
            },
            WorkerFile {
                file: "bad.py".into(),
                module: "bad".into(),
                origin: ObservationOrigin::Source,
            },
        ];
        let request = WorkerRequest {
            schema_version: worker::PROTOCOL.into(),
            root: source.to_string_lossy().into(),
            files: files.clone(),
            max_observations: 100000,
            max_memory_bytes: 1024 * 1024 * 1024,
            max_cpu_seconds: 30,
        };
        let directory = Arc::new(tempfile::tempdir_in(root.path()).unwrap());
        let python = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.venv/bin/python");
        let mut process = Command::new(python)
            .args(["-I", "-B", "-m", "enrichment_worker"])
            .env("OPENBLAS_NUM_THREADS", "1")
            .current_dir(directory.path())
            .stdin(Stdio::piped())
            .stdout(File::create(directory.path().join("worker.arrow")).unwrap())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        process
            .stdin
            .take()
            .unwrap()
            .write_all(&serde_json::to_vec(&request).unwrap())
            .unwrap();
        let output = process.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default()).unwrap();
        let facts = PythonFacts::open(&runtime, directory.clone(), &files)
            .await
            .unwrap();
        assert_eq!(
            facts.identity().await.unwrap().encoder_version,
            worker::ENCODER
        );
        let summary = facts.summary().await.unwrap();
        assert!(summary.observations > 5);
        assert_eq!(summary.unresolved, 3);
        assert_eq!(facts.gaps().await.unwrap().len(), 1);
        let context = IngestContext {
            ecosystem: Ecosystem::Python,
            symbol_package: "python:fixture".into(),
            release_id: format!("rel_{}", "1".repeat(64)).try_into().unwrap(),
            environment_id: format!("env_{}", "1".repeat(64)).try_into().unwrap(),
            source_version_match: SourceVersionMatch::Exact,
            producing_attempt: "attempt".into(),
            producer_runs: vec![],
            artifacts: vec![],
            indexed: vec![],
            missing: vec![],
            gaps: vec![],
        };
        let artifact = Artifact::describe(
            &std::fs::read(directory.path().join("worker.arrow")).unwrap(),
            ArtifactKind::Other,
            "application/vnd.apache.arrow.stream",
            "producer:griffe-static",
            enrichment_core::native_time::AcquisitionTime::try_from(
                "2026-09-16T00:00:00.000000Z".to_owned(),
            )
            .unwrap(),
        );
        let source = FactSource {
            producer_binding_id: format!("producer_{}", "a".repeat(64)),
            extractor: "python-static".into(),
            extractor_version: enrichment_core::producer::python::VERSION.into(),
            artifact_id: artifact.artifact_id,
            source_uri: Some(artifact.source_uri),
            source_version_match: SourceVersionMatch::Exact,
            evidence_class: EvidenceClass::StaticallyExtracted,
            locator: Locator::Artifact,
        };
        let plans = facts.evidence(&context, &source).await.unwrap();
        let store =
            crate::native_delta::DeltaStore::new(&root.path().join("delta"), runtime.clone())
                .unwrap();
        let mut apis = Vec::new();
        let mut bindings = Vec::new();
        let mut definitions = Vec::new();
        let mut edges = Vec::new();
        for (relation, plan) in plans {
            if relation == Relation::Fragments {
                let shape = plan
                    .clone()
                    .into_optimized_plan()
                    .unwrap()
                    .display_indent()
                    .to_string();
                for operator in ["RecursiveQuery", "Unnest", "Join", "Aggregate"] {
                    assert!(shape.contains(operator), "missing native {operator}");
                }
            }
            let expected = runtime.execute(plan.clone()).await.unwrap().rows;
            let contract =
                crate::native_delta::StorageContract::new(relation.schema().unwrap()).unwrap();
            let table = store
                .create(relation.name(), &contract, true)
                .await
                .unwrap();
            let table = store
                .append(table, &contract, plan, vec![])
                .await
                .unwrap_or_else(|e| panic!("Delta {}: {e}", relation.name()));
            let restored = runtime
                .session()
                .read_table(store.provider(&table, &contract).await.unwrap())
                .unwrap();
            let output = runtime
                .execute(restored)
                .await
                .unwrap_or_else(|e| panic!("{}: {e}", relation.name()));
            assert_eq!(output.rows, expected, "{}", relation.name());
            assert!(output.rows > 0, "{}", relation.name());
            for batch in output.batches {
                // The independently owned semantic decoder recomputes identities and proves
                // that native plans meet the existing high-level evidence contract.
                match relation {
                    Relation::ApiObservations => {
                        apis.extend(crate::projection::decode::observations(&batch).unwrap());
                    }
                    Relation::Definitions => {
                        definitions.extend(crate::projection::decode::definitions(&batch).unwrap());
                    }
                    Relation::Symbols => {
                        bindings.extend(crate::projection::decode::bindings(&batch).unwrap());
                    }
                    Relation::Relationships => {
                        edges.extend(crate::projection::relationships_from_batch(&batch).unwrap());
                    }
                    Relation::Fragments => {
                        crate::projection::fragments_from_batch(&batch).unwrap();
                    }
                    _ => unreachable!(),
                }
            }
        }
        assert!(apis.len() >= summary.observations as usize);
        let binding = |path: &str| bindings.iter().find(|b| b.path.display() == path).unwrap();
        assert_eq!(
            binding("a.Public").definition_id,
            binding("a.Thing").definition_id
        );
        assert_eq!(
            binding("b.Thing").definition_id,
            binding("a.Thing").definition_id
        );
        assert_eq!(
            bindings
                .iter()
                .filter(|b| b.path.display() == "c.Choice")
                .count(),
            2
        );
        let ambiguous = binding("a.Ambiguous");
        assert_eq!(
            definitions
                .iter()
                .find(|d| d.definition_id == ambiguous.definition_id)
                .unwrap()
                .kind,
            enrichment_core::evidence::SymbolKind::Import
        );
        let method = binding("a.Thing.f");
        let variants = apis
            .iter()
            .filter(|a| {
                a.subject
                    == enrichment_core::evidence::relational::SubjectRef::Symbol {
                        symbol_id: method.symbol_id.clone(),
                    }
            })
            .collect::<Vec<_>>();
        assert_eq!(variants.len(), 2);
        assert_ne!(variants[0].origin, variants[1].origin);
        assert_ne!(variants[0].payload.signature, variants[1].payload.signature);
        assert_eq!(
            edges
                .iter()
                .filter(
                    |r| r.relation == enrichment_core::evidence::RelationKind::Reexports
                        && matches!(
                            r.target,
                            enrichment_core::evidence::relational::TargetRef::Unresolved { .. }
                        )
                )
                .count(),
            3
        );
        let mut limits = crate::runtime::QueryLimits::default();
        limits.native.normalization_depth = 1;
        let bounded = QueryRuntime::new(&root.path().join("bounded-spill"), limits).unwrap();
        let limited = PythonFacts::open(&bounded, directory.clone(), &files)
            .await
            .unwrap();
        assert_eq!(limited.identity().await.unwrap().normalization_depth, 1);
        let failures = limited.alias_failures().await.unwrap();
        assert_eq!(
            failures
                .iter()
                .find(|f| f.reason == "depth_limit")
                .unwrap()
                .count,
            3
        );
        assert_eq!(
            failures
                .iter()
                .find(|f| f.reason == "ambiguous_declarations")
                .unwrap()
                .count,
            1
        );
        let bytes = std::fs::read(directory.path().join("worker.arrow")).unwrap();
        std::fs::write(
            directory.path().join("worker.arrow"),
            &bytes[..bytes.len() - 8],
        )
        .unwrap();
        assert!(validate_transport(&directory.path().join("worker.arrow")).is_err());
    }
}
