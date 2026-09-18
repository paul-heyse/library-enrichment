# `arrow_flight::sql::server::FlightSqlService`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.server.FlightSqlService.json).

<a id="op-2ac451ffe44d8bae3d0da263"></a>
## FlightSqlService

`trait` · `arrow_flight::sql::server::FlightSqlService` · arrow-flight 59.3.0

```rust
trait FlightSqlService: Sync + Send + Sized + 'static
```

Source: `src/sql/server.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Implements FlightSqlService to handle the flight sql protocol

<a id="op-a9e62af6ed4fa326068b776f"></a>
## FlightService

`assoc_type` · `arrow_flight::sql::server::FlightSqlService::FlightService` · arrow-flight 59.3.0

```rust
FlightService
```

Source: `src/sql/server.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

When impl FlightSqlService, you can always set FlightService to Self

<a id="op-9b569843e586e9e0ad9a4d5b"></a>
## do_action_begin_savepoint

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_begin_savepoint` · arrow-flight 59.3.0

```rust
async fn do_action_begin_savepoint(&self, _query: ActionBeginSavepointRequest, _request: Request<Action>) -> Result<ActionBeginSavepointResult, Status>
```

Source: `src/sql/server.rs:531`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Begin a savepoint

<a id="op-062b40466146f253d9f5e73b"></a>
## do_action_begin_transaction

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_begin_transaction` · arrow-flight 59.3.0

```rust
async fn do_action_begin_transaction(&self, _query: ActionBeginTransactionRequest, _request: Request<Action>) -> Result<ActionBeginTransactionResult, Status>
```

Source: `src/sql/server.rs:509`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Begin a transaction

<a id="op-c0351c501c42528ec1f51d5e"></a>
## do_action_cancel_query

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_cancel_query` · arrow-flight 59.3.0

```rust
async fn do_action_cancel_query(&self, _query: ActionCancelQueryRequest, _request: Request<Action>) -> Result<ActionCancelQueryResult, Status>
```

Source: `src/sql/server.rs:553`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Cancel a query

<a id="op-0678acefb4095fee9f86014c"></a>
## do_action_close_prepared_statement

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_close_prepared_statement` · arrow-flight 59.3.0

```rust
async fn do_action_close_prepared_statement(&self, _query: ActionClosePreparedStatementRequest, _request: Request<Action>) -> Result<(), Status>
```

Source: `src/sql/server.rs:487`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Close a prepared statement.

<a id="op-052b419a047c8aeb22fd483f"></a>
## do_action_create_prepared_statement

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_create_prepared_statement` · arrow-flight 59.3.0

```rust
async fn do_action_create_prepared_statement(&self, _query: ActionCreatePreparedStatementRequest, _request: Request<Action>) -> Result<ActionCreatePreparedStatementResult, Status>
```

Source: `src/sql/server.rs:476`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a prepared statement from given SQL statement.

<a id="op-f628f4f4d67b982e9b1c7721"></a>
## do_action_create_prepared_substrait_plan

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_create_prepared_substrait_plan` · arrow-flight 59.3.0

```rust
async fn do_action_create_prepared_substrait_plan(&self, _query: ActionCreatePreparedSubstraitPlanRequest, _request: Request<Action>) -> Result<ActionCreatePreparedStatementResult, Status>
```

Source: `src/sql/server.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a prepared substrait plan.

<a id="op-086e4eed9aa395d15113242e"></a>
## do_action_end_savepoint

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_end_savepoint` · arrow-flight 59.3.0

```rust
async fn do_action_end_savepoint(&self, _query: ActionEndSavepointRequest, _request: Request<Action>) -> Result<(), Status>
```

Source: `src/sql/server.rs:542`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

End a savepoint

