# `arrow_flight::sql::gen::SqlSupportedTransaction`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlSupportedTransaction.json).

<a id="op-098b722731dd5e64c199c712"></a>
## SqlSupportedTransaction

`enum` · `arrow_flight::sql::gen::SqlSupportedTransaction` · arrow-flight 59.3.0

```rust
enum SqlSupportedTransaction
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1931`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

The level of support for Flight SQL transaction RPCs.

<a id="op-43c85863aca2ea4e455fbb46"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlSupportedTransaction::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 68], "end": [1929, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-217ba6650f66c33f0a1fe58b"></a>
## None

`variant` · `arrow_flight::sql::gen::SqlSupportedTransaction::None` · arrow-flight 59.3.0

```rust
None
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1933`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Unknown/not indicated/no support

<a id="op-f52981b51e832d4e175266b9"></a>
## Savepoint

`variant` · `arrow_flight::sql::gen::SqlSupportedTransaction::Savepoint` · arrow-flight 59.3.0

```rust
Savepoint
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1939`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Transactions and savepoints

<a id="op-bc88c30a09069af800600c94"></a>
## Transaction

`variant` · `arrow_flight::sql::gen::SqlSupportedTransaction::Transaction` · arrow-flight 59.3.0

```rust
Transaction
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1937`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Transactions, but not savepoints.
A savepoint is a mark within a transaction that can be individually
rolled back to. Not all databases support savepoints.

<a id="op-4a79c4cb7c45290d99f1e438"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1941, 1], "end": [1962, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1946`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-a491baf9410bcd4709e2d15f"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlSupportedTransaction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 10], "end": [1929, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4a97bb82e32857f33d5e305"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlSupportedTransaction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 63], "end": [1929, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afdd173e06063118ce7f189e"></a>
## default

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlSupportedTransaction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 68], "end": [1929, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3223d9941c97296c3c6b795"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlSupportedTransaction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 30], "end": [1929, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-999c4f46fd7e2815680fa183"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 23], "end": [1929, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34875dbac235ac985ce278e0"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedTransaction>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 68], "end": [1929, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlSupportedTransaction`, or `None` if `value` is not a valid variant.

<a id="op-9e3e3c301159c7b8ba3ec28f"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1941, 1], "end": [1962, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1954`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-5004b199d85a7c9904baebd8"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 45], "end": [1929, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5773f66766d721eeecdf5895"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 68], "end": [1929, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlSupportedTransaction`.

<a id="op-fe6122576ff4484813c7e81e"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlSupportedTransaction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 51], "end": [1929, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-588c7aaeb61fd7d4b4cb6170"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlSupportedTransaction::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedTransaction, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlSupportedTransaction", "path": "SqlSupportedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1929, 68], "end": [1929, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1929`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
