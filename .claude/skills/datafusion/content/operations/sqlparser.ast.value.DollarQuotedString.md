# `sqlparser::ast::value::DollarQuotedString`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.DollarQuotedString.json).

<a id="op-e972c19ba749b036364b50c9"></a>
## DollarQuotedString

`struct` · `sqlparser::ast::value::DollarQuotedString` · sqlparser 0.62.0

```rust
struct DollarQuotedString
```

Source: `src/ast/value.rs:299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A dollar-quoted string literal, e.g. `$$...$$` or `$tag$...$tag$`.

<a id="op-e01ac8100ccf89e2a9423ddf"></a>
## clone

`function` · `sqlparser::ast::value::DollarQuotedString::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DollarQuotedString
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 17], "end": [296, 22], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/value.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-683bba03754b842c1b9c97d8"></a>
## cmp

`function` · `sqlparser::ast::value::DollarQuotedString::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DollarQuotedString) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 51], "end": [296, 54], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/value.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3792215abf67fc905020c126"></a>
## deserialize

`function` · `sqlparser::ast::value::DollarQuotedString::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 49], "end": [297, 60], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/value.rs:297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-383afb7469011ea304f94f45"></a>
## eq

`function` · `sqlparser::ast::value::DollarQuotedString::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DollarQuotedString) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 24], "end": [296, 33], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/value.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e18bd283d9a13a77d347a19"></a>
## fmt

`function` · `sqlparser::ast::value::DollarQuotedString::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 1], "end": [317, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/value.rs:307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d26c3bb86a1bc5c8ae483641"></a>
## fmt

`function` · `sqlparser::ast::value::DollarQuotedString::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 10], "end": [296, 15], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/value.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fb2c41b304f5ebaba580677"></a>
## hash

`function` · `sqlparser::ast::value::DollarQuotedString::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 56], "end": [296, 60], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/value.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab65ff90b58dbc2d677c16cc"></a>
## partial_cmp

`function` · `sqlparser::ast::value::DollarQuotedString::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DollarQuotedString) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 35], "end": [296, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/value.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b02da6928ba5641e6ff7f608"></a>
## serialize

`function` · `sqlparser::ast::value::DollarQuotedString::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 38], "end": [297, 47], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/value.rs:297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8906d458a95784c9314a713"></a>
## tag

`struct_field` · `sqlparser::ast::value::DollarQuotedString::tag` · sqlparser 0.62.0

```rust
tag: Option<String>
```

Source: `src/ast/value.rs:303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional tag used in the opening/closing delimiter.

<a id="op-48f98bf13a7bbca18386d70e"></a>
## value

`struct_field` · `sqlparser::ast::value::DollarQuotedString::value` · sqlparser 0.62.0

```rust
value: String
```

Source: `src/ast/value.rs:301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Inner string contents.

<a id="op-0960e432f3aa8ad912d4bb89"></a>
## visit

`function` · `sqlparser::ast::value::DollarQuotedString::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 47], "end": [298, 55], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/value.rs:298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad746f00a4f31ce164c5e6d4"></a>
## visit

`function` · `sqlparser::ast::value::DollarQuotedString::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DollarQuotedString", "path": "DollarQuotedString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 40], "end": [298, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/value.rs:298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
