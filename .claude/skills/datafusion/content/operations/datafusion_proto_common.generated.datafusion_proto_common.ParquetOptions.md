# `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.ParquetOptions.json).

<a id="op-68385cd52b5d9249ec6beb84"></a>
## ParquetOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions` · datafusion-proto-common 55.1.0

```rust
struct ParquetOptions
```

Source: `src/generated/prost.rs:803`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e89bde4be1380d975d333515"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "protobuf::ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [895, 1], "end": [938, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetOptions", "path": "ParquetOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:896`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae621141ceaff8ae79e80643"></a>
## allow_single_file_parallelism

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::allow_single_file_parallelism` · datafusion-proto-common 55.1.0

```rust
allow_single_file_parallelism: bool
```

Source: `src/generated/prost.rs:837`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

bool bloom_filter_enabled = 20; // default = false

default = true

<a id="op-e8c3991b1dd3755bb90b2fcf"></a>
## binary_as_string

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::binary_as_string` · datafusion-proto-common 55.1.0

```rust
binary_as_string: bool
```

Source: `src/generated/prost.rs:855`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = false

<a id="op-eccd0b1c4c3f5f3ae361a2e2"></a>
## bloom_filter_fpp_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::bloom_filter_fpp_opt` · datafusion-proto-common 55.1.0

```rust
bloom_filter_fpp_opt: ::core::option::Option<parquet_options::BloomFilterFppOpt>
```

Source: `src/generated/prost.rs:896`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd5078661df948de4ddb8e26"></a>
## bloom_filter_ndv_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::bloom_filter_ndv_opt` · datafusion-proto-common 55.1.0

```rust
bloom_filter_ndv_opt: ::core::option::Option<parquet_options::BloomFilterNdvOpt>
```

Source: `src/generated/prost.rs:898`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e38a22e3bdaaef8975dcdc7b"></a>
## bloom_filter_on_read

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::bloom_filter_on_read` · datafusion-proto-common 55.1.0

```rust
bloom_filter_on_read: bool
```

Source: `src/generated/prost.rs:846`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = true

<a id="op-17fffcdb5d858976ee8c780e"></a>
## bloom_filter_on_write

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::bloom_filter_on_write` · datafusion-proto-common 55.1.0

```rust
bloom_filter_on_write: bool
```

Source: `src/generated/prost.rs:849`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = false

<a id="op-eb4fcc0f88e0487b7fc66f25"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe638b800a24d7c3d0031dd9"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> ParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 10], "end": [802, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ff6c71d26e4d66595ce9c45"></a>
## coerce_int96_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::coerce_int96_opt` · datafusion-proto-common 55.1.0

```rust
coerce_int96_opt: ::core::option::Option<parquet_options::CoerceInt96Opt>
```

Source: `src/generated/prost.rs:900`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7711b57f3d651e0cf2abd619"></a>
## coerce_int96_tz_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::coerce_int96_tz_opt` · datafusion-proto-common 55.1.0

```rust
coerce_int96_tz_opt: ::core::option::Option<parquet_options::CoerceInt96TzOpt>
```

Source: `src/generated/prost.rs:914`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional timezone applied to INT96-coerced timestamps when `coerce_int96`
is set. When `Some`, INT96 columns coerce to
`Timestamp(<coerce_int96>, Some(<tz>))` instead of the default
`Timestamp(<coerce_int96>, None)`. No effect when `coerce_int96` is unset.

<a id="op-bf971ea55ff423bf47202760"></a>
## column_index_truncate_length_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::column_index_truncate_length_opt` · datafusion-proto-common 55.1.0

```rust
column_index_truncate_length_opt: ::core::option::Option<parquet_options::ColumnIndexTruncateLengthOpt>
```

Source: `src/generated/prost.rs:886`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22d39e473058ec1e52e00459"></a>
## compression_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::compression_opt` · datafusion-proto-common 55.1.0

```rust
compression_opt: ::core::option::Option<parquet_options::CompressionOpt>
```

Source: `src/generated/prost.rs:876`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe13e58a6da28f443cc256dd"></a>
## content_defined_chunking

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::content_defined_chunking` · datafusion-proto-common 55.1.0

```rust
content_defined_chunking: ::core::option::Option<ParquetCdcOptions>
```

Source: `src/generated/prost.rs:870`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-639d8d14b49e63f715695e1d"></a>
## created_by

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::created_by` · datafusion-proto-common 55.1.0

```rust
created_by: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:868`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23db0d8d04bed31be519776c"></a>
## data_page_row_count_limit

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::data_page_row_count_limit` · datafusion-proto-common 55.1.0

```rust
data_page_row_count_limit: u64
```

Source: `src/generated/prost.rs:862`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cc9ac8e426fbbff8b8501ce"></a>
## data_pagesize_limit

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::data_pagesize_limit` · datafusion-proto-common 55.1.0

```rust
data_pagesize_limit: u64
```

Source: `src/generated/prost.rs:826`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = 1024 * 1024

<a id="op-7eed042e79267f911c557d51"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c5459a096e0a96f59a6b745"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6652, 1], "end": [7158, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:6654`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70e503e1d942a51df9af594d"></a>
## dictionary_enabled_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::dictionary_enabled_opt` · datafusion-proto-common 55.1.0

```rust
dictionary_enabled_opt: ::core::option::Option<parquet_options::DictionaryEnabledOpt>
```

Source: `src/generated/prost.rs:878`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-526625eafc9637dc56a3bbf8"></a>
## dictionary_page_size_limit

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::dictionary_page_size_limit` · datafusion-proto-common 55.1.0

```rust
dictionary_page_size_limit: u64
```

Source: `src/generated/prost.rs:860`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2b8a0b4467029734029d3a2"></a>
## enable_page_index

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::enable_page_index` · datafusion-proto-common 55.1.0

```rust
enable_page_index: bool
```

Source: `src/generated/prost.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Regular fields

default = true

<a id="op-ab18282cd296471c03a5b312"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2fb3797646280a5dba7d63e"></a>
## encoding_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::encoding_opt` · datafusion-proto-common 55.1.0

```rust
encoding_opt: ::core::option::Option<parquet_options::EncodingOpt>
```

Source: `src/generated/prost.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3880040eb5fab7e016cb87f"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &ParquetOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 17], "end": [802, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ac03e1a8c8fa5b760b7cfc3"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5e3cbad052720bc14d3cfb5"></a>
## force_filter_selections

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::force_filter_selections` · datafusion-proto-common 55.1.0

```rust
force_filter_selections: bool
```

Source: `src/generated/prost.rs:823`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = false

<a id="op-a31caf914e2cad64577dab6b"></a>
## max_in_list_size

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::max_in_list_size` · datafusion-proto-common 55.1.0

```rust
max_in_list_size: u64
```

Source: `src/generated/prost.rs:866`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cf8b69b882f51ec99419e6f"></a>
## max_predicate_cache_size_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::max_predicate_cache_size_opt` · datafusion-proto-common 55.1.0

```rust
max_predicate_cache_size_opt: ::core::option::Option<parquet_options::MaxPredicateCacheSizeOpt>
```

Source: `src/generated/prost.rs:902`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81834116caf643da07017d73"></a>
## max_row_group_bytes_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::max_row_group_bytes_opt` · datafusion-proto-common 55.1.0

```rust
max_row_group_bytes_opt: ::core::option::Option<parquet_options::MaxRowGroupBytesOpt>
```

Source: `src/generated/prost.rs:906`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e5cd4ade82ab49f1466694b"></a>
## max_row_group_size

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::max_row_group_size` · datafusion-proto-common 55.1.0

```rust
max_row_group_size: u64
```

Source: `src/generated/prost.rs:864`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5ab501046d126c0a1aedc4b"></a>
## maximum_buffered_record_batches_per_stream

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::maximum_buffered_record_batches_per_stream` · datafusion-proto-common 55.1.0

```rust
maximum_buffered_record_batches_per_stream: u64
```

Source: `src/generated/prost.rs:843`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = 2

<a id="op-88e1e70b082724ab4ae0ef56"></a>
## maximum_parallel_row_group_writers

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::maximum_parallel_row_group_writers` · datafusion-proto-common 55.1.0

```rust
maximum_parallel_row_group_writers: u64
```

Source: `src/generated/prost.rs:840`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = 1

<a id="op-de6d6b7b9cfe3439ea052614"></a>
## metadata_size_hint_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::metadata_size_hint_opt` · datafusion-proto-common 55.1.0

```rust
metadata_size_hint_opt: ::core::option::Option<parquet_options::MetadataSizeHintOpt>
```

Source: `src/generated/prost.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e30df0d249d9263c77b9e75a"></a>
## pruning

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::pruning` · datafusion-proto-common 55.1.0

```rust
pruning: bool
```

Source: `src/generated/prost.rs:811`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = true

<a id="op-6133530d93f0b8e81bcecd6e"></a>
## pushdown_filters

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::pushdown_filters` · datafusion-proto-common 55.1.0

```rust
pushdown_filters: bool
```

Source: `src/generated/prost.rs:817`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = false

<a id="op-1dd99c926ccb5010596b5712"></a>
## reorder_filters

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::reorder_filters` · datafusion-proto-common 55.1.0

```rust
reorder_filters: bool
```

Source: `src/generated/prost.rs:820`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = false

<a id="op-3ab87faaa32ed982298e44cd"></a>
## schema_force_view_types

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::schema_force_view_types` · datafusion-proto-common 55.1.0

```rust
schema_force_view_types: bool
```

Source: `src/generated/prost.rs:852`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = false

<a id="op-959a99bb32bf6134048d8b03"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6344, 1], "end": [6651, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:6346`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6f7100a424279e47b1fb897"></a>
## skip_arrow_metadata

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::skip_arrow_metadata` · datafusion-proto-common 55.1.0

```rust
skip_arrow_metadata: bool
```

Source: `src/generated/prost.rs:858`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = false

<a id="op-33e67bc0cdd294a32b72df9d"></a>
## skip_metadata

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::skip_metadata` · datafusion-proto-common 55.1.0

```rust
skip_metadata: bool
```

Source: `src/generated/prost.rs:814`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = true

<a id="op-f5c4b63c44b454f9a1e4e0bc"></a>
## statistics_enabled_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::statistics_enabled_opt` · datafusion-proto-common 55.1.0

```rust
statistics_enabled_opt: ::core::option::Option<parquet_options::StatisticsEnabledOpt>
```

Source: `src/generated/prost.rs:882`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ab22af477da2881e64ae59d"></a>
## statistics_truncate_length_opt

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::statistics_truncate_length_opt` · datafusion-proto-common 55.1.0

```rust
statistics_truncate_length_opt: ::core::option::Option<parquet_options::StatisticsTruncateLengthOpt>
```

Source: `src/generated/prost.rs:890`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5b21ac53434e366ec746fca"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(value: &ParquetOptions) -> datafusion_common::Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions", "path": "protobuf::ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [895, 1], "end": [938, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetOptions", "path": "ParquetOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:898`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d352eb5bfc37af026a9625e8"></a>
## write_batch_size

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::write_batch_size` · datafusion-proto-common 55.1.0

```rust
write_batch_size: u64
```

Source: `src/generated/prost.rs:829`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = 1024

<a id="op-8e27a061299a85c025eb8877"></a>
## writer_version

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions::writer_version` · datafusion-proto-common 55.1.0

```rust
writer_version: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:832`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

default = "1.0"
