# `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.CsvWriterOptions.json).

<a id="op-8b53a2402263aa4d242ee58b"></a>
## CsvWriterOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions` · datafusion-proto-common 55.1.0

```rust
struct CsvWriterOptions
```

Source: `src/generated/prost.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deef3772ec197fa595de2241"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "protobuf::CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [871, 1], "end": [880, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0e621fab67e92ad4907ef05"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c72ee35aa8a74639faac759"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> CsvWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 10], "end": [587, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a10fb9d4969f697bc0922029"></a>
## compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::compression` · datafusion-proto-common 55.1.0

```rust
fn compression(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the enum value of `compression`, or the default if the field is set to an invalid enum value.

<a id="op-f24a0b8780549109d5a67c8d"></a>
## compression

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::compression` · datafusion-proto-common 55.1.0

```rust
compression: i32
```

Source: `src/generated/prost.rs:591`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Compression type

<a id="op-ab6483bde5799d931b5c7c0b"></a>
## date_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::date_format` · datafusion-proto-common 55.1.0

```rust
date_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional date format for date arrays

<a id="op-4fb79db31d08562277cfc9cb"></a>
## datetime_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::datetime_format` · datafusion-proto-common 55.1.0

```rust
datetime_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional datetime format for datetime arrays

<a id="op-98b195d0569a544b74648a1f"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a50e313ca4e1bc532e413503"></a>
## delimiter

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::delimiter` · datafusion-proto-common 55.1.0

```rust
delimiter: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional column delimiter. Defaults to `b','`

<a id="op-a4e002e9f0213d6306a41996"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2362, 1], "end": [2587, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:2364`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d315b73a1ed00573a2c9b21"></a>
## double_quote

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::double_quote` · datafusion-proto-common 55.1.0

```rust
double_quote: bool
```

Source: `src/generated/prost.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional flag whether to double quotes, instead of escaping. Defaults to `true`

<a id="op-a920ed30fc3ee9e9f0faabf6"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-573f773b6006a267a6da99f3"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &CsvWriterOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 17], "end": [587, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-478e3460a123a659f931e327"></a>
## escape

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::escape` · datafusion-proto-common 55.1.0

```rust
escape: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:618`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional escape. Defaults to `'\\'`

<a id="op-94fedcdedb5dd7c9785665f5"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5891f95f476490b80d8b0cdb"></a>
## has_header

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::has_header` · datafusion-proto-common 55.1.0

```rust
has_header: bool
```

Source: `src/generated/prost.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Whether to write column names as file headers. Defaults to `true`

<a id="op-1d0be6a3d012177bbc75c482"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 32], "end": [587, 36], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d4e9772f9147a03e116e8ac"></a>
## ignore_leading_whitespace

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::ignore_leading_whitespace` · datafusion-proto-common 55.1.0

```rust
ignore_leading_whitespace: bool
```

Source: `src/generated/prost.rs:627`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Whether to ignore leading whitespace in string values

<a id="op-3d05afbaa03ea63c1dec3253"></a>
## ignore_trailing_whitespace

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::ignore_trailing_whitespace` · datafusion-proto-common 55.1.0

```rust
ignore_trailing_whitespace: bool
```

Source: `src/generated/prost.rs:630`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Whether to ignore trailing whitespace in string values

<a id="op-c3907cabe8bda5dcbb10dd85"></a>
## null_value

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::null_value` · datafusion-proto-common 55.1.0

```rust
null_value: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:612`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional value to represent null

<a id="op-48bcb8ed6fea5b773a2eb274"></a>
## quote

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::quote` · datafusion-proto-common 55.1.0

```rust
quote: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:615`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional quote. Defaults to `b'"'`

<a id="op-a1b5dcf61405a09fb475315e"></a>
## quote_style

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::quote_style` · datafusion-proto-common 55.1.0

```rust
quote_style: i32
```

Source: `src/generated/prost.rs:624`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Quote style for CSV writing

<a id="op-ad4ff6d395984690ff0edb15"></a>
## quote_style

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::quote_style` · datafusion-proto-common 55.1.0

```rust
fn quote_style(&self) -> CsvQuoteStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the enum value of `quote_style`, or the default if the field is set to an invalid enum value.

<a id="op-8ccb99bf0afcc740e7e1d96c"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2262, 1], "end": [2361, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:2264`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c47a6dc0064e4faca55c6490"></a>
## set_compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::set_compression` · datafusion-proto-common 55.1.0

```rust
fn set_compression(&mut self, value: CompressionTypeVariant)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Sets `compression` to the provided enum value.

<a id="op-9177fdf43737edaf99c4b4e9"></a>
## set_quote_style

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::set_quote_style` · datafusion-proto-common 55.1.0

```rust
fn set_quote_style(&mut self, value: CsvQuoteStyle)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Sets `quote_style` to the provided enum value.

<a id="op-cba6cd7161edd2b2bcd83967"></a>
## time_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::time_format` · datafusion-proto-common 55.1.0

```rust
time_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:609`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional time format for time arrays

<a id="op-92c504245371f2ca6a34c983"></a>
## timestamp_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::timestamp_format` · datafusion-proto-common 55.1.0

```rust
timestamp_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional timestamp format for timestamp arrays

<a id="op-60f8a03e6fa3842d7a483268"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(opts: &CsvWriterOptions) -> datafusion_common::Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions", "path": "protobuf::CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [871, 1], "end": [880, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::file_options::csv_writer::CsvWriterOptions", "path": "CsvWriterOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:874`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