<a id="op-98db9f8b894a3b0a296dfe16"></a>
## do_action_end_transaction

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_end_transaction` · arrow-flight 59.3.0

```rust
async fn do_action_end_transaction(&self, _query: ActionEndTransactionRequest, _request: Request<Action>) -> Result<(), Status>
```

Source: `src/sql/server.rs:520`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

End a transaction

<a id="op-802d41095dcf75a6d0a5e03b"></a>
## do_action_fallback

`function` · `arrow_flight::sql::server::FlightSqlService::do_action_fallback` · arrow-flight 59.3.0

```rust
async fn do_action_fallback(&self, request: Request<Action>) -> Result<Response<<Self as FlightService>::DoActionStream>, Status>
```

Source: `src/sql/server.rs:460`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Implementors may override to handle additional calls to do_action()

<a id="op-3af367e936b0021356de0378"></a>
## do_exchange_fallback

`function` · `arrow_flight::sql::server::FlightSqlService::do_exchange_fallback` · arrow-flight 59.3.0

```rust
async fn do_exchange_fallback(&self, _request: Request<Streaming<FlightData>>) -> Result<Response<<Self as FlightService>::DoExchangeStream>, Status>
```

Source: `src/sql/server.rs:565`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

do_exchange
Implementors may override to handle additional calls to do_exchange()

<a id="op-f20527cd6044c4713687282d"></a>
## do_get_catalogs

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_catalogs` · arrow-flight 59.3.0

```rust
async fn do_get_catalogs(&self, _query: CommandGetCatalogs, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:266`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the list of catalogs.

