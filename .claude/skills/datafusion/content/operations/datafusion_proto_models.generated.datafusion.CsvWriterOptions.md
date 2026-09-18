# `datafusion_proto_models::generated::datafusion::CsvWriterOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.CsvWriterOptions.json).

<a id="op-44045d375c9411d657267084"></a>
## CsvWriterOptions

`struct` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions` · datafusion-proto-models 55.1.0

```rust
struct CsvWriterOptions
```

Source: `src/generated/datafusion_proto_common.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-583305f6c964cb0dd900b35a"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94fc221a866850cf2d819491"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> CsvWriterOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 10], "end": [587, 15], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bee75f095ba9e28e0a2ed317"></a>
## compression

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::compression` · datafusion-proto-models 55.1.0

```rust
fn compression(&self) -> CompressionTypeVariant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `compression`, or the default if the field is set to an invalid enum value.

<a id="op-f8904f3446206a81f6d28d27"></a>
## compression

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::compression` · datafusion-proto-models 55.1.0

```rust
compression: i32
```

Source: `src/generated/datafusion_proto_common.rs:591`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Compression type

<a id="op-348d441d710b570d868748c7"></a>
## date_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::date_format` · datafusion-proto-models 55.1.0

```rust
date_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional date format for date arrays

<a id="op-1926f94dc0aed0afeba3011b"></a>
## datetime_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::datetime_format` · datafusion-proto-models 55.1.0

```rust
datetime_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional datetime format for datetime arrays

<a id="op-b4af38e2757d17f37632d994"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25b6cebd21752dea1ce6dfc6"></a>
## delimiter

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::delimiter` · datafusion-proto-models 55.1.0

```rust
delimiter: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional column delimiter. Defaults to `b','`

<a id="op-0ea73a6546b3c1648acda8c6"></a>
## double_quote

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::double_quote` · datafusion-proto-models 55.1.0

```rust
double_quote: bool
```

Source: `src/generated/datafusion_proto_common.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional flag whether to double quotes, instead of escaping. Defaults to `true`

<a id="op-c77ab1fdffcd2ea118202de1"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff0185501e80b5cb12775fb2"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &CsvWriterOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 17], "end": [587, 26], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33f88ee6a05bd27afdd7de1f"></a>
## escape

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::escape` · datafusion-proto-models 55.1.0

```rust
escape: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:618`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional escape. Defaults to `'\\'`

<a id="op-8e0eb065ca91e8301dd142de"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8df462abb17f989c36a72ef5"></a>
## has_header

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::has_header` · datafusion-proto-models 55.1.0

```rust
has_header: bool
```

Source: `src/generated/datafusion_proto_common.rs:597`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Whether to write column names as file headers. Defaults to `true`

<a id="op-4308d589b7883968fb504662"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 32], "end": [587, 36], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f9072a92a471f19522d71c7"></a>
## ignore_leading_whitespace

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::ignore_leading_whitespace` · datafusion-proto-models 55.1.0

```rust
ignore_leading_whitespace: bool
```

Source: `src/generated/datafusion_proto_common.rs:627`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Whether to ignore leading whitespace in string values

<a id="op-8633b6828eca4a0d8b9e7ea6"></a>
## ignore_trailing_whitespace

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::ignore_trailing_whitespace` · datafusion-proto-models 55.1.0

```rust
ignore_trailing_whitespace: bool
```

Source: `src/generated/datafusion_proto_common.rs:630`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Whether to ignore trailing whitespace in string values

<a id="op-df5aeb57c04b429f51657972"></a>
## null_value

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::null_value` · datafusion-proto-models 55.1.0

```rust
null_value: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:612`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional value to represent null

<a id="op-a362962d965d82505a6c4036"></a>
## quote

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::quote` · datafusion-proto-models 55.1.0

```rust
quote: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:615`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional quote. Defaults to `b'"'`

<a id="op-38d139bf0e5d00e5d049afdf"></a>
## quote_style

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::quote_style` · datafusion-proto-models 55.1.0

```rust
fn quote_style(&self) -> CsvQuoteStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the enum value of `quote_style`, or the default if the field is set to an invalid enum value.

<a id="op-897a552230e57187d0d92bc8"></a>
## quote_style

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::quote_style` · datafusion-proto-models 55.1.0

```rust
quote_style: i32
```

Source: `src/generated/datafusion_proto_common.rs:624`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Quote style for CSV writing

<a id="op-8f4fd07aca7c53c155c7b8ae"></a>
## set_compression

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::set_compression` · datafusion-proto-models 55.1.0

```rust
fn set_compression(&mut self, value: CompressionTypeVariant)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `compression` to the provided enum value.

<a id="op-6f5d561ada497cd809eed4dc"></a>
## set_quote_style

`function` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::set_quote_style` · datafusion-proto-models 55.1.0

```rust
fn set_quote_style(&mut self, value: CsvQuoteStyle)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::CsvWriterOptions", "path": "CsvWriterOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 38], "end": [587, 54], "filename": "src/generated/datafusion_proto_common.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/datafusion_proto_common.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Sets `quote_style` to the provided enum value.

<a id="op-f2a313df87be55a773072c20"></a>
## time_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::time_format` · datafusion-proto-models 55.1.0

```rust
time_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:609`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional time format for time arrays

<a id="op-43f09a16399087c8fcf157f9"></a>
## timestamp_format

`struct_field` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions::timestamp_format` · datafusion-proto-models 55.1.0

```rust
timestamp_format: ::prost::alloc::string::String
```

Source: `src/generated/datafusion_proto_common.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional timestamp format for timestamp arrays
