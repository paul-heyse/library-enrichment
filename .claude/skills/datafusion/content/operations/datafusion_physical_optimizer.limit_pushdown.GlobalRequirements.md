# `datafusion_physical_optimizer::limit_pushdown::GlobalRequirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.limit_pushdown.GlobalRequirements.json).

<a id="op-7035f412cc688fb64880d925"></a>
## GlobalRequirements

`struct` · `datafusion_physical_optimizer::limit_pushdown::GlobalRequirements` · datafusion-physical-optimizer 55.1.0

```rust
struct GlobalRequirements
```

Source: `src/limit_pushdown.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This is a "data class" we use within the [`LimitPushdown`] rule to push
down limits in the plan. GlobalRequirements are hold as a rule-wide state
and holds the fetch and skip information. The struct also has a field named
satisfied which means if the "current" plan is valid in terms of limits or not.

For example: If the plan is satisfied with current fetch info, we decide to not add a LocalLimit

[`LimitPushdown`]: crate::limit_pushdown::LimitPushdown

<a id="op-8b6cead3df8029d6dfc0d828"></a>
## clone

`function` · `datafusion_physical_optimizer::limit_pushdown::GlobalRequirements::clone` · datafusion-physical-optimizer 55.1.0

```rust
fn clone(&self) -> GlobalRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::GlobalRequirements", "path": "GlobalRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 19], "end": [95, 24], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/limit_pushdown.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d62bde2124c516a8dffbf33"></a>
## default

`function` · `datafusion_physical_optimizer::limit_pushdown::GlobalRequirements::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> GlobalRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::GlobalRequirements", "path": "GlobalRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 17], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/limit_pushdown.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5388613008d3b75462e9e38"></a>
## fmt

`function` · `datafusion_physical_optimizer::limit_pushdown::GlobalRequirements::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown::GlobalRequirements", "path": "GlobalRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 26], "end": [95, 31], "filename": "src/limit_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/limit_pushdown.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
