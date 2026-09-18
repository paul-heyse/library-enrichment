# `arrow_flight::sql::Command`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.Command.json).

<a id="op-cf7f20617e0f5971ea16d973"></a>
## Command

`enum` · `arrow_flight::sql::Command` · arrow-flight 59.3.0

```rust
enum Command
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Helper to convert to/from protobuf [`Any`](../operations/arrow_flight.sql.Any.md#op-a83fa9be583ec3b019e51a4e) message
to a specific FlightSQL command message.

# Example
```rust
# use arrow_flight::sql::{Any, CommandStatementQuery, Command};
let flightsql_message = CommandStatementQuery {
  query: "SELECT * FROM foo".to_string(),
  transaction_id: None,
};

// Given a packed FlightSQL Any message
let any_message = Any::pack(&flightsql_message).unwrap();

// decode it to Command:
match Command::try_from(any_message).unwrap() {
  Command::CommandStatementQuery(decoded) => {
   assert_eq!(flightsql_message, decoded);
  }
  _ => panic!("Unexpected decoded message"),
}
```

<a id="op-77380395dd4f6eda45c88adf"></a>
## ActionBeginSavepointRequest

`variant` · `arrow_flight::sql::Command::ActionBeginSavepointRequest` · arrow-flight 59.3.0

```rust
ActionBeginSavepointRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionBeginSavepointRequestvariant

<a id="op-ba7b5837a9a35a720d45948c"></a>
## ActionBeginSavepointResult

`variant` · `arrow_flight::sql::Command::ActionBeginSavepointResult` · arrow-flight 59.3.0

```rust
ActionBeginSavepointResult
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionBeginSavepointResultvariant

<a id="op-fb4661bdb862e02d9060c71c"></a>
## ActionBeginTransactionRequest

`variant` · `arrow_flight::sql::Command::ActionBeginTransactionRequest` · arrow-flight 59.3.0

```rust
ActionBeginTransactionRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionBeginTransactionRequestvariant

<a id="op-e3b9a9dd1193279303d1cb13"></a>
## ActionBeginTransactionResult

`variant` · `arrow_flight::sql::Command::ActionBeginTransactionResult` · arrow-flight 59.3.0

```rust
ActionBeginTransactionResult
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionBeginTransactionResultvariant

<a id="op-c26e283ca12d59df017cc9e6"></a>
## ActionCancelQueryRequest

`variant` · `arrow_flight::sql::Command::ActionCancelQueryRequest` · arrow-flight 59.3.0

```rust
ActionCancelQueryRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionCancelQueryRequestvariant

<a id="op-f5f166481634dbf86897dda1"></a>
## ActionCancelQueryResult

`variant` · `arrow_flight::sql::Command::ActionCancelQueryResult` · arrow-flight 59.3.0

```rust
ActionCancelQueryResult
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionCancelQueryResultvariant

<a id="op-a9b57a11f2f219f368e3a57c"></a>
## ActionClosePreparedStatementRequest

`variant` · `arrow_flight::sql::Command::ActionClosePreparedStatementRequest` · arrow-flight 59.3.0

```rust
ActionClosePreparedStatementRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionClosePreparedStatementRequestvariant

<a id="op-a444bd457009de0965321d72"></a>
## ActionCreatePreparedStatementRequest

`variant` · `arrow_flight::sql::Command::ActionCreatePreparedStatementRequest` · arrow-flight 59.3.0

```rust
ActionCreatePreparedStatementRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionCreatePreparedStatementRequestvariant

<a id="op-9a1bb919e8f516d8d548c530"></a>
## ActionCreatePreparedStatementResult

`variant` · `arrow_flight::sql::Command::ActionCreatePreparedStatementResult` · arrow-flight 59.3.0

```rust
ActionCreatePreparedStatementResult
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionCreatePreparedStatementResultvariant

<a id="op-dd65c2b1213a54bed78ceb17"></a>
## ActionCreatePreparedSubstraitPlanRequest

