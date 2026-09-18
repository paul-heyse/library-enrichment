# `datafusion_proto_models::generated::datafusion::ParquetOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.ParquetOptions.json).

<a id="op-92d85dc85516d3a0a4c574f4"></a>
## ParquetOptions

`struct` · `datafusion_proto_models::generated::datafusion::ParquetOptions` · datafusion-proto-models 55.1.0

```rust
struct ParquetOptions
```

Source: `src/generated/datafusion_proto_common.rs:803`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bb6a7122a71046c5966049b"></a>
## allow_single_file_parallelism

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::allow_single_file_parallelism` · datafusion-proto-models 55.1.0

```rust
allow_single_file_parallelism: bool
```

Source: `src/generated/datafusion_proto_common.rs:837`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

bool bloom_filter_enabled = 20; // default = false

default = true

<a id="op-ffe604d2f246226e7877b304"></a>
## binary_as_string

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::binary_as_string` · datafusion-proto-models 55.1.0

```rust
binary_as_string: bool
```

Source: `src/generated/datafusion_proto_common.rs:855`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = false

<a id="op-bec163968fd3321a6f72c173"></a>
## bloom_filter_fpp_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::bloom_filter_fpp_opt` · datafusion-proto-models 55.1.0

```rust
bloom_filter_fpp_opt: ::core::option::Option<parquet_options::BloomFilterFppOpt>
```

Source: `src/generated/datafusion_proto_common.rs:896`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30bc6120246ee44961ff88c7"></a>
## bloom_filter_ndv_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::bloom_filter_ndv_opt` · datafusion-proto-models 55.1.0

```rust
bloom_filter_ndv_opt: ::core::option::Option<parquet_options::BloomFilterNdvOpt>
```

Source: `src/generated/datafusion_proto_common.rs:898`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa87eff206eb736122a2732f"></a>
## bloom_filter_on_read

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::bloom_filter_on_read` · datafusion-proto-models 55.1.0

```rust
bloom_filter_on_read: bool
```

Source: `src/generated/datafusion_proto_common.rs:846`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = true

<a id="op-1dc8efa50d4f2392e46d3ca3"></a>
## bloom_filter_on_write

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::bloom_filter_on_write` · datafusion-proto-models 55.1.0

```rust
bloom_filter_on_write: bool
```

Source: `src/generated/datafusion_proto_common.rs:849`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = false

<a id="op-5df4ed91f2a33135e8e25f07"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::ParquetOptions::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d8fc8e8b3425782e1991a10"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::ParquetOptions::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> ParquetOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 10], "end": [802, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cacb279cd7d59d4b766fa8a7"></a>
## coerce_int96_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::coerce_int96_opt` · datafusion-proto-models 55.1.0

```rust
coerce_int96_opt: ::core::option::Option<parquet_options::CoerceInt96Opt>
```

Source: `src/generated/datafusion_proto_common.rs:900`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a736b1126732d746d9a49be8"></a>
## coerce_int96_tz_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::coerce_int96_tz_opt` · datafusion-proto-models 55.1.0

```rust
coerce_int96_tz_opt: ::core::option::Option<parquet_options::CoerceInt96TzOpt>
```

Source: `src/generated/datafusion_proto_common.rs:914`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional timezone applied to INT96-coerced timestamps when `coerce_int96`
is set. When `Some`, INT96 columns coerce to
`Timestamp(<coerce_int96>, Some(<tz>))` instead of the default
`Timestamp(<coerce_int96>, None)`. No effect when `coerce_int96` is unset.

<a id="op-326f4b7f9dff33f1370b19c7"></a>
## column_index_truncate_length_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::column_index_truncate_length_opt` · datafusion-proto-models 55.1.0

```rust
column_index_truncate_length_opt: ::core::option::Option<parquet_options::ColumnIndexTruncateLengthOpt>
```

Source: `src/generated/datafusion_proto_common.rs:886`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-495cbc80ae7dc0f3f2640366"></a>
## compression_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::compression_opt` · datafusion-proto-models 55.1.0

