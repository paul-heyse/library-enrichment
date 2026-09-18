# `datafusion_physical_plan::execution_plan::EmissionType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.EmissionType.json).

<a id="op-05c5e3937550b6fc12eae8da"></a>
## EmissionType

`enum` · `datafusion_physical_plan::execution_plan::EmissionType` · datafusion-physical-plan 55.1.0

```rust
enum EmissionType
```

Source: `src/execution_plan.rs:1346`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Represents how an operator emits its output records.

This is used to determine whether an operator emits records incrementally as they arrive,
only emits a final result at the end, or can do both. Note that it generates the output -- record batch with `batch_size` rows
but it may still buffer data internally until it has enough data to emit a record batch or the source is exhausted.

For example, in the following plan:
```text
  SortExec [EmissionType::Final]
    |_ on: [col1 ASC]
    FilterExec [EmissionType::Incremental]
      |_ pred: col2 > 100
      DataSourceExec [EmissionType::Incremental]
        |_ file: "data.csv"
```
- DataSourceExec emits records incrementally as it reads from the file
- FilterExec processes and emits filtered records incrementally as they arrive
- SortExec must wait for all input records before it can emit the sorted result,
  since it needs to see all values to determine their final order

Left joins can emit both incrementally and finally:
- Incrementally emit matches as they are found
- Finally emit non-matches after all input is processed

<a id="op-ff29ff29955a04b7e9d8267d"></a>
## Both

`variant` · `datafusion_physical_plan::execution_plan::EmissionType::Both` · datafusion-physical-plan 55.1.0

```rust
Both
```

Source: `src/execution_plan.rs:1352`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Records can be emitted both incrementally and as a final result

<a id="op-625804c3976566fc01a64050"></a>
## Final

`variant` · `datafusion_physical_plan::execution_plan::EmissionType::Final` · datafusion-physical-plan 55.1.0

```rust
Final
```

Source: `src/execution_plan.rs:1350`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Records are only emitted once all input has been processed

<a id="op-34dfb60f79b6c701b6ed56b0"></a>
## Incremental

`variant` · `datafusion_physical_plan::execution_plan::EmissionType::Incremental` · datafusion-physical-plan 55.1.0

```rust
Incremental
```

Source: `src/execution_plan.rs:1348`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Records are emitted incrementally as they arrive and are processed

<a id="op-e58b2bde7146591c3f65407e"></a>
## clone

`function` · `datafusion_physical_plan::execution_plan::EmissionType::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> EmissionType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::EmissionType", "path": "EmissionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1345, 17], "end": [1345, 22], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_plan.rs:1345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ec1f95d5542cd7c9197080f"></a>
## eq

`function` · `datafusion_physical_plan::execution_plan::EmissionType::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &EmissionType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::EmissionType", "path": "EmissionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1345, 30], "end": [1345, 39], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/execution_plan.rs:1345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f78814da45ef81ddfd68d18"></a>
## fmt

`function` · `datafusion_physical_plan::execution_plan::EmissionType::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::EmissionType", "path": "EmissionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1345, 10], "end": [1345, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:1345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
