# `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.json).

<a id="op-204596f00d03b5d0c6d4dde1"></a>
## GroupColumn

`trait` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn` · datafusion-physical-plan 55.1.0

```rust
trait GroupColumn: Send + Sync
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Trait for storing a single column of group values in [`GroupValuesColumn`]

Implementations of this trait store an in-progress collection of group values
(similar to various builders in Arrow-rs) that allow for quick comparison to
incoming rows.

[`GroupValuesColumn`]: crate::aggregates::group_values::GroupValuesColumn

<a id="op-54ef532686279ebc6e2ffaac"></a>
## append_val

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::append_val` · datafusion-physical-plan 55.1.0

```rust
fn append_val(&mut self, array: &ArrayRef, row: usize) -> Result<()>
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Appends the row at `row` in `array` to this builder

<a id="op-9aabdcf28ac24bd9ee395d91"></a>
## build

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::build` · datafusion-physical-plan 55.1.0

```rust
fn build(Box<self>) -> ArrayRef
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builds a new array from all of the stored rows

<a id="op-3990906b4b3b05f0b11bde14"></a>
## equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::equal_to` · datafusion-physical-plan 55.1.0

```rust
fn equal_to(&self, lhs_row: usize, array: &ArrayRef, rhs_row: usize) -> bool
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns equal if the row stored in this builder at `lhs_row` is equal to
the row in `array` at `rhs_row`

Note that this comparison returns true if both elements are NULL

<a id="op-73f11b7804cb920380a8d77d"></a>
## is_empty

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

true if len == 0

<a id="op-8dd81db56948126ac6e8bbc1"></a>
## len

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number of rows stored in this builder

<a id="op-fdccb0c207d9970c05e8e6a5"></a>
## size

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number of bytes used by this [`GroupColumn`](../operations/datafusion_physical_plan.aggregates.group_values.multi_group_by.GroupColumn.md#op-204596f00d03b5d0c6d4dde1)

<a id="op-a1f60d6b10840188fe5e313c"></a>
## take_n

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::take_n` · datafusion-physical-plan 55.1.0

```rust
fn take_n(&mut self, n: usize) -> ArrayRef
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builds a new array from the first `n` stored rows, shifting the
remaining rows to the start of the builder

<a id="op-a496efce921aa90a558aec86"></a>
## vectorized_append

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::vectorized_append` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_append(&mut self, array: &ArrayRef, rows: &[usize]) -> Result<()>
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The vectorized version `append_val`

<a id="op-5d600606f43b0f23dde1c095"></a>
## vectorized_equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::GroupColumn::vectorized_equal_to` · datafusion-physical-plan 55.1.0

```rust
fn vectorized_equal_to(&self, lhs_rows: &[usize], array: &ArrayRef, rhs_rows: &[usize], equal_to_results: &mut BooleanBufferBuilder)
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The vectorized version equal to

When found nth row stored in this builder at `lhs_row`
is equal to the row in `array` at `rhs_row`,
it will record the `true` result at the corresponding
position in `equal_to_results`.

And if found nth result in `equal_to_results` is already
`false`, the check for nth row will be skipped.
