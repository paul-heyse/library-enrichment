//! Explicit Python runtime selection; normalized facts never overwrite declarations.
use super::{
    common,
    inspect_execution::{self, Produced},
    semantics::preparation_error,
};
use crate::{
    execution::{Runner, capsule},
    service::Service,
};
use enrichment_core::{
    evidence::{SymbolHeader, execution::ExecutionPayload, relational::SubjectRef},
    policy::ExecutionProfile,
    request::InspectionOptions,
    wire::EvidenceClass,
};
use std::{
    io,
    sync::{Arc, atomic::AtomicBool},
};

use enrichment_core::execution::producer::RUNTIME_OBJECT_HELPER as HELPER;

pub async fn produce(
    service: &Service,
    opened: &common::Opened,
    symbol: &SymbolHeader,
    options: &InspectionOptions,
    cancel: Arc<AtomicBool>,
) -> io::Result<Produced> {
    let image = service
        .config
        .execution
        .python_image
        .as_ref()
        .ok_or_else(|| io::Error::other("Python image missing"))?;
    let selection = options
        .runtime
        .as_ref()
        .ok_or_else(|| io::Error::other("runtime selection missing"))?;
    let lease = service
        .lsp
        .execution_lease(&service.execution, &cancel)
        .await?
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Interrupted,
                "cancelled during runtime admission",
            )
        })?;
    let runner = Runner::new(
        &service.config.execution,
        &service.paths.cache_root,
        service.execution.clone(),
        service.ownership.clone(),
    )?
    .using_lease(lease.clone());
    let containment = runner.containment_identity()?;
    let started_at =
        enrichment_core::native_time::ObservationTime::now().map_err(std::io::Error::other)?;
    let outcome = async {
        let prepared = capsule::prepare(service, opened, &runner, image, &format!("runtime-{}", uuid::Uuid::new_v4().simple()), cancel.clone()).await.map_err(preparation_error)?;
        let mut inputs = inspect_execution::capsule_inputs(service, opened, &prepared).await?;
        let encoded = enrichment_store::runtime_object_plan::selection_transport(&service.repository.runtime, selection, service.config.execution.output_bytes.clamp(1024,1_048_576)).await.map_err(io::Error::other)?;
        prepared.write_input("runtime_object.py", HELPER)?;
        prepared.write_input("runtime-selection.json", &encoded)?;
        inputs.push(inspect_execution::store(service, HELPER.as_bytes(), "producer://runtime-object/2/helper", "text/x-python")?);
        inputs.push(inspect_execution::store(service, encoded.as_bytes(), "consumer://runtime-selection/2", "application/json")?);
        let observation = runner.for_capsule(&prepared).run(image, &prepared.root, &enrichment_core::execution::producer::Invocation::RuntimeObject, cancel).await?;
        let result = enrichment_store::runtime_object_plan::lower(
            &service.repository.runtime,
            &enrichment_core::native_runtime::Capture {
                selection: selection.clone(),
                subject: SubjectRef::Symbol { symbol_id: symbol.symbol_id.clone() },
                observation: observation.clone(),
            },
        ).await.map_err(io::Error::other)?;
        Ok(Produced { environment: prepared.prepared.environment.clone(), image: image.clone(), containment,
            producer: "runtime-object".into(), version: inspect_execution::producer_identity(true)?.1, profile: ExecutionProfile::Runtime,
            started_at, finished_at: enrichment_core::native_time::ObservationTime::now().map_err(std::io::Error::other)?,
            facts: vec![(SubjectRef::Symbol { symbol_id: symbol.symbol_id.clone() }, ExecutionPayload::RuntimeObject(result), EvidenceClass::RuntimeObserved)],
            inputs, lock: prepared.prepared.lock.as_bytes().to_vec(), transcript: serde_json::json!({"preparation":prepared.observations,"runtime":observation}) })
    }.await;
    service.execution.wait_for_cleanup(&lease).await?;
    outcome
}
