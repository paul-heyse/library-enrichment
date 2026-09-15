# `arrow_flight::sql::client`

Crate `arrow-flight` · 4 public items · structured records in [`model/arrow_flight.sql.client.json`](../model/arrow_flight.sql.client.json)

## ArrowFlightData

`enum` · `arrow_flight::sql::client::ArrowFlightData`

```rust
enum ArrowFlightData
```

**Variants**: `RecordBatch`, `Schema`

A polymorphic structure to natively represent different types of data contained in `FlightData`

---

## arrow_data_from_flight_data

`function` · `arrow_flight::sql::client::arrow_data_from_flight_data`

```rust
fn arrow_data_from_flight_data(flight_data: FlightData, arrow_schema_ref: &arrow_schema::SchemaRef) -> std::result::Result<ArrowFlightData, arrow_schema::ArrowError>
```

Extract `Schema` or `RecordBatch`es from the `FlightData` wire representation

---

## FlightSqlServiceClient

`struct` · `arrow_flight::sql::client::FlightSqlServiceClient`

```rust
struct FlightSqlServiceClient<T>
```

**Derives**: Clone, Debug

**Methods** (30)

```rust
async fn begin_transaction(&mut self) -> Result<Bytes>
fn clear_token(&mut self)
async fn close(&mut self) -> Result<()>
async fn do_action(&mut self, request: impl IntoRequest<Action>) -> Result<Streaming<Result>>
async fn do_get(&mut self, ticket: impl IntoRequest<Ticket>) -> Result<FlightRecordBatchStream>
async fn do_put(&mut self, request: impl tonic::IntoStreamingRequest<Message = FlightData>) -> Result<Streaming<PutResult>>
async fn end_transaction(&mut self, transaction_id: Bytes, action: EndTransaction) -> Result<()>
async fn execute(&mut self, query: String, transaction_id: Option<Bytes>) -> Result<FlightInfo>
async fn execute_ingest<S>(&mut self, command: CommandStatementIngest, stream: S) -> Result<i64> where S: Stream<Item = error::Result<RecordBatch>> + Send + 'static
async fn execute_update(&mut self, query: String, transaction_id: Option<Bytes>) -> Result<i64>
async fn get_catalogs(&mut self) -> Result<FlightInfo>
async fn get_cross_reference(&mut self, request: CommandGetCrossReference) -> Result<FlightInfo>
async fn get_db_schemas(&mut self, request: CommandGetDbSchemas) -> Result<FlightInfo>
async fn get_exported_keys(&mut self, request: CommandGetExportedKeys) -> Result<FlightInfo>
async fn get_imported_keys(&mut self, request: CommandGetImportedKeys) -> Result<FlightInfo>
async fn get_primary_keys(&mut self, request: CommandGetPrimaryKeys) -> Result<FlightInfo>
async fn get_sql_info(&mut self, sql_infos: Vec<SqlInfo>) -> Result<FlightInfo>
async fn get_table_types(&mut self) -> Result<FlightInfo>
async fn get_tables(&mut self, request: CommandGetTables) -> Result<FlightInfo>
async fn get_xdbc_type_info(&mut self, request: CommandGetXdbcTypeInfo) -> Result<FlightInfo>
async fn handshake(&mut self, username: &str, password: &str) -> Result<Bytes>
fn inner(&self) -> &FlightServiceClient<T>
fn inner_mut(&mut self) -> &mut FlightServiceClient<T>
fn into_inner(self) -> FlightServiceClient<T>
fn new(channel: T) -> Self
fn new_from_inner(inner: FlightServiceClient<T>) -> Self
async fn prepare(&mut self, query: String, transaction_id: Option<Bytes>) -> Result<PreparedStatement<T>> where T: Clone
fn set_header(&mut self, key: impl Into<String>, value: impl Into<String>)
fn set_token(&mut self, token: String)
fn token(&self) -> Option<&String>
```

A FlightSQLServiceClient is an endpoint for retrieving or storing Arrow data
by FlightSQL protocol.

---

## PreparedStatement

`struct` · `arrow_flight::sql::client::PreparedStatement`

```rust
struct PreparedStatement<T>
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
async fn close(self) -> Result<()>
fn dataset_schema(&self) -> Result<&Schema>
async fn execute(&mut self) -> Result<FlightInfo>
async fn execute_update(&mut self) -> Result<i64>
fn parameter_schema(&self) -> Result<&Schema>
fn set_parameters(&mut self, parameter_binding: RecordBatch) -> Result<()>
```

A PreparedStatement

---
