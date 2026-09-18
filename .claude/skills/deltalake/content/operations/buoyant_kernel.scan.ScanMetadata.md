# `buoyant_kernel::scan::ScanMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.ScanMetadata.json).

<a id="op-291499a74f7b78c8790cd5b2"></a>
## ScanMetadata

`struct` · `buoyant_kernel::scan::ScanMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ScanMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L609).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:609`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2) contains (1) a batch of [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab) specifying data files to be
scanned and (2) a vector of transforms (one transform per scan file) that must be applied to the
data read from those files.

<a id="op-16698202230c7538e6bad396"></a>
## has_selected_rows

`function` · `buoyant_kernel::scan::ScanMetadata::has_selected_rows` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_selected_rows(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L643).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanMetadata", "path": "ScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [646, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::log_replay::HasSelectionVector", "path": "HasSelectionVector"}, "trait_path": "buoyant_kernel::log_replay::HasSelectionVector"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:643`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4a43e323d258d84029ee124"></a>
## scan_file_transforms

`struct_field` · `buoyant_kernel::scan::ScanMetadata::scan_file_transforms` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
scan_file_transforms: Vec<Option<expressions::ExpressionRef>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L626).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:626`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Row-level transformations to apply to data read from files.

Each entry in this vector corresponds to a row in the `scan_files` data. The entry is an
optional expression that must be applied to convert the file's data into the logical schema
expected by the scan:

- `Some(expr)`: Apply this expression to transform the data to match
  [`Scan::logical_schema()`](../operations/buoyant_kernel.scan.Scan.md#op-2ba19820ffce66b9b38e59e3).
- `None`: No transformation is needed; the data is already in the correct logical form.

Note: This vector can be indexed by row number, as rows masked by the selection vector will
have corresponding entries that will be `None`.

<a id="op-49157827e191ab7bc70d71c8"></a>
## scan_files

`struct_field` · `buoyant_kernel::scan::ScanMetadata::scan_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
scan_files: engine_data::FilteredEngineData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L612).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:612`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Filtered engine data with one row per file to scan (and only selected rows should be
scanned)

<a id="op-8fb785698d4d1c62ec32fe60"></a>
## visit_scan_files

`function` · `buoyant_kernel::scan::ScanMetadata::visit_scan_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_scan_files<T>(&self, context: T, callback: ScanCallback<T>) -> DeltaResult<T>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/state.rs#L159).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::ScanMetadata", "path": "super::ScanMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [168, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/state.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/state.rs:159`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
