# `arrow_ipc::gen::File::FooterBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.File.FooterBuilder.json).

<a id="op-93e77534f0a5c8bdf4bd78c6"></a>
## FooterBuilder

`struct` · `arrow_ipc::gen::File::FooterBuilder` · arrow-ipc 59.3.0

```rust
struct FooterBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

Source: `src/gen/File.rs:358`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b44a4879c4103df210177581"></a>
## add_custom_metadata

`function` · `arrow_ipc::gen::File::FooterBuilder::add_custom_metadata` · arrow-ipc 59.3.0

```rust
fn add_custom_metadata(&mut self, custom_metadata: flatbuffers::WIPOffset<flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<KeyValue<'b>>>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::File::FooterBuilder", "path": "FooterBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [414, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2d044ef4d8bdc84f0e1244b"></a>
## add_dictionaries

`function` · `arrow_ipc::gen::File::FooterBuilder::add_dictionaries` · arrow-ipc 59.3.0

```rust
fn add_dictionaries(&mut self, dictionaries: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Block>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::File::FooterBuilder", "path": "FooterBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [414, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:374`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28d2987a0031f13dbcb18a8a"></a>
## add_recordBatches

`function` · `arrow_ipc::gen::File::FooterBuilder::add_recordBatches` · arrow-ipc 59.3.0

```rust
fn add_recordBatches(&mut self, recordBatches: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Block>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::File::FooterBuilder", "path": "FooterBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [414, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:382`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c85a756571b525d5fb3867f"></a>
## add_schema

`function` · `arrow_ipc::gen::File::FooterBuilder::add_schema` · arrow-ipc 59.3.0

```rust
fn add_schema(&mut self, schema: flatbuffers::WIPOffset<Schema<'b>>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::File::FooterBuilder", "path": "FooterBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [414, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:369`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbc1651fb5679be8f3238348"></a>
## add_version

`function` · `arrow_ipc::gen::File::FooterBuilder::add_version` · arrow-ipc 59.3.0

```rust
fn add_version(&mut self, version: MetadataVersion)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::File::FooterBuilder", "path": "FooterBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [414, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:364`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93b7bb4985a03168099cd885"></a>
## finish

`function` · `arrow_ipc::gen::File::FooterBuilder::finish` · arrow-ipc 59.3.0

```rust
fn finish(self) -> flatbuffers::WIPOffset<Footer<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::File::FooterBuilder", "path": "FooterBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [414, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:410`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3c85fd3262bdf554ede7f24"></a>
## new

`function` · `arrow_ipc::gen::File::FooterBuilder::new` · arrow-ipc 59.3.0

```rust
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> FooterBuilder<'a, 'b, A>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}, {"type": {"generic": "A"}}], "constraints": []}}, "id": "arrow_ipc::gen::File::FooterBuilder", "path": "FooterBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": ["'b"]}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "flatbuffers::builder::Allocator", "path": "flatbuffers::Allocator"}}}, {"outlives": "'a"}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 1], "end": [414, 2], "filename": "src/gen/File.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/File.rs:402`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
