# `arrow_flight::sql`

Crate `arrow-flight` · 3 public items · structured records in [`model/arrow_flight.sql.json`](../model/arrow_flight.sql.json)

## Command

`enum` · `arrow_flight::sql::Command`

```rust
enum Command
```

**Variants**: `ActionBeginSavepointRequest`, `ActionBeginSavepointResult`, `ActionBeginTransactionRequest`, `ActionBeginTransactionResult`, `ActionCancelQueryRequest`, `ActionCancelQueryResult`, `ActionClosePreparedStatementRequest`, `ActionCreatePreparedStatementRequest`, `ActionCreatePreparedStatementResult`, `ActionCreatePreparedSubstraitPlanRequest`, `ActionEndSavepointRequest`, `ActionEndTransactionRequest`, `CommandGetCatalogs`, `CommandGetCrossReference`, `CommandGetDbSchemas`, `CommandGetExportedKeys`, `CommandGetImportedKeys`, `CommandGetPrimaryKeys`, `CommandGetSqlInfo`, `CommandGetTableTypes`, `CommandGetTables`, `CommandGetXdbcTypeInfo`, `CommandPreparedStatementQuery`, `CommandPreparedStatementUpdate`, `CommandStatementIngest`, `CommandStatementQuery`, `CommandStatementSubstraitPlan`, `CommandStatementUpdate`, `DoPutPreparedStatementResult`, `DoPutUpdateResult`, `TicketStatementQuery`, `Unknown`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn into_any(self) -> Any
fn type_url(&self) -> &str
```

**via `core::convert::TryFrom`**

```rust
fn try_from(any: Any) -> Result<Self, Self::Error>
```

Helper to convert to/from protobuf [`Any`] message
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

---

## Any

`struct` · `arrow_flight::sql::Any`

```rust
struct Any
```

**Fields**: `type_url`, `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is<M: ProstMessageExt>(&self) -> bool
fn pack<M: ProstMessageExt>(message: &M) -> Result<Any, ArrowError>
fn unpack<M: ProstMessageExt>(&self) -> Result<Option<M>, ArrowError>
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

An implementation of the protobuf [`Any`] message type

Encoded protobuf messages are not self-describing, nor contain any information
on the schema of the encoded payload. Consequently to decode a protobuf a client
must know the exact schema of the message.

This presents a problem for loosely typed APIs, where the exact message payloads
are not enumerable, and therefore cannot be enumerated as variants in a [oneof].

One solution is [`Any`] where the encoded payload is paired with a `type_url`
identifying the type of encoded message, and the resulting combination encoded.

Clients can then decode the outer [`Any`], inspect the `type_url` and if it is
a type they recognise, proceed to decode the embedded message `value`

[`Any`]: https://developers.google.com/protocol-buffers/docs/proto3#any
[oneof]: https://developers.google.com/protocol-buffers/docs/proto3#oneof

---

## ProstMessageExt

`trait` · `arrow_flight::sql::ProstMessageExt`

```rust
trait ProstMessageExt: prost::Message + Default
```

**Implementors** (31)

- `arrow_flight::sql::gen::ActionBeginSavepointRequest`
- `arrow_flight::sql::gen::ActionBeginSavepointResult`
- `arrow_flight::sql::gen::ActionBeginTransactionRequest`
- `arrow_flight::sql::gen::ActionBeginTransactionResult`
- `arrow_flight::sql::gen::ActionCancelQueryRequest`
- `arrow_flight::sql::gen::ActionCancelQueryResult`
- `arrow_flight::sql::gen::ActionClosePreparedStatementRequest`
- `arrow_flight::sql::gen::ActionCreatePreparedStatementRequest`
- `arrow_flight::sql::gen::ActionCreatePreparedStatementResult`
- `arrow_flight::sql::gen::ActionCreatePreparedSubstraitPlanRequest`
- `arrow_flight::sql::gen::ActionEndSavepointRequest`
- `arrow_flight::sql::gen::ActionEndTransactionRequest`
- `arrow_flight::sql::gen::CommandGetCatalogs`
- `arrow_flight::sql::gen::CommandGetCrossReference`
- `arrow_flight::sql::gen::CommandGetDbSchemas`
- `arrow_flight::sql::gen::CommandGetExportedKeys`
- `arrow_flight::sql::gen::CommandGetImportedKeys`
- `arrow_flight::sql::gen::CommandGetPrimaryKeys`
- `arrow_flight::sql::gen::CommandGetSqlInfo`
- `arrow_flight::sql::gen::CommandGetTableTypes`
- `arrow_flight::sql::gen::CommandGetTables`
- `arrow_flight::sql::gen::CommandGetXdbcTypeInfo`
- `arrow_flight::sql::gen::CommandPreparedStatementQuery`
- `arrow_flight::sql::gen::CommandPreparedStatementUpdate`
- `arrow_flight::sql::gen::CommandStatementIngest`
- `arrow_flight::sql::gen::CommandStatementQuery`
- `arrow_flight::sql::gen::CommandStatementSubstraitPlan`
- `arrow_flight::sql::gen::CommandStatementUpdate`
- `arrow_flight::sql::gen::DoPutPreparedStatementResult`
- `arrow_flight::sql::gen::DoPutUpdateResult`
- `arrow_flight::sql::gen::TicketStatementQuery`

**Methods** (2)

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

ProstMessageExt are useful utility methods for prost::Message types

---
