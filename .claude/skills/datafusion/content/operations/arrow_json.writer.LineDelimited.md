# `arrow_json::writer::LineDelimited`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.LineDelimited.json).

<a id="op-6032a386d63609e6a52a5f3d"></a>
## LineDelimited

`struct` · `arrow_json::writer::LineDelimited` · arrow-json 59.3.0

```rust
struct LineDelimited
```

Source: `src/writer/mod.rs:154`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Produces JSON output with one record per line.

For example:

```json
{"foo":1}
{"bar":1}

```

<a id="op-01caaed0ec6f3efb6b8fcf8b"></a>
## default

`function` · `arrow_json::writer::LineDelimited::default` · arrow-json 59.3.0

```rust
fn default() -> LineDelimited
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::LineDelimited", "path": "LineDelimited"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 17], "end": [153, 24], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer/mod.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e1a8b5e51f642aec5131bd6"></a>
## end_row

`function` · `arrow_json::writer::LineDelimited::end_row` · arrow-json 59.3.0

```rust
fn end_row<W: Write>(&self, writer: &mut W) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::LineDelimited", "path": "LineDelimited"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [161, 2], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "arrow_json::writer::JsonFormat", "path": "JsonFormat"}, "trait_path": "arrow_json::writer::JsonFormat"}`

Source: `src/writer/mod.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d2624e97fbc72087ba6e848"></a>
## fmt

`function` · `arrow_json::writer::LineDelimited::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::writer::LineDelimited", "path": "LineDelimited"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 10], "end": [153, 15], "filename": "src/writer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer/mod.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
