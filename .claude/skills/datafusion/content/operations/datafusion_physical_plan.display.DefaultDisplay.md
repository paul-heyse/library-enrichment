# `datafusion_physical_plan::display::DefaultDisplay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.display.DefaultDisplay.json).

<a id="op-1315a2f379fa506db7e67eda"></a>
## DefaultDisplay

`struct` · `datafusion_physical_plan::display::DefaultDisplay` · datafusion-physical-plan 55.1.0

```rust
struct DefaultDisplay<T>
```

Source: `src/display.rs:1449`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A new type wrapper to display `T` implementing`DisplayAs` using the `Default` mode

<a id="op-93e6f43bee8bec0aa39843bb"></a>
## 0

`struct_field` · `datafusion_physical_plan::display::DefaultDisplay::0` · datafusion-physical-plan 55.1.0

```rust
0: T
```

Source: `src/display.rs:1449`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e03dfd3ddd4083509c7e1c6f"></a>
## fmt

`function` · `datafusion_physical_plan::display::DefaultDisplay::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::display::DefaultDisplay", "path": "DefaultDisplay"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1451, 1], "end": [1455, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/display.rs:1452`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
