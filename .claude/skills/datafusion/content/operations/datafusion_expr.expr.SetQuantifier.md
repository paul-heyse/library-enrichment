# `datafusion_expr::expr::SetQuantifier`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.SetQuantifier.json).

<a id="op-a348d4343e93a9fe0ab524a7"></a>
## SetQuantifier

`enum` · `datafusion_expr::expr::SetQuantifier` · datafusion-expr 55.1.0

```rust
enum SetQuantifier
```

Source: `src/expr.rs:1314`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the set comparison uses `ANY`/`SOME` or `ALL`

<a id="op-26fef2133c022b6b0baf2e9d"></a>
## All

`variant` · `datafusion_expr::expr::SetQuantifier::All` · datafusion-expr 55.1.0

```rust
All
```

Source: `src/expr.rs:1318`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`ALL`

<a id="op-cb91275d29e55aa41cdce4b5"></a>
## Any

`variant` · `datafusion_expr::expr::SetQuantifier::Any` · datafusion-expr 55.1.0

```rust
Any
```

Source: `src/expr.rs:1316`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`ANY` (or `SOME`)

<a id="op-9fa69f0d351d993cfb2017b6"></a>
## clone

`function` · `datafusion_expr::expr::SetQuantifier::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SetQuantifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1313, 10], "end": [1313, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1313`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8820e425d320770f334b49d"></a>
## eq

`function` · `datafusion_expr::expr::SetQuantifier::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SetQuantifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1313, 23], "end": [1313, 32], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1313`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32dc013aa7241b093f72270a"></a>
## fmt

`function` · `datafusion_expr::expr::SetQuantifier::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1321, 1], "end": [1328, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:1322`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74bf59ced2b52189803d3993"></a>
## fmt

`function` · `datafusion_expr::expr::SetQuantifier::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1313, 56], "end": [1313, 61], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1313`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc1807652557df893857751"></a>
## hash

`function` · `datafusion_expr::expr::SetQuantifier::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1313, 50], "end": [1313, 54], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1313`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec78457aca4fbddbe2ccb0ec"></a>
## partial_cmp

`function` · `datafusion_expr::expr::SetQuantifier::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &SetQuantifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::SetQuantifier", "path": "SetQuantifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1313, 38], "end": [1313, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1313`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