```rust
compression_opt: ::core::option::Option<parquet_options::CompressionOpt>
```

Source: `src/generated/datafusion_proto_common.rs:876`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b55f3ec4146cad47873be178"></a>
## content_defined_chunking

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::content_defined_chunking` · datafusion-proto-models 55.1.0

```rust
content_defined_chunking: ::core::option::Option<ParquetCdcOptions>
```

Source: `src/generated/datafusion_proto_common.rs:870`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3be846d525585022094cec0e"></a>
## created_by

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::created_by` · datafusion-proto-models 55.1.0

```rust
created_by: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:868`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5419322e50aef43f275f9524"></a>
## data_page_row_count_limit

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::data_page_row_count_limit` · datafusion-proto-models 55.1.0

```rust
data_page_row_count_limit: u64
```

Source: `src/generated/datafusion_proto_common.rs:862`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca9d40417f70756185309a21"></a>
## data_pagesize_limit

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::data_pagesize_limit` · datafusion-proto-models 55.1.0

```rust
data_pagesize_limit: u64
```

Source: `src/generated/datafusion_proto_common.rs:826`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = 1024 * 1024

<a id="op-96582d317bdfb4e372a51753"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::ParquetOptions::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-486c256ea56e75c2b723f280"></a>
## dictionary_enabled_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::dictionary_enabled_opt` · datafusion-proto-models 55.1.0

```rust
dictionary_enabled_opt: ::core::option::Option<parquet_options::DictionaryEnabledOpt>
```

Source: `src/generated/datafusion_proto_common.rs:878`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ffceb4014f7f2cb6a81eecc"></a>
## dictionary_page_size_limit

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::dictionary_page_size_limit` · datafusion-proto-models 55.1.0

```rust
dictionary_page_size_limit: u64
```

Source: `src/generated/datafusion_proto_common.rs:860`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-957b1f94b494d657e050aed6"></a>
## enable_page_index

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::enable_page_index` · datafusion-proto-models 55.1.0

```rust
enable_page_index: bool
```

Source: `src/generated/datafusion_proto_common.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Regular fields

default = true

<a id="op-2973e24fd3862f6c0a5cbd16"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::ParquetOptions::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82ecb28d1d91025b5b3f9cc6"></a>
## encoding_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::encoding_opt` · datafusion-proto-models 55.1.0

```rust
encoding_opt: ::core::option::Option<parquet_options::EncodingOpt>
```

Source: `src/generated/datafusion_proto_common.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b7d9113a3bc99f98e5a5ad7"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::ParquetOptions::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &ParquetOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 17], "end": [802, 26], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3e8f3881120745d99e1c6eb"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::ParquetOptions::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::ParquetOptions", "path": "ParquetOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [802, 28], "end": [802, 44], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:802`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77beda5b226342b4d40df1c7"></a>
## force_filter_selections

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::force_filter_selections` · datafusion-proto-models 55.1.0

```rust
force_filter_selections: bool
```

Source: `src/generated/datafusion_proto_common.rs:823`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = false

<a id="op-224a8d2189cc0371b173ddfa"></a>
## max_in_list_size

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::max_in_list_size` · datafusion-proto-models 55.1.0

```rust
max_in_list_size: u64
```

Source: `src/generated/datafusion_proto_common.rs:866`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17cdb3ddaf35d28eb7c2ccb7"></a>
## max_predicate_cache_size_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::max_predicate_cache_size_opt` · datafusion-proto-models 55.1.0

```rust
max_predicate_cache_size_opt: ::core::option::Option<parquet_options::MaxPredicateCacheSizeOpt>
```

Source: `src/generated/datafusion_proto_common.rs:902`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f7ebddd98bbcae26cad0b8c"></a>
## max_row_group_bytes_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::max_row_group_bytes_opt` · datafusion-proto-models 55.1.0

```rust
max_row_group_bytes_opt: ::core::option::Option<parquet_options::MaxRowGroupBytesOpt>
```

