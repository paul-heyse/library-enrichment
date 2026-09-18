# `arrow_ipc::gen::Schema::FieldArgs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.FieldArgs.json).

<a id="op-f38e329286a51d76b998ae4a"></a>
## FieldArgs

`struct` · `arrow_ipc::gen::Schema::FieldArgs` · arrow-ipc 59.3.0

```rust
struct FieldArgs<'a>
```

Source: `src/gen/Schema.rs:4964`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cd78dc6335911b1f8cb955a"></a>
## children

`struct_field` · `arrow_ipc::gen::Schema::FieldArgs::children` · arrow-ipc 59.3.0

```rust
children: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Field<'a>>>>>
```

Source: `src/gen/Schema.rs:4970`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f88ffadc3f37068b6a35f1a"></a>
## custom_metadata

`struct_field` · `arrow_ipc::gen::Schema::FieldArgs::custom_metadata` · arrow-ipc 59.3.0

```rust
custom_metadata: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>
```

Source: `src/gen/Schema.rs:4973`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96522f376e7d9de59b0b82ba"></a>
## default

`function` · `arrow_ipc::gen::Schema::FieldArgs::default` · arrow-ipc 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::FieldArgs", "path": "FieldArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4977, 1], "end": [4990, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/gen/Schema.rs:4979`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5f6f2eab23ff3bbf127b343"></a>
## dictionary

`struct_field` · `arrow_ipc::gen::Schema::FieldArgs::dictionary` · arrow-ipc 59.3.0

```rust
dictionary: Option<flatbuffers::WIPOffset<DictionaryEncoding<'a>>>
```

Source: `src/gen/Schema.rs:4969`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5acb96b421640f97de9f296b"></a>
## name

`struct_field` · `arrow_ipc::gen::Schema::FieldArgs::name` · arrow-ipc 59.3.0

```rust
name: Option<flatbuffers::WIPOffset<&'a str>>
```

Source: `src/gen/Schema.rs:4965`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b636bad53ecfcc728725a82"></a>
## nullable

`struct_field` · `arrow_ipc::gen::Schema::FieldArgs::nullable` · arrow-ipc 59.3.0

```rust
nullable: bool
```

Source: `src/gen/Schema.rs:4966`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a39481641a2b8655347ac7f"></a>
## type_

`struct_field` · `arrow_ipc::gen::Schema::FieldArgs::type_` · arrow-ipc 59.3.0

```rust
type_: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>
```

Source: `src/gen/Schema.rs:4968`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f100e7b087a96306925d8c5e"></a>
## type_type

`struct_field` · `arrow_ipc::gen::Schema::FieldArgs::type_type` · arrow-ipc 59.3.0

```rust
type_type: Type
```

Source: `src/gen/Schema.rs:4967`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
