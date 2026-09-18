# `datafusion_datasource_parquet::access_plan::ParquetAccessPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.access_plan.ParquetAccessPlan.json).

<a id="op-2786e9792f5bf477380471b9"></a>
## ParquetAccessPlan

`struct` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan` · datafusion-datasource-parquet 55.1.0

```rust
struct ParquetAccessPlan
```

Source: `src/access_plan.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

A selection of rows and row groups within a ParquetFile to decode.

A `ParquetAccessPlan` is used to limit the row groups and data pages a `DataSourceExec`
will read and decode to improve performance.

Note that page level pruning based on ArrowPredicate is applied after all of
these selections

# Example

For example, given a Parquet file with 4 row groups, a `ParquetAccessPlan`
can be used to specify skipping row group 0 and 2, scanning a range of rows
in row group 1, and scanning all rows in row group 3 as follows:

```rust
# use parquet::arrow::arrow_reader::{RowSelection, RowSelector};
# use datafusion_datasource_parquet::ParquetAccessPlan;
// Default to scan all row groups
let mut access_plan = ParquetAccessPlan::new_all(4);
access_plan.skip(0); // skip row group
// Use parquet reader RowSelector to specify scanning rows 100-200 and 350-400
// in a row group that has 1000 rows
let row_selection = RowSelection::from(vec![
   RowSelector::skip(100),
   RowSelector::select(100),
   RowSelector::skip(150),
   RowSelector::select(50),
   RowSelector::skip(600),  // skip last 600 rows
]);
access_plan.scan_selection(1, row_selection);
access_plan.skip(2); // skip row group 2
// row group 3 is scanned by default
```

The resulting plan would look like:

```text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐

│                   │  SKIP

└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
 Row Group 0
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
 ┌────────────────┐    SCAN ONLY ROWS
│└────────────────┘ │  100-200
 ┌────────────────┐    350-400
│└────────────────┘ │
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
 Row Group 1
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                       SKIP
│                   │

└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
 Row Group 2
┌───────────────────┐
│                   │  SCAN ALL ROWS
│                   │
│                   │
└───────────────────┘
 Row Group 3
```

For more background, please also see the [Embedding User-Defined Indexes in Apache Parquet Files blog]

[Embedding User-Defined Indexes in Apache Parquet Files blog]: https://datafusion.apache.org/blog/2025/07/14/user-defined-parquet-indexes

<a id="op-e8424e4327f9abe00e5d5070"></a>
## clone

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> ParquetAccessPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 17], "end": [95, 22], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/access_plan.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11ca32d74a2540f88238056e"></a>
## eq

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::eq` · datafusion-datasource-parquet 55.1.0

```rust
fn eq(&self, other: &ParquetAccessPlan) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 24], "end": [95, 33], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/access_plan.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d51f362e2f6d2c7674644bf0"></a>
## fmt

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 15], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/access_plan.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bf2a1d9b32fb0aa82192b30"></a>
## inner

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::inner` · datafusion-datasource-parquet 55.1.0

```rust
fn inner(&self) -> &[RowGroupAccess]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:559`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Get a reference to the inner accesses

<a id="op-6591177afcef207560a0563d"></a>
## into_inner

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::into_inner` · datafusion-datasource-parquet 55.1.0

```rust
fn into_inner(self) -> Vec<RowGroupAccess>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:564`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Covert into the inner row group accesses

<a id="op-79be63fad7daf68c25627524"></a>
## into_overall_row_selection

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::into_overall_row_selection` · datafusion-datasource-parquet 55.1.0

```rust
fn into_overall_row_selection(self, row_group_meta_data: &[RowGroupMetaData]) -> Result<Option<RowSelection>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return an overall `RowSelection`, if needed

This is used to compute the row selection for the parquet reader. See
[`ArrowReaderBuilder::with_row_selection`] for more details.

Returns
* `None` if there are no  [`RowGroupAccess::Selection`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-9dc55412f081ee8c5854d131)
* `Some(selection)` if there are [`RowGroupAccess::Selection`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-9dc55412f081ee8c5854d131)s

The returned selection represents which rows to scan across any row
row groups which are not skipped.

# Notes

