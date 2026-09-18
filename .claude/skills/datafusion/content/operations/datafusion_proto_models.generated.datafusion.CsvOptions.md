# `datafusion_proto_models::generated::datafusion::CsvOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.CsvOptions.json).

<a id="op-9aa44826fb9c6696ed4356af"></a>
## CsvOptions

`struct` · `datafusion_proto_models::generated::datafusion::CsvOptions` · datafusion-proto-models 55.1.0

```rust
struct CsvOptions
```

Source: `src/generated/datafusion_proto_common.rs:634`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Options controlling CSV format

<a id="op-a13a411e1364b3500b9267c9"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a62d30c26f0aa16ccb790548"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> CsvOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 10], "end": [633, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c43aca81f25df8930ff7b80a"></a>
## comment

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::comment` · datafusion-proto-models 55.1.0

```rust
comment: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:676`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional comment character as a byte

<a id="op-c0d6bfbeb8a7e8dc9cf8dc36"></a>
## compression

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::compression` · datafusion-proto-models 55.1.0

```rust
compression: i32
```

Source: `src/generated/datafusion_proto_common.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Compression type

<a id="op-fb9f2270934781f3ca2dea79"></a>
## compression

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::compression` · datafusion-proto-models 55.1.0

```rust
fn compression(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `compression`, or the default if the field is set to an invalid enum value.

<a id="op-ac2f4d0587a8ff303ebf3fd1"></a>
## compression_level

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::compression_level` · datafusion-proto-models 55.1.0

```rust
compression_level: ::core::option::Option<u32>
```

Source: `src/generated/datafusion_proto_common.rs:691`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional compression level

<a id="op-b9a0a7995afdd127fb5f56ef"></a>
## compression_level

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::compression_level` · datafusion-proto-models 55.1.0

```rust
fn compression_level(&self) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the value of `compression_level`, or the default value if `compression_level` is unset.

<a id="op-6497da3ee39d4ce1836bcb04"></a>
## date_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::date_format` · datafusion-proto-models 55.1.0

```rust
date_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional date format

<a id="op-166287941dcecf9606769ab9"></a>
## datetime_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::datetime_format` · datafusion-proto-models 55.1.0

```rust
datetime_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional datetime format

<a id="op-c1ef84687048d5fa7daa884f"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d7e57d6e79f934c7f2a2f89"></a>
## delimiter

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::delimiter` · datafusion-proto-models 55.1.0

```rust
delimiter: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:640`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Delimiter character as a byte

<a id="op-48833415ad683b825764352c"></a>
## double_quote

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::double_quote` · datafusion-proto-models 55.1.0

```rust
double_quote: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Indicates if quotes are doubled

<a id="op-b9f8a5f8ff9dfa3c1d270d50"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74afb4c9364edfef78238fa6"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &CsvOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 17], "end": [633, 26], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e55e1e30b729b883b3c4a1e5"></a>
## escape

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::escape` · datafusion-proto-models 55.1.0

```rust
escape: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:646`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional escape character as a byte

<a id="op-110de2fcfb8f22c3b174c550"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e8ba4a35a85b88c21178004"></a>
## has_header

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::has_header` · datafusion-proto-models 55.1.0

```rust
has_header: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:637`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Indicates if the CSV has a header row

<a id="op-dfad6267ac01c9a966aa0de4"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 32], "end": [633, 36], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0701526e2c18daf269f7184"></a>
## ignore_leading_whitespace

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::ignore_leading_whitespace` · datafusion-proto-models 55.1.0

```rust
ignore_leading_whitespace: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Whether to ignore leading whitespace in string values

<a id="op-62b5bd20c21788981942fa06"></a>
## ignore_trailing_whitespace

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::ignore_trailing_whitespace` · datafusion-proto-models 55.1.0

```rust
ignore_trailing_whitespace: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:700`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Whether to ignore trailing whitespace in string values

<a id="op-57e35c679ca1e2811368f6df"></a>
## newlines_in_values

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::newlines_in_values` · datafusion-proto-models 55.1.0

```rust
newlines_in_values: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:682`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Indicates if newlines are supported in values

<a id="op-4587c5dc2e3ed1e847f06b9f"></a>
## null_regex

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::null_regex` · datafusion-proto-models 55.1.0

```rust
null_regex: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:673`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional representation of null loading regex

<a id="op-28f137054ab954ea1647c700"></a>
## null_value

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::null_value` · datafusion-proto-models 55.1.0

```rust
null_value: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:670`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional representation of null value

<a id="op-23bfdc242f7d82b11b023f11"></a>
## quote

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::quote` · datafusion-proto-models 55.1.0

```rust
quote: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:643`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Quote character as a byte

<a id="op-51a1f225aca96c8714d24d4a"></a>
## quote_style

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::quote_style` · datafusion-proto-models 55.1.0

```rust
fn quote_style(&self) -> CsvQuoteStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `quote_style`, or the default if the field is set to an invalid enum value.

<a id="op-beb87535b6b8669d5b47dac2"></a>
## quote_style

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::quote_style` · datafusion-proto-models 55.1.0

```rust
quote_style: i32
```

Source: `src/generated/datafusion_proto_common.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Quote style for CSV writing

<a id="op-8bb1ef4e7eaafec7b2358c3c"></a>
## schema_infer_max_rec

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::schema_infer_max_rec` · datafusion-proto-models 55.1.0

```rust
schema_infer_max_rec: ::core::option::Option<u64>
```

Source: `src/generated/datafusion_proto_common.rs:652`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional max records for schema inference

<a id="op-f805a975c865eed1afff827b"></a>
## schema_infer_max_rec

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::schema_infer_max_rec` · datafusion-proto-models 55.1.0

```rust
fn schema_infer_max_rec(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the value of `schema_infer_max_rec`, or the default value if `schema_infer_max_rec` is unset.

<a id="op-ada87c61ecbdf747d5fa587f"></a>
## set_compression

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::set_compression` · datafusion-proto-models 55.1.0

```rust
fn set_compression(&mut self, value: CompressionTypeVariant)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `compression` to the provided enum value.

<a id="op-f0af9b4ea158d2b85b8c3240"></a>
## set_quote_style

`function` · `datafusion_proto_models::generated::datafusion::CsvOptions::set_quote_style` · datafusion-proto-models 55.1.0

```rust
fn set_quote_style(&mut self, value: CsvQuoteStyle)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `quote_style` to the provided enum value.

<a id="op-b2bd6cefb07b78c9e84a78d1"></a>
## terminator

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::terminator` · datafusion-proto-models 55.1.0

```rust
terminator: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:685`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional terminator character as a byte

<a id="op-a2bbf829fe7ac71a00804be2"></a>
## time_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::time_format` · datafusion-proto-models 55.1.0

```rust
time_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:667`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional time format

<a id="op-c4602ac129948185b6e483ed"></a>
## timestamp_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::timestamp_format` · datafusion-proto-models 55.1.0

```rust
timestamp_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional timestamp format

<a id="op-bb63094c32c437ede534c310"></a>
## timestamp_tz_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::timestamp_tz_format` · datafusion-proto-models 55.1.0

```rust
timestamp_tz_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:664`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional timestamp with timezone format

<a id="op-30e917e4f0d89be76f63d748"></a>
## truncated_rows

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvOptions::truncated_rows` · datafusion-proto-models 55.1.0

```rust
truncated_rows: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/datafusion_proto_common.rs:688`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Indicates if truncated rows are allowed
