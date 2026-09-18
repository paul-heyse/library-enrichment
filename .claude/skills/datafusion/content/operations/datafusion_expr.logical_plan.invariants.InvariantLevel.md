# `datafusion_expr::logical_plan::invariants::InvariantLevel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.invariants.InvariantLevel.json).

<a id="op-46069243d94c3adfc97eed26"></a>
## InvariantLevel

`enum` · `datafusion_expr::logical_plan::invariants::InvariantLevel` · datafusion-expr 55.1.0

```rust
enum InvariantLevel
```

Source: `src/logical_plan/invariants.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3271b45be88f980009ff9eff"></a>
## Always

`variant` · `datafusion_expr::logical_plan::invariants::InvariantLevel::Always` · datafusion-expr 55.1.0

```rust
Always
```

Source: `src/logical_plan/invariants.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invariants that are always true in DataFusion `LogicalPlan`s
such as the number of expected children and no duplicated output fields

<a id="op-aa1700a7e83e645770ebe369"></a>
## Executable

`variant` · `datafusion_expr::logical_plan::invariants::InvariantLevel::Executable` · datafusion-expr 55.1.0

```rust
Executable
```

Source: `src/logical_plan/invariants.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invariants that must hold true for the plan to be "executable"
such as the type and number of function arguments are correct and
that wildcards have been expanded

To ensure a LogicalPlan satisfies the `Executable` invariants, run the
`Analyzer`

<a id="op-66d6e7d0189258de42709cb3"></a>
## clone

`function` · `datafusion_expr::logical_plan::invariants::InvariantLevel::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> InvariantLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::invariants::InvariantLevel", "path": "InvariantLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 23], "end": [32, 28], "filename": "src/logical_plan/invariants.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/invariants.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abbd493725047168f23aa97a"></a>
## eq

`function` · `datafusion_expr::logical_plan::invariants::InvariantLevel::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &InvariantLevel) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::invariants::InvariantLevel", "path": "InvariantLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 30], "end": [32, 39], "filename": "src/logical_plan/invariants.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/invariants.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-643158bd073c9c11c53e43a7"></a>
## fmt

`function` · `datafusion_expr::logical_plan::invariants::InvariantLevel::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::invariants::InvariantLevel", "path": "InvariantLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/logical_plan/invariants.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/invariants.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce7867250d653966d96d262e"></a>
## hash

`function` · `datafusion_expr::logical_plan::invariants::InvariantLevel::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::invariants::InvariantLevel", "path": "InvariantLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 57], "end": [32, 61], "filename": "src/logical_plan/invariants.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/invariants.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeec7840b3b90d67a594d22a"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::invariants::InvariantLevel::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &InvariantLevel) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::invariants::InvariantLevel", "path": "InvariantLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 45], "end": [32, 55], "filename": "src/logical_plan/invariants.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/invariants.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