If there are no [`RowGroupAccess::Selection`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-9dc55412f081ee8c5854d131)s, the overall row
selection is `None` because each row group is either entirely skipped or
scanned, which is covered by [`Self::row_group_indexes`](../operations/datafusion_datasource_parquet.access_plan.ParquetAccessPlan.md#op-fdf7eab13e10c54059bf6896).

If there are any [`RowGroupAccess::Selection`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-9dc55412f081ee8c5854d131), an overall row selection
is returned for *all* the rows in the row groups that are not skipped.
Thus it includes a `Select` selection for any [`RowGroupAccess::Scan`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-ac7b3116c91c00b620d74af7).

# Errors

Returns an error if any specified row selection does not specify
the same number of rows as in it's corresponding `row_group_metadata`.

# Example: No Selections

Given an access plan like this

```text
  RowGroupAccess::Scan (scan all row group 0)
  RowGroupAccess::Skip (skip row group 1)
  RowGroupAccess::Scan (scan all row group 2)
  RowGroupAccess::Scan (scan all row group 3)
```

The overall row selection would be `None` because there are no
[`RowGroupAccess::Selection`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-9dc55412f081ee8c5854d131)s. The row group indexes
returned by [`Self::row_group_indexes`](../operations/datafusion_datasource_parquet.access_plan.ParquetAccessPlan.md#op-fdf7eab13e10c54059bf6896) would be `0, 2, 3` .

# Example: With Selections

Given an access plan like this:

```text
  RowGroupAccess::Scan (scan all row group 0)
  RowGroupAccess::Skip (skip row group 1)
  RowGroupAccess::Select (skip 50, scan 50, skip 900) (scan rows 50-100 in row group 2)
  RowGroupAccess::Scan (scan all row group 3)
```

Assuming each row group has 1000 rows, the resulting row selection would
be the rows to scan in row group 0, 2 and 4:

```text
 RowSelection::Select(1000) (scan all rows in row group 0)
 RowSelection::Skip(50)     (skip first 50 rows in row group 2)
 RowSelection::Select(50)   (scan rows 50-100 in row group 2)
 RowSelection::Skip(900)    (skip last 900 rows in row group 2)
 RowSelection::Select(1000) (scan all rows in row group 3)
```

Note there is no entry for the (entirely) skipped row group 1.

The row group indexes returned by [`Self::row_group_indexes`](../operations/datafusion_datasource_parquet.access_plan.ParquetAccessPlan.md#op-fdf7eab13e10c54059bf6896) would
still be `0, 2, 3` .

[`ArrowReaderBuilder::with_row_selection`]: parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_selection

Unresolved upstream links (retained, not inferred): `parquet::arrow::arrow_reader::ArrowReaderBuilder::with_row_selection`.

<a id="op-1ecf5bfef162ccb16c1af45c"></a>
## is_empty

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::is_empty` · datafusion-datasource-parquet 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:554`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return true if there are no row groups

<a id="op-78530d0abcfbf5af52e7e8fd"></a>
## len

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::len` · datafusion-datasource-parquet 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:549`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return the total number of row groups (not the total number or groups to
scan)

<a id="op-6672c74d6b9f894e27401a16"></a>
## new

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(row_groups: Vec<RowGroupAccess>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new `ParquetAccessPlan` from the specified [`RowGroupAccess`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-78e2484ee2017bec500ea3e7)es

<a id="op-73550c794a12978be21c7295"></a>
## new_all

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::new_all` · datafusion-datasource-parquet 55.1.0

```rust
fn new_all(row_group_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new `ParquetAccessPlan` that scans all row groups

<a id="op-06c8841266f90ea9fcea5d99"></a>
## new_none

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::new_none` · datafusion-datasource-parquet 55.1.0

```rust
fn new_none(row_group_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new `ParquetAccessPlan` that scans no row groups

<a id="op-bb9ca96f28b8cbe8f45d4e84"></a>
## row_group_index_iter

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::row_group_index_iter` · datafusion-datasource-parquet 55.1.0

```rust
fn row_group_index_iter(&self) -> impl Iterator<Item = usize> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return an iterator over the row group indexes that should be scanned

<a id="op-fdf7eab13e10c54059bf6896"></a>
## row_group_indexes

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::row_group_indexes` · datafusion-datasource-parquet 55.1.0

```rust
fn row_group_indexes(&self) -> Vec<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:543`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return a vec of all row group indexes to scan

<a id="op-11b86ad0b78556eca6aeee63"></a>
## scan

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::scan` · datafusion-datasource-parquet 55.1.0

```rust
fn scan(&mut self, idx: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

scan the i-th row group

<a id="op-4911843e003ccf55f6d7f783"></a>
## scan_selection

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::scan_selection` · datafusion-datasource-parquet 55.1.0

```rust
fn scan_selection(&mut self, idx: usize, selection: RowSelection)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set to scan only the [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5) in the specified row group.

Behavior is different depending on the existing access
* [`RowGroupAccess::Skip`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-43889f9fbbe5544a160eab6f): does nothing
* [`RowGroupAccess::Scan`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-ac7b3116c91c00b620d74af7): Updates to scan only the rows in the `RowSelection`
* [`RowGroupAccess::Selection`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-9dc55412f081ee8c5854d131): Updates to scan only the intersection of the existing selection and the new selection

<a id="op-dafb56ac9b062181ae8873a6"></a>
## set

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::set` · datafusion-datasource-parquet 55.1.0

```rust
fn set(&mut self, idx: usize, access: RowGroupAccess)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Set the i-th row group to the specified [`RowGroupAccess`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-78e2484ee2017bec500ea3e7)

<a id="op-6be0d4b0b970882d0e517bdd"></a>
## should_scan

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::should_scan` · datafusion-datasource-parquet 55.1.0

```rust
fn should_scan(&self, idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return true if the i-th row group should be scanned

<a id="op-9c6ab9d21db3f6fdf62b76bd"></a>
## skip

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::skip` · datafusion-datasource-parquet 55.1.0

```rust
fn skip(&mut self, idx: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

skips the i-th row group (should not be scanned)

<a id="op-9692220602ae45f7d95e0908"></a>
## try_new_from_overall_row_selection

`function` · `datafusion_datasource_parquet::access_plan::ParquetAccessPlan::try_new_from_overall_row_selection` · datafusion-datasource-parquet 55.1.0

```rust
fn try_new_from_overall_row_selection(selection: RowSelection, row_group_meta_data: &[RowGroupMetaData]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::ParquetAccessPlan", "path": "ParquetAccessPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [578, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new `ParquetAccessPlan` from a file-level [`RowSelection`](../operations/parquet.arrow.arrow_reader.selection.RowSelection.md#op-42015b29e86ada83b6a84ed5).

The selection is interpreted across all rows in the file, in row group
order, and is split into row-group level access using `row_group_meta_data`.
Fully skipped row groups become [`RowGroupAccess::Skip`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-43889f9fbbe5544a160eab6f), fully selected
row groups become [`RowGroupAccess::Scan`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-ac7b3116c91c00b620d74af7), and partially selected row
groups become [`RowGroupAccess::Selection`](../operations/datafusion_datasource_parquet.access_plan.RowGroupAccess.md#op-9dc55412f081ee8c5854d131).

# Errors

Returns an error if the selection does not specify exactly the same
number of rows as the file metadata.
