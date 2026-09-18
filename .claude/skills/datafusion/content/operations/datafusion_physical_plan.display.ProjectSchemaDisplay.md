# `datafusion_physical_plan::display::ProjectSchemaDisplay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.display.ProjectSchemaDisplay.json).

<a id="op-e7bf5dcdaa5e21b522fc7c6e"></a>
## ProjectSchemaDisplay

`struct` · `datafusion_physical_plan::display::ProjectSchemaDisplay` · datafusion-physical-plan 55.1.0

```rust
struct ProjectSchemaDisplay<'a>
```

Source: `src/display.rs:1468`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A wrapper to customize partitioned file display

<a id="op-3a696ef7b77214e93780455c"></a>
## 0

`struct_field` · `datafusion_physical_plan::display::ProjectSchemaDisplay::0` · datafusion-physical-plan 55.1.0

```rust
0: &'a arrow::datatypes::SchemaRef
```

Source: `src/display.rs:1468`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a79b47c83a496d9a1f75a69"></a>
## fmt

`function` · `datafusion_physical_plan::display::ProjectSchemaDisplay::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_plan::display::ProjectSchemaDisplay", "path": "ProjectSchemaDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1470, 1], "end": [1480, 2], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/display.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-722173c7f177b2049637f51c"></a>
## fmt

`function` · `datafusion_physical_plan::display::ProjectSchemaDisplay::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::display::ProjectSchemaDisplay", "path": "ProjectSchemaDisplay"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1467, 10], "end": [1467, 15], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display.rs:1467`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
