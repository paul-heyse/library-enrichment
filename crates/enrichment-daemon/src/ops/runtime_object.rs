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
    canonical,
    evidence::{
        SymbolHeader,
        execution::{ExecutionOutcome, ExecutionPayload, RuntimeObject},
        relational::SubjectRef,
    },
    execution::ProcessEnd,
    policy::ExecutionProfile,
    request::InspectionOptions,
    wire::EvidenceClass,
};
use std::{
    io,
    sync::{Arc, atomic::AtomicBool},
};

const HELPER: &str = include_str!("../execution/runtime_object.py");

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
        let encoded = canonical::to_canonical_string(&serde_json::json!({"selection": selection, "max_output_bytes": service.config.execution.output_bytes.clamp(1024,1_048_576)}));
        prepared.write_input("runtime_object.py", HELPER)?;
        prepared.write_input("runtime-selection.json", &encoded)?;
        inputs.push(inspect_execution::store(service, HELPER.as_bytes(), "producer://runtime-object/2/helper", "text/x-python")?);
        inputs.push(inspect_execution::store(service, encoded.as_bytes(), "consumer://runtime-selection/2", "application/json")?);
        let observation = runner.run(image, &prepared.root, &capsule::strings(&["/usr/local/bin/python3", "-I", "-S", "/capsule/runtime_object.py"]), cancel).await?;
        let (result, raw) = if observation.end == ProcessEnd::Exited && observation.exit_code == Some(0) {
            let raw: serde_json::Value = serde_json::from_str(&observation.stdout).map_err(|e| io::Error::other(format!("runtime producer returned invalid bounded JSON: {e}")))?;
            let result: RuntimeObject = serde_json::from_value(raw["result"].clone())?;
            (result, raw)
        } else {
            (RuntimeObject { module: selection.module.clone(), selection: selection.attributes.clone(), outcome: match observation.end {
                ProcessEnd::Cancelled => ExecutionOutcome::Cancelled,
                ProcessEnd::Deadline | ProcessEnd::OutputLimit => ExecutionOutcome::Incomplete,
                ProcessEnd::Exited => ExecutionOutcome::Failed,
            }, type_name: None, signature: None, docstring: None, attributes: Vec::new(), limitations: vec![format!("Runtime observation ended {:?} with exit {:?}; the process log is retained.", observation.end, observation.exit_code)] }, serde_json::Value::Null)
        };
        if result.module != selection.module || result.selection != selection.attributes { return Err(io::Error::other("runtime result differs from explicit selection")); }
        ExecutionPayload::RuntimeObject(result.clone()).validate(&SubjectRef::Symbol { symbol_id: symbol.symbol_id.clone() }).map_err(io::Error::other)?;
        Ok(Produced { environment: prepared.environment.clone(), image: image.clone(), containment,
            producer: "runtime-object".into(), version: inspect_execution::producer_identity(true).1, profile: ExecutionProfile::Runtime,
            started_at, finished_at: enrichment_core::native_time::ObservationTime::now().map_err(std::io::Error::other)?,
            facts: vec![(SubjectRef::Symbol { symbol_id: symbol.symbol_id.clone() }, ExecutionPayload::RuntimeObject(result), EvidenceClass::RuntimeObserved)],
            inputs, lock: prepared.lock.clone(), transcript: serde_json::json!({"preparation":prepared.observations,"runtime":observation,"report":raw}) })
    }.await;
    service.execution.wait_for_cleanup(&lease).await?;
    outcome
}
