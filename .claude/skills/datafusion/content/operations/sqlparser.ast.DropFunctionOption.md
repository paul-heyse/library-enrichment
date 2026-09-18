# `sqlparser::ast::DropFunctionOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DropFunctionOption.json).

<a id="op-fca73b8d1972978f1ceb04b8"></a>
## DropFunctionOption

`enum` · `sqlparser::ast::DropFunctionOption` · sqlparser 0.62.0

```rust
enum DropFunctionOption
```

Source: `src/ast/mod.rs:9834`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function describe in DROP FUNCTION.

<a id="op-13fc16be89e211f20f8c022d"></a>
## Cascade

`variant` · `sqlparser::ast::DropFunctionOption::Cascade` · sqlparser 0.62.0

```rust
Cascade
```

Source: `src/ast/mod.rs:9838`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE` option for DROP FUNCTION.

<a id="op-1ed3e4dadb8e7c13ce0eda45"></a>
## Restrict

`variant` · `sqlparser::ast::DropFunctionOption::Restrict` · sqlparser 0.62.0

```rust
Restrict
```

Source: `src/ast/mod.rs:9836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RESTRICT` option for DROP FUNCTION.

<a id="op-9c91dc07084be837ff438886"></a>
## clone

`function` · `sqlparser::ast::DropFunctionOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropFunctionOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9832, 17], "end": [9832, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5a9c56c179ed6a1ad115aa7"></a>
## cmp

`function` · `sqlparser::ast::DropFunctionOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropFunctionOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9832, 51], "end": [9832, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88f6370cfc36d8d82c76edeb"></a>
## deserialize

`function` · `sqlparser::ast::DropFunctionOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9833, 49], "end": [9833, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d42ea53ab4468965f23ae232"></a>
## eq

`function` · `sqlparser::ast::DropFunctionOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropFunctionOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9832, 24], "end": [9832, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e426ebff1a0a72400f785587"></a>
## fmt

`function` · `sqlparser::ast::DropFunctionOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9841, 1], "end": [9848, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9842`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec019b2e12e92536cbb3df96"></a>
## fmt

`function` · `sqlparser::ast::DropFunctionOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9832, 10], "end": [9832, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-163aac786b79162d3a418966"></a>
## hash

`function` · `sqlparser::ast::DropFunctionOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9832, 56], "end": [9832, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3e509a9b163f12bda94dcb6"></a>
## partial_cmp

`function` · `sqlparser::ast::DropFunctionOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropFunctionOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9832, 35], "end": [9832, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6ff6c6030b7873b8b8db57a"></a>
## serialize

`function` · `sqlparser::ast::DropFunctionOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropFunctionOption", "path": "DropFunctionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9833, 38], "end": [9833, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
