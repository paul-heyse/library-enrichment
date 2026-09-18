# `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.read_plan.ReadPlanBuilder.json).

<a id="op-30e8c3efe26b6ff86f1af283"></a>
## ReadPlanBuilder

`struct` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder` · parquet 59.3.0

```rust
struct ReadPlanBuilder
```

Source: `src/arrow/arrow_reader/read_plan.rs:83`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A builder for [`ReadPlan`](../operations/parquet.arrow.arrow_reader.read_plan.ReadPlan.md#op-250f294604629c29b7c627e9)

<a id="op-6622efaec04e2814e91d964a"></a>
## build

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::build` · parquet 59.3.0

```rust
fn build(self) -> ReadPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:294`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a final `ReadPlan` the read plan for the scan

<a id="op-ea78902f68e459396ce6402f"></a>
## clone

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ReadPlanBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 10], "end": [82, 15], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_reader/read_plan.rs:82`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19c21e403fecfa532ce89417"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 17], "end": [82, 22], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/read_plan.rs:82`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74e0d5bd9990214c2c125cf4"></a>
## new

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::new` · parquet 59.3.0

```rust
fn new(batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:95`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a `ReadPlanBuilder` with the given batch size

<a id="op-22d8e2fb0dfba7805e30583c"></a>
## num_rows_selected

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::num_rows_selected` · parquet 59.3.0

```rust
fn num_rows_selected(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:153`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of rows selected, or `None` if all rows are selected.

<a id="op-3f172cb9cc0aa1d06f63c438"></a>
## row_selection_policy

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::row_selection_policy` · parquet 59.3.0

```rust
fn row_selection_policy(&self) -> &RowSelectionPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:124`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the current row selection policy

<a id="op-89f06d54ac300cc6d86fd8d6"></a>
## selection

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::selection` · parquet 59.3.0

```rust
fn selection(&self) -> Option<&RowSelection>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:129`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the current selection, if any

<a id="op-b79b4d5234760da06382a657"></a>
## selects_any

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::selects_any` · parquet 59.3.0

```rust
fn selects_any(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:145`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns true if the current plan selects any rows

<a id="op-aac51dbf6857ef8a33e757bf"></a>
## with_predicate

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::with_predicate` · parquet 59.3.0

```rust
fn with_predicate(self, array_reader: Box<dyn ArrayReader>, predicate: &mut dyn ArrowPredicate) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:184`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Evaluates an [`ArrowPredicate`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicate.md#op-bf093cc5f111fb8b2d44eb01), updating this plan's `selection`

If the current `selection` is `Some`, the resulting [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)
will be the conjunction of the existing selection and the rows selected
by `predicate`.

Note: pre-existing selections may come from evaluating a previous predicate
or if the [`ParquetRecordBatchReader`](../operations/parquet.arrow.arrow_reader.ParquetRecordBatchReader.md#op-d346feb61c9116e17f0cd9c3) specified an explicit
[`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) in addition to one or more predicates.

<a id="op-8f1b196ed4f6fa92a341263d"></a>
## with_predicate_options

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::with_predicate_options` · parquet 59.3.0

```rust
fn with_predicate_options(self, options: PredicateOptions<'_>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Evaluates an [`ArrowPredicate`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicate.md#op-bf093cc5f111fb8b2d44eb01) with the given [`PredicateOptions`](../operations/parquet.arrow.arrow_reader.read_plan.PredicateOptions.md#op-192292cc7a9c950140c706b9),
updating this plan's `selection`.

Like [`Self::with_predicate`](../operations/parquet.arrow.arrow_reader.read_plan.ReadPlanBuilder.md#op-aac51dbf6857ef8a33e757bf), but allows additional options such as a
match-count limit for early termination (see
[`PredicateOptions::with_limit`](../operations/parquet.arrow.arrow_reader.read_plan.PredicateOptions.md#op-4405a2fd7a8ab3e2bf5540c0)).

<a id="op-f4d28b56c73ff3e6e330a512"></a>
## with_row_selection_policy

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::with_row_selection_policy` · parquet 59.3.0

```rust
fn with_row_selection_policy(self, policy: RowSelectionPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:113`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Configure the policy to use when materialising the [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5)

Defaults to [`RowSelectionPolicy::Auto`](../operations/parquet.arrow.arrow_reader.selection.cursor.RowSelectionPolicy.md#op-3e7fb3598db721a8e8b8d9c6)

<a id="op-1d92bbe6e682b8eab6272353"></a>
## with_selection

`function` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder::with_selection` · parquet 59.3.0

```rust
fn with_selection(self, selection: Option<RowSelection>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder", "path": "ReadPlanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [319, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:105`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Set the current selection to the given value
