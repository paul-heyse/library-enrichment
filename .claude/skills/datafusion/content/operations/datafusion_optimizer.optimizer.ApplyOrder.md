# `datafusion_optimizer::optimizer::ApplyOrder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimizer.ApplyOrder.json).

<a id="op-f004ab45507daf3f6c265a54"></a>
## ApplyOrder

`enum` · `datafusion_optimizer::optimizer::ApplyOrder` · datafusion-optimizer 55.1.0

```rust
enum ApplyOrder
```

Source: `src/optimizer.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Specifies how recursion for an `OptimizerRule` should be handled.

* `Some(apply_order)`: The Optimizer will recursively apply the rule to the plan.
* `None`: the rule must handle any required recursion itself.

<a id="op-e8e47b44ba80562918beb7b2"></a>
## BottomUp

`variant` · `datafusion_optimizer::optimizer::ApplyOrder::BottomUp` · datafusion-optimizer 55.1.0

```rust
BottomUp
```

Source: `src/optimizer.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Apply the rule to the node after its inputs

<a id="op-f993efd2be89c1538c913e7e"></a>
## TopDown

`variant` · `datafusion_optimizer::optimizer::ApplyOrder::TopDown` · datafusion-optimizer 55.1.0

```rust
TopDown
```

Source: `src/optimizer.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Apply the rule to the node before its inputs

<a id="op-fe88866c6f2149f682af91e1"></a>
## clone

`function` · `datafusion_optimizer::optimizer::ApplyOrder::clone` · datafusion-optimizer 55.1.0

```rust
fn clone(&self) -> ApplyOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::ApplyOrder", "path": "ApplyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 17], "end": [264, 22], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/optimizer.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4d492a927e12906e604ab50"></a>
## eq

`function` · `datafusion_optimizer::optimizer::ApplyOrder::eq` · datafusion-optimizer 55.1.0

```rust
fn eq(&self, other: &ApplyOrder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::ApplyOrder", "path": "ApplyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 30], "end": [264, 39], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/optimizer.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90247372592cd3df53df0fab"></a>
## fmt

`function` · `datafusion_optimizer::optimizer::ApplyOrder::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::ApplyOrder", "path": "ApplyOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 10], "end": [264, 15], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/optimizer.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
