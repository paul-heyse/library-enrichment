# `parquet::arrow::arrow_reader::read_plan`

Crate `parquet` · 3 public items · structured records in [`model/parquet.arrow.arrow_reader.read_plan.json`](../model/parquet.arrow.arrow_reader.read_plan.json)

## PredicateOptions

`struct` · `parquet::arrow::arrow_reader::read_plan::PredicateOptions`

Also reachable as `parquet::arrow::arrow_reader::PredicateOptions`

```rust
struct PredicateOptions<'a>
```

**Methods** (2)

```rust
fn new(array_reader: Box<dyn ArrayReader>, predicate: &'a mut dyn ArrowPredicate) -> Self
fn with_limit(self, limit: usize, total_rows: usize) -> Self
```

Options for [`ReadPlanBuilder::with_predicate_options`].

---

## ReadPlan

`struct` · `parquet::arrow::arrow_reader::read_plan::ReadPlan`

Also reachable as `parquet::arrow::arrow_reader::ReadPlan`

```rust
struct ReadPlan
```

**Derives**: Debug

**Methods** (3)

```rust
fn batch_size(&self) -> usize
fn row_selection_cursor_mut(&mut self) -> &mut RowSelectionCursor
fn selection_mut(&mut self) -> Option<&mut VecDeque<RowSelector>>
```

A plan reading specific rows from a Parquet Row Group.

See [`ReadPlanBuilder`] to create `ReadPlan`s

---

## ReadPlanBuilder

`struct` · `parquet::arrow::arrow_reader::read_plan::ReadPlanBuilder`

Also reachable as `parquet::arrow::arrow_reader::ReadPlanBuilder`

```rust
struct ReadPlanBuilder
```

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn build(self) -> ReadPlan
fn new(batch_size: usize) -> Self
fn num_rows_selected(&self) -> Option<usize>
fn row_selection_policy(&self) -> &RowSelectionPolicy
fn selection(&self) -> Option<&RowSelection>
fn selects_any(&self) -> bool
fn with_predicate(self, array_reader: Box<dyn ArrayReader>, predicate: &mut dyn ArrowPredicate) -> Result<Self>
fn with_predicate_options(self, options: PredicateOptions<'_>) -> Result<Self>
fn with_row_selection_policy(self, policy: RowSelectionPolicy) -> Self
fn with_selection(self, selection: Option<RowSelection>) -> Self
```

A builder for [`ReadPlan`]

---
