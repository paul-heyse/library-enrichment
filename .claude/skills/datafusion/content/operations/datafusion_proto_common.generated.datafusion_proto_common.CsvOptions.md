# `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.generated.datafusion_proto_common.CsvOptions.json).

<a id="op-6ab20e61021cef4f6b63afcd"></a>
## CsvOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions` · datafusion-proto-common 55.1.0

```rust
struct CsvOptions
```

Source: `src/generated/prost.rs:634`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Options controlling CSV format

<a id="op-7421756cae145c19b2cacdf8"></a>
## Error

`assoc_type` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::Error` · datafusion-proto-common 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "protobuf::CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1017, 1], "end": [1054, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:1018`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5e3c9f33ba4ae32e07d69e1"></a>
## clear

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::clear` · datafusion-proto-common 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15ada0fcbbe6bc3c19693843"></a>
## clone

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::clone` · datafusion-proto-common 55.1.0

```rust
fn clone(&self) -> CsvOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 10], "end": [633, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6594c6a449a8fe4d1e10eb6c"></a>
## comment

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::comment` · datafusion-proto-common 55.1.0

```rust
comment: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:676`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional comment character as a byte

<a id="op-1b3dc938e5eb3d234e964275"></a>
## compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::compression` · datafusion-proto-common 55.1.0

```rust
fn compression(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the enum value of `compression`, or the default if the field is set to an invalid enum value.

<a id="op-9ae71deafa174e20f458140b"></a>
## compression

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::compression` · datafusion-proto-common 55.1.0

```rust
compression: i32
```

Source: `src/generated/prost.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Compression type

<a id="op-0930bf444f1c250f3746875f"></a>
## compression_level

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::compression_level` · datafusion-proto-common 55.1.0

```rust
compression_level: ::core::option::Option<u32>
```

Source: `src/generated/prost.rs:691`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional compression level

<a id="op-bec3377dc4dfe48acf32dd22"></a>
## compression_level

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::compression_level` · datafusion-proto-common 55.1.0

```rust
fn compression_level(&self) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the value of `compression_level`, or the default value if `compression_level` is unset.

<a id="op-b32e60dbb854f406a1563845"></a>
## date_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::date_format` · datafusion-proto-common 55.1.0

```rust
date_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional date format

<a id="op-87075981a9026468099c5591"></a>
## datetime_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::datetime_format` · datafusion-proto-common 55.1.0

