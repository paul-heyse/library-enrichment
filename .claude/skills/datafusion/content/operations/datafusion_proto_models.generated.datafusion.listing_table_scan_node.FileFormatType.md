# `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.listing_table_scan_node.FileFormatType.json).

<a id="op-69623b9fa625b39c877a2ab8"></a>
## FileFormatType

`enum` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType` · datafusion-proto-models 55.1.0

```rust
enum FileFormatType
```

Source: `src/generated/prost.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-673f31bf95565af6d53eea42"></a>
## Arrow

`variant` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::Arrow` · datafusion-proto-models 55.1.0

```rust
Arrow
```

Source: `src/generated/prost.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48961d374a83d7916e0e7cde"></a>
## Avro

`variant` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::Avro` · datafusion-proto-models 55.1.0

```rust
Avro
```

Source: `src/generated/prost.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af387250795846d383a5435e"></a>
## Csv

`variant` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::Csv` · datafusion-proto-models 55.1.0

```rust
Csv
```

Source: `src/generated/prost.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cb38c5281e7d5a5d17e4ed5"></a>
## Json

`variant` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::Json` · datafusion-proto-models 55.1.0

```rust
Json
```

Source: `src/generated/prost.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-724cfa70f4011d83882fba54"></a>
## Parquet

`variant` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::Parquet` · datafusion-proto-models 55.1.0

```rust
Parquet
```

Source: `src/generated/prost.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0149f93fe1a33127d309a164"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> FileFormatType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType", "path": "FileFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 14], "end": [134, 19], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5d8532180ab47329ca75a30"></a>
## encode

`function` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::encode` · datafusion-proto-models 55.1.0

```rust
fn encode(&self, buf: &mut impl ::prost::bytes::BufMut)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType", "path": "FileFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 32], "end": [134, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Encodes the message to a buffer.

<a id="op-57bc1c0d9712121d89bedb80"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType", "path": "FileFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 32], "end": [134, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the encoded length of the message without a length delimiter.

<a id="op-0eb3a34867ec2792ea408ffe"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &FileFormatType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType", "path": "FileFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 21], "end": [134, 30], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bac265aded16554d25fe3436"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType", "path": "FileFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 32], "end": [134, 46], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4470f257ce8e400bdc327c3a"></a>
## merge

`function` · `datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType::merge` · datafusion-proto-models 55.1.0

```rust
fn merge(field: &mut ::core::option::Option<FileFormatType>, tag: u32, wire_type: ::prost::encoding::wire_type::WireType, buf: &mut impl ::prost::bytes::Buf, ctx: ::prost::encoding::DecodeContext) -> ::core::result::Result<(), ::prost::DecodeError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::listing_table_scan_node::FileFormatType", "path": "FileFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 32], "end": [134, 46], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Decodes an instance of the message from a buffer, and merges it into self.
