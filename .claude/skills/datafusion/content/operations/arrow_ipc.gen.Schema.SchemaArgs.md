# `arrow_ipc::gen::Schema::SchemaArgs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.SchemaArgs.json).

<a id="op-1b0af22504f2138321d643b2"></a>
## SchemaArgs

`struct` · `arrow_ipc::gen::Schema::SchemaArgs` · arrow-ipc 59.3.0

```rust
struct SchemaArgs<'a>
```

Source: `src/gen/Schema.rs:5468`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdc2c03589687cf644138957"></a>
## custom_metadata

`struct_field` · `arrow_ipc::gen::Schema::SchemaArgs::custom_metadata` · arrow-ipc 59.3.0

```rust
custom_metadata: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>
```

Source: `src/gen/Schema.rs:5473`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e80a6e15feeb9f347146abdd"></a>
## default

`function` · `arrow_ipc::gen::Schema::SchemaArgs::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::SchemaArgs", "path": "SchemaArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5478, 1], "end": [5488, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Schema.rs:5480`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4ab264aa3726220f501ab3f"></a>
## endianness

`struct_field` · `arrow_ipc::gen::Schema::SchemaArgs::endianness` · arrow-ipc 59.3.0

```rust
endianness: Endianness
```

Source: `src/gen/Schema.rs:5469`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-744436353c6a7a2bae3fb6e9"></a>
## features

`struct_field` · `arrow_ipc::gen::Schema::SchemaArgs::features` · arrow-ipc 59.3.0

```rust
features: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Feature>>>
```

Source: `src/gen/Schema.rs:5476`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b08c223e4aea0db75055e38"></a>
## fields

`struct_field` · `arrow_ipc::gen::Schema::SchemaArgs::fields` · arrow-ipc 59.3.0

```rust
fields: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Field<'a>>>>>
```

Source: `src/gen/Schema.rs:5470`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
