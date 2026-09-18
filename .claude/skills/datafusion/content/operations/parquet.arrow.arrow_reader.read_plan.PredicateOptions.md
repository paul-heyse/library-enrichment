# `parquet::arrow::arrow_reader::read_plan::PredicateOptions`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.read_plan.PredicateOptions.json).

<a id="op-192292cc7a9c950140c706b9"></a>
## PredicateOptions

`struct` · `parquet::arrow::arrow_reader::read_plan::PredicateOptions` · parquet 59.3.0

```rust
struct PredicateOptions<'a>
```

Source: `src/arrow/arrow_reader/read_plan.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Options for [`ReadPlanBuilder::with_predicate_options`](../operations/parquet.arrow.arrow_reader.read_plan.ReadPlanBuilder.md#op-8f1b196ed4f6fa92a341263d).

<a id="op-c91e37b3ec4c9982b29598ff"></a>
## new

`function` · `parquet::arrow::arrow_reader::read_plan::PredicateOptions::new` · parquet 59.3.0

```rust
fn new(array_reader: Box<dyn ArrayReader>, predicate: &'a mut dyn ArrowPredicate) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::read_plan::PredicateOptions", "path": "PredicateOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [79, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:50`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create options for evaluating `predicate` against rows produced by
`array_reader`.

By default there is no match-count limit; the predicate is evaluated
over every row the reader yields. Use [`Self::with_limit`](../operations/parquet.arrow.arrow_reader.read_plan.PredicateOptions.md#op-4405a2fd7a8ab3e2bf5540c0) to enable
early termination.

<a id="op-4405a2fd7a8ab3e2bf5540c0"></a>
## with_limit

`function` · `parquet::arrow::arrow_reader::read_plan::PredicateOptions::with_limit` · parquet 59.3.0

```rust
fn with_limit(self, limit: usize, total_rows: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::read_plan::PredicateOptions", "path": "PredicateOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [79, 2], "filename": "src/arrow/arrow_reader/read_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/read_plan.rs:74`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Stop scanning `array_reader` once `limit` matches have accumulated.

Performance optimization for `LIMIT` / TopK: when the cumulative
`true_count` reaches `limit`, the current filter batch is truncated
at the `limit`-th match and remaining batches are never decoded.

`limit` counts predicate matches, not output rows — callers applying
an offset must pass `offset + limit`.

`total_rows` is the row count `array_reader` would yield if iterated
to completion. It is used to pad un-evaluated trailing rows as "not
selected" so the returned [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) covers the full row group.

Only valid for the *last* predicate in a filter chain: intermediate
predicates' match counts do not map 1:1 to output rows.
