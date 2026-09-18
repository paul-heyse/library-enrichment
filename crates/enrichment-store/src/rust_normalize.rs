//! Rust semantics expressed as native DataFusion relations over mechanical rustdoc facts.
use crate::{
    admission::Relation, native_rustdoc::Captured, repository::EvidencePlans, runtime::QueryRuntime,
};
use arrow::datatypes::DataType;
use datafusion::{
    common::ScalarValue,
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::core::expr_fn::get_field,
    logical_expr::{Expr, when},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    evidence::{
        arrow_model::expressions::{child, null, record, variant, variants as union_variants},
        ingest::IngestContext,
        relational::FactSource,
    },
    native_key::Key,
    producer::rustdoc::facts::Fact,
};
use std::sync::Arc;

enrichment_core::native_struct! {
pub struct Header {
    crate_name: String => enrichment_core::native_union::Rule::Text,
    crate_version: Option<String> => enrichment_core::native_union::Rule::Text,
    target: String => enrichment_core::native_union::Rule::Text,
    producer_items: u64 => enrichment_core::native_union::Rule::Text,
}
}
enrichment_core::native_struct! {
pub struct Gap {
    reason: String => enrichment_core::native_union::Rule::Text,
    count: u64 => enrichment_core::native_union::Rule::Text,
    witnesses: Vec<String> => enrichment_core::native_union::Rule::Sequence,
}
}
pub struct RustFacts {
    pub header: Header,
    pub normalization_depth: u32,
    pub producer_revision: String,
    session: SessionContext,
    runtime: QueryRuntime,
    summary_chars: i64,
    captured: Captured,
    derivations: Vec<datafusion::logical_expr::LogicalPlan>,
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn field_type(relation: Relation, name: &str) -> Result<DataType> {
    Ok(relation
        .schema()?
        .field_with_name(name)?
        .data_type()
        .clone())
}
impl RustFacts {
    pub(crate) async fn open(
        runtime: &QueryRuntime,
        captured: Captured,
        summary_chars: usize,
    ) -> Result<Self> {
        if !(16..=65536).contains(&summary_chars) {
            return Err(invalid("Rust summary bound"));
        }
        let session = runtime.session();
        for fact in Fact::ALL {
            let path = captured
                .directory
                .path()
                .join(fact.name())
                .with_extension("arrow");
            let provider = crate::arrow_input::provider(runtime, &path, fact.schema()).await?;
            let source = session.read_table(provider)?.into_view();
            crate::native_catalog::work(
                &session,
                fact.name(),
                crate::leases::input_view(&source, &session, Arc::clone(&captured.directory))?,
            )?;
        }
        runtime.require_empty(session.sql("SELECT CAST(id AS VARCHAR) AS witness_id FROM rust_items WHERE id<>index_id OR line=0").await?, "rust_fact_identity", "rustdoc").await?;
        runtime
            .require_empty(
                session
                    .sql("SELECT 'header' AS witness_id FROM rust_header HAVING count(*)<>1")
                    .await?,
                "rust_header_cardinality",
                "rustdoc",
            )
            .await?;
        runtime.require_empty(session.sql("SELECT CAST(h.root AS VARCHAR) AS witness_id FROM rust_header h LEFT JOIN rust_items i ON h.root=i.id WHERE i.id IS NULL OR i.raw_kind<>'module' OR i.name IS NULL OR i.name='' OR i.crate_id<>0").await?, "rust_root_identity", "rustdoc").await?;
        let header = crate::registry::rows(runtime,session.sql("SELECT i.name AS crate_name,h.crate_version,h.target,h.producer_items FROM rust_header h JOIN rust_items i ON h.root=i.id").await?,1).await?.pop().ok_or_else(|| invalid("Rust header absent"))?;
        let mut this = Self {
            header,
            normalization_depth: crate::native_policy::normalization_depth(&session.state())?,
            session,
            runtime: runtime.clone(),
            summary_chars: summary_chars as i64,
            producer_revision: captured.producer_revision.clone(),
            captured,
            derivations: Vec::new(),
        };
        this.admit().await?;
        this.define().await?;
        Ok(this)
    }
    async fn view(&self, name: &str, sql: &str) -> Result<()> {
        crate::native_catalog::work(
            &self.session,
            name,
            self.session.sql(sql).await?.into_view(),
        )?;
        Ok(())
    }
    async fn reject(&self, sql: &str, rule: &str) -> Result<()> {
        self.runtime
            .require_empty(self.session.sql(sql).await?, rule, "rustdoc")
            .await
    }
    async fn admit(&self) -> Result<()> {
        for (table, key) in [
            ("rust_items", "id"),
            ("rust_paths", "id"),
            ("rust_externals", "crate_id"),
            ("rust_signatures", "ordinal"),
        ] {
            self.reject(&format!("SELECT CAST({key} AS VARCHAR) AS witness_id FROM {table} GROUP BY {key} HAVING count(*)<>1"),"rust_fact_key").await?;
        }
        self.reject("SELECT CAST(owner AS VARCHAR) AS witness_id FROM rust_links GROUP BY owner,role,ordinal HAVING count(*)<>1", "rust_link_key").await?;
        self.reject("SELECT CAST(l.owner AS VARCHAR) AS witness_id FROM rust_links l LEFT ANTI JOIN rust_items i ON l.owner=i.id", "rust_link_owner").await?;
        self.reject("SELECT 'items' AS witness_id FROM rust_header WHERE producer_items<>(SELECT count(*) FROM rust_items)", "rust_fact_count").await?;
        self.reject("SELECT CAST(id AS VARCHAR) AS witness_id FROM rust_paths WHERE array_length(components)=0 OR array_has(components,'')", "rust_path_components").await?;
        Ok(())
    }
    /// Reused native fixed points are materialized once. Each private Delta source retains
    /// its exact input-directory/evidence ownership through final evidence publication.
    async fn materialize(&mut self, name: &str) -> Result<()> {
        let frame = self.session.table(name).await?;
        self.derivations.push(frame.logical_plan().clone());
        let schema = Arc::new(frame.schema().as_arrow().clone());
        let contract = crate::native_delta::StorageContract::new(schema)?;
        let delta = crate::native_delta::DeltaStore::new(
            &self.captured.directory.path().join("native_stages"),
            self.runtime.clone(),
        )?;
        let table = delta.create(name, &contract, false).await?;
        let max_rows = enrichment_core::producer::rustdoc::facts::MAX_ROWS;
        let bounded = frame.limit(
            0,
            Some(usize::try_from(max_rows + 1).map_err(|_| invalid("Rust stage row bound"))?),
        )?;
        let table = delta.append(table, &contract, bounded, vec![]).await?;
        let view = self
            .session
            .read_table(delta.provider(&table, &contract).await?)?
            .into_view();
        self.session.deregister_table(name)?;
        crate::native_catalog::work(
            &self.session,
            name,
            crate::leases::input_view(&view, &self.session, Arc::clone(&self.captured.directory))?,
        )?;
        self.reject(
            &format!("SELECT '{name}' AS witness_id FROM {name} HAVING count(*)>{max_rows}"),
            "rust_stage_row_budget",
        )
        .await?;
        Ok(())
    }
    /// Actual native derivations behind the shared private Delta providers, for inspection.
    pub fn derivation_plans(&self) -> &[datafusion::logical_expr::LogicalPlan] {
        &self.derivations
    }

