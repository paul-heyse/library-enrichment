# `sqlparser::ast::CopyOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CopyOption.json).

<a id="op-22f4c6cb07699f227fbb3dcb"></a>
## CopyOption

`enum` · `sqlparser::ast::CopyOption` · sqlparser 0.62.0

```rust
enum CopyOption
```

Source: `src/ast/mod.rs:9323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An option in `COPY` statement.

<https://www.postgresql.org/docs/14/sql-copy.html>

<a id="op-5157c3de6b14b7199091dfe2"></a>
## Delimiter

`variant` · `sqlparser::ast::CopyOption::Delimiter` · sqlparser 0.62.0

```rust
Delimiter
```

Source: `src/ast/mod.rs:9329`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DELIMITER 'delimiter_character'

<a id="op-8ac9eb061121b6ec9a2866ef"></a>
## Encoding

`variant` · `sqlparser::ast::CopyOption::Encoding` · sqlparser 0.62.0

```rust
Encoding
```

Source: `src/ast/mod.rs:9345`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ENCODING 'encoding_name'

<a id="op-37fc6baf7ca2f9937378e27f"></a>
## Escape

`variant` · `sqlparser::ast::CopyOption::Escape` · sqlparser 0.62.0

```rust
Escape
```

Source: `src/ast/mod.rs:9337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ESCAPE 'escape_character'

<a id="op-cfe2169826b2d970fcfb8764"></a>
## ForceNotNull

`variant` · `sqlparser::ast::CopyOption::ForceNotNull` · sqlparser 0.62.0

```rust
ForceNotNull
```

Source: `src/ast/mod.rs:9341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FORCE_NOT_NULL ( column_name [, ...] )

<a id="op-724bee96a159a271d2a96f4d"></a>
## ForceNull

`variant` · `sqlparser::ast::CopyOption::ForceNull` · sqlparser 0.62.0

```rust
ForceNull
```

Source: `src/ast/mod.rs:9343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FORCE_NULL ( column_name [, ...] )

<a id="op-ca56e9826dacb13488cac0ef"></a>
## ForceQuote

`variant` · `sqlparser::ast::CopyOption::ForceQuote` · sqlparser 0.62.0

```rust
ForceQuote
```

Source: `src/ast/mod.rs:9339`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FORCE_QUOTE { ( column_name [, ...] ) | * }

<a id="op-5336d2b9a1b40ea92c976f9f"></a>
## Format

`variant` · `sqlparser::ast::CopyOption::Format` · sqlparser 0.62.0

```rust
Format
```

Source: `src/ast/mod.rs:9325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FORMAT format_name

<a id="op-d34b1d1047ec678390005124"></a>
## Freeze

`variant` · `sqlparser::ast::CopyOption::Freeze` · sqlparser 0.62.0

```rust
Freeze
```

Source: `src/ast/mod.rs:9327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FREEZE \[ boolean \]

<a id="op-2bfd008828f225d879753f81"></a>
## Header

`variant` · `sqlparser::ast::CopyOption::Header` · sqlparser 0.62.0

```rust
Header
```

Source: `src/ast/mod.rs:9333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

HEADER \[ boolean \]

<a id="op-88854b6f66a1867d40cb8dee"></a>
## Null

`variant` · `sqlparser::ast::CopyOption::Null` · sqlparser 0.62.0

```rust
Null
```

Source: `src/ast/mod.rs:9331`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

NULL 'null_string'

<a id="op-d7fa82ad32cb86f67e7e6b19"></a>
## Quote

`variant` · `sqlparser::ast::CopyOption::Quote` · sqlparser 0.62.0

```rust
Quote
```

Source: `src/ast/mod.rs:9335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

QUOTE 'quote_character'

<a id="op-047003db785387f2d3c01acd"></a>
## clone

`function` · `sqlparser::ast::CopyOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CopyOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9320, 17], "end": [9320, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49afa5259108f40e33708a16"></a>
## cmp

`function` · `sqlparser::ast::CopyOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CopyOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9320, 51], "end": [9320, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36fd305a7f449cb7a6e62dd3"></a>
## deserialize

`function` · `sqlparser::ast::CopyOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9321, 49], "end": [9321, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e065b28b1b9cf222a7434989"></a>
## eq

`function` · `sqlparser::ast::CopyOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CopyOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9320, 24], "end": [9320, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02e1c03dea7e615b651447d2"></a>
## fmt

`function` · `sqlparser::ast::CopyOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9320, 10], "end": [9320, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b25088f7ebaab81e823919c"></a>
## fmt

`function` · `sqlparser::ast::CopyOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9348, 1], "end": [9369, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9349`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d99b783b283c7d21e414d0"></a>
## hash

`function` · `sqlparser::ast::CopyOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9320, 56], "end": [9320, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00752643e0fb4527dcf0a78e"></a>
## partial_cmp

`function` · `sqlparser::ast::CopyOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CopyOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9320, 35], "end": [9320, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7f046603d1338dc3f41da8b"></a>
## serialize

`function` · `sqlparser::ast::CopyOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9321, 38], "end": [9321, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bac9cbd9541330a26a73f177"></a>
## visit

`function` · `sqlparser::ast::CopyOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9322, 47], "end": [9322, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f07fb446e759558f714d3e1f"></a>
## visit

`function` · `sqlparser::ast::CopyOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyOption", "path": "CopyOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9322, 40], "end": [9322, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
