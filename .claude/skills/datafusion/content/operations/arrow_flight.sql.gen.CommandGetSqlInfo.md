# `arrow_flight::sql::gen::CommandGetSqlInfo`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.CommandGetSqlInfo.json).

<a id="op-5df287408ae3ccad4071070f"></a>
## CommandGetSqlInfo

`struct` · `arrow_flight::sql::gen::CommandGetSqlInfo` · arrow-flight 59.3.0

```rust
struct CommandGetSqlInfo
```

Source: `src/sql/arrow.flight.protocol.sql.rs:23`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Represents a metadata request. Used in the command member of FlightDescriptor
for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the metadata request.

The returned Arrow schema will be:
<
  info_name: uint32 not null,
  value: dense_union<
              string_value: utf8,
              bool_value: bool,
              bigint_value: int64,
              int32_bitmask: int32,
              string_list: list<string_data: utf8>
              int32_to_int32_list_map: map<key: int32, value: list<$data$: int32>>
>
where there is one row per requested piece of metadata information.

<a id="op-d67274ab0d1cc5047381e9b8"></a>
## as_any

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::as_any` · arrow-flight 59.3.0

```rust
fn as_any(&self) -> Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdf18dd5a328e198bbcb937d"></a>
## clear

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e823061438bd7984ac7a626f"></a>
## clone

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> CommandGetSqlInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49875090213bd99b5e44d31e"></a>
## default

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f4fc4cf570cefe1fbe3b6a9"></a>
## encoded_len

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11871aca0bdb4ac424da2e44"></a>
## eq

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &CommandGetSqlInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 17], "end": [22, 26], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bf8c7224a671899c2c1f89c"></a>
## fmt

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2ba0de399ff8fcb20699eed"></a>
## hash

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 32], "end": [22, 36], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e44bb4205544ce7c6338ea3f"></a>
## info

`struct_field` · `arrow_flight::sql::gen::CommandGetSqlInfo::info` · arrow-flight 59.3.0

```rust
info: ::prost::alloc::vec::Vec<u32>
```

Source: `src/sql/arrow.flight.protocol.sql.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Values are modelled after ODBC's SQLGetInfo() function. This information is intended to provide
Flight SQL clients with basic, SQL syntax and SQL functions related information.
More information types can be added in future releases.
E.g. more SQL syntax support types, scalar functions support, type conversion support etc.

Note that the set of metadata may expand.

Initially, Flight SQL will support the following information types:
- Server Information - Range [0-500)
- Syntax Information - Range [500-1000)
Range [0-10,000) is reserved for defaults (see SqlInfo enum for default options).
Custom options should start at 10,000.

If omitted, then all metadata will be retrieved.
Flight SQL Servers may choose to include additional metadata above and beyond the specified set, however they must
at least return the specified set. IDs ranging from 0 to 10,000 (exclusive) are reserved for future use.
If additional metadata is included, the metadata IDs should start from 10,000.

<a id="op-362488b54e8bde5cc190613a"></a>
## into_builder

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::into_builder` · arrow-flight 59.3.0

```rust
fn into_builder(self, infos: &SqlInfoData) -> GetSqlInfoBuilder<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "crate::sql::CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 1], "end": [450, 2], "filename": "src/sql/metadata/sql_info.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/metadata/sql_info.rs:444`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a builder suitable for constructing a response

<a id="op-dafac7ccc8e83d9ba9ebda3a"></a>
## type_url

`function` · `arrow_flight::sql::gen::CommandGetSqlInfo::type_url` · arrow-flight 59.3.0

```rust
fn type_url() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::CommandGetSqlInfo", "path": "CommandGetSqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [251, 2], "filename": "src/sql/mod.rs"}, "trait": {"args": null, "id": "arrow_flight::sql::ProstMessageExt", "path": "ProstMessageExt"}, "trait_path": "arrow_flight::sql::ProstMessageExt"}`

Source: `src/sql/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
