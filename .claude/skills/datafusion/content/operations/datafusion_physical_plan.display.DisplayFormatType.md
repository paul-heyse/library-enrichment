# `datafusion_physical_plan::display::DisplayFormatType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.display.DisplayFormatType.json).

<a id="op-bb83a2fce924c64c83a12fe0"></a>
## DisplayFormatType

`enum` · `datafusion_physical_plan::display::DisplayFormatType` · datafusion-physical-plan 55.1.0

```rust
enum DisplayFormatType
```

Source: `src/display.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Options for controlling how each [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) should format itself

<a id="op-4326121ac8ec97de9309e77f"></a>
## Default

`variant` · `datafusion_physical_plan::display::DisplayFormatType::Default` · datafusion-physical-plan 55.1.0

```rust
Default
```

Source: `src/display.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Default, compact format. Example: `FilterExec: c12 < 10.0`

This format is designed to provide a detailed textual description
of all parts of the plan.

<a id="op-7c86717fca73df82a1e6ac1d"></a>
## TreeRender

`variant` · `datafusion_physical_plan::display::DisplayFormatType::TreeRender` · datafusion-physical-plan 55.1.0

```rust
TreeRender
```

Source: `src/display.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

TreeRender, displayed in the `tree` explain type.

This format is inspired by DuckDB's explain plans. The information
presented should be "user friendly", and contain only the most relevant
information for understanding a plan. It should NOT contain the same level
of detail information as the  [`Self::Default`](../operations/datafusion_physical_plan.display.DisplayFormatType.md#op-4326121ac8ec97de9309e77f) format.

In this mode, each line has one of two formats:

1. A string without a `=`, which is printed in its own line

2. A string with a `=` that is treated as a `key=value pair`. Everything
   before the first `=` is treated as the key, and everything after the
   first `=` is treated as the value.

For example, if the output of `TreeRender` is this:
```text
Parquet
partition_sizes=[1]
```

It is rendered in the center of a box in the following way:

```text
┌───────────────────────────┐
│       DataSourceExec      │
│    --------------------   │
│    partition_sizes: [1]   │
│          Parquet          │
└───────────────────────────┘
```

<a id="op-24139c6a725e9514010adc88"></a>
## Verbose

`variant` · `datafusion_physical_plan::display::DisplayFormatType::Verbose` · datafusion-physical-plan 55.1.0

```rust
Verbose
```

Source: `src/display.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Verbose, showing all available details.

This form is even more detailed than [`Self::Default`](../operations/datafusion_physical_plan.display.DisplayFormatType.md#op-4326121ac8ec97de9309e77f)

<a id="op-26b55b03db354a27d1e35ea3"></a>
## clone

`function` · `datafusion_physical_plan::display::DisplayFormatType::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> DisplayFormatType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::display::DisplayFormatType", "path": "DisplayFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 17], "end": [40, 22], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/display.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd9f8c9d0b3a6807bb66b093"></a>
## eq

`function` · `datafusion_physical_plan::display::DisplayFormatType::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &DisplayFormatType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::display::DisplayFormatType", "path": "DisplayFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 30], "end": [40, 39], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/display.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91b4970d40f7fc0a38a30a5a"></a>
## fmt

`function` · `datafusion_physical_plan::display::DisplayFormatType::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::display::DisplayFormatType", "path": "DisplayFormatType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 15], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
