# `arrow::util::string_writer::StringWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.string_writer.StringWriter.json).

<a id="op-3c1d1a5c13d7bf00babec016"></a>
## StringWriter

`struct` · `arrow::util::string_writer::StringWriter` · arrow 59.3.0

```rust
struct StringWriter
```

Source: `src/util/string_writer.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

A writer that allows writing to a `String`
like an `std::io::Write` object.

<a id="op-93acfa460ac281b74495032f"></a>
## default

`function` · `arrow::util::string_writer::StringWriter::default` · arrow 59.3.0

```rust
fn default() -> StringWriter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow::util::string_writer::StringWriter", "path": "StringWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 17], "end": [72, 24], "filename": "src/util/string_writer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/util/string_writer.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a100ebc2e55e05159e335745"></a>
## flush

`function` · `arrow::util::string_writer::StringWriter::flush` · arrow 59.3.0

```rust
fn flush(&mut self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow::util::string_writer::StringWriter", "path": "StringWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [105, 2], "filename": "src/util/string_writer.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/util/string_writer.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3864dc157629195d80595b25"></a>
## fmt

`function` · `arrow::util::string_writer::StringWriter::fmt` · arrow 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow::util::string_writer::StringWriter", "path": "StringWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 10], "end": [72, 15], "filename": "src/util/string_writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/string_writer.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-576ea55d3b311a572c15afcb"></a>
## fmt

`function` · `arrow::util::string_writer::StringWriter::fmt` · arrow 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow::util::string_writer::StringWriter", "path": "StringWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [88, 2], "filename": "src/util/string_writer.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/util/string_writer.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77fced4f7b8a3492fd7433a4"></a>
## new

`function` · `arrow::util::string_writer::StringWriter::new` · arrow 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow::util::string_writer::StringWriter", "path": "StringWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [82, 2], "filename": "src/util/string_writer.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/string_writer.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create a new `StringWriter`

<a id="op-9c1f809e7aed33a5b816833f"></a>
## write

`function` · `arrow::util::string_writer::StringWriter::write` · arrow 59.3.0

```rust
fn write(&mut self, buf: &[u8]) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow::util::string_writer::StringWriter", "path": "StringWriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [105, 2], "filename": "src/util/string_writer.rs"}, "trait": {"args": null, "id": "core::io::write::Write", "path": "Write"}, "trait_path": "core::io::write::Write"}`

Source: `src/util/string_writer.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
