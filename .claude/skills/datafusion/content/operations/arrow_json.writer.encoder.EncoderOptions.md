# `arrow_json::writer::encoder::EncoderOptions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.encoder.EncoderOptions.json).

<a id="op-25b22d634d24220b72b5de35"></a>
## EncoderOptions

`struct` · `arrow_json::writer::encoder::EncoderOptions` · arrow-json 59.3.0

```rust
struct EncoderOptions
```

Source: `src/writer/encoder.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Configuration options for the JSON encoder.

<a id="op-80f108b28f4b4e78a89fbd10"></a>
## clone

`function` · `arrow_json::writer::encoder::EncoderOptions::clone` · arrow-json 59.3.0

```rust
fn clone(&self) -> EncoderOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 22], "filename": "src/writer/encoder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer/encoder.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4405cdd2670c4e16f19f7ae2"></a>
## date_format

`function` · `arrow_json::writer::encoder::EncoderOptions::date_format` · arrow-json 59.3.0

```rust
fn date_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get the JSON file's date format if set, defaults to RFC3339

<a id="op-1f26443bb7ffd3dab47bc644"></a>
## datetime_format

`function` · `arrow_json::writer::encoder::EncoderOptions::datetime_format` · arrow-json 59.3.0

```rust
fn datetime_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get the JSON file's datetime format if set, defaults to RFC3339

<a id="op-9dcfa5ae9ee8e04ce373d1cf"></a>
## default

`function` · `arrow_json::writer::encoder::EncoderOptions::default` · arrow-json 59.3.0

```rust
fn default() -> EncoderOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 24], "end": [32, 31], "filename": "src/writer/encoder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer/encoder.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6d5c8cebcd5a6eec74e04c2"></a>
## encoder_factory

`function` · `arrow_json::writer::encoder::EncoderOptions::encoder_factory` · arrow-json 59.3.0

```rust
fn encoder_factory(&self) -> Option<&Arc<dyn EncoderFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get the optional hook for customizing encoding behavior.

<a id="op-2acfed155c16034ada228fc1"></a>
## explicit_nulls

`function` · `arrow_json::writer::encoder::EncoderOptions::explicit_nulls` · arrow-json 59.3.0

```rust
fn explicit_nulls(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get whether to include nulls in the output or elide them.

<a id="op-d634f0e7654030bb5d31bb16"></a>
## fmt

`function` · `arrow_json::writer::encoder::EncoderOptions::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/writer/encoder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/encoder.rs:32`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d405a95c91922b093d327957"></a>
## struct_mode

`function` · `arrow_json::writer::encoder::EncoderOptions::struct_mode` · arrow-json 59.3.0

```rust
fn struct_mode(&self) -> StructMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get whether to encode structs as JSON objects or JSON arrays of their values.

<a id="op-d9884a3f1d00031865f4488d"></a>
## time_format

`function` · `arrow_json::writer::encoder::EncoderOptions::time_format` · arrow-json 59.3.0

```rust
fn time_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get the JSON file's datetime time if set, defaults to RFC3339

<a id="op-b01c8baef39a3d49db2159e5"></a>
## timestamp_format

`function` · `arrow_json::writer::encoder::EncoderOptions::timestamp_format` · arrow-json 59.3.0

```rust
fn timestamp_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:126`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get the JSON file's timestamp format if set, defaults to RFC3339

<a id="op-2095dd2b2bf11bc32ee25093"></a>
## timestamp_tz_format

`function` · `arrow_json::writer::encoder::EncoderOptions::timestamp_tz_format` · arrow-json 59.3.0

```rust
fn timestamp_tz_format(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:137`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Get the JSON file's timestamp tz format if set, defaults to RFC3339

<a id="op-36397768457041e8afd3ffdf"></a>
## with_date_format

`function` · `arrow_json::writer::encoder::EncoderOptions::with_date_format` · arrow-json 59.3.0

```rust
fn with_date_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's date format

<a id="op-688848513b5dea76352d7cba"></a>
## with_datetime_format

`function` · `arrow_json::writer::encoder::EncoderOptions::with_datetime_format` · arrow-json 59.3.0

```rust
fn with_datetime_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's datetime format

<a id="op-2f9e0560cbf7c2e5ed48b7e7"></a>
## with_encoder_factory

`function` · `arrow_json::writer::encoder::EncoderOptions::with_encoder_factory` · arrow-json 59.3.0

```rust
fn with_encoder_factory(self, encoder_factory: Arc<dyn EncoderFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set an optional hook for customizing encoding behavior.

<a id="op-4d5e8faf35d73e25b791100b"></a>
## with_explicit_nulls

`function` · `arrow_json::writer::encoder::EncoderOptions::with_explicit_nulls` · arrow-json 59.3.0

```rust
fn with_explicit_nulls(self, explicit_nulls: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set whether to include nulls in the output or elide them.

<a id="op-8a9a7acbfbe2a65e17d185a3"></a>
## with_struct_mode

`function` · `arrow_json::writer::encoder::EncoderOptions::with_struct_mode` · arrow-json 59.3.0

```rust
fn with_struct_mode(self, struct_mode: StructMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set whether to encode structs as JSON objects or JSON arrays of their values.

<a id="op-3d217604e52548b2c78bc60a"></a>
## with_time_format

`function` · `arrow_json::writer::encoder::EncoderOptions::with_time_format` · arrow-json 59.3.0

```rust
fn with_time_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's time format

<a id="op-1ccc57a4a5c16320f9ded9dd"></a>
## with_timestamp_format

`function` · `arrow_json::writer::encoder::EncoderOptions::with_timestamp_format` · arrow-json 59.3.0

```rust
fn with_timestamp_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's timestamp format

<a id="op-ef8061ae8598065226f2b3d6"></a>
## with_timestamp_tz_format

`function` · `arrow_json::writer::encoder::EncoderOptions::with_timestamp_tz_format` · arrow-json 59.3.0

```rust
fn with_timestamp_tz_format(self, tz_format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::encoder::EncoderOptions", "path": "EncoderOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [140, 2], "filename": "src/writer/encoder.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/encoder.rs:131`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's timestamp tz format