Source: `src/generated/datafusion_proto_common.rs:906`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c56d515bf533d361d9a1d40"></a>
## max_row_group_size

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::max_row_group_size` · datafusion-proto-models 55.1.0

```rust
max_row_group_size: u64
```

Source: `src/generated/datafusion_proto_common.rs:864`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-335804563fd2d3b91ee3f129"></a>
## maximum_buffered_record_batches_per_stream

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::maximum_buffered_record_batches_per_stream` · datafusion-proto-models 55.1.0

```rust
maximum_buffered_record_batches_per_stream: u64
```

Source: `src/generated/datafusion_proto_common.rs:843`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = 2

<a id="op-22a890f49d067b37e7c3f1b5"></a>
## maximum_parallel_row_group_writers

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::maximum_parallel_row_group_writers` · datafusion-proto-models 55.1.0

```rust
maximum_parallel_row_group_writers: u64
```

Source: `src/generated/datafusion_proto_common.rs:840`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = 1

<a id="op-0f95ca37a5e388f3b30864b3"></a>
## metadata_size_hint_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::metadata_size_hint_opt` · datafusion-proto-models 55.1.0

```rust
metadata_size_hint_opt: ::core::option::Option<parquet_options::MetadataSizeHintOpt>
```

Source: `src/generated/datafusion_proto_common.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8229965d20a0468bde13995d"></a>
## pruning

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::pruning` · datafusion-proto-models 55.1.0

```rust
pruning: bool
```

Source: `src/generated/datafusion_proto_common.rs:811`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = true

<a id="op-81f86859d10700ae5c71d34e"></a>
## pushdown_filters

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::pushdown_filters` · datafusion-proto-models 55.1.0

```rust
pushdown_filters: bool
```

Source: `src/generated/datafusion_proto_common.rs:817`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = false

<a id="op-91619c58486f732a9db1548d"></a>
## reorder_filters

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::reorder_filters` · datafusion-proto-models 55.1.0

```rust
reorder_filters: bool
```

Source: `src/generated/datafusion_proto_common.rs:820`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = false

<a id="op-cef717761640c02ddcf5ace6"></a>
## schema_force_view_types

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::schema_force_view_types` · datafusion-proto-models 55.1.0

```rust
schema_force_view_types: bool
```

Source: `src/generated/datafusion_proto_common.rs:852`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = false

<a id="op-77fa377eea3b2e13c7bd446d"></a>
## skip_arrow_metadata

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::skip_arrow_metadata` · datafusion-proto-models 55.1.0

```rust
skip_arrow_metadata: bool
```

Source: `src/generated/datafusion_proto_common.rs:858`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = false

<a id="op-80234d148e79c9e7481c994b"></a>
## skip_metadata

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::skip_metadata` · datafusion-proto-models 55.1.0

```rust
skip_metadata: bool
```

Source: `src/generated/datafusion_proto_common.rs:814`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = true

<a id="op-b5a9f97304a2c1628f54d385"></a>
## statistics_enabled_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::statistics_enabled_opt` · datafusion-proto-models 55.1.0

```rust
statistics_enabled_opt: ::core::option::Option<parquet_options::StatisticsEnabledOpt>
```

Source: `src/generated/datafusion_proto_common.rs:882`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd3e03a93cfb44775d231e36"></a>
## statistics_truncate_length_opt

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::statistics_truncate_length_opt` · datafusion-proto-models 55.1.0

```rust
statistics_truncate_length_opt: ::core::option::Option<parquet_options::StatisticsTruncateLengthOpt>
```

Source: `src/generated/datafusion_proto_common.rs:890`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d25f27583e9b6c7a5584517d"></a>
## write_batch_size

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::write_batch_size` · datafusion-proto-models 55.1.0

```rust
write_batch_size: u64
```

Source: `src/generated/datafusion_proto_common.rs:829`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = 1024

<a id="op-93ac848477481e71c454e901"></a>
## writer_version

`struct_field` · `datafusion_proto_models::generated::datafusion::ParquetOptions::writer_version` · datafusion-proto-models 55.1.0

```rust
writer_version: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:832`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

default = "1.0"
