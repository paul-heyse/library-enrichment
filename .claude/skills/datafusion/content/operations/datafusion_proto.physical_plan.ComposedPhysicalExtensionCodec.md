# `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.ComposedPhysicalExtensionCodec.json).

<a id="op-0417ea1654066cc6b2bbd5f8"></a>
## ComposedPhysicalExtensionCodec

`struct` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec` · datafusion-proto 55.1.0

```rust
struct ComposedPhysicalExtensionCodec
```

Source: `src/physical_plan/mod.rs:1909`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

A PhysicalExtensionCodec that tries one of multiple inner codecs
until one works

<a id="op-ebd823c24c69b245e794d150"></a>
## fmt

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::fmt` · datafusion-proto 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1908, 10], "end": [1908, 15], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_plan/mod.rs:1908`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d311355d953554878252a6"></a>
## new

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::new` · datafusion-proto 55.1.0

```rust
fn new(codecs: Vec<Arc<dyn PhysicalExtensionCodec>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1913, 1], "end": [1972, 2], "filename": "src/physical_plan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_plan/mod.rs:1916`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac2f58d7e307dd8680181b30"></a>
## try_decode

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], ctx: &TaskContext, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1974, 1], "end": [2013, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:1975`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1aa26c8a4a058f2ce326f871"></a>
## try_decode_udaf

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::try_decode_udaf` · datafusion-proto 55.1.0

```rust
fn try_decode_udaf(&self, name: &str, buf: &[u8]) -> Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1974, 1], "end": [2013, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:2006`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87a4778f7b2f8b5e645b61d1"></a>
## try_decode_udf

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::try_decode_udf` · datafusion-proto 55.1.0

```rust
fn try_decode_udf(&self, name: &str, buf: &[u8]) -> Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1974, 1], "end": [2013, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:1998`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09595f3bf0cb4db8eb2fe580"></a>
## try_encode

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1974, 1], "end": [2013, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:1987`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-072da6c9f6769dedae6f93a5"></a>
## try_encode_udaf

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::try_encode_udaf` · datafusion-proto 55.1.0

```rust
fn try_encode_udaf(&self, node: &AggregateUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1974, 1], "end": [2013, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:2010`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-691833a22f81fe9ddc4446a9"></a>
## try_encode_udf

`function` · `datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec::try_encode_udf` · datafusion-proto 55.1.0

```rust
fn try_encode_udf(&self, node: &ScalarUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto::physical_plan::ComposedPhysicalExtensionCodec", "path": "ComposedPhysicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1974, 1], "end": [2013, 2], "filename": "src/physical_plan/mod.rs"}, "trait": {"args": null, "id": "datafusion_proto::physical_plan::PhysicalExtensionCodec", "path": "PhysicalExtensionCodec"}, "trait_path": "datafusion_proto::physical_plan::PhysicalExtensionCodec"}`

Source: `src/physical_plan/mod.rs:2002`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
