# `arrow_json::writer::WriterBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.WriterBuilder.json).

<a id="op-eab011f6b5e700c1657bc7e0"></a>
## WriterBuilder

`struct` · `arrow_json::writer::WriterBuilder` · arrow-json 59.3.0

```rust
struct WriterBuilder
```

Source: `src/writer/mod.rs:200`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

JSON writer builder.

<a id="op-bd0f5786d8e8b8a6c2b32fe0"></a>
## build

`function` · `arrow_json::writer::WriterBuilder::build` · arrow-json 59.3.0

```rust
fn build<W, F>(self, writer: W) -> Writer<W, F> where W: Write, F: JsonFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Create a new `Writer` with specified `JsonFormat` and builder options.

<a id="op-0027149984aee5c539c97278"></a>
## clone

`function` · `arrow_json::writer::WriterBuilder::clone` · arrow-json 59.3.0

```rust
fn clone(&self) -> WriterBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 17], "end": [199, 22], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer/mod.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b26f4e81e196211e2b0a80c3"></a>
## default

`function` · `arrow_json::writer::WriterBuilder::default` · arrow-json 59.3.0

```rust
fn default() -> WriterBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 24], "end": [199, 31], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer/mod.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a462646dcb5136d06ecdd609"></a>
## explicit_nulls

`function` · `arrow_json::writer::WriterBuilder::explicit_nulls` · arrow-json 59.3.0

```rust
fn explicit_nulls(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:227`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns `true` if this writer is configured to keep keys with null values.

<a id="op-b4426aff6cad53f3b332ca27"></a>
## fmt

`function` · `arrow_json::writer::WriterBuilder::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 10], "end": [199, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-128d2ba97c69fb86ae1095c4"></a>
## new

`function` · `arrow_json::writer::WriterBuilder::new` · arrow-json 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:222`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Create a new builder for configuring JSON writing options.

# Example

```
# use arrow_json::{Writer, WriterBuilder};
# use arrow_json::writer::LineDelimited;
# use std::fs::File;

fn example() -> Writer<File, LineDelimited> {
    let file = File::create("target/out.json").unwrap();

    // create a builder that keeps keys with null values
    let builder = WriterBuilder::new().with_explicit_nulls(true);
    let writer = builder.build::<_, LineDelimited>(file);

    writer
}
```

<a id="op-2336e4eb10e1d1c2e5ae4747"></a>
## struct_mode

`function` · `arrow_json::writer::WriterBuilder::struct_mode` · arrow-json 59.3.0

```rust
fn struct_mode(&self) -> StructMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:259`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns if this writer is configured to write structs as JSON Objects or Arrays.

<a id="op-f9128befcda3c8f3da107c07"></a>
## with_date_format

`function` · `arrow_json::writer::WriterBuilder::with_date_format` · arrow-json 59.3.0

```rust
fn with_date_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:283`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's date format

<a id="op-e05b48046f84a5d38a50f925"></a>
## with_datetime_format

`function` · `arrow_json::writer::WriterBuilder::with_datetime_format` · arrow-json 59.3.0

```rust
fn with_datetime_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:289`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's datetime format

<a id="op-bccc8dd178f8a47ed1f05fef"></a>
## with_encoder_factory

`function` · `arrow_json::writer::WriterBuilder::with_encoder_factory` · arrow-json 59.3.0

```rust
fn with_encoder_factory(self, factory: Arc<dyn EncoderFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set an encoder factory to use when creating encoders for writing JSON.

This can be used to override how some types are encoded or to provide
a fallback for types that are not supported by the default encoder.

<a id="op-a71b8dc2783ed30343873be5"></a>
## with_explicit_nulls

`function` · `arrow_json::writer::WriterBuilder::with_explicit_nulls` · arrow-json 59.3.0

```rust
fn with_explicit_nulls(self, explicit_nulls: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:253`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set whether to keep keys with null values, or to omit writing them.

For example, with [`LineDelimited`](../operations/arrow_json.writer.LineDelimited.md#op-6032a386d63609e6a52a5f3d) format:

Skip nulls (set to `false`):

```json
{"foo":1}
{"foo":1,"bar":2}
{}
```

Keep nulls (set to `true`):

```json
{"foo":1,"bar":null}
{"foo":1,"bar":2}
{"foo":null,"bar":null}
```

Default is to skip nulls (set to `false`). If `struct_mode == ListOnly`,
nulls will be written explicitly regardless of this setting.

<a id="op-0ee5d4254814fac46a1709f7"></a>
## with_struct_mode

`function` · `arrow_json::writer::WriterBuilder::with_struct_mode` · arrow-json 59.3.0

```rust
fn with_struct_mode(self, struct_mode: StructMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the [`StructMode`](../operations/arrow_json.StructMode.md#op-a862c9daf0b023e9659efe1e) for the writer, which determines whether structs
are encoded to JSON as objects or lists. For more details refer to the
enum documentation. Default is to use `ObjectOnly`. If this is set to
`ListOnly`, nulls will be written explicitly regardless of the
`explicit_nulls` setting.

<a id="op-bf1c83412c7fc82a66f146e4"></a>
## with_time_format

`function` · `arrow_json::writer::WriterBuilder::with_time_format` · arrow-json 59.3.0

```rust
fn with_time_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's time format

<a id="op-c8faca47c1a6309cc75fdb1f"></a>
## with_timestamp_format

`function` · `arrow_json::writer::WriterBuilder::with_timestamp_format` · arrow-json 59.3.0

```rust
fn with_timestamp_format(self, format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:301`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's timestamp format

<a id="op-de0f9627485e7b346dc67675"></a>
## with_timestamp_tz_format

`function` · `arrow_json::writer::WriterBuilder::with_timestamp_tz_format` · arrow-json 59.3.0

```rust
fn with_timestamp_tz_format(self, tz_format: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::WriterBuilder", "path": "WriterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [326, 2], "filename": "src/writer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/writer/mod.rs:307`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the JSON file's timestamp tz format
