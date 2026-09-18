# `datafusion_expr::logical_plan::plan::Extension`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Extension.json).

<a id="op-614f2e23843d2f54dfd2dbf7"></a>
## Extension

`struct` · `datafusion_expr::logical_plan::plan::Extension` · datafusion-expr 55.1.0

```rust
struct Extension
```

Source: `src/logical_plan/plan.rs:3649`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Extension operator defined outside of DataFusion

<a id="op-c2036a92a79cb2f76fab7720"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Extension::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Extension
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Extension", "path": "Extension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3648, 17], "end": [3648, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3648`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-620412f398cdf11a58eac246"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Extension::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Extension", "path": "Extension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3657, 1], "end": [3661, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3658`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-581c4c5fcfb841854e9274d5"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Extension::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Extension", "path": "Extension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3648, 10], "end": [3648, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3648`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b4dd24a22c7a9db416801c9"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Extension::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Extension", "path": "Extension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3648, 28], "end": [3648, 32], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3648`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-721baeffb9fd566adaa65625"></a>
## node

`struct_field` · `datafusion_expr::logical_plan::plan::Extension::node` · datafusion-expr 55.1.0

```rust
node: std::sync::Arc<dyn UserDefinedLogicalNode>
```

Source: `src/logical_plan/plan.rs:3651`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The runtime extension operator

<a id="op-cc3a81e3473ad01cc99828ca"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Extension::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Extension", "path": "Extension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3663, 1], "end": [3667, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3664`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
