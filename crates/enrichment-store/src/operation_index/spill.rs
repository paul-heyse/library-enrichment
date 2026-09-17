//! Native IPC mechanics. No row-count channel, schema interpreter or private spill format.
use super::Binding;
use crate::runtime::QueryRuntime;
use datafusion::{
    common::Result,
    execution::{TaskContext, memory_pool::MemoryConsumer, spill_file::SpillFile},
    physical_plan::{
        ExecutionPlan, SendableRecordBatchStream, execute_stream,
        metrics::{ExecutionPlanMetricsSet, SpillMetrics},
        spill::SpillManager,
        stream::RecordBatchStreamAdapter,
    },
};
use enrichment_core::telemetry::MaterializationActivity;
use futures::TryStreamExt;
use std::sync::Arc;

pub(super) struct Filled {
    manager: SpillManager,
    file: Option<Arc<dyn SpillFile>>,
    maximum: usize,
    bytes: u64,
}
impl Filled {
    pub(super) fn bytes(&self) -> u64 {
        self.bytes
    }
}

pub(super) async fn fill(
    runtime: QueryRuntime,
    input: Arc<dyn ExecutionPlan>,
    context: Arc<TaskContext>,
    metrics: ExecutionPlanMetricsSet,
) -> Result<Filled> {
    let manager = SpillManager::new(
        context.runtime_env(),
        SpillMetrics::new(&metrics, 0),
        input.schema(),
    );
    let writer_manager = manager.clone();
    let mut writer = runtime
        .spawn_blocking(move || writer_manager.create_in_progress_file("operation materialization"))
        .await
        .map_err(|error| super::invalid(&format!("spill writer task: {error}")))??;
    let mut maximum = 0;
    let mut stream = execute_stream(input, context.clone())?;
    while let Some(batch) = stream.try_next().await? {
        // Charge arrays and IPC workspace before native GC/encoding. The callback owns this
        // reservation through physical exit even when its async parent has been cancelled.
        let reservation =
            MemoryConsumer::new("operation_cache_writer").register(context.memory_pool());
        reservation.try_grow(
            batch
                .get_array_memory_size()
                .saturating_mul(2)
                .saturating_add(128 * 1024),
        )?;
        let (returned, size) = runtime
            .spawn_blocking(move || -> Result<_> {
                let _reservation = reservation;
                let size = writer.append_batch(&batch)?;
                Ok((writer, size))
            })
            .await
            .map_err(|error| super::invalid(&format!("spill append task: {error}")))??;
        writer = returned;
        maximum = maximum.max(size);
    }
    let file = runtime
        .spawn_blocking(move || writer.finish())
        .await
        .map_err(|error| super::invalid(&format!("spill finish task: {error}")))??;
    let bytes = file.as_ref().map_or(Ok(0), |file| {
        file.size()
            .ok_or_else(|| super::invalid("unaccounted cache spill"))
    })?;
    Ok(Filled {
        manager,
        file,
        maximum,
        bytes,
    })
}

pub(super) fn read(
    filled: Arc<Filled>,
    binding: Arc<Binding>,
    context: Arc<TaskContext>,
) -> Result<SendableRecordBatchStream> {
    let reservation = MemoryConsumer::new("operation_cache_reader").register(context.memory_pool());
    let bytes = filled.maximum.saturating_mul(2).saturating_add(128 * 1024);
    reservation.try_grow(bytes)?;
    let schema = filled.manager.schema().clone();
    let input: SendableRecordBatchStream = match &filled.file {
        Some(file) => filled
            .manager
            .read_spill_as_stream_unbuffered(file.clone(), Some(filled.maximum))?,
        None => Box::pin(RecordBatchStreamAdapter::new(
            schema.clone(),
            futures::stream::empty(),
        )),
    };
    binding.observer.emit(MaterializationActivity::Read {
        reserved_bytes: bytes,
    });
    let cancellation = binding.observer.operation.cancellation()?;
    let reader = Reader {
        reservation,
        filled,
        binding,
        bytes,
    };
    let stream = futures::stream::try_unfold(
        (input, reader, cancellation),
        |(mut input, reader, cancellation)| async move {
            let batch = tokio::select! {
                biased;
                () = cancellation.cancelled() => return Err(super::invalid("operation materialization reader cancelled")),
                batch = input.try_next() => batch?,
            };
            Ok(batch.map(|batch| (batch, (input, reader, cancellation))))
        },
    );
    Ok(Box::pin(RecordBatchStreamAdapter::new(schema, stream)))
}

// Drop means the actual native reader released its reservation, not that a waiter went away.
struct Reader {
    reservation: datafusion::execution::memory_pool::MemoryReservation,
    filled: Arc<Filled>,
    binding: Arc<Binding>,
    bytes: usize,
}
impl Drop for Reader {
    fn drop(&mut self) {
        self.reservation.free();
        let _spill = &self.filled;
        self.binding
            .observer
            .emit(MaterializationActivity::ReaderReleased {
                reserved_bytes: self.bytes,
            });
    }
}
