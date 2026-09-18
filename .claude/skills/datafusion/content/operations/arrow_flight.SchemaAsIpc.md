# `arrow_flight::SchemaAsIpc`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.SchemaAsIpc.json).

<a id="op-4c4da7437618a0a42f1213d0"></a>
## SchemaAsIpc

`struct` · `arrow_flight::SchemaAsIpc` · arrow-flight 59.3.0

```rust
struct SchemaAsIpc<'a>
```

Source: `src/lib.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

SchemaAsIpc represents a pairing of a `Schema` with IpcWriteOptions

<a id="op-51353c5f2f4c3c89611e6229"></a>
## Target

`assoc_type` · `arrow_flight::SchemaAsIpc::Target` · arrow-flight 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [180, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/lib.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-637a9beeeba2dbf50d0d3f5d"></a>
## deref

`function` · `arrow_flight::SchemaAsIpc::deref` · arrow-flight 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 1], "end": [180, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/lib.rs:177`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a9dfe7a4d6f0bd784c07c37"></a>
## new

`function` · `arrow_flight::SchemaAsIpc::new` · arrow-flight 59.3.0

```rust
fn new(schema: &'a Schema, options: &'a IpcWriteOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_flight::SchemaAsIpc", "path": "SchemaAsIpc"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 1], "end": [710, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:705`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new `SchemaAsIpc` from a `Schema` and `IpcWriteOptions`

<a id="op-bded19bd23febbb5411fe300"></a>
## pair

`struct_field` · `arrow_flight::SchemaAsIpc::pair` · arrow-flight 59.3.0

```rust
pair: (&'a arrow_schema::Schema, &'a arrow_ipc::writer::IpcWriteOptions)
```

Source: `src/lib.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Data type representing a schema and its IPC write options
