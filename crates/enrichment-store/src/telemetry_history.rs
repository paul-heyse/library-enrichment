//! Bounded Arrow ingress and one Delta event authority for all runtime diagnostics.
use crate::{
    native_delta::{DeltaStore, StorageContract, missing_table, transaction_conflict},
    runtime::QueryRuntime,
};
use arrow::record_batch::RecordBatch;
use datafusion::{
    error::{DataFusionError, Result},
    execution::memory_pool::{MemoryConsumer, MemoryPool, MemoryReservation},
    prelude::col,
};
use enrichment_core::native_union::NativeStruct;
use enrichment_core::telemetry::{
    self, Event, EventPayload, OperationDiagnostics, OperationEnd, QueryDiagnostics, Summary,
};
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};
use tokio::sync::{mpsc, oneshot};

const TABLE: &str = "native_events";
const QUEUE: usize = 512;
const WRITE_ROWS: usize = 128;
pub(crate) const READ_ROWS: usize = 32;
pub(crate) const READ_BYTES: usize = 8 * 1024 * 1024;
#[derive(Default, Clone)]
pub(crate) struct History(Option<Arc<Ingress>>);
struct Ingress {
    runtime_id: String,
    next_query: AtomicU64,
    next_event: AtomicU64,
    dropped: AtomicU64,
    closed: Arc<AtomicBool>,
    send: mpsc::Sender<Message>,
    pool: Arc<dyn MemoryPool>,
    completion: tokio::sync::Mutex<Completion>,
}
struct Completion {
    task: Option<tokio::task::JoinHandle<Result<()>>>,
    failure: Option<String>,
}
struct Entry {
    event: Event,
    memory: MemoryReservation,
}
enum Message {
    Entry(Entry),
    Read(Selection, oneshot::Sender<Result<Vec<RecordBatch>>>),
}
#[derive(Clone, Copy)]
enum Selection {
    Queries,
    Operations,
    Summary,
    ServiceCounters,
    Kernel,
    Failures,
    Flush,
    Close,
}
impl History {
    /// The writer uses the same executor/pool and an unobserved runtime clone. Its queue owns
    /// capture/encoding reservations; it never owns an observed History and cannot form a cycle.
    pub(crate) fn start(
        runtime: QueryRuntime,
        root: &Path,
        pool: Arc<dyn MemoryPool>,
    ) -> Result<Self> {
        let (send, receive) = mpsc::channel(QUEUE);
        let runtime_id = uuid::Uuid::new_v4().to_string();
        let store = DeltaStore::new(root, runtime.clone())?;
        store.prepare_root(TABLE)?;
        let closed = Arc::new(AtomicBool::new(false));
        let task_closed = closed.clone();
        let task_id = runtime_id.clone();
        let task = runtime
            .executor_handle()
            .spawn(async move { actor(runtime, store, task_id, receive, task_closed).await });
        let ingress = Arc::new(Ingress {
            runtime_id: runtime_id.clone(),
            next_query: AtomicU64::new(0),
            next_event: AtomicU64::new(0),
            dropped: AtomicU64::new(0),
            closed: closed.clone(),
            send,
            pool,
            completion: tokio::sync::Mutex::new(Completion {
                task: Some(task),
                failure: None,
            }),
        });
        Ok(Self(Some(ingress)))
    }
    pub(crate) fn next_query(&self) -> u64 {
        self.0
            .as_ref()
            .map_or(0, |h| h.next_query.fetch_add(1, Ordering::Relaxed) + 1)
    }
    pub(crate) fn dropped(&self) -> u64 {
        self.0
            .as_ref()
            .map_or(0, |h| h.dropped.load(Ordering::Acquire))
    }
    pub(crate) fn enabled(&self) -> bool {
        self.0.is_some()
    }
    fn emit(&self, operation_id: Option<&str>, payload: impl FnOnce() -> EventPayload) {
        self.emit_reserved(operation_id, 2 * READ_BYTES, payload);
    }
    fn emit_reserved(
        &self,
        operation_id: Option<&str>,
        capture_bytes: usize,
        payload: impl FnOnce() -> EventPayload,
    ) {
        let Some(h) = &self.0 else {
            return;
        };
        let write = || -> Result<()> {
            // Capture only a bounded owned event here. The actor encodes a whole group
            // once; native/kernel callbacks must not construct the wide Arrow schema.
            // Reserve for both captured values and encoding before either allocation.
            let memory = MemoryConsumer::new("native_telemetry_ingress").register(&h.pool);
            memory.try_grow(capture_bytes)?;
            let event = Event {
                runtime_id: h.runtime_id.clone(),
                sequence: h.next_event.fetch_add(1, Ordering::Relaxed) + 1,
                recorded_at: enrichment_core::native_time::EventTime::now()?,
                operation_id: operation_id.map(str::to_owned),
                payload: payload(),
            };
            h.send
                .try_send(Message::Entry(Entry { event, memory }))
                .map_err(|e| {
                    DataFusionError::ResourcesExhausted(format!("diagnostic ingress: {e}"))
                })
        };
        if let Err(error) = write() {
            let dropped = h.dropped.fetch_add(1, Ordering::AcqRel).saturating_add(1);
            if dropped.is_power_of_two() {
                eprintln!("{dropped} native diagnostic observations not retained; latest: {error}");
            }
        }
    }
    pub(crate) fn query(&self, query: &QueryDiagnostics) {
        self.emit(query.operation_id.as_deref(), || EventPayload::Query {
            value: Box::new(query.clone()),
        });
    }
    pub(crate) fn operation(&self, operation: OperationEnd) {
        let id = operation.operation_id.clone();
        self.emit(Some(&id), || EventPayload::Operation { value: operation });
    }
    pub(crate) fn materialization(
        &self,
        operation_id: Option<&str>,
        value: telemetry::MaterializationObservation,
    ) {
        self.emit_reserved(operation_id, 64 * 1024, || EventPayload::Materialization {
            value,
        });
    }
    pub(crate) fn service(&self, observation: telemetry::ServiceObservation) {
        let id = crate::runtime::operation_id();
        self.emit(id.as_deref(), || EventPayload::Service {
            value: observation,
        });
    }
    pub(crate) fn kernel(
        &self,
        operation_id: Option<&str>,
        event: delta_kernel::metrics::MetricEvent,
    ) {
        // Kernel variants have only fixed numeric fields and at most two bounded 1-KiB
        // attributes. Shared Arrow encoding receives a separate actor-side reservation.
        self.emit_reserved(operation_id, 64 * 1024, || EventPayload::Kernel {
            value: crate::kernel_metrics::capture(event),
        });
    }
    pub(crate) async fn service_counters(&self) -> Result<telemetry::ServiceCounters> {
        decode::<telemetry::ServiceCounters>(self.select(Selection::ServiceCounters).await?)?
            .pop()
            .ok_or_else(|| invalid("native service diagnostic aggregate missing"))
    }
    pub(crate) async fn kernel_diagnostics(&self) -> Result<Vec<telemetry::kernel::Diagnostic>> {
        decode(self.select(Selection::Kernel).await?)
    }
    pub(crate) fn failure(&self, diagnostic: enrichment_core::wire::Diagnostic) {
        let id = diagnostic.correlation_id.clone();
        self.emit(id.as_deref(), || EventPayload::Failure {
            value: diagnostic,
        });
    }
    async fn select(&self, selection: Selection) -> Result<Vec<RecordBatch>> {
        let Some(h) = &self.0 else {
            return Ok(Vec::new());
        };
        let (send, receive) = oneshot::channel();
        h.send
            .send(Message::Read(selection, send))
            .await
            .map_err(|_| invalid("native diagnostic writer unavailable"))?;
        receive
            .await
            .map_err(|_| invalid("native diagnostic response unavailable"))?
    }
    pub(crate) async fn read(&self) -> Result<Vec<QueryDiagnostics>> {
        decode(self.select(Selection::Queries).await?)
    }
    pub(crate) async fn operations(&self) -> Result<Vec<OperationDiagnostics>> {
        decode(self.select(Selection::Operations).await?)
    }
    pub(crate) async fn summary(&self) -> Result<Summary> {
        if !self.enabled() {
            return Ok(Summary::default());
        }
        decode::<Summary>(self.select(Selection::Summary).await?)?
            .pop()
            .ok_or_else(|| invalid("native diagnostic aggregate missing"))
    }
    pub(crate) async fn failures(&self) -> Result<Vec<RecordBatch>> {
        self.select(Selection::Failures).await
    }
    pub(crate) async fn flush(&self) -> Result<()> {
        self.select(Selection::Flush).await.map(|_| ())
    }
    pub(crate) async fn close(&self) -> Result<()> {
        let Some(history) = &self.0 else {
            return Ok(());
        };
        // Serialize concurrent close calls. Keep the handle in its owner while awaiting it:
        // cancelling a closer must not detach the actor or lose its persistence failure.
        let mut completion = history.completion.lock().await;
        if let Some(task) = completion.task.as_mut() {
            let barrier = if history.closed.load(Ordering::Acquire) {
                Ok(())
            } else {
                self.select(Selection::Close).await.map(|_| ())
            };
            let exited = task
                .await
                .map_err(|error| invalid(&format!("native diagnostic writer task: {error}")))
                .and_then(std::convert::identity);
            // A cancelled closer can leave a successful Close queued with no receiver.
            // Joined physical success and the actor's close flag survive that lost reply.
            let acknowledged = if history.closed.load(Ordering::Acquire) {
                Ok(())
            } else {
                barrier
            };
            completion.failure = exited
                .and(acknowledged)
                .err()
                .map(|error| error.to_string());
            completion.task = None;
        }
        completion
            .failure
            .as_ref()
            .map_or(Ok(()), |failure| Err(invalid(failure)))
    }
}
fn decode<T: NativeStruct>(batches: Vec<RecordBatch>) -> Result<Vec<T>> {
    let mut output = Vec::new();
    for batch in batches {
        let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        for index in 0..batch.num_rows() {
            output.push(T::decode(rows.row(index))?);
        }
    }
    Ok(output)
}

