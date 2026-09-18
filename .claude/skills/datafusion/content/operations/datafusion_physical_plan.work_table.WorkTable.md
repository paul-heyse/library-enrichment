# `datafusion_physical_plan::work_table::WorkTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.work_table.WorkTable.json).

<a id="op-8d9b6e8105db5401bab36667"></a>
## WorkTable

`struct` · `datafusion_physical_plan::work_table::WorkTable` · datafusion-physical-plan 55.1.0

```rust
struct WorkTable
```

Source: `src/work_table.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The name is from PostgreSQL's terminology.
See <https://wiki.postgresql.org/wiki/CTEReadme#How_Recursion_Works>
This table serves as a mirror or buffer between each iteration of a recursive query.

<a id="op-bf582b37b8beb725c98073c9"></a>
## fmt

`function` · `datafusion_physical_plan::work_table::WorkTable::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::work_table::WorkTable", "path": "WorkTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 15], "filename": "src/work_table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/work_table.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
