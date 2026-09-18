# `datafusion_physical_plan::display::VerboseDisplay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.display.VerboseDisplay.json).

<a id="op-e654c739a7ac9693840f5430"></a>
## VerboseDisplay

`struct` · `datafusion_physical_plan::display::VerboseDisplay` · datafusion-physical-plan 55.1.0

```rust
struct VerboseDisplay<T>
```

Source: `src/display.rs:1458`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A new type wrapper to display `T` implementing `DisplayAs` using the `Verbose` mode

<a id="op-0556ecb72fba9ad3f2364921"></a>
## 0

`struct_field` · `datafusion_physical_plan::display::VerboseDisplay::0` · datafusion-physical-plan 55.1.0

```rust
0: T
```

Source: `src/display.rs:1458`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e605c7e90a2a3243be892c9"></a>
## fmt

`function` · `datafusion_physical_plan::display::VerboseDisplay::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::display::VerboseDisplay", "path": "VerboseDisplay"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1460, 1], "end": [1464, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/display.rs:1461`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
