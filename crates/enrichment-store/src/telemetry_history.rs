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
use enrichment_core::telemetry::{
    self, OperationDiagnostics, OperationEnd, QueryDiagnostics, Summary,
};
use serde::Serialize;
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
    Failures,
    Flush,
    Close,
}
#[derive(Serialize)]
struct Event<'a> {
    runtime_id: &'a str,
    sequence: u64,
    recorded_at: String,
    kind: &'a str,
    operation_id: Option<&'a str>,
    query: Option<&'a QueryDiagnostics>,
    operation: Option<&'a OperationEnd>,
    failure: Option<&'a telemetry::Failure>,
    index_bytes: Option<u64>,
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
    fn emit(
        &self,
        kind: &str,
        operation_id: Option<&str>,
        query: Option<&QueryDiagnostics>,
        operation: Option<&OperationEnd>,
        failure: Option<&telemetry::Failure>,
        index_bytes: Option<u64>,
    ) {
        let Some(h) = &self.0 else {
            return;
        };
        let write = || -> Result<()> {
            let batch = crate::control_jobs::encode(
                telemetry::schema::events(),
                &[Event {
                    runtime_id: &h.runtime_id,
                    sequence: h.next_event.fetch_add(1, Ordering::Relaxed) + 1,
                    recorded_at: enrichment_core::clock::now_rfc3339(),
                    kind,
                    operation_id,
                    query,
                    operation,
                    failure,
                    index_bytes,
                }],
            )?;
            let memory = MemoryConsumer::new("native_telemetry_ingress").register(&h.pool);
            memory.try_grow(batch.get_array_memory_size())?;
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
        self.emit(
            "query",
            query.operation_id.as_deref(),
            Some(query),
            None,
            None,
            None,
        );
    }
    pub(crate) fn operation(&self, operation: OperationEnd) {
        self.emit(
            "operation",
            Some(&operation.operation_id),
            None,
            Some(&operation),
            None,
            None,
        );
    }
    pub(crate) fn index(&self, operation_id: Option<&str>, bytes: Option<u64>) {
        self.emit(
            if bytes.is_some() {
                "index_materialization"
            } else {
                "index_read"
            },
            operation_id,
            None,
            None,
            None,
            bytes,
        );
    }
    pub(crate) fn failure(&self, diagnostic: enrichment_core::wire::Diagnostic) {
        match telemetry::Failure::try_from(diagnostic) {
            Ok(failure) => self.emit(
                "failure",
                failure.correlation_id.as_deref(),
                None,
                None,
                Some(&failure),
                None,
            ),
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
fn decode<T: serde::de::DeserializeOwned>(batches: Vec<RecordBatch>) -> Result<Vec<T>> {
    let mut writer = arrow::json::ArrayWriter::new(Vec::new());
    writer.write_batches(&batches.iter().collect::<Vec<_>>())?;
    writer.finish()?;
    serde_json::from_slice(&writer.into_inner()).map_err(|e| DataFusionError::External(Box::new(e)))
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
        "SELECT {} CAST(count(*) FILTER (WHERE kind='query') AS BIGINT UNSIGNED) AS executions, CAST(count(*) FILTER (WHERE kind='query' AND query.completed) AS BIGINT UNSIGNED) AS completed, CAST(count(*) FILTER (WHERE kind='query' AND NOT query.completed) AS BIGINT UNSIGNED) AS incomplete, coalesce(sum(query.planning_micros),0) AS planning_micros, coalesce(sum(query.elapsed_micros),0) AS elapsed_micros, CAST(count(*) FILTER (WHERE kind='index_materialization') AS BIGINT UNSIGNED) AS index_materializations, coalesce(sum(index_bytes),0) AS index_spill_bytes, CAST(count(*) FILTER (WHERE kind='index_read') AS BIGINT UNSIGNED) AS index_reads, coalesce(sum(query.queue_micros),0) AS queue_micros, coalesce(sum(query.output_rows),0) AS output_rows, coalesce(sum(query.output_arrow_bytes),0) AS output_arrow_bytes FROM current_events {}",
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