```rust
datetime_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional datetime format

<a id="op-636e2537ff5963203b553b7e"></a>
## default

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::default` · datafusion-proto-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d39fe5a383b4decf29dbce15"></a>
## delimiter

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::delimiter` · datafusion-proto-common 55.1.0

```rust
delimiter: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:640`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Delimiter character as a byte

<a id="op-70ad625c6699044cc7cf05be"></a>
## deserialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::deserialize` · datafusion-proto-common 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1839, 1], "end": [2184, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:1841`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6c88a3382713247ace35758"></a>
## double_quote

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::double_quote` · datafusion-proto-common 55.1.0

```rust
double_quote: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:679`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Indicates if quotes are doubled

<a id="op-94c84c7d814597af8e9d49ee"></a>
## encoded_len

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::encoded_len` · datafusion-proto-common 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efc417d89d9297e949715974"></a>
## eq

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::eq` · datafusion-proto-common 55.1.0

```rust
fn eq(&self, other: &CsvOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 17], "end": [633, 26], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe80e6f837d4b850e214eefb"></a>
## escape

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::escape` · datafusion-proto-common 55.1.0

```rust
escape: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:646`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional escape character as a byte

<a id="op-402fd7ed16a2f5c62a6fd4b0"></a>
## fmt

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::fmt` · datafusion-proto-common 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09cdb6d355b0d3b3b3c707cd"></a>
## has_header

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::has_header` · datafusion-proto-common 55.1.0

```rust
has_header: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:637`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Indicates if the CSV has a header row

<a id="op-01fab3064347d2fc4e9e8b34"></a>
## hash

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::hash` · datafusion-proto-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 32], "end": [633, 36], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45e72972c5b3905f93dff153"></a>
## ignore_leading_whitespace

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::ignore_leading_whitespace` · datafusion-proto-common 55.1.0

```rust
ignore_leading_whitespace: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:697`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Whether to ignore leading whitespace in string values

<a id="op-3ac6ff72b4956ae6e0efbd5a"></a>
## ignore_trailing_whitespace

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::ignore_trailing_whitespace` · datafusion-proto-common 55.1.0

```rust
ignore_trailing_whitespace: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:700`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Whether to ignore trailing whitespace in string values

<a id="op-2bd6c2464361ec25757b036f"></a>
## newlines_in_values

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::newlines_in_values` · datafusion-proto-common 55.1.0

```rust
newlines_in_values: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:682`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Indicates if newlines are supported in values

<a id="op-cb3387fa9c6fa2a8383daaf8"></a>
## null_regex

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::null_regex` · datafusion-proto-common 55.1.0

```rust
null_regex: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:673`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional representation of null loading regex

<a id="op-9e63bf4b02a780ba8b5a4235"></a>
## null_value

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::null_value` · datafusion-proto-common 55.1.0

```rust
null_value: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:670`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional representation of null value

<a id="op-3a4aad16bd6a70d4e099db69"></a>
## quote

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::quote` · datafusion-proto-common 55.1.0

```rust
quote: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:643`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Quote character as a byte

<a id="op-2557d6c47e9c6dee71cbfdf6"></a>
## quote_style

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::quote_style` · datafusion-proto-common 55.1.0

```rust
fn quote_style(&self) -> CsvQuoteStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the enum value of `quote_style`, or the default if the field is set to an invalid enum value.

<a id="op-50ac2f9bab6fc21c582d1597"></a>
## quote_style

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::quote_style` · datafusion-proto-common 55.1.0

```rust
quote_style: i32
```

Source: `src/generated/prost.rs:694`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Quote style for CSV writing

<a id="op-02bb8918244555b0872025fa"></a>
## schema_infer_max_rec

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::schema_infer_max_rec` · datafusion-proto-common 55.1.0

```rust
fn schema_infer_max_rec(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Returns the value of `schema_infer_max_rec`, or the default value if `schema_infer_max_rec` is unset.

<a id="op-a5890fd0fd64787babbb1915"></a>
## schema_infer_max_rec

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::schema_infer_max_rec` · datafusion-proto-common 55.1.0

```rust
schema_infer_max_rec: ::core::option::Option<u64>
```

Source: `src/generated/prost.rs:652`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional max records for schema inference

<a id="op-6644410a55a113d8e8f28131"></a>
## serialize

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::serialize` · datafusion-proto-common 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1667, 1], "end": [1838, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:1669`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0edce25a1fbb729057d471c8"></a>
## set_compression

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::set_compression` · datafusion-proto-common 55.1.0

```rust
fn set_compression(&mut self, value: CompressionTypeVariant)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Sets `compression` to the provided enum value.

<a id="op-db43795e8ccee80953bf0d0a"></a>
## set_quote_style

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::set_quote_style` · datafusion-proto-common 55.1.0

```rust
fn set_quote_style(&mut self, value: CsvQuoteStyle)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 38], "end": [633, 54], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Sets `quote_style` to the provided enum value.

<a id="op-3c5eb3608b1c68f46e52475e"></a>
## terminator

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::terminator` · datafusion-proto-common 55.1.0

```rust
terminator: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:685`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional terminator character as a byte

<a id="op-6fc4dacf6743a5dd7a450c9b"></a>
## time_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::time_format` · datafusion-proto-common 55.1.0

```rust
time_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:667`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional time format

<a id="op-b10861a8e3397a09b579ab19"></a>
## timestamp_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::timestamp_format` · datafusion-proto-common 55.1.0

```rust
timestamp_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional timestamp format

<a id="op-24bb5cf36e2f6ad157682083"></a>
## timestamp_tz_format

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::timestamp_tz_format` · datafusion-proto-common 55.1.0

```rust
timestamp_tz_format: ::prost::alloc::string::String
```

Source: `src/generated/prost.rs:664`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Optional timestamp with timezone format

<a id="op-b38aee7d67da0752433d2b24"></a>
## truncated_rows

`struct_field` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::truncated_rows` · datafusion-proto-common 55.1.0

```rust
truncated_rows: ::prost::alloc::vec::Vec<u8>
```

Source: `src/generated/prost.rs:688`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Indicates if truncated rows are allowed

<a id="op-48c70636c2e4c8c9405fb5c1"></a>
## try_from

`function` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions::try_from` · datafusion-proto-common 55.1.0

```rust
fn try_from(opts: &CsvOptions) -> datafusion_common::Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_common::generated::datafusion_proto_common::CsvOptions", "path": "protobuf::CsvOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1017, 1], "end": [1054, 2], "filename": "src/to_proto/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::CsvOptions", "path": "CsvOptions"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/to_proto/mod.rs:1020`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
