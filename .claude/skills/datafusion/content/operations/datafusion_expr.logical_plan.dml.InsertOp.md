# `datafusion_expr::logical_plan::dml::InsertOp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.InsertOp.json).

<a id="op-73020397ce56e45da7fa9592"></a>
## InsertOp

`enum` · `datafusion_expr::logical_plan::dml::InsertOp` · datafusion-expr 55.1.0

```rust
enum InsertOp
```

Source: `src/logical_plan/dml.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b10ef7bde4e6b684c30c155"></a>
## Append

`variant` · `datafusion_expr::logical_plan::dml::InsertOp::Append` · datafusion-expr 55.1.0

```rust
Append
```

Source: `src/logical_plan/dml.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Appends new rows to the existing table without modifying any
existing rows. This corresponds to the SQL `INSERT INTO` query.

<a id="op-a9a68c3d4760dfae53544a36"></a>
## Overwrite

`variant` · `datafusion_expr::logical_plan::dml::InsertOp::Overwrite` · datafusion-expr 55.1.0

```rust
Overwrite
```

Source: `src/logical_plan/dml.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Overwrites all existing rows in the table with the new rows.
This corresponds to the SQL `INSERT OVERWRITE` query.

<a id="op-340e30d101617626f134d4f2"></a>
## Replace

`variant` · `datafusion_expr::logical_plan::dml::InsertOp::Replace` · datafusion-expr 55.1.0

```rust
Replace
```

Source: `src/logical_plan/dml.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If any existing rows collides with the inserted rows (typically based
on a unique key or primary key), those existing rows are replaced.
This corresponds to the SQL `REPLACE INTO` query and its equivalents.

<a id="op-75703e960cb98cee1fd49a5d"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::InsertOp::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> InsertOp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 17], "end": [270, 22], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16933fdcd9ade8e9766156d3"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::InsertOp::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &InsertOp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 30], "end": [270, 39], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7877afd1f52215191323f7e9"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::InsertOp::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [299, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/logical_plan/dml.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9fc543d6705694fca8a0d20"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::InsertOp::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 10], "end": [270, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5341bfbcc7844607037325ec"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::InsertOp::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 57], "end": [270, 61], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-322b19a1eb32c72f4b9a6e80"></a>
## name

`function` · `datafusion_expr::logical_plan::dml::InsertOp::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [293, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a descriptive name of this [`InsertOp`](../operations/datafusion_expr.logical_plan.dml.InsertOp.md#op-73020397ce56e45da7fa9592)

<a id="op-ff80d75163a97a85250d0c55"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::InsertOp::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &InsertOp) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::InsertOp", "path": "InsertOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [270, 45], "end": [270, 55], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
