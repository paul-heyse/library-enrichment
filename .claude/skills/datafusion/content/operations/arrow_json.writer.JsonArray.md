# `arrow_json::writer::JsonArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.JsonArray.json).

<a id="op-2738a78d2fd33cec9147f331"></a>
## JsonArray

`struct` · `arrow_json::writer::JsonArray` · arrow-json 59.3.0

```rust
struct JsonArray
```

Source: `src/writer/mod.rs:171`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Produces JSON output as a single JSON array.

For example:

```json
[{"foo":1},{"bar":1}]
```

<a id="op-d4ad8fc3f0a47fd1eddadd6b"></a>
## default

`function` · `arrow_json::writer::JsonArray::default` · arrow-json 59.3.0

```rust
fn default() -> JsonArray
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::JsonArray", "path": "JsonArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 17], "end": [170, 24], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer/mod.rs:170`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-366aabc314438a42838ad262"></a>
## end_stream

`function` · `arrow_json::writer::JsonArray::end_stream` · arrow-json 59.3.0

```rust
fn end_stream<W: Write>(&self, writer: &mut W) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::JsonArray", "path": "JsonArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [190, 2], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}, "trait_path": "arrow_json::writer::JsonFormat"}`

Source: `src/writer/mod.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfb109c61c23945e10b4e459"></a>
## fmt

`function` · `arrow_json::writer::JsonArray::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::JsonArray", "path": "JsonArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [170, 10], "end": [170, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:170`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d62b96423e734211a8652002"></a>
## start_row

`function` · `arrow_json::writer::JsonArray::start_row` · arrow-json 59.3.0

```rust
fn start_row<W: Write>(&self, writer: &mut W, is_first_row: bool) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::JsonArray", "path": "JsonArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [190, 2], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}, "trait_path": "arrow_json::writer::JsonFormat"}`

Source: `src/writer/mod.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf631ed5413d7077703ccb21"></a>
## start_stream

`function` · `arrow_json::writer::JsonArray::start_stream` · arrow-json 59.3.0

```rust
fn start_stream<W: Write>(&self, writer: &mut W) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::JsonArray", "path": "JsonArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [190, 2], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}, "trait_path": "arrow_json::writer::JsonFormat"}`

Source: `src/writer/mod.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
