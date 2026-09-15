# `arrow_flight::sql::server`

Crate `arrow-flight` · 3 public items · structured records in [`model/arrow_flight.sql.server.json`](../model/arrow_flight.sql.server.json)

## DoPutError

`enum` · `arrow_flight::sql::server::DoPutError`

```rust
enum DoPutError
```

**Variants**: `MissingCommand`, `MissingFlightDescriptor`

**Implements**: `core::fmt::Display`

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Unrecoverable errors associated with `do_put` requests

---

## PeekableFlightDataStream

`struct` · `arrow_flight::sql::server::PeekableFlightDataStream`

```rust
struct PeekableFlightDataStream
```

**Implements**: `futures_core::stream::Stream`

**Methods** (3)

```rust
fn into_inner(self) -> Streaming<FlightData>
fn into_peekable(self) -> Peekable<Streaming<FlightData>>
async fn peek(&mut self) -> Option<&Result<FlightData, Status>>
```

**via `futures_core::stream::Stream`**

```rust
fn poll_next(Pin<&mut self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>>
```

A wrapper around [`Streaming<FlightData>`] that allows "peeking" at the
message at the front of the stream without consuming it.

This is needed because sometimes the first message in the stream will contain
a [`FlightDescriptor`] in addition to potentially any data, and the dispatch logic
must inspect this information.

# Example

[`PeekableFlightDataStream::peek`] can be used to peek at the first message without
discarding it; otherwise, `PeekableFlightDataStream` can be used as a regular stream.
See the following example:

```no_run
use arrow_array::RecordBatch;
use arrow_flight::decode::FlightRecordBatchStream;
use arrow_flight::FlightDescriptor;
use arrow_flight::error::FlightError;
use arrow_flight::sql::server::PeekableFlightDataStream;
use tonic::{Request, Status};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Status> {
    let request: Request<PeekableFlightDataStream> = todo!();
    let stream: PeekableFlightDataStream = request.into_inner();

    // The first message contains the flight descriptor and the schema.
    // Read the flight descriptor without discarding the schema:
    let flight_descriptor: FlightDescriptor = stream
        .peek()
        .await
        .cloned()
        .transpose()?
        .and_then(|data| data.flight_descriptor)
        .expect("first message should contain flight descriptor");

    // Pass the stream through a decoder
    let batches: Vec<RecordBatch> = FlightRecordBatchStream::new_from_flight_data(
        request.into_inner().map_err(|e| e.into()),
    )
    .try_collect()
    .await?;
}
```

---

## FlightSqlService

`trait` · `arrow_flight::sql::server::FlightSqlService`

```rust
trait FlightSqlService: Sync + Send + Sized + 'static
```

**Methods** (47)

```rust
async fn do_action_begin_savepoint(&self, _query: ActionBeginSavepointRequest, _request: Request<Action>) -> Result<ActionBeginSavepointResult, Status>
async fn do_action_begin_transaction(&self, _query: ActionBeginTransactionRequest, _request: Request<Action>) -> Result<ActionBeginTransactionResult, Status>
async fn do_action_cancel_query(&self, _query: ActionCancelQueryRequest, _request: Request<Action>) -> Result<ActionCancelQueryResult, Status>
async fn do_action_close_prepared_statement(&self, _query: ActionClosePreparedStatementRequest, _request: Request<Action>) -> Result<(), Status>
async fn do_action_create_prepared_statement(&self, _query: ActionCreatePreparedStatementRequest, _request: Request<Action>) -> Result<ActionCreatePreparedStatementResult, Status>
async fn do_action_create_prepared_substrait_plan(&self, _query: ActionCreatePreparedSubstraitPlanRequest, _request: Request<Action>) -> Result<ActionCreatePreparedStatementResult, Status>
async fn do_action_end_savepoint(&self, _query: ActionEndSavepointRequest, _request: Request<Action>) -> Result<(), Status>
async fn do_action_end_transaction(&self, _query: ActionEndTransactionRequest, _request: Request<Action>) -> Result<(), Status>
async fn do_action_fallback(&self, request: Request<Action>) -> Result<Response<<Self as FlightService>::DoActionStream>, Status>
async fn do_exchange_fallback(&self, _request: Request<Streaming<FlightData>>) -> Result<Response<<Self as FlightService>::DoExchangeStream>, Status>
async fn do_get_catalogs(&self, _query: CommandGetCatalogs, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_cross_reference(&self, _query: CommandGetCrossReference, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_exported_keys(&self, _query: CommandGetExportedKeys, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_fallback(&self, _request: Request<Ticket>, message: Any) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_imported_keys(&self, _query: CommandGetImportedKeys, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_prepared_statement(&self, _query: CommandPreparedStatementQuery, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_primary_keys(&self, _query: CommandGetPrimaryKeys, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_schemas(&self, _query: CommandGetDbSchemas, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_sql_info(&self, _query: CommandGetSqlInfo, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_statement(&self, _ticket: TicketStatementQuery, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_table_types(&self, _query: CommandGetTableTypes, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_tables(&self, _query: CommandGetTables, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_get_xdbc_type_info(&self, _query: CommandGetXdbcTypeInfo, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
async fn do_handshake(&self, _request: Request<Streaming<HandshakeRequest>>) -> Result<Response<Pin<Box<dyn Stream<Item = Result<HandshakeResponse, Status>> + Send>>>, Status>
async fn do_put_error_callback(&self, _request: Request<PeekableFlightDataStream>, error: DoPutError) -> Result<Response<<Self as FlightService>::DoPutStream>, Status>
async fn do_put_fallback(&self, _request: Request<PeekableFlightDataStream>, message: Any) -> Result<Response<<Self as FlightService>::DoPutStream>, Status>
async fn do_put_prepared_statement_query(&self, _query: CommandPreparedStatementQuery, _request: Request<PeekableFlightDataStream>) -> Result<DoPutPreparedStatementResult, Status>
async fn do_put_prepared_statement_update(&self, _query: CommandPreparedStatementUpdate, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
async fn do_put_statement_ingest(&self, _ticket: CommandStatementIngest, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
async fn do_put_statement_update(&self, _ticket: CommandStatementUpdate, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
async fn do_put_substrait_plan(&self, _query: CommandStatementSubstraitPlan, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
async fn get_flight_info_catalogs(&self, _query: CommandGetCatalogs, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_cross_reference(&self, _query: CommandGetCrossReference, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_exported_keys(&self, _query: CommandGetExportedKeys, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_fallback(&self, cmd: Command, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_imported_keys(&self, _query: CommandGetImportedKeys, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_prepared_statement(&self, _query: CommandPreparedStatementQuery, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_primary_keys(&self, _query: CommandGetPrimaryKeys, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_schemas(&self, _query: CommandGetDbSchemas, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_sql_info(&self, _query: CommandGetSqlInfo, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_statement(&self, _query: CommandStatementQuery, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_substrait_plan(&self, _query: CommandStatementSubstraitPlan, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_table_types(&self, _query: CommandGetTableTypes, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_tables(&self, _query: CommandGetTables, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn get_flight_info_xdbc_type_info(&self, _query: CommandGetXdbcTypeInfo, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
async fn list_custom_actions(&self) -> Option<Vec<Result<ActionType, Status>>>
async fn register_sql_info(&self, id: i32, result: &SqlInfo) -> ()
```

Implements FlightSqlService to handle the flight sql protocol

---