async fn actor(
    runtime: QueryRuntime,
    store: DeltaStore,
    runtime_id: String,
    mut receive: mpsc::Receiver<Message>,
    closed: Arc<AtomicBool>,
) -> Result<()> {
    let contract = StorageContract::new(telemetry::schema::events())?;
    let mut pending = Vec::new();
    let mut commit_sequence = 0i64;
    let mut waiting = None;
    let mut table = None;
    let mut close_replies = Vec::new();
    while let Some(message) = match waiting.take() {
        Some(message) => Some(message),
        None => receive.recv().await,
    } {
        match message {
            Message::Entry(entry) => {
                pending.push(entry);
                // Coalesce bursts into native Delta writes. Read/close barriers flush
                // immediately; otherwise the maximum added visibility delay is 10 ms.
                let deadline = tokio::time::Instant::now() + std::time::Duration::from_millis(10);
                while pending.len() < WRITE_ROWS {
                    match tokio::time::timeout_at(deadline, receive.recv()).await {
                        Ok(Some(Message::Entry(entry))) => pending.push(entry),
                        Ok(Some(message)) => {
                            waiting = Some(message);
                            break;
                        }
                        Ok(None) | Err(_) => break,
                    }
                }
                commit_sequence += 1;
                match persist(
                    &runtime,
                    &store,
                    &contract,
                    &runtime_id,
                    commit_sequence,
                    std::mem::take(&mut pending),
                    table.take(),
                )
                .await
                {
                    Ok(committed) => table = Some(committed),
                    Err(error) => {
                        // A failed/indeterminate diagnostic commit cannot produce a successful
                        // aggregate. Closing the channel makes subsequent read barriers fail.
                        eprintln!("native diagnostic persistence stopped: {error}");
                        return Err(error);
                    }
                }
            }
            Message::Read(Selection::Close, response) => {
                // Closing admission preserves entries already queued after this barrier,
                // including permits acquired before close. Tokio returns None only after
                // every accepted message/permit has been consumed or released.
                receive.close();
                close_replies.push(response);
            }
            Message::Read(selection, response) => {
                let _ = response
                    .send(select(&runtime, &store, &contract, &runtime_id, selection).await);
            }
        }
    }
    closed.store(true, Ordering::Release);
    for response in close_replies {
        let _ = response.send(Ok(Vec::new()));
    }
    Ok(())
}
async fn persist(
    runtime: &QueryRuntime,
    store: &DeltaStore,
    contract: &StorageContract,
    runtime_id: &str,
    commit_sequence: i64,
    entries: Vec<Entry>,
    mut captured: Option<crate::native_delta::LoadedTable>,
) -> Result<crate::native_delta::LoadedTable> {
    let (events, reservations): (Vec<_>, Vec<_>) = entries
        .into_iter()
        .map(|entry| (entry.event, entry.memory))
        .unzip();
    let encoding = MemoryConsumer::new("native_telemetry_encoding")
        .register(&runtime.session().runtime_env().memory_pool);
    encoding.try_grow(READ_BYTES)?;
    let reserved = reservations
        .iter()
        .map(MemoryReservation::size)
        .sum::<usize>()
        + encoding.size();
    let batch = Event::batch(&events)?;
    let bytes = datafusion::common::utils::memory::get_record_batch_memory_size(&batch);
    if bytes > reserved {
        return Err(DataFusionError::ResourcesExhausted(
            "diagnostic Arrow ingress bound".into(),
        ));
    }
    drop(events);
    // Keep the capture/encoding reservations through the complete Delta writer.
    let _reservations = reservations;
    let input = crate::native_catalog::batch(&runtime.session(), "telemetry_history", batch)?;
    // Producer observations can enter the queue out of allocation order. Delta transaction
    // versions follow this sole writer's commit order, independently of observation sequence.
    for _ in 0..16 {
        // Native append returns its updated snapshot. Reuse that exact state instead of
        // rebuilding it from the complete log for every event batch. Another writer's
        // conflict discards it and returns to the controlled native opener on the next try.
        let table = match captured.take() {
            Some(table) => table,
            None => store.open_or_create(TABLE, contract, false, &[]).await?,
        };
        match store
            .append(
                table,
                contract,
                input.clone(),
                vec![deltalake::kernel::Transaction::new(
                    format!("telemetry/{runtime_id}"),
                    commit_sequence,
                )],
            )
            .await
        {
            Ok(committed) => return Ok(committed),
            Err(error) if transaction_conflict(&error) => continue,
            Err(error) => return Err(error),
        }
    }
    Err(invalid("diagnostic conflict bound exceeded"))
}
fn aggregate(grouped: bool) -> String {
    format!(
        "SELECT {} CAST(count(*) FILTER (WHERE kind='query') AS BIGINT UNSIGNED) AS executions, CAST(count(*) FILTER (WHERE kind='query' AND query.completed) AS BIGINT UNSIGNED) AS completed, CAST(count(*) FILTER (WHERE kind='query' AND NOT query.completed) AS BIGINT UNSIGNED) AS incomplete, CAST(coalesce(sum(query.planning_micros),0) AS BIGINT UNSIGNED) AS planning_micros, CAST(coalesce(sum(query.elapsed_micros),0) AS BIGINT UNSIGNED) AS elapsed_micros, CAST(count(*) FILTER (WHERE kind='materialization' AND materialization.activity.kind='ready') AS BIGINT UNSIGNED) AS materialization_fills, CAST(coalesce(sum(materialization.activity.ready.spill_bytes),0) AS BIGINT UNSIGNED) AS materialization_spill_bytes, CAST(count(*) FILTER (WHERE kind='materialization' AND materialization.activity.kind='read') AS BIGINT UNSIGNED) AS materialization_reads, CAST(coalesce(sum(query.queue_micros),0) AS BIGINT UNSIGNED) AS queue_micros, CAST(coalesce(sum(query.output_rows),0) AS BIGINT UNSIGNED) AS output_rows, CAST(coalesce(sum(query.output_arrow_bytes),0) AS BIGINT UNSIGNED) AS output_arrow_bytes FROM current_events {}",
        if grouped { "operation_id," } else { "" },
        if grouped { "GROUP BY operation_id" } else { "" }
    )
}
async fn select(
    runtime: &QueryRuntime,
    store: &DeltaStore,
    contract: &StorageContract,
    runtime_id: &str,
    selection: Selection,
) -> Result<Vec<RecordBatch>> {
    if matches!(selection, Selection::Flush | Selection::Close) {
        return Ok(Vec::new());
    }
    let session = runtime.session();
    let all = match store.load(TABLE, None).await {
        Ok(table) => session.read_table(store.provider(&table, contract).await?)?,
        Err(error) if missing_table(&error) => session
            .read_empty()?
            .filter(datafusion::prelude::lit(false))?
            .select(
                contract
                    .semantic_schema()
                    .fields()
                    .iter()
                    .map(|field| {
                        datafusion::prelude::lit(
                            datafusion::common::ScalarValue::try_new_null(field.data_type())
                                .expect("declared telemetry type"),
                        )
                        .alias(field.name())
                    })
                    .collect::<Vec<_>>(),
            )?,
        Err(error) => return Err(error),
    };
    crate::native_catalog::work(&session, "event_records", all.into_view())?;
    let all = session.sql("SELECT runtime_id,sequence,recorded_at,operation_id,payload.kind AS kind,payload.query.value AS query,payload.operation.value AS operation,payload.failure.value AS failure,payload.materialization.value AS materialization,payload.service.value AS service,payload.kernel.value AS kernel FROM event_records").await?;
    crate::native_catalog::work(&session, "all_events", all.clone().into_view())?;
    crate::native_catalog::work(
        &session,
        "current_events",
        all.filter(col("runtime_id").eq(datafusion::prelude::lit(runtime_id)))?
            .into_view(),
    )?;
    let sql = match selection {
        Selection::Queries => {
            let arrow::datatypes::DataType::Struct(fields) = telemetry::schema::query() else { unreachable!() };
            let fields = fields.iter().map(|field| format!("query.{} AS {}",field.name(),field.name())).collect::<Vec<_>>().join(",");
            format!("SELECT {fields} FROM (SELECT sequence, query FROM current_events WHERE kind='query' ORDER BY sequence DESC LIMIT 8) ORDER BY sequence")
        }
        Selection::Summary => aggregate(false),
        Selection::ServiceCounters => service_totals(),
        Selection::Kernel => "SELECT operation_id,sequence,recorded_at,kernel AS value FROM (SELECT * FROM current_events WHERE kind='kernel' ORDER BY sequence DESC LIMIT 32) ORDER BY sequence".into(),
        Selection::Operations => {
            crate::native_catalog::work(&session, "operation_totals", session.sql(&aggregate(true)).await?.into_view())?;
            let arrow::datatypes::DataType::Struct(fields) = telemetry::schema::operation() else { unreachable!() };
            let fields = fields.iter().map(|field| format!("e.operation.{} AS {}",field.name(),field.name())).collect::<Vec<_>>().join(",");
            let totals = telemetry::schema::summary_fields().iter().map(|field| format!("'{}',coalesce(t.{},CAST(0 AS BIGINT UNSIGNED))",field.name(),field.name())).collect::<Vec<_>>().join(",");
            format!("SELECT {fields}, named_struct({totals}) AS native FROM (SELECT * FROM current_events WHERE kind='operation' ORDER BY sequence DESC LIMIT 32) e LEFT JOIN operation_totals t ON e.operation_id=t.operation_id ORDER BY e.sequence")
        }
        Selection::Failures => "SELECT f.runtime_id,f.recorded_at,f.failure,q.query AS last_operation_query FROM (SELECT * FROM all_events WHERE kind='failure' ORDER BY recorded_at DESC,sequence DESC LIMIT 32) f LEFT JOIN all_events q ON f.runtime_id=q.runtime_id AND f.operation_id=q.operation_id AND q.kind='query' AND q.sequence<f.sequence QUALIFY row_number() OVER (PARTITION BY f.runtime_id,f.sequence ORDER BY q.sequence DESC)=1 ORDER BY f.recorded_at,f.sequence".into(),
        Selection::Flush | Selection::Close => unreachable!(),
    };
    Ok(runtime.execute(session.sql(&sql).await?).await?.batches)
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

/// Native reductions share the finite observed vocabulary and generated output schema.
fn service_totals() -> String {
    fn count(predicate: &str) -> String {
        format!("CAST(count(*) FILTER (WHERE {predicate}) AS BIGINT UNSIGNED)")
    }
    fn sum(value: &str, predicate: &str) -> String {
        format!("CAST(coalesce(sum({value}) FILTER (WHERE {predicate}),0) AS BIGINT UNSIGNED)")
    }
    fn record(fields: Vec<(&str, String)>) -> String {
        format!(
            "named_struct({})",
            fields
                .into_iter()
                .map(|(name, value)| format!("'{name}',{value}"))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    let fetch = record(vec![
        (
            "hits",
            count("service.kind='fetch' AND service.fetch.outcome='hit'"),
        ),
        (
            "revalidated",
            count("service.kind='fetch' AND service.fetch.outcome='revalidated'"),
        ),
        (
            "misses",
            count("service.kind='fetch' AND service.fetch.outcome='miss'"),
        ),
        ("failures", count("service.kind='fetch_failure'")),
        (
            "fetched_bytes",
            sum(
                "service.fetch.transferred",
                "service.kind='fetch' AND service.fetch.outcome='miss'",
            ),
        ),
    ]);
    let evidence = record(vec![
        ("requests", count("service.kind='response'")),
        (
            "ok",
            count("service.kind='response' AND service.response.status='ok'"),
        ),
        (
            "partial",
            count("service.kind='response' AND service.response.status='partial'"),
        ),
        (
            "pending",
            count("service.kind='response' AND service.response.status='pending'"),
        ),
        (
            "errors",
            count(
                "service.kind='response' AND (service.response.status='error' OR service.response.status IS NULL)",
            ),
        ),
        (
            "gaps",
            count("service.kind='response' AND service.response.has_gap"),
        ),
        (
            "response_bytes",
            sum("service.response.bytes", "service.kind='response'"),
        ),
    ]);
    let verification = record(vec![
        (
            "succeeded",
            count("service.kind='probe' AND service.probe.outcome='succeeded'"),
        ),
        (
            "failed",
            count("service.kind='probe' AND service.probe.outcome='failed'"),
        ),
        (
            "unresolved",
            count("service.kind='probe' AND service.probe.outcome='unresolved'"),
        ),
    ]);
    format!(
        "SELECT {fetch} AS fetch,{evidence} AS evidence,{verification} AS verification FROM current_events WHERE kind='service'"
    )
}

#[cfg(test)]
mod tests {
    fn observation(bytes: u64) -> enrichment_core::telemetry::MaterializationObservation {
        enrichment_core::telemetry::MaterializationObservation {
            binding: 1,
            family: enrichment_core::telemetry::MaterializationFamily::ComparisonKeys,
            activity: enrichment_core::telemetry::MaterializationActivity::Ready {
                spill_bytes: bytes,
            },
        }
    }

    use super::*;

    #[test]
    fn close_drains_events_accepted_across_the_close_barrier() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let owned = runtime.clone();
        runtime.bootstrap(async move {
            let history = owned.native_history();
            let ingress = history.0.as_ref().unwrap();
            // Hold real channel permits to make the admission/close race deterministic.
            let event = ingress
                .send
                .reserve()
                .await
                .map_err(|_| invalid("event permit"))?;
            let query = ingress
                .send
                .reserve()
                .await
                .map_err(|_| invalid("read permit"))?;
            let (close, mut closed) = oneshot::channel();
            ingress
                .send
                .send(Message::Read(Selection::Close, close))
                .await
                .map_err(|_| invalid("close admission"))?;
            ingress.send.closed().await;
            assert!(matches!(
                closed.try_recv(),
                Err(oneshot::error::TryRecvError::Empty)
            ));
            let memory = MemoryConsumer::new("close_race_capture").register(&ingress.pool);
            memory.try_grow(64 * 1024)?;
            event.send(Message::Entry(Entry {
                event: Event {
                    runtime_id: ingress.runtime_id.clone(),
                    sequence: ingress.next_event.fetch_add(1, Ordering::Relaxed) + 1,
                    recorded_at: enrichment_core::native_time::EventTime::now()?,
                    operation_id: None,
                    payload: EventPayload::Materialization {
                        value: observation(29),
                    },
                },
                memory,
            }));
            let (reply, received) = oneshot::channel();
            query.send(Message::Read(Selection::Summary, reply));
            let summary = decode::<Summary>(received.await.map_err(|_| invalid("read reply"))??)?;
            assert_eq!(
                (
                    summary[0].materialization_fills,
                    summary[0].materialization_spill_bytes
                ),
                (1, 29)
            );
            closed.await.map_err(|_| invalid("close reply"))??;
            owned.close_diagnostics().await?;
            assert_eq!(history.dropped(), 0);
            Ok(())
        })?
    }

    #[test]
    fn cached_native_snapshot_reloads_after_another_writer_advances() -> Result<()> {
        let root = tempfile::tempdir()?;
        let history = root.path().join("history");
        let first = QueryRuntime::with_diagnostics(
            &root.path().join("first"),
            &history,
            Default::default(),
        )?;
        let second = QueryRuntime::with_diagnostics(
            &root.path().join("second"),
            &history,
            Default::default(),
        )?;
        let driver = first.clone();
        driver.bootstrap(async move {
            first.native_history().materialization(None, observation(1));
            first.flush_diagnostics().await?;
            second
                .native_history()
                .materialization(None, observation(2));
            second.flush_diagnostics().await?;
            // The first actor still holds its earlier exact Delta snapshot.
            first.native_history().materialization(None, observation(4));
            first.flush_diagnostics().await?;
            let left = first.diagnostic_summary().await?;
            let right = second.diagnostic_summary().await?;
            assert_eq!(
                (left.materialization_fills, left.materialization_spill_bytes),
                (2, 5)
            );
            assert_eq!(
                (
                    right.materialization_fills,
                    right.materialization_spill_bytes
                ),
                (1, 2)
            );
            let (a, b) = tokio::join!(first.close_diagnostics(), second.close_diagnostics());
            a?;
            b
        })?
    }

    #[test]
    fn lost_close_acknowledgement_still_joins_the_native_writer() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let owned = runtime.clone();
        runtime.bootstrap(async move {
            let history = owned.native_history();
            history.materialization(None, observation(19));
            assert_eq!(history.summary().await?.materialization_spill_bytes, 19);
            let (send, receive) = oneshot::channel();
            history
                .0
                .as_ref()
                .unwrap()
                .send
                .send(Message::Read(Selection::Close, send))
                .await
                .map_err(|_| invalid("test close admission"))?;
            drop(receive); // Actual close is queued, but its caller has disconnected.
            let (first, second) =
                tokio::join!(owned.close_diagnostics(), owned.close_diagnostics());
            first?;
            second?;
            let completion = history.0.as_ref().unwrap().completion.lock().await;
            assert!(completion.task.is_none(), "writer must have been joined");
            assert!(completion.failure.is_none());
            Ok(())
        })?
    }
}