<a id="op-2626dc93c52c59a17f8fae9d"></a>
## do_get_cross_reference

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_cross_reference` · arrow-flight 59.3.0

```rust
async fn do_get_cross_reference(&self, _query: CommandGetCrossReference, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:354`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the data related to the cross reference.

<a id="op-6bcae4616c94a6c1ff83ba75"></a>
## do_get_exported_keys

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_exported_keys` · arrow-flight 59.3.0

```rust
async fn do_get_exported_keys(&self, _query: CommandGetExportedKeys, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:332`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the data related to the exported keys.

<a id="op-18d810b66ca2870424d6067e"></a>
## do_get_fallback

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_fallback` · arrow-flight 59.3.0

```rust
async fn do_get_fallback(&self, _request: Request<Ticket>, message: Any) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Implementors may override to handle additional calls to do_get()

<a id="op-e26dce2e23985de1dd854939"></a>
## do_get_imported_keys

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_imported_keys` · arrow-flight 59.3.0

```rust
async fn do_get_imported_keys(&self, _query: CommandGetImportedKeys, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:343`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the data related to the imported keys.

<a id="op-7f0d038dfd30d90cdfe4f7b3"></a>
## do_get_prepared_statement

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_prepared_statement` · arrow-flight 59.3.0

```rust
async fn do_get_prepared_statement(&self, _query: CommandPreparedStatementQuery, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the prepared statement query results.

<a id="op-95facf1df2248b47f188e143"></a>
## do_get_primary_keys

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_primary_keys` · arrow-flight 59.3.0

```rust
async fn do_get_primary_keys(&self, _query: CommandGetPrimaryKeys, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the data related to the primary and foreign keys.

<a id="op-78051b84e425a4897aff717d"></a>
## do_get_schemas

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_schemas` · arrow-flight 59.3.0

```rust
async fn do_get_schemas(&self, _query: CommandGetDbSchemas, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the list of schemas.

<a id="op-ec1a37015e9f626da18e6078"></a>
## do_get_sql_info

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_sql_info` · arrow-flight 59.3.0

```rust
async fn do_get_sql_info(&self, _query: CommandGetSqlInfo, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the list of SqlInfo results.

<a id="op-57bbe3a7e1a708f191149265"></a>
## do_get_statement

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_statement` · arrow-flight 59.3.0

```rust
async fn do_get_statement(&self, _ticket: TicketStatementQuery, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:244`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the query results.

<a id="op-0d7a3774b5c0fdb283574f26"></a>
## do_get_table_types

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_table_types` · arrow-flight 59.3.0

```rust
async fn do_get_table_types(&self, _query: CommandGetTableTypes, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the data related to the table types.

<a id="op-56faf648bbe995f5d79a6421"></a>
## do_get_tables

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_tables` · arrow-flight 59.3.0

```rust
async fn do_get_tables(&self, _query: CommandGetTables, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:288`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the list of tables.

<a id="op-78b7b3c4302047e45803edb7"></a>
## do_get_xdbc_type_info

`function` · `arrow_flight::sql::server::FlightSqlService::do_get_xdbc_type_info` · arrow-flight 59.3.0

```rust
async fn do_get_xdbc_type_info(&self, _query: CommandGetXdbcTypeInfo, _request: Request<Ticket>) -> Result<Response<<Self as FlightService>::DoGetStream>, Status>
```

Source: `src/sql/server.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightDataStream containing the data related to the supported XDBC types.

<a id="op-d3b1a832c3b783788937db9b"></a>
## do_handshake

`function` · `arrow_flight::sql::server::FlightSqlService::do_handshake` · arrow-flight 59.3.0

```rust
async fn do_handshake(&self, _request: Request<Streaming<HandshakeRequest>>) -> Result<Response<Pin<Box<dyn Stream<Item = Result<HandshakeResponse, Status>> + Send>>>, Status>
```

Source: `src/sql/server.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Accept authentication and return a token
<https://arrow.apache.org/docs/format/Flight.html#authentication>

<a id="op-b945499309f1fd5902838380"></a>
## do_put_error_callback

`function` · `arrow_flight::sql::server::FlightSqlService::do_put_error_callback` · arrow-flight 59.3.0

```rust
async fn do_put_error_callback(&self, _request: Request<PeekableFlightDataStream>, error: DoPutError) -> Result<Response<<Self as FlightService>::DoPutStream>, Status>
```

Source: `src/sql/server.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Implementors may override to handle do_put errors

<a id="op-6ceca98271a12ab8c3d5f26c"></a>
## do_put_fallback

`function` · `arrow_flight::sql::server::FlightSqlService::do_put_fallback` · arrow-flight 59.3.0

```rust
async fn do_put_fallback(&self, _request: Request<PeekableFlightDataStream>, message: Any) -> Result<Response<<Self as FlightService>::DoPutStream>, Status>
```

Source: `src/sql/server.rs:378`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Implementors may override to handle additional calls to do_put()

<a id="op-73e205457281c7e6cfa2ae36"></a>
## do_put_prepared_statement_query

`function` · `arrow_flight::sql::server::FlightSqlService::do_put_prepared_statement_query` · arrow-flight 59.3.0

```rust
async fn do_put_prepared_statement_query(&self, _query: CommandPreparedStatementQuery, _request: Request<PeekableFlightDataStream>) -> Result<DoPutPreparedStatementResult, Status>
```

Source: `src/sql/server.rs:425`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Bind parameters to given prepared statement.

Returns an opaque handle that the client should pass
back to the server during subsequent requests with this
prepared statement.

<a id="op-9d816d43555dd4a6b6e57671"></a>
## do_put_prepared_statement_update

`function` · `arrow_flight::sql::server::FlightSqlService::do_put_prepared_statement_update` · arrow-flight 59.3.0

```rust
async fn do_put_prepared_statement_update(&self, _query: CommandPreparedStatementUpdate, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
```

Source: `src/sql/server.rs:436`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Execute an update SQL prepared statement.

<a id="op-39e62c6772cea47c7ef730ff"></a>
## do_put_statement_ingest

`function` · `arrow_flight::sql::server::FlightSqlService::do_put_statement_ingest` · arrow-flight 59.3.0

```rust
async fn do_put_statement_ingest(&self, _ticket: CommandStatementIngest, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
```

Source: `src/sql/server.rs:410`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Execute a bulk ingestion.

<a id="op-8be1bdb9fb8e1c8ae7b8f661"></a>
## do_put_statement_update

`function` · `arrow_flight::sql::server::FlightSqlService::do_put_statement_update` · arrow-flight 59.3.0

```rust
async fn do_put_statement_update(&self, _ticket: CommandStatementUpdate, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
```

Source: `src/sql/server.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Execute an update SQL statement.

<a id="op-27758786ae7f85b692d6d254"></a>
## do_put_substrait_plan

`function` · `arrow_flight::sql::server::FlightSqlService::do_put_substrait_plan` · arrow-flight 59.3.0

```rust
async fn do_put_substrait_plan(&self, _query: CommandStatementSubstraitPlan, _request: Request<PeekableFlightDataStream>) -> Result<i64, Status>
```

Source: `src/sql/server.rs:447`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Execute a substrait plan

<a id="op-669ce5984c461d9c109437ee"></a>
## get_flight_info_catalogs

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_catalogs` · arrow-flight 59.3.0

```rust
async fn get_flight_info_catalogs(&self, _query: CommandGetCatalogs, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo for listing catalogs.

<a id="op-36781e5680b6aa3428b53b30"></a>
## get_flight_info_cross_reference

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_cross_reference` · arrow-flight 59.3.0

```rust
async fn get_flight_info_cross_reference(&self, _query: CommandGetCrossReference, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:208`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo to extract information about cross reference.

<a id="op-a191274ffe398eec863e9500"></a>
## get_flight_info_exported_keys

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_exported_keys` · arrow-flight 59.3.0

```rust
async fn get_flight_info_exported_keys(&self, _query: CommandGetExportedKeys, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo to extract information about exported keys.

<a id="op-f74c5493107f9308ebb70fa6"></a>
## get_flight_info_fallback

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_fallback` · arrow-flight 59.3.0

```rust
async fn get_flight_info_fallback(&self, cmd: Command, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Implementors may override to handle additional calls to get_flight_info()

<a id="op-4012b125683b428b2ecf81c5"></a>
## get_flight_info_imported_keys

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_imported_keys` · arrow-flight 59.3.0

```rust
async fn get_flight_info_imported_keys(&self, _query: CommandGetImportedKeys, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:197`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo to extract information about imported keys.

<a id="op-4a49237ce74b8f1678fcdc8b"></a>
## get_flight_info_prepared_statement

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_prepared_statement` · arrow-flight 59.3.0

```rust
async fn get_flight_info_prepared_statement(&self, _query: CommandPreparedStatementQuery, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo for executing an already created prepared statement.

<a id="op-c30d39c158b61fc5db0a77bc"></a>
## get_flight_info_primary_keys

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_primary_keys` · arrow-flight 59.3.0

```rust
async fn get_flight_info_primary_keys(&self, _query: CommandGetPrimaryKeys, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo to extract information about primary and foreign keys.

<a id="op-81bcc2cd84bfec905ee45e0f"></a>
## get_flight_info_schemas

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_schemas` · arrow-flight 59.3.0

```rust
async fn get_flight_info_schemas(&self, _query: CommandGetDbSchemas, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:131`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo for listing schemas.

<a id="op-3d0b1946c78179f559644769"></a>
## get_flight_info_sql_info

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_sql_info` · arrow-flight 59.3.0

```rust
async fn get_flight_info_sql_info(&self, _query: CommandGetSqlInfo, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:164`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo for retrieving other information (See SqlInfo).

<a id="op-94fc4b64ad63f1fc23b0fec9"></a>
## get_flight_info_statement

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_statement` · arrow-flight 59.3.0

```rust
async fn get_flight_info_statement(&self, _query: CommandStatementQuery, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo for executing a SQL query.

<a id="op-9ad67ce1819ebe8cd92433ca"></a>
## get_flight_info_substrait_plan

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_substrait_plan` · arrow-flight 59.3.0

```rust
async fn get_flight_info_substrait_plan(&self, _query: CommandStatementSubstraitPlan, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo for executing a substrait plan.

<a id="op-c645d9a730f24ce6aec69905"></a>
## get_flight_info_table_types

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_table_types` · arrow-flight 59.3.0

```rust
async fn get_flight_info_table_types(&self, _query: CommandGetTableTypes, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo to extract information about the table types.

<a id="op-cc9d3e60c07825eb5d1d944a"></a>
## get_flight_info_tables

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_tables` · arrow-flight 59.3.0

```rust
async fn get_flight_info_tables(&self, _query: CommandGetTables, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo for listing tables.

<a id="op-98de119c6e2e73a5117c461e"></a>
## get_flight_info_xdbc_type_info

`function` · `arrow_flight::sql::server::FlightSqlService::get_flight_info_xdbc_type_info` · arrow-flight 59.3.0

```rust
async fn get_flight_info_xdbc_type_info(&self, _query: CommandGetXdbcTypeInfo, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status>
```

Source: `src/sql/server.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get a FlightInfo to extract information about the supported XDBC types.

<a id="op-771435f583ecd6ce96953370"></a>
## list_custom_actions

`function` · `arrow_flight::sql::server::FlightSqlService::list_custom_actions` · arrow-flight 59.3.0

```rust
async fn list_custom_actions(&self) -> Option<Vec<Result<ActionType, Status>>>
```

Source: `src/sql/server.rs:471`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add custom actions to list_actions() result

<a id="op-59110d87c6fa41e8412d805d"></a>
## register_sql_info

`function` · `arrow_flight::sql::server::FlightSqlService::register_sql_info` · arrow-flight 59.3.0

```rust
async fn register_sql_info(&self, id: i32, result: &SqlInfo) -> ()
```

Source: `src/sql/server.rs:573`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Register a new SqlInfo result, making it available when calling GetSqlInfo.
