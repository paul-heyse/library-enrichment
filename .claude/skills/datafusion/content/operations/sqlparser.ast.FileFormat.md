# `sqlparser::ast::FileFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FileFormat.json).

<a id="op-0f6b382c1062c777560359c0"></a>
## FileFormat

`enum` · `sqlparser::ast::FileFormat` · sqlparser 0.62.0

```rust
enum FileFormat
```

Source: `src/ast/mod.rs:8355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

External table's available file format

<a id="op-c4744495ab4df6d6db2650ee"></a>
## AVRO

`variant` · `sqlparser::ast::FileFormat::AVRO` · sqlparser 0.62.0

```rust
AVRO
```

Source: `src/ast/mod.rs:8365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Avro file format.

<a id="op-300fe457a9120b69578e98b0"></a>
## JSONFILE

`variant` · `sqlparser::ast::FileFormat::JSONFILE` · sqlparser 0.62.0

```rust
JSONFILE
```

Source: `src/ast/mod.rs:8369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

JSON file format.

<a id="op-991916f92ced09d044171255"></a>
## ORC

`variant` · `sqlparser::ast::FileFormat::ORC` · sqlparser 0.62.0

```rust
ORC
```

Source: `src/ast/mod.rs:8361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ORC file format.

<a id="op-e6d1aa6f75fa2d78ba3ff1ba"></a>
## PARQUET

`variant` · `sqlparser::ast::FileFormat::PARQUET` · sqlparser 0.62.0

```rust
PARQUET
```

Source: `src/ast/mod.rs:8363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parquet file format.

<a id="op-bdc97f0d00b0e3010a7f6f13"></a>
## RCFILE

`variant` · `sqlparser::ast::FileFormat::RCFILE` · sqlparser 0.62.0

```rust
RCFILE
```

Source: `src/ast/mod.rs:8367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RCFile format.

<a id="op-e197862f08a0aed50b0f33a5"></a>
## SEQUENCEFILE

`variant` · `sqlparser::ast::FileFormat::SEQUENCEFILE` · sqlparser 0.62.0

```rust
SEQUENCEFILE
```

Source: `src/ast/mod.rs:8359`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sequence file format.

<a id="op-f9dc25eeb4fadee20e2fe3c0"></a>
## TEXTFILE

`variant` · `sqlparser::ast::FileFormat::TEXTFILE` · sqlparser 0.62.0

```rust
TEXTFILE
```

Source: `src/ast/mod.rs:8357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Text file format.

<a id="op-415d70da4c5f0a252b5f84a3"></a>
## clone

`function` · `sqlparser::ast::FileFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FileFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8352, 23], "end": [8352, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28378f0550ee8080b2d242db"></a>
## cmp

`function` · `sqlparser::ast::FileFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FileFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8352, 57], "end": [8352, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-312456cd4b711308d2812d00"></a>
## deserialize

`function` · `sqlparser::ast::FileFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8353, 49], "end": [8353, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef561f437deb9ff31c62182b"></a>
## eq

`function` · `sqlparser::ast::FileFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FileFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8352, 30], "end": [8352, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4af239a9627c7b477544fbfe"></a>
## fmt

`function` · `sqlparser::ast::FileFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8372, 1], "end": [8385, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef25ae3c4c4a747d6828a2a7"></a>
## fmt

`function` · `sqlparser::ast::FileFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8352, 10], "end": [8352, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31575d2daed250deeccf4224"></a>
## hash

`function` · `sqlparser::ast::FileFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8352, 62], "end": [8352, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e17ff127ebfaad25845e9604"></a>
## partial_cmp

`function` · `sqlparser::ast::FileFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FileFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8352, 41], "end": [8352, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2880a826d9fb8995aa26acff"></a>
## serialize

`function` · `sqlparser::ast::FileFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8353, 38], "end": [8353, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4449ac6e4c0ae9ec41bca498"></a>
## visit

`function` · `sqlparser::ast::FileFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8354, 40], "end": [8354, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff2e2be203f77eed72e2f0ea"></a>
## visit

`function` · `sqlparser::ast::FileFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileFormat", "path": "FileFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8354, 47], "end": [8354, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
