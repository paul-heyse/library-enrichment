# `parquet::arrow::arrow_reader::read_plan::ReadPlan`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.read_plan.ReadPlan.json).

<a id="op-250f294604629c29b7c627e9"></a>
## ReadPlan

`struct` · `parquet::arrow::arrow_reader::read_plan::ReadPlan` · parquet 59.3.0

```rust
struct ReadPlan
```

Source: `src/arrow/arrow_reader/read_plan.rs:442`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A plan reading specific rows from a Parquet Row Group.

See [`ReadPlanBuilder`](../operations/parquet.arrow.arrow_reader.read_plan.ReadPlanBuilder.md#op-30e8c3efe26b6ff86f1af283) to create `ReadPlan`s

<a id="op-694080d540a12093e08e2dc2"></a>
## batch_size

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlan::batch_size` · parquet 59.3.0

```rust
fn batch_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlan", "path": "ReadPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [470, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:467`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the number of rows to read in each output batch

<a id="op-03d59c2292884a256fb41139"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlan::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlan", "path": "ReadPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [441, 10], "end": [441, 15], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/read_plan.rs:441`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-449a7f016d090f97d5a3bad6"></a>
## row_selection_cursor_mut

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlan::row_selection_cursor_mut` · parquet 59.3.0

```rust
fn row_selection_cursor_mut(&mut self) -> &mut RowSelectionCursor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlan", "path": "ReadPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [470, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:461`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a mutable reference to the row selection cursor

<a id="op-7fd8383ce7db0033ace3caab"></a>
## selection_mut

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlan::selection_mut` · parquet 59.3.0

```rust
fn selection_mut(&mut self) -> Option<&mut VecDeque<RowSelector>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlan", "path": "ReadPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 1], "end": [470, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:452`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a mutable reference to the selection selectors, if any
