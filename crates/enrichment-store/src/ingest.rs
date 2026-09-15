//! One native ingestion path: bounded typed staging, relational reference resolution, then
//! canonical evidence records. Invoked on a blocking producer task, never an async worker.
use crate::{
    dataset::WriteLimits,
    projection::staging,
    provider::{ExactParquet, FileWitness},
    record_writer::RelationBuffer,
    runtime::QueryRuntime,
};
use datafusion::error::{DataFusionError, Result};
use enrichment_core::evidence::{
    Artifact,
    ingest::{EvidenceSink, IngestBudget, IngestContext, Normalizer, ProducerSource},
};
use std::{collections::BTreeMap, path::Path, sync::Arc};

/// Normalize actual producer records using the shared query runtime from a blocking task.
/// # Errors
/// Invalid references, ambiguous identities, resource limits and staging/query errors abort.
pub fn normalize_into(
    context: IngestContext,
    input: impl ProducerSource,
    sink: &mut impl EvidenceSink,
    root: &Path,
    runtime: &QueryRuntime,
    limits: &WriteLimits,
    executor: &tokio::runtime::Handle,
) -> Result<BTreeMap<String, Vec<Artifact>>> {
    let normalizer = Normalizer::new(context).map_err(invalid)?;
    let context = normalizer.context();
    let directory = tempfile::Builder::new()
        .prefix("producer-references-")
        .tempdir_in(root)?;
    let mut bindings = RelationBuffer::staging(
        directory.path(),
        "producer_bindings",
        limits,
        staging::bindings,
    )?;
    let mut relationships = RelationBuffer::staging(
        directory.path(),
        "producer_relationships",
        limits,
        staging::relationships,
    )?;
    let mut fragments = RelationBuffer::staging(
        directory.path(),
        "producer_fragments",
        limits,
        staging::fragments,
    )?;
    let mut declarations = RelationBuffer::staging(
        directory.path(),
        "producer_inputs",
        limits,
        staging::declarations,
    )?;
    let mut artifacts = RelationBuffer::staging(
        directory.path(),
        "producer_artifacts",
        limits,
        staging::artifacts,
    )?;
    let mut attempts = context
        .producer_runs
        .iter()
        .map(|r| (r.attempt_id.clone(), Vec::new()))
        .collect::<BTreeMap<_, _>>();
    for run in &context.producer_runs {
        let binding_id = run.semantic_binding_id();
        for (role, digest) in &run.inputs {
            declarations.push(staging::InputDeclaration {
                attempt_id: run.attempt_id.clone(),
                binding_id: binding_id.clone(),
                role: Some(role.clone()),
                digest: Some(digest.clone()),
                log: None,
            })?;
        }
        if let Some(log) = &run.log {
            declarations.push(staging::InputDeclaration {
                attempt_id: run.attempt_id.clone(),
                binding_id: binding_id.clone(),
                role: None,
                digest: None,
                log: Some(log.clone()),
            })?;
        }
    }
    for artifact in &context.artifacts {
        artifacts.push(artifact.clone())?;
    }
    let mut budget = IngestBudget::default();
    input
        .visit_symbols(&mut |symbol| {
            budget.record(&symbol)?;
            bindings
                .push(normalizer.symbol(symbol, sink)?)
                .map_err(|e| e.to_string())
        })
        .map_err(invalid)?;
    input
        .visit_relationships(&mut |relationship| {
            budget.record(&relationship)?;
            relationships.push(relationship).map_err(|e| e.to_string())
        })
        .map_err(invalid)?;
    let mut ordinal = 0u64;
    input
        .visit_fragments(&mut |fragment| {
            budget.record(&fragment)?;
            ordinal += 1;
            fragments
                .push(staging::Fragment {
                    ordinal,
                    pending: normalizer.fragment(fragment)?,
                })
                .map_err(|e| e.to_string())
        })
        .map_err(invalid)?;
    drop(input);
    let bindings = bindings.finish_staging()?;
    let relationships = relationships.finish_staging()?;
    let fragments = fragments.finish_staging()?;
    let declarations = declarations.finish_staging()?;
    let artifacts = artifacts.finish_staging()?;
    executor.block_on(async {
        let mut tables = crate::native_catalog::Tables::new();
        for (name, path, schema) in [
            ("producer_bindings", &bindings.0, staging::bindings(&[])?.schema()),
            ("producer_relationships", &relationships.0, staging::relationships(&[])?.schema()),
            ("producer_fragments", &fragments.0, staging::fragments(&[])?.schema()),
            ("producer_inputs", &declarations.0, staging::declarations(&[])?.schema()),
            ("producer_artifacts", &artifacts.0, staging::artifacts(&[])?.schema()),
        ] {
            let provider = ExactParquet::new(path.to_owned(), FileWitness::read(path)?, schema, datafusion::common::Constraints::default()).await?;
            tables.insert(name.to_owned(), Arc::new(provider) as Arc<dyn datafusion::catalog::TableProvider>);
        }
        let session = runtime.bound_session(std::collections::BTreeMap::from([("candidate".into(), Arc::new(crate::native_catalog::BoundCatalog::default().with_schema(crate::native_catalog::BindingKind::CandidateProducer, tables)) as Arc<dyn datafusion::catalog::CatalogProvider>)]))?;
        let inputs = session.sql("SELECT DISTINCT d.binding_id, d.role, a.artifact FROM candidate.producer.producer_inputs d JOIN candidate.producer.producer_artifacts a ON d.digest = a.artifact.sha256 WHERE d.role IS NOT NULL").await?;
        runtime.visit(inputs, limits.table_rows, |batch| {
            for row in staging::resolved_inputs(batch)? { sink.input(row).map_err(invalid)?; }
            Ok(())
        }).await?;
        let acquisitions = session.sql("SELECT DISTINCT d.attempt_id, a.artifact FROM candidate.producer.producer_inputs d JOIN candidate.producer.producer_artifacts a ON d.digest = a.artifact.sha256 OR d.log = a.artifact.artifact_id").await?;
        let mut attempt_bytes = 0usize;
        runtime.visit(acquisitions, limits.table_rows, |batch| {
            for (attempt, artifact) in staging::attempt_artifacts(batch)? {
                attempt_bytes = attempt_bytes.checked_add(enrichment_core::canonical::serialized_size(&artifact, limits.record_bytes)?)
                    .filter(|n| *n <= 64 * 1024 * 1024).ok_or_else(|| invalid("attempt acquisition byte budget exceeded"))?;
                attempts.get_mut(&attempt).ok_or_else(|| invalid("unknown producing attempt"))?.push(artifact);
            }
            Ok(())
        }).await?;
        if runtime.execute(session.sql("SELECT producer_id FROM candidate.producer.producer_bindings GROUP BY producer_id HAVING count(*) > 1 LIMIT 1").await?).await?.rows != 0 {
            return Err(invalid("duplicate producer symbol identity"));
        }
        let relationships = session.sql("SELECT r.*, b.producer_id AS from_producer_id, b.path AS from_path, b.symbol_id AS from_symbol_id, b.definition_id AS from_definition_id, b.producer_local_id AS from_producer_local_id, b.source AS from_source, t.symbol_id AS target_symbol, d.definition_id AS target_definition FROM candidate.producer.producer_relationships r LEFT JOIN candidate.producer.producer_bindings b ON r.source_id = b.producer_id LEFT JOIN candidate.producer.producer_bindings t ON r.target_id = t.producer_id LEFT JOIN (SELECT DISTINCT definition_id FROM candidate.producer.producer_bindings) d ON r.target_id = d.definition_id").await?;
        runtime.visit(relationships, limits.table_rows, |batch| {
            for row in staging::resolved_relationships(batch, &normalizer)? { sink.relationship(row).map_err(invalid)?; }
            Ok(())
        }).await?;
        let fragments = session.sql("WITH candidates AS (SELECT f.ordinal, count(b.producer_id) AS matches, count(DISTINCT b.definition_id) AS definitions, min(b.symbol_id) AS resolved_symbol, min(b.definition_id) AS resolved_definition FROM candidate.producer.producer_fragments f LEFT JOIN candidate.producer.producer_bindings b ON f.resolve AND f.display_subject = b.path AND (f.binding_id IS NULL OR f.binding_id = b.producer_id) AND (f.binding_id IS NOT NULL OR f.local_id IS NULL OR f.local_id = b.producer_local_id) GROUP BY f.ordinal), paths AS (SELECT path, count(*) AS path_count FROM candidate.producer.producer_bindings GROUP BY path) SELECT f.*, c.matches, c.definitions, c.resolved_symbol, c.resolved_definition, p.path_count FROM candidate.producer.producer_fragments f JOIN candidates c ON f.ordinal = c.ordinal LEFT JOIN paths p ON f.display_subject = p.path").await?;
        runtime.visit(fragments, limits.table_rows, |batch| {
            for row in staging::resolved_fragments(batch)? { sink.fragment(row).map_err(invalid)?; }
            Ok(())
        }).await?;
        Ok::<_, DataFusionError>(())
    })?;
    normalizer.finish(sink).map_err(invalid)?;
    Ok(attempts)
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