`variant` · `arrow_flight::sql::Command::ActionCreatePreparedSubstraitPlanRequest` · arrow-flight 59.3.0

```rust
ActionCreatePreparedSubstraitPlanRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionCreatePreparedSubstraitPlanRequestvariant

<a id="op-ac80ecdb215dc8d029f05abf"></a>
## ActionEndSavepointRequest

`variant` · `arrow_flight::sql::Command::ActionEndSavepointRequest` · arrow-flight 59.3.0

```rust
ActionEndSavepointRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionEndSavepointRequestvariant

<a id="op-dfd9bb257decd2dfeaf0d5ae"></a>
## ActionEndTransactionRequest

`variant` · `arrow_flight::sql::Command::ActionEndTransactionRequest` · arrow-flight 59.3.0

```rust
ActionEndTransactionRequest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

ActionEndTransactionRequestvariant

<a id="op-b165f1efeaf5bc396d1c12cc"></a>
## CommandGetCatalogs

`variant` · `arrow_flight::sql::Command::CommandGetCatalogs` · arrow-flight 59.3.0

```rust
CommandGetCatalogs
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetCatalogsvariant

<a id="op-2ed358e73aa73907f1c3e8cd"></a>
## CommandGetCrossReference

`variant` · `arrow_flight::sql::Command::CommandGetCrossReference` · arrow-flight 59.3.0

```rust
CommandGetCrossReference
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetCrossReferencevariant

<a id="op-7a83aeb1eeb05f17e145b32d"></a>
## CommandGetDbSchemas

`variant` · `arrow_flight::sql::Command::CommandGetDbSchemas` · arrow-flight 59.3.0

```rust
CommandGetDbSchemas
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetDbSchemasvariant

<a id="op-fea9f75988a61f4a5250a5dd"></a>
## CommandGetExportedKeys

`variant` · `arrow_flight::sql::Command::CommandGetExportedKeys` · arrow-flight 59.3.0

```rust
CommandGetExportedKeys
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetExportedKeysvariant

<a id="op-07b7e5a78517a38c8f654139"></a>
## CommandGetImportedKeys

`variant` · `arrow_flight::sql::Command::CommandGetImportedKeys` · arrow-flight 59.3.0

```rust
CommandGetImportedKeys
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetImportedKeysvariant

<a id="op-4e9a84297032b1e35fc40d36"></a>
## CommandGetPrimaryKeys

`variant` · `arrow_flight::sql::Command::CommandGetPrimaryKeys` · arrow-flight 59.3.0

```rust
CommandGetPrimaryKeys
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetPrimaryKeysvariant

<a id="op-2059ceaf7f0df16f7bbfa7df"></a>
## CommandGetSqlInfo

`variant` · `arrow_flight::sql::Command::CommandGetSqlInfo` · arrow-flight 59.3.0

```rust
CommandGetSqlInfo
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetSqlInfovariant

<a id="op-6d46ff232f9fe36d371d18b3"></a>
## CommandGetTableTypes

`variant` · `arrow_flight::sql::Command::CommandGetTableTypes` · arrow-flight 59.3.0

```rust
CommandGetTableTypes
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetTableTypesvariant

<a id="op-c10eff88129214d931c1887b"></a>
## CommandGetTables

`variant` · `arrow_flight::sql::Command::CommandGetTables` · arrow-flight 59.3.0

```rust
CommandGetTables
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetTablesvariant

<a id="op-e81b5df179aa26a7e69aa252"></a>
## CommandGetXdbcTypeInfo

`variant` · `arrow_flight::sql::Command::CommandGetXdbcTypeInfo` · arrow-flight 59.3.0

```rust
CommandGetXdbcTypeInfo
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandGetXdbcTypeInfovariant

<a id="op-c82a1c60228ec3bb64188276"></a>
## CommandPreparedStatementQuery

`variant` · `arrow_flight::sql::Command::CommandPreparedStatementQuery` · arrow-flight 59.3.0

```rust
CommandPreparedStatementQuery
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandPreparedStatementQueryvariant

