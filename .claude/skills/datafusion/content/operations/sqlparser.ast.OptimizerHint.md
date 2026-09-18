# `sqlparser::ast::OptimizerHint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OptimizerHint.json).

<a id="op-2bf9ee7614e2a50eb440a1c8"></a>
## OptimizerHint

`struct` · `sqlparser::ast::OptimizerHint` · sqlparser 0.62.0

```rust
struct OptimizerHint
```

Source: `src/ast/mod.rs:11983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query optimizer hints are optionally supported comments after the
`SELECT`, `INSERT`, `UPDATE`, `REPLACE`, `MERGE`, and `DELETE` keywords in
the corresponding statements.

See [Select::optimizer_hints](../operations/sqlparser.ast.query.Select.md#op-d62a955c614f585e49f79aa0)

<a id="op-edb9c867a85cee5945a2b342"></a>
## clone

`function` · `sqlparser::ast::OptimizerHint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OptimizerHint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11980, 17], "end": [11980, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-117f767e4b8f67d2c397f796"></a>
## cmp

`function` · `sqlparser::ast::OptimizerHint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OptimizerHint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11980, 51], "end": [11980, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-599efcb8552c89bfaec7622c"></a>
## deserialize

`function` · `sqlparser::ast::OptimizerHint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11981, 49], "end": [11981, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b00a4e9c21d51f20cd7b8f13"></a>
## eq

`function` · `sqlparser::ast::OptimizerHint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OptimizerHint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11980, 24], "end": [11980, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52f30d613ebfe54181abe615"></a>
## fmt

`function` · `sqlparser::ast::OptimizerHint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11980, 10], "end": [11980, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc778bb8d6565fe6980c2405"></a>
## fmt

`function` · `sqlparser::ast::OptimizerHint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12016, 1], "end": [12034, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:12017`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1df8e8ea95ed6358717c4904"></a>
## hash

`function` · `sqlparser::ast::OptimizerHint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11980, 56], "end": [11980, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e3ca030fbaf515b4cf58ca0"></a>
## partial_cmp

`function` · `sqlparser::ast::OptimizerHint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OptimizerHint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11980, 35], "end": [11980, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c362670f62c809a71cf60338"></a>
## prefix

`struct_field` · `sqlparser::ast::OptimizerHint::prefix` · sqlparser 0.62.0

```rust
prefix: String
```

Source: `src/ast/mod.rs:11990`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An optional prefix between the comment marker and `+`.

Standard optimizer hints like `/*+ ... */` have an empty prefix,
while system-specific hints like `/*abc+ ... */` have `prefix = "abc"`.
The prefix is any sequence of ASCII alphanumeric characters
immediately before the `+` marker.

<a id="op-29c796f2da54367b64c01385"></a>
## serialize

`function` · `sqlparser::ast::OptimizerHint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11981, 38], "end": [11981, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc879cb27024756599e6f16e"></a>
## style

`struct_field` · `sqlparser::ast::OptimizerHint::style` · sqlparser 0.62.0

```rust
style: OptimizerHintStyle
```

Source: `src/ast/mod.rs:11997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the style of the comment which `text` was extracted from,
e.g. `/*+...*/` or `--+...`

Not all dialects support all styles, though.

<a id="op-8906b673319eebb73a19a0a4"></a>
## text

`struct_field` · `sqlparser::ast::OptimizerHint::text` · sqlparser 0.62.0

```rust
text: String
```

Source: `src/ast/mod.rs:11992`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the raw text of the optimizer hint without its markers

<a id="op-74fc4e5b4dbd6c36af076ca7"></a>
## visit

`function` · `sqlparser::ast::OptimizerHint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11982, 40], "end": [11982, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a63b4c540215b385a289c73"></a>
## visit

`function` · `sqlparser::ast::OptimizerHint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OptimizerHint", "path": "OptimizerHint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11982, 47], "end": [11982, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