    async fn define(&mut self) -> Result<()> {
        // Direct containment facts retain private paths. They establish definition coordinates;
        // the separate public walk below grants reachability through explicit public uses.
        self.view("rust_children", r#"SELECT l.owner,l.child,l.role,
            coalesce(i.name,CASE WHEN l.role IN ('struct.tuple','variant.tuple') THEN CAST(l.ordinal AS VARCHAR) END) AS name
            FROM rust_links l JOIN rust_items i ON l.child=i.id
            WHERE l.role NOT IN ('type.impls','impl.items','trait.implementations') AND i.raw_kind<>'use'"#).await?;
        self.view("rust_structural_paths", &format!(r#"WITH RECURSIVE paths(id,path,depth,visited) AS (
            SELECT i.id,arrow_cast(i.name,'Utf8View'),CAST(0 AS BIGINT),make_array(i.id) FROM rust_header h JOIN rust_items i ON h.root=i.id
            UNION ALL SELECT c.child,concat(p.path,'::',c.name),p.depth+1,array_append(p.visited,c.child)
              FROM paths p JOIN rust_children c ON p.id=c.owner
              WHERE c.name IS NOT NULL AND p.depth<{} AND NOT array_has(p.visited,c.child)
        ) SELECT DISTINCT id,path FROM paths"#,self.normalization_depth)).await?;
        self.view("rust_canonical_candidates", r#"SELECT p.id,array_to_string(p.components,'::') AS path,
            CASE WHEN p.crate_id=0 THEN h.crate_name ELSE e.name END AS package
            FROM rust_paths p CROSS JOIN (SELECT i.name AS crate_name FROM rust_header h JOIN rust_items i ON h.root=i.id) h
            LEFT JOIN rust_externals e ON p.crate_id=e.crate_id
            UNION ALL SELECT s.id,s.path,split_part(s.path,'::',1) AS package FROM rust_structural_paths s
            LEFT ANTI JOIN rust_paths p ON s.id=p.id"#).await?;
        self.view("rust_canonical", "SELECT id,min(path) AS path,min(package) AS package FROM rust_canonical_candidates GROUP BY id HAVING count(DISTINCT path)=1 AND count(DISTINCT package)=1").await?;
        self.materialize("rust_canonical").await?;
        self.view("rust_impl_members", r#"SELECT l.owner AS type_id,i.id AS impl_id,m.child,i.trait_path,
            i.impl_negative,i.impl_synthetic,i.impl_blanket FROM rust_links l
            JOIN rust_items i ON l.child=i.id JOIN rust_links m ON i.id=m.owner AND m.role='impl.items'
            WHERE l.role='type.impls'
            UNION ALL SELECT l.owner,i.id,m.child,i.trait_path,i.impl_negative,i.impl_synthetic,i.impl_blanket
            FROM rust_links l JOIN rust_items i ON l.child=i.id
            JOIN rust_links m ON i.trait_id=m.owner AND m.role='trait.items'
            JOIN rust_items t ON m.child=t.id
            WHERE l.role='type.impls' AND array_has(i.provided_methods,t.name)"#).await?;
        // A use is a real node. Its jump to a local target keeps the public spelling; glob
        // jumps keep the parent path and emit only the target's children. All other edges
        // append one component. No Rust recursion or visited set participates in semantics.
        self.view("rust_edges", r#"SELECT l.owner,l.child,
            coalesce(CASE WHEN i.raw_kind='use' THEN i.use_name ELSE i.name END,
                CASE WHEN l.role IN ('struct.tuple','variant.tuple') THEN CAST(l.ordinal AS VARCHAR) END) AS name,
            l.role,CAST(NULL AS VARCHAR) AS qualifier,CAST(NULL AS INT UNSIGNED) AS impl_id,l.owner AS render_parent,
            i.visibility='public' OR i.visibility='default' AND l.role IN ('trait.items','enum.variants','variant.fields','variant.tuple') AS granted,
            false AS same_path, i.raw_kind='use' AS reexport,coalesce(i.use_glob,false) AS glob
            FROM rust_links l JOIN rust_items i ON l.child=i.id
            WHERE l.role NOT IN ('type.impls','impl.items','trait.implementations')
            UNION ALL SELECT i.id,i.use_target,CAST(NULL AS VARCHAR),'use.target',CAST(NULL AS VARCHAR),
                CAST(NULL AS INT UNSIGNED),CAST(NULL AS INT UNSIGNED),true,true,true,i.use_glob
            FROM rust_items i JOIN rust_items t ON i.use_target=t.id LEFT JOIN rust_canonical c ON t.id=c.id
            WHERE i.raw_kind='use' AND (t.crate_id=0 OR c.id IS NOT NULL)
            UNION ALL SELECT m.type_id,m.child,t.name,'impl.items',m.trait_path,m.impl_id,m.impl_id AS render_parent,
                t.visibility='public' OR t.visibility='default' AND m.trait_path IS NOT NULL,
                false,false,false FROM rust_impl_members m JOIN rust_items t ON m.child=t.id
            WHERE NOT m.impl_negative AND NOT m.impl_synthetic AND NOT m.impl_blanket"#).await?;
        self.materialize("rust_edges").await?;
        self.view("rust_walk", &format!(r#"WITH RECURSIVE walk(id,path,definition_path,package,qualifier,depth,visited,emit,is_reexport,render_parent,owner_path) AS (
            SELECT i.id,arrow_cast(i.name,'Utf8View') AS path,arrow_cast(i.name,'Utf8View') AS definition_path,arrow_cast(i.name,'Utf8View') AS package,arrow_cast(CAST(NULL AS VARCHAR),'Utf8View') AS qualifier,CAST(0 AS BIGINT) AS depth,make_array(i.id) AS visited,false AS emit,false AS is_reexport,
                CAST(NULL AS INT UNSIGNED) AS render_parent,arrow_cast(CAST(NULL AS VARCHAR),'Utf8View') AS owner_path FROM rust_header h JOIN rust_items i ON h.root=i.id
            UNION ALL SELECT e.child,
                CASE WHEN e.same_path OR e.glob THEN w.path ELSE concat(w.path,'::',e.name) END,
                coalesce(c.path,CASE WHEN e.same_path THEN w.definition_path ELSE concat(w.definition_path,'::',e.name) END),
                coalesce(c.package,w.package),e.qualifier,w.depth+1,array_append(w.visited,e.child),
                NOT e.glob,w.is_reexport OR e.reexport,
                CASE WHEN e.same_path THEN w.render_parent ELSE e.render_parent END,
                CASE WHEN e.same_path THEN w.owner_path ELSE w.path END
            FROM walk w JOIN rust_edges e ON w.id=e.owner LEFT JOIN rust_canonical c ON e.child=c.id
            WHERE e.granted AND (e.name IS NOT NULL OR e.same_path) AND w.depth<{}
                AND NOT array_has(w.visited,e.child)
        ) SELECT * FROM walk"#,self.normalization_depth)).await?;
        self.materialize("rust_walk").await?;
        self.view("rust_variants", r#"SELECT DISTINCT w.id,w.path,w.definition_path,w.package,w.qualifier,
            w.is_reexport,w.render_parent,w.owner_path,
            CASE WHEN i.proc_macro_kind IS NOT NULL THEN 'proc_macro' ELSE CASE i.raw_kind WHEN 'use' THEN 'import'
                WHEN 'function' THEN CASE WHEN p.raw_kind IN ('impl','trait') THEN 'method' ELSE 'function' END
                
                WHEN 'proc_attribute' THEN 'proc_macro' WHEN 'proc_derive' THEN 'proc_macro'
                ELSE i.raw_kind END END AS kind
            FROM rust_walk w JOIN rust_items i ON w.id=i.id LEFT JOIN rust_items p ON w.render_parent=p.id
            LEFT JOIN rust_items target ON i.use_target=target.id
            LEFT JOIN rust_canonical canonical_target ON target.id=canonical_target.id
            WHERE w.emit AND i.raw_kind<>'impl' AND (i.raw_kind<>'use' OR target.id IS NULL OR target.crate_id<>0 AND canonical_target.id IS NULL)"#).await?;
        let variants = self
            .session
            .table("rust_variants")
            .await?
            .with_column(
                "components",
                datafusion::functions_nested::expr_fn::string_to_array(
                    col("path"),
                    lit("::"),
                    null(&DataType::Utf8)?,
                ),
            )?
            .with_column(
                "definition_id",
                Key::Definition.bind(vec![
                    col("package"),
                    col("definition_path"),
                    col("kind"),
                    col("qualifier"),
                ])?,
            )?
            .with_column(
                "symbol_id",
                Key::PublicBinding.bind(vec![
                    lit(&self.header.crate_name),
                    lit("rust"),
                    col("components"),
                    col("kind"),
                    col("qualifier"),
                ])?,
            )?
            .with_column(
                "path_id",
                Key::PublicPath.bind(vec![lit("rust"), col("components")])?,
            )?;
        crate::native_catalog::work(
            &self.session,
            "rust_identified",
            variants.clone().into_view(),
        )?;
        self.materialize("rust_identified").await?;
        self.view("rust_render_associations",r#"SELECT s.ordinal,s.id,s.parent_id,s.text,
            count(DISTINCT v.symbol_id) AS bindings,count(DISTINCT v.definition_id) AS definitions,
            CASE WHEN count(DISTINCT v.symbol_id)=1 THEN min(v.symbol_id) END AS symbol_id,
            CASE WHEN count(DISTINCT v.definition_id)=1 THEN min(v.definition_id) END AS definition_id,
            CASE WHEN count(DISTINCT v.kind)=1 THEN min(v.kind) END AS kind,
            CASE WHEN count(DISTINCT v.definition_path)=1 THEN min(v.definition_path) END AS display_path
            FROM rust_signatures s LEFT JOIN rust_identified v ON s.id=v.id AND (s.parent_id IS NOT DISTINCT FROM v.render_parent)
            GROUP BY s.ordinal,s.id,s.parent_id,s.text"#).await?;
        self.view("rust_normalization_gaps", &format!(r#"SELECT 'depth_limit' AS reason,CAST(w.id AS VARCHAR) AS witness
            FROM rust_walk w JOIN rust_edges e ON w.id=e.owner WHERE w.depth={} AND e.granted AND NOT array_has(w.visited,e.child)
            UNION ALL SELECT 'cycle',CAST(w.id AS VARCHAR) FROM rust_walk w JOIN rust_edges e ON w.id=e.owner WHERE e.granted AND array_has(w.visited,e.child)
            UNION ALL SELECT 'unresolved_reexport',CAST(i.id AS VARCHAR) FROM rust_walk w JOIN rust_items i ON w.id=i.id
                LEFT JOIN rust_items t ON i.use_target=t.id LEFT JOIN rust_canonical c ON t.id=c.id
                WHERE i.raw_kind='use' AND (t.id IS NULL OR t.crate_id<>0 AND c.id IS NULL)
            UNION ALL SELECT 'missing_render_input',CAST(m.id AS VARCHAR) FROM rust_missing m
                LEFT ANTI JOIN (
                    SELECT DISTINCT i.trait_id AS id FROM rust_items i
                    JOIN rust_paths p ON i.trait_id=p.id AND p.crate_id<>0
                    JOIN rust_canonical c ON p.id=c.id WHERE i.raw_kind='impl'
                ) external_traits ON m.id=external_traits.id
            UNION ALL SELECT 'ambiguous_definition_path',CAST(id AS VARCHAR) FROM rust_canonical_candidates
                GROUP BY id HAVING count(DISTINCT path)>1 OR count(DISTINCT package)<>1
            UNION ALL SELECT 'missing_public_child',CAST(l.child AS VARCHAR) FROM rust_walk w JOIN rust_links l ON w.id=l.owner
                LEFT JOIN rust_items t ON l.child=t.id WHERE l.child IS NOT NULL AND t.id IS NULL
                AND l.role NOT IN ('type.impls','trait.implementations')
            UNION ALL SELECT 'ambiguous_rendering',CAST(ordinal AS VARCHAR) FROM rust_render_associations WHERE definitions>1"#,self.normalization_depth)).await?;
        Ok(())
    }
    pub async fn gaps(&self) -> Result<Vec<Gap>> {
        crate::registry::rows(&self.runtime,self.session.sql("WITH unique_gaps AS (
            SELECT DISTINCT reason,witness FROM rust_normalization_gaps
        ), ranked AS (
            SELECT reason,witness,row_number() OVER (PARTITION BY reason ORDER BY witness) AS position FROM unique_gaps
        ) SELECT reason,CAST(count(*) AS BIGINT UNSIGNED) AS count,array_agg(witness ORDER BY witness) FILTER (WHERE position<=8) AS witnesses
          FROM ranked GROUP BY reason ORDER BY reason").await?,8).await
    }
}

fn source_expr(source: &FactSource) -> Result<Expr> {
    let kind = field_type(Relation::ApiObservations, "source")?;
    let locator = variant(
        &child(&kind, "locator")?,
        "rustdoc_item",
        &[
            ("item", col("id")),
            ("reported_file", col("file")),
            ("reported_line", col("line")),
        ],
    )?;
    let encoded = enrichment_core::evidence::arrow_model::encode::source(&[source])?;
    let descriptor = lit(ScalarValue::try_from_array(&encoded, 0)?);
    let DataType::Struct(fields) = &kind else {
        return Err(invalid("source struct missing"));
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
fn keyed(frame: DataFrame, key: Key, name: &str, relation: Relation) -> Result<DataFrame> {
    crate::native_delta::project(
        frame.with_column(name, key.expression())?,
        relation.schema()?.as_ref(),
    )?
    .distinct()
}
impl RustFacts {
    pub async fn evidence(
        &self,
        context: &IngestContext,
        source: &FactSource,
    ) -> Result<EvidencePlans> {
        if context.symbol_package != self.header.crate_name {
            return Err(invalid("Rust binding owner differs from producer header"));
        }
        let variants = self.session.table("rust_identified").await?;
        let definitions = crate::native_delta::project(
            variants
                .clone()
                .with_column("defined_in_package", col("package"))?,
            Relation::Definitions.schema()?.as_ref(),
        )?
        .distinct()?;
        let bindings = self.session.sql("SELECT *,array_element(components,-1) AS name,array_slice(components,1,CAST(array_length(components) AS BIGINT)-1) AS parent_components FROM rust_identified").await?
            .with_column("ecosystem",lit("rust"))?
            .with_column("parent_path_id",when(datafusion::functions_nested::expr_fn::array_length(col("components")).gt(lit(1_u64)),
                Key::PublicPath.bind(vec![lit("rust"),col("parent_components")])?).otherwise(null(&DataType::Utf8)?)?)?;
        let bindings =
            crate::native_delta::project(bindings, Relation::Symbols.schema()?.as_ref())?
                .distinct()?;
        let attr = Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
            "attr".into(),
            Some(Arc::new(arrow::datatypes::Field::new(
                "attr",
                DataType::Utf8,
                false,
            ))),
        ));
        let observed = self.session.sql("SELECT v.*,i.file,i.line,i.docs,i.deprecated,i.deprecated_since,i.deprecated_note,i.attrs,i.rust FROM rust_identified v JOIN rust_items i ON v.id=i.id").await?
            .with_column("cfg_hints",datafusion::functions_nested::expr_fn::array_filter(col("attrs"),
                datafusion::logical_expr::expr_fn::lambda(vec!["attr"],datafusion::functions::string::expr_fn::contains(attr,lit("cfg")))))?;
        crate::native_catalog::work(&self.session, "rust_observed", observed.into_view())?;
        // item/parent IDs are candidate associations, not alias occurrence IDs. A unique
        // binding is symbol-scoped; several bindings of one definition are definition-scoped.
        // Other renderings remain library fragments with a diagnostic, never invented APIs.
        self.view("rust_render_observed",r#"SELECT a.*,i.file,i.line,i.rust FROM rust_render_associations a LEFT JOIN rust_items i ON a.id=i.id"#).await?;
        let symbol_subject = variant(
            &field_type(Relation::ApiObservations, "subject")?,
            "symbol",
            &[("symbol_id", col("symbol_id"))],
        )?;
        let rendered_subject = union_variants(
            &field_type(Relation::Fragments, "subject")?,
            when(col("bindings").eq(lit(1_u64)), lit("symbol"))
                .when(col("definitions").eq(lit(1_u64)), lit("definition"))
                .otherwise(lit("library"))?,
            &[
                ("symbol", vec![("symbol_id", col("symbol_id"))]),
                ("definition", vec![("definition_id", col("definition_id"))]),
                ("library", vec![("release_id", lit(&context.release_id))]),
            ],
        )?;
        let payload_type = field_type(Relation::ApiObservations, "payload")?;
        let deprecated_type = child(&payload_type, "deprecated")?;
        let deprecated = when(
            col("deprecated"),
            record(
                &deprecated_type,
                &[
                    ("since", col("deprecated_since")),
                    ("note", col("deprecated_note")),
                ],
            )?,
        )
        .otherwise(null(&deprecated_type)?)?;
        let payload = record(
            &payload_type,
            &[
                ("declared_kind", col("kind")),
                (
                    "doc_summary",
                    datafusion::functions::unicode::expr_fn::left(
                        col("docs"),
                        lit(self.summary_chars),
                    ),
                ),
                ("deprecated", deprecated),
                ("cfg_hints", col("cfg_hints")),
                ("rust", col("rust")),
            ],
        )?;
        let raw = self.session.table("rust_observed").await?.select(vec![
            symbol_subject.clone().alias("subject"),
            lit("rustdoc").alias("origin"),
            lit(&context.environment_id).alias("environment_id"),
            payload.alias("payload"),
            col("docs"),
            source_expr(source)?.alias("source"),
        ])?;
        let rendered =
            self.session
                .table("rust_render_observed")
                .await?
                .filter(col("definitions").eq(lit(1_u64)))?
                .select(vec![
                    rendered_subject.clone().alias("subject"),
                    lit("rustdoc").alias("origin"),
                    lit(&context.environment_id).alias("environment_id"),
                    record(
                        &payload_type,
                        &[
                            ("declared_kind", col("kind")),
                            ("signature", col("text")),
                            ("rust", col("rust")),
                            (
                                "cfg_hints",
                                enrichment_core::evidence::arrow_model::expressions::literal(
                                    &Vec::<String>::new(),
                                )?,
                            ),
                        ],
                    )?
                    .alias("payload"),
                    null(&DataType::Utf8)?.alias("docs"),
                    source_expr(source)?.alias("source"),
                ])?;
        let raw = crate::native_delta::project(raw, Key::ApiObservation.schema().as_ref())?;
        let rendered =
            crate::native_delta::project(rendered, Key::ApiObservation.schema().as_ref())?;
        let observations = keyed(
            raw.union(rendered)?,
            Key::ApiObservation,
            "observation_id",
            Relation::ApiObservations,
        )?;
        let docs = self
            .session
            .table("rust_observed")
            .await?
            .filter(col("docs").is_not_null())?
            .select(vec![
                lit("doc_text").alias("kind"),
                symbol_subject.alias("subject"),
                col("path").alias("display_subject"),
                col("docs").alias("text"),
                source_expr(source)?.alias("source"),
            ])?;
        let signatures = self
            .session
            .table("rust_render_observed")
            .await?
            .select(vec![
                lit("api_signature").alias("kind"),
                rendered_subject.alias("subject"),
                datafusion::functions::core::expr_fn::coalesce(vec![
                    col("display_path"),
                    lit(&context.symbol_package),
                ])
                .alias("display_subject"),
                col("text"),
                source_expr(source)?.alias("source"),
            ])?;
        let docs = crate::native_delta::project(docs, Key::TextFragment.schema().as_ref())?;
        let signatures =
            crate::native_delta::project(signatures, Key::TextFragment.schema().as_ref())?;
        let fragments = keyed(
            docs.union(signatures)?,
            Key::TextFragment,
            "fragment_id",
            Relation::Fragments,
        )?;
        let relationships = self.relationships(source).await?;
        Ok(EvidencePlans::from([
            (Relation::Definitions, definitions),
            (Relation::Symbols, bindings),
            (Relation::ApiObservations, observations),
            (Relation::Relationships, relationships),
            (Relation::Fragments, fragments),
        ]))
    }
    async fn relationships(&self, source: &FactSource) -> Result<DataFrame> {
        self.view("rust_relationship_facts",r#"SELECT DISTINCT b.symbol_id,'member_of' AS relation,
            'symbol' AS target_kind,p.symbol_id AS target_symbol,CAST(NULL AS VARCHAR) AS target_definition,
            CAST(NULL AS VARCHAR) AS target_package,CAST(NULL AS VARCHAR) AS target_path,b.qualifier,b.id,i.file,i.line
            FROM rust_identified b JOIN rust_identified p ON b.owner_path=p.path
            JOIN rust_items i ON b.id=i.id JOIN rust_items parent ON b.render_parent=parent.id
            WHERE parent.raw_kind IN ('impl','trait','struct','enum','variant','union')
            UNION ALL SELECT DISTINCT b.symbol_id,'reexports','definition',CAST(NULL AS VARCHAR),b.definition_id,
                CAST(NULL AS VARCHAR),CAST(NULL AS VARCHAR),CAST(NULL AS VARCHAR),u.id,u.file,u.line
            FROM rust_walk w JOIN rust_items u ON w.id=u.id JOIN rust_identified b ON w.path=b.path
            WHERE u.raw_kind='use' AND NOT u.use_glob AND b.kind<>'import'
            UNION ALL SELECT DISTINCT b.symbol_id,'reexports','external',CAST(NULL AS VARCHAR),CAST(NULL AS VARCHAR),
                CAST(NULL AS VARCHAR),u.use_source,CAST(NULL AS VARCHAR),u.id,u.file,u.line
            FROM rust_identified b JOIN rust_items u ON b.id=u.id WHERE u.raw_kind='use' AND b.kind='import'
            UNION ALL SELECT DISTINCT b.symbol_id,'implements',
                CASE WHEN d.definition_id IS NULL THEN 'external' ELSE 'definition' END,CAST(NULL AS VARCHAR),d.definition_id,
                CASE WHEN d.definition_id IS NULL THEN c.package END,
                CASE WHEN d.definition_id IS NULL THEN coalesce(c.path,i.trait_path) END,
                CASE WHEN i.impl_negative THEN 'negative_impl' WHEN i.impl_synthetic THEN 'auto_trait'
                    WHEN i.impl_blanket THEN 'blanket_impl' ELSE 'trait_impl' END,i.id,i.file,i.line
            FROM rust_identified b JOIN rust_links l ON b.id=l.owner AND l.role='type.impls'
            JOIN rust_items i ON l.child=i.id LEFT JOIN rust_canonical c ON i.trait_id=c.id
            LEFT JOIN (SELECT DISTINCT id,definition_id FROM rust_identified WHERE kind='trait') d ON i.trait_id=d.id
            WHERE i.trait_path IS NOT NULL"#).await?;
        let frame = self
            .session
            .table("rust_relationship_facts")
            .await?
            .select(vec![
                variant(
                    &field_type(Relation::Relationships, "subject")?,
                    "symbol",
                    &[("symbol_id", col("symbol_id"))],
                )?
                .alias("subject"),
                union_variants(
                    &field_type(Relation::Relationships, "target")?,
                    col("target_kind"),
                    &[
                        ("symbol", vec![("symbol_id", col("target_symbol"))]),
                        (
                            "definition",
                            vec![("definition_id", col("target_definition"))],
                        ),
                        (
                            "external",
                            vec![
                                ("package", col("target_package")),
                                ("path", col("target_path")),
                            ],
                        ),
                        ("unresolved", vec![("path", col("target_path"))]),
                    ],
                )?
                .alias("target"),
                col("relation"),
                col("qualifier"),
                source_expr(source)?.alias("source"),
            ])?;
        keyed(
            frame,
            Key::Relationship,
            "relationship_id",
            Relation::Relationships,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::{
            Artifact, ArtifactKind,
            relational::{Locator, SubjectRef},
        },
        identity::Ecosystem,
        wire::{EvidenceClass, SourceVersionMatch},
    };
    #[tokio::test]
    async fn native_rust_facts_preserve_aliases_members_and_signature_alternatives() {
        let root = tempfile::tempdir().unwrap();
        crate::leases::initialize(root.path()).unwrap();
        let blobs = crate::BlobStore::open(root.path()).unwrap();
        let payload =
            include_bytes!("../../../tests/fixtures/rustdoc/enr-fixture-0.2.0-all-features.json");
        let artifact = blobs
            .put(payload, |_| {
                Artifact::describe(
                    payload,
                    ArtifactKind::RustdocJson,
                    "application/json",
                    "https://docs.rs/enr-fixture/0.2.0/rustdoc.json",
                    enrichment_core::native_time::AcquisitionTime::try_from(
                        "2026-09-16T00:00:00.000000Z".to_owned(),
                    )
                    .unwrap(),
                )
            })
            .unwrap()
            .acquired;
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default()).unwrap();
        let control = crate::control::ControlStore::open(root.path(), runtime.clone()).unwrap();
        let retention = crate::retention::RetentionStore::new(control.clone(), runtime.clone());
        // This worker/storage fixture runs only after the deletion barrier. Exercise
        // the real claim path; a test must not bypass the decoder's effect admission.
        let config = Arc::new(enrichment_core::config::Config::default());
        let jobs = crate::control_jobs::JobStore::new(
            control,
            runtime.clone(),
            "rustdoc_fixture".into(),
            config.clone(),
        );
        let command = jobs
            .command_input(crate::control_jobs::Arguments::Resolve {
                request: enrichment_core::request::ResolveRequest {
                    name: "enr-fixture".into(),
                    ..Default::default()
                },
            })
            .await
            .unwrap();
        let (id, _) = jobs
            .submit(command, enrichment_core::identity::InterestId::new())
            .await
            .unwrap();
        let policy = crate::execution_policy::Policy::bind(
            &runtime,
            &config,
            crate::execution_policy::Capture {
                execution_root: root.path().join("execution").display().to_string(),
                containment_identity: None,
                containment_error: None,
                receipt: None,
                receipt_error: None,
                cleanup_error: None,
            },
        )
        .await
        .unwrap();
        assert!(
            jobs.start(&id, &enrichment_core::identity::AttemptId::new(), &policy)
                .await
                .unwrap()
        );
        let pin = jobs.pin().await.unwrap();
        let claim = jobs.claim(&pin, &id).await.unwrap().unwrap();
        let grant = jobs.grant(&id, claim.fence).await.unwrap();
        drop(pin);
        let (sender, result) = tokio::sync::oneshot::channel();
        let worker_runtime = runtime.clone();
        let input = artifact.clone();
        runtime
            .command(id, crate::native_effect::CommandKind::Resolve, async move {
                crate::native_effect::bind_claim(grant).unwrap();
                let value = crate::native_rustdoc::from_artifact(
                    &worker_runtime,
                    retention,
                    blobs,
                    input,
                    240,
                )
                .await;
                assert!(sender.send(value).is_ok());
            })
            .await
            .unwrap();
        let facts = result.await.unwrap().unwrap();
        assert_eq!(facts.header.crate_name, "enr_fixture");
        let context = IngestContext {
            ecosystem: Ecosystem::Rust,
            symbol_package: "enr_fixture".into(),
            release_id: format!("rel_{}", "1".repeat(64)).try_into().unwrap(),
            environment_id: format!("env_{}", "1".repeat(64)).try_into().unwrap(),
            source_version_match: SourceVersionMatch::Exact,
            producing_attempt: enrichment_core::identity::AttemptId::new(),
            producer_runs: vec![],
            artifacts: vec![],
            indexed: vec![],
            missing: vec![],
            gaps: vec![],
        };
        let source = FactSource {
            producer_binding_id: format!("producer_{}", "a".repeat(64)),
            extractor: "rustdoc-json".into(),
            extractor_version: enrichment_core::producer::rustdoc::NORMALIZER_VERSION.into(),
            artifact_id: artifact.artifact_id,
            source_uri: Some(artifact.source_uri),
            source_version_match: SourceVersionMatch::Exact,
            locator: Locator::Artifact,
            evidence_class: EvidenceClass::StaticallyExtracted,
        };
        let gaps = facts.gaps().await.unwrap();
        assert!(gaps.is_empty(), "complete fixture normalization: {gaps:?}");
        let plans = facts.evidence(&context, &source).await.unwrap();
        let delta =
            crate::native_delta::DeltaStore::new(&root.path().join("delta"), runtime.clone())
                .unwrap();
        let mut bindings = vec![];
        let mut definitions = vec![];
        let mut apis = vec![];
        let mut edges = vec![];
        let mut fragments = vec![];
        for (relation, plan) in plans {
            let optimized = plan
                .clone()
                .into_optimized_plan()
                .unwrap()
                .display_indent()
                .to_string();
            assert!(optimized.contains("TableScan"));
            let contract =
                crate::native_delta::StorageContract::new(relation.schema().unwrap()).unwrap();
            let table = delta
                .create(relation.name(), &contract, true)
                .await
                .unwrap();
            let table = delta
                .append(table, &contract, plan, vec![])
                .await
                .unwrap_or_else(|e| panic!("Delta {}: {e}", relation.name()));
            let restored = runtime
                .session()
                .read_table(delta.provider(&table, &contract).await.unwrap())
                .unwrap();
            for batch in runtime
                .execute(restored)
                .await
                .unwrap_or_else(|e| panic!("{}: {e}", relation.name()))
                .batches
            {
                match relation {
                    Relation::Symbols => {
                        bindings.extend(crate::projection::decode::bindings(&batch).unwrap())
                    }
                    Relation::Definitions => {
                        definitions.extend(crate::projection::decode::definitions(&batch).unwrap())
                    }
                    Relation::ApiObservations => {
                        apis.extend(crate::projection::decode::observations(&batch).unwrap())
                    }
                    Relation::Relationships => {
                        edges.extend(crate::projection::relationships_from_batch(&batch).unwrap())
                    }
                    Relation::Fragments => {
                        fragments.extend(crate::projection::fragments_from_batch(&batch).unwrap())
                    }
                    _ => unreachable!(),
                }
            }
        }
        let binding = |path: &str| {
            bindings
                .iter()
                .find(|b| b.path.display() == path)
                .unwrap_or_else(|| panic!("missing {path}"))
        };
        let original = binding("enr_fixture::inner::Widget");
        let alias = binding("enr_fixture::Widget");
        assert_eq!(original.definition_id, alias.definition_id);
        assert_ne!(original.symbol_id, alias.symbol_id);
        assert_eq!(
            binding("enr_fixture::Widget::new").definition_id,
            binding("enr_fixture::inner::Widget::new").definition_id
        );
        assert!(
            definitions
                .iter()
                .any(|d| d.definition_path.ends_with("::Widget::size"))
        );
        assert!(apis.iter().any(|a| {
            a.payload
                .deprecated
                .as_ref()
                .is_some_and(|d| d.since.as_deref() == Some("0.1.0"))
        }));
        assert!(
            apis.iter()
                .any(|a| matches!(&a.subject, SubjectRef::Definition { .. })
                    && a.payload.signature.is_some())
        );
        assert!(
            edges
                .iter()
                .any(|e| e.relation == enrichment_core::evidence::RelationKind::Implements)
        );
        assert!(fragments.iter().any(|f| f.text.contains("Widget::new")));
        // Private fact scans retain their own owned bytes, not a blanket artifact-root fence.
        drop(crate::leases::exclusive(root.path()).unwrap());
        assert_eq!(facts.derivation_plans().len(), 4);
        assert!(
            facts
                .derivation_plans()
                .iter()
                .any(|plan| plan.display_indent().to_string().contains("RecursiveQuery"))
        );
        assert!(
            facts
                .gaps()
                .await
                .unwrap()
                .iter()
                .all(|g| g.reason != "depth_limit")
        );
        let directory = facts.captured.directory.path().to_owned();
        assert!(directory.exists());
        drop(facts);
        runtime.close_diagnostics().await.unwrap();
        assert!(!directory.exists());
    }
}
