# `arrow_flight::sql::gen::action_end_transaction_request`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.sql.gen.action_end_transaction_request.json`](../model/arrow_flight.sql.gen.action_end_transaction_request.json)

## EndTransaction

`enum` · `arrow_flight::sql::gen::action_end_transaction_request::EndTransaction`

Also reachable as `arrow_flight::sql::EndTransaction`

```rust
enum EndTransaction
```

**Variants**: `Unspecified`, `Commit`, `Rollback`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<EndTransaction>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<EndTransaction, ::prost::UnknownEnumValue>
```

---
