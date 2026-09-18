# `buoyant_kernel::expressions::ExpressionRef`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.ExpressionRef.json).

<a id="op-7054cb2b00981e6ab8964f61"></a>
## ExpressionRef

`type_alias` · `buoyant_kernel::expressions::ExpressionRef` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type ExpressionRef = std::sync::Arc<Expression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L34).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:34`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de933b82a88025bdd7d9d237"></a>
## expr

`function` · `buoyant_kernel::expressions::ExpressionRef::expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn expr(&self) -> &ExpressionRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L495).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::ExpressionRef", "path": "crate::expressions::ExpressionRef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [494, 1], "end": [498, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionItem", "path": "ExpressionItem"}, "trait_path": "buoyant_kernel::struct_patch::ExpressionItem"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:495`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