<a id="op-b3d8788b857b1370fc10c4cf"></a>
## CommandPreparedStatementUpdate

`variant` · `arrow_flight::sql::Command::CommandPreparedStatementUpdate` · arrow-flight 59.3.0

```rust
CommandPreparedStatementUpdate
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandPreparedStatementUpdatevariant

<a id="op-6b2e16295b9d14cb48942ff2"></a>
## CommandStatementIngest

`variant` · `arrow_flight::sql::Command::CommandStatementIngest` · arrow-flight 59.3.0

```rust
CommandStatementIngest
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandStatementIngestvariant

<a id="op-d237ebe222c0b2ced636ff56"></a>
## CommandStatementQuery

`variant` · `arrow_flight::sql::Command::CommandStatementQuery` · arrow-flight 59.3.0

```rust
CommandStatementQuery
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandStatementQueryvariant

<a id="op-811bf38a0ceaf848566ba32d"></a>
## CommandStatementSubstraitPlan

`variant` · `arrow_flight::sql::Command::CommandStatementSubstraitPlan` · arrow-flight 59.3.0

```rust
CommandStatementSubstraitPlan
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandStatementSubstraitPlanvariant

<a id="op-fa1748380fb14c3d9a6d6f84"></a>
## CommandStatementUpdate

`variant` · `arrow_flight::sql::Command::CommandStatementUpdate` · arrow-flight 59.3.0

```rust
CommandStatementUpdate
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

CommandStatementUpdatevariant

<a id="op-329309eb81eb82a310298f91"></a>
## DoPutPreparedStatementResult

`variant` · `arrow_flight::sql::Command::DoPutPreparedStatementResult` · arrow-flight 59.3.0

```rust
DoPutPreparedStatementResult
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

DoPutPreparedStatementResultvariant

<a id="op-bb9fe3ef73a779e6c8a60bf6"></a>
## DoPutUpdateResult

`variant` · `arrow_flight::sql::Command::DoPutUpdateResult` · arrow-flight 59.3.0

```rust
DoPutUpdateResult
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

DoPutUpdateResultvariant

<a id="op-303eced4d9fff510e74e1065"></a>
## Error

`assoc_type` · `arrow_flight::sql::Command::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Command", "path": "Command"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c53273632e4106a362ed4c77"></a>
## TicketStatementQuery

`variant` · `arrow_flight::sql::Command::TicketStatementQuery` · arrow-flight 59.3.0

```rust
TicketStatementQuery
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

TicketStatementQueryvariant

<a id="op-10fe4a7f5b9cf1fb12b4d62b"></a>
## Unknown

`variant` · `arrow_flight::sql::Command::Unknown` · arrow-flight 59.3.0

```rust
Unknown
```

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Any message that is not any FlightSQL command.

<a id="op-6374814e5c88ba5723bb4eb3"></a>
## clone

`function` · `arrow_flight::sql::Command::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> Command
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Command", "path": "Command"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbd0043c0112b32a396c4b8d"></a>
## eq

`function` · `arrow_flight::sql::Command::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &Command) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Command", "path": "Command"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c87e435839f9b743ea34b77d"></a>
## fmt

`function` · `arrow_flight::sql::Command::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Command", "path": "Command"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-922e29f0bc44548be06c77d0"></a>
## into_any

`function` · `arrow_flight::sql::Command::into_any` · arrow-flight 59.3.0

```rust
fn into_any(self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Command", "path": "Command"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Convert the command to [`Any`](../operations/arrow_flight.sql.Any.md#op-a83fa9be583ec3b019e51a4e).

<a id="op-f28a2603c7804d551471c1dc"></a>
## try_from

`function` · `arrow_flight::sql::Command::try_from` · arrow-flight 59.3.0

```rust
fn try_from(any: Any) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Command", "path": "Command"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Any", "path": "Any"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3998dd8df870715a9eff689f"></a>
## type_url

`function` · `arrow_flight::sql::Command::type_url` · arrow-flight 59.3.0

```rust
fn type_url(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::Command", "path": "Command"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Get the URL for the command.
