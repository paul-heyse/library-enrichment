# `datafusion_expr::logical_plan::ddl::DropFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.DropFunction.json).

<a id="op-89caaaf3e5eba672bad2cf70"></a>
## DropFunction

`struct` · `datafusion_expr::logical_plan::ddl::DropFunction` · datafusion-expr 55.1.0

```rust
struct DropFunction
```

Source: `src/logical_plan/ddl.rs:759`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4d70a91be40df01ebdc6329"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::DropFunction::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DropFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [758, 10], "end": [758, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:758`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd198b7fdf7416ea0b5313bb"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::DropFunction::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &DropFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [758, 17], "end": [758, 26], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:758`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3897d32e66f43e22031084ec"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::DropFunction::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [758, 38], "end": [758, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:758`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1f0f4e0b7ffe341a95429d9"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::DropFunction::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [758, 32], "end": [758, 36], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:758`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62d120ac97a9dd75900d2ccf"></a>
## if_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::DropFunction::if_exists` · datafusion-expr 55.1.0

```rust
if_exists: bool
```

Source: `src/logical_plan/ddl.rs:761`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-755e295c8986b6712772a5bc"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::DropFunction::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/logical_plan/ddl.rs:760`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33c382d1d1174cb674a62dd5"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::DropFunction::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [765, 1], "end": [774, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:766`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18f4e6bae5183c6b017f85c0"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::DropFunction::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:762`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
