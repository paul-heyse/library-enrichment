# `datafusion_expr::logical_plan::plan::Distinct`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Distinct.json).

<a id="op-28302c630380381883e33eed"></a>
## Distinct

`enum` · `datafusion_expr::logical_plan::plan::Distinct` · datafusion-expr 55.1.0

```rust
enum Distinct
```

Source: `src/logical_plan/plan.rs:3742`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Removes duplicate rows from the input

<a id="op-68b9039414837ef812400d10"></a>
## All

`variant` · `datafusion_expr::logical_plan::plan::Distinct::All` · datafusion-expr 55.1.0

```rust
All
```

Source: `src/logical_plan/plan.rs:3744`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Plain `DISTINCT` referencing all selection expressions

<a id="op-05de03c0388071f7452069d1"></a>
## On

`variant` · `datafusion_expr::logical_plan::plan::Distinct::On` · datafusion-expr 55.1.0

```rust
On
```

Source: `src/logical_plan/plan.rs:3746`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The `Postgres` addition, allowing separate control over DISTINCT'd and selected columns

<a id="op-af96fb03e86ab431868de49f"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Distinct::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Distinct
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 17], "end": [3741, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3741`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-944395a85c5ef614481827fe"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Distinct::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Distinct) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 24], "end": [3741, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3741`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3daacdb1ca03d687439dae17"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Distinct::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 10], "end": [3741, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3741`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e203eecf16ae751598a8927"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Distinct::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 51], "end": [3741, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3741`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ada95335bd2f784a5c2112c5"></a>
## input

`function` · `datafusion_expr::logical_plan::plan::Distinct::input` · datafusion-expr 55.1.0

```rust
fn input(&self) -> &Arc<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3749, 1], "end": [3757, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3751`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

return a reference to the nodes input

<a id="op-9571c170cbda9b7aa6e9feb7"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Distinct::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Distinct) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 39], "end": [3741, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3741`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
