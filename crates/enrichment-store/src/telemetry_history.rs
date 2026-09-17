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
const QUEUE: usize = 32;
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
}
struct Entry {
    batch: RecordBatch,
    _memory: MemoryReservation,
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
    Failures,
    Flush,
    Close,
}
impl History {
    /// The writer uses the same executor/pool and an unobserved runtime clone. Its queue owns
    /// Arrow memory reservations; it never owns an observed History and cannot form a cycle.
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
        let ingress = Arc::new(Ingress {
            runtime_id: runtime_id.clone(),
            next_query: AtomicU64::new(0),
            next_event: AtomicU64::new(0),
            dropped: AtomicU64::new(0),
            closed: closed.clone(),
            send,
            pool,
        });
        let executor = runtime.executor_handle();
        executor.spawn(async move {
            actor(runtime, store, runtime_id, receive, closed).await;
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
        let Some(h) = &self.0 else {
            return;
        };
        let write = || -> Result<()> {
            // Reserve the bounded ingress arena before cloning or Arrow encoding. The
            // resulting buffers retain their reservation until the writer releases them.
            let memory = MemoryConsumer::new("native_telemetry_ingress").register(&h.pool);
            memory.try_grow(READ_BYTES)?;
            let event = Event {
                runtime_id: h.runtime_id.clone(),
                sequence: h.next_event.fetch_add(1, Ordering::Relaxed) + 1,
                recorded_at: enrichment_core::native_time::EventTime::now()?,
                operation_id: operation_id.map(str::to_owned),
                payload: payload(),
            };
            let batch = Event::batch(&[event])?;
            let bytes = datafusion::common::utils::memory::get_record_batch_memory_size(&batch);
            if bytes > READ_BYTES {
                return Err(DataFusionError::ResourcesExhausted(
                    "diagnostic Arrow ingress bound".into(),
                ));
            }
            memory.shrink(READ_BYTES - bytes);
            h.send
                .try_send(Message::Entry(Entry {
                    batch,
                    _memory: memory,
                }))
                .map_err(|e| {
                    DataFusionError::ResourcesExhausted(format!("diagnostic ingress: {e}"))
                })
        };
        if let Err(error) = write() {
            h.dropped.fetch_add(1, Ordering::AcqRel);
            eprintln!("native diagnostic observation not retained: {error}");
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
    pub(crate) fn index(&self, operation_id: Option<&str>, bytes: Option<u64>) {
        self.emit(operation_id, || {
            bytes.map_or(EventPayload::IndexRead, |bytes| {
                EventPayload::IndexMaterialization { bytes }
            })
        });
    }
    pub(crate) fn service(&self, observation: telemetry::ServiceObservation) {
        let id = crate::runtime::operation_id();
        self.emit(id.as_deref(), || EventPayload::Service {
            value: observation,
        });
    }
    pub(crate) async fn service_counters(&self) -> Result<telemetry::ServiceCounters> {
        decode::<telemetry::ServiceCounters>(self.select(Selection::ServiceCounters).await?)?
            .pop()
            .ok_or_else(|| invalid("native service diagnostic aggregate missing"))
    }
    pub(crate) fn failure(&self, diagnostic: enrichment_core::wire::Diagnostic) {
        match telemetry::Failure::try_from(diagnostic) {
            Ok(failure) => {
                let id = failure.correlation_id.clone();
                self.emit(id.as_deref(), || EventPayload::Failure { value: failure });
            }
            Err(error) => {
                if let Some(h) = &self.0 {
                    h.dropped.fetch_add(1, Ordering::AcqRel);
                }
                eprintln!("diagnostic recovery payload could not be encoded: {error}");
            }
        }
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
        if self
            .0
            .as_ref()
            .is_some_and(|h| h.closed.load(Ordering::Acquire))
        {
            return Ok(());
        }
        let closed = self.select(Selection::Close).await.map(|_| ());
        if self
            .0
            .as_ref()
            .is_some_and(|h| h.closed.load(Ordering::Acquire))
        {
            Ok(())
        } else {
            closed
        }
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
) {
    let mut pending = Vec::new();
    let mut commit_sequence = 0i64;
    let mut waiting = None;
    while let Some(message) = match waiting.take() {
        Some(message) => Some(message),
        None => receive.recv().await,
    } {
        match message {
            Message::Entry(entry) => {
                pending.push(entry);
                while pending.len() < QUEUE {
                    match receive.try_recv() {
                        Ok(Message::Entry(entry)) => pending.push(entry),
                        Ok(message) => {
                            waiting = Some(message);
                            break;
                        }
                        Err(_) => break,
                    }
                }
                commit_sequence += 1;
                if let Err(error) =
                    persist(&runtime, &store, &runtime_id, commit_sequence, &pending).await
                {
                    // A failed/indeterminate diagnostic commit cannot produce a successful
                    // aggregate. Closing the channel makes subsequent read barriers fail.
                    eprintln!("native diagnostic persistence stopped: {error}");
                    return;
                }
                pending.clear();
            }
            Message::Read(Selection::Close, response) => {
                closed.store(true, Ordering::Release);
                let _ = response.send(Ok(Vec::new()));
                return;
            }
            Message::Read(selection, response) => {
                let _ = response.send(select(&runtime, &store, &runtime_id, selection).await);
            }
        }
    }
}
async fn persist(
    runtime: &QueryRuntime,
    store: &DeltaStore,
    runtime_id: &str,
    commit_sequence: i64,
    entries: &[Entry],
) -> Result<()> {
    let contract = StorageContract::new(telemetry::schema::events())?;
    store.prepare_root(TABLE)?;
    let batches: Vec<_> = entries.iter().map(|entry| entry.batch.clone()).collect();
    let input = runtime.session().read_batches(batches)?;
    // Producer observations can enter the queue out of allocation order. Delta transaction
    // versions follow this sole writer's commit order, independently of observation sequence.
    for _ in 0..16 {
        let table = match store.load(TABLE, None).await {
            Ok(table) => table,
            Err(error) if missing_table(&error) => {
                match store.create(TABLE, &contract, false).await {
                    Ok(table) => table,
                    Err(error) if transaction_conflict(&error) => continue,
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        };
        match store
            .append(
                table,
                &contract,
                input.clone(),
                vec![deltalake::kernel::Transaction::new(
                    format!("telemetry/{runtime_id}"),
                    commit_sequence,
                )],
            )
            .await
        {
            Ok(_) => return Ok(()),
            Err(error) if transaction_conflict(&error) => continue,
            Err(error) => return Err(error),
        }
    }
    Err(invalid("diagnostic conflict bound exceeded"))
}
fn aggregate(grouped: bool) -> String {
    format!(
        "SELECT {} CAST(count(*) FILTER (WHERE kind='query') AS BIGINT UNSIGNED) AS executions, CAST(count(*) FILTER (WHERE kind='query' AND query.completed) AS BIGINT UNSIGNED) AS completed, CAST(count(*) FILTER (WHERE kind='query' AND NOT query.completed) AS BIGINT UNSIGNED) AS incomplete, CAST(coalesce(sum(query.planning_micros),0) AS BIGINT UNSIGNED) AS planning_micros, CAST(coalesce(sum(query.elapsed_micros),0) AS BIGINT UNSIGNED) AS elapsed_micros, CAST(count(*) FILTER (WHERE kind='index_materialization') AS BIGINT UNSIGNED) AS index_materializations, CAST(coalesce(sum(index_bytes),0) AS BIGINT UNSIGNED) AS index_spill_bytes, CAST(count(*) FILTER (WHERE kind='index_read') AS BIGINT UNSIGNED) AS index_reads, CAST(coalesce(sum(query.queue_micros),0) AS BIGINT UNSIGNED) AS queue_micros, CAST(coalesce(sum(query.output_rows),0) AS BIGINT UNSIGNED) AS output_rows, CAST(coalesce(sum(query.output_arrow_bytes),0) AS BIGINT UNSIGNED) AS output_arrow_bytes FROM current_events {}",
        if grouped { "operation_id," } else { "" },
        if grouped { "GROUP BY operation_id" } else { "" }
    )
}
async fn select(
    runtime: &QueryRuntime,
    store: &DeltaStore,
    runtime_id: &str,
    selection: Selection,
) -> Result<Vec<RecordBatch>> {
    if matches!(selection, Selection::Flush | Selection::Close) {
        return Ok(Vec::new());
    }
    let session = runtime.session();
    let all = match store.load(TABLE, None).await {
        Ok(table) => session.read_table(
            store
                .provider(&table, &StorageContract::new(telemetry::schema::events())?)
                .await?,
        )?,
        Err(error) if missing_table(&error) => session
            .read_empty()?
            .filter(datafusion::prelude::lit(false))?
            .select(
                telemetry::schema::events()
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
    let all = session.sql("SELECT runtime_id,sequence,recorded_at,operation_id,payload.kind AS kind,payload.query.value AS query,payload.operation.value AS operation,payload.failure.value AS failure,payload.index_materialization.bytes AS index_bytes,payload.service.value AS service FROM event_records").await?;
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
