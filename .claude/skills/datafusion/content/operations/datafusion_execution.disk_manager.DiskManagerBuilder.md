# `datafusion_execution::disk_manager::DiskManagerBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.disk_manager.DiskManagerBuilder.json).

<a id="op-3c6231e792a03a3bc83aef37"></a>
## DiskManagerBuilder

`struct` · `datafusion_execution::disk_manager::DiskManagerBuilder` · datafusion-execution 55.1.0

```rust
struct DiskManagerBuilder
```

Source: `src/disk_manager.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Builder pattern for the [DiskManager](../operations/datafusion_execution.disk_manager.DiskManager.md#op-e778066e9d4969684552adc1) structure

<a id="op-084b380ab98c345bd7ec057e"></a>
## build

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::build` · datafusion-execution 55.1.0

```rust
fn build(self) -> Result<DiskManager>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a DiskManager given the builder

<a id="op-a8668b888a5f9f2ab49549fb"></a>
## clone

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> DiskManagerBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/disk_manager.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3681073fa4728c4996326e9"></a>
## default

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::default` · datafusion-execution 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [65, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/disk_manager.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-167f2fd6129dc06a4df55516"></a>
## fmt

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [56, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/disk_manager.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39c6dfe734b309d357e7a649"></a>
## set_max_spill_merge_fan_in

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::set_max_spill_merge_fan_in` · datafusion-execution 55.1.0

```rust
fn set_max_spill_merge_fan_in(&mut self, value: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac7f628b77c1139d48433159"></a>
## set_max_temp_directory_size

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::set_max_temp_directory_size` · datafusion-execution 55.1.0

```rust
fn set_max_temp_directory_size(&mut self, value: u64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d74a618cffc1bb2b4faaccc3"></a>
## set_mode

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::set_mode` · datafusion-execution 55.1.0

```rust
fn set_mode(&mut self, mode: DiskManagerMode)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adfeecb5fd02cd58bb1a0ff9"></a>
## set_temp_file_factory

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::set_temp_file_factory` · datafusion-execution 55.1.0

```rust
fn set_temp_file_factory(&mut self, temp_file_factory: Arc<dyn TempFileFactory>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Configure a custom factory for creating temporary spill files.

This sets the disk manager mode to [`DiskManagerMode::Custom`](../operations/datafusion_execution.disk_manager.DiskManagerMode.md#op-f713d022dce22fb17a0efafd), so
operators that spill during query execution create files through the
provided [`TempFileFactory`](../operations/datafusion_execution.spill_file.TempFileFactory.md#op-dd44208d8318480df91986d6) instead of using local temporary files.

<a id="op-2a0f02e570c5b47275583d94"></a>
## with_max_spill_merge_fan_in

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::with_max_spill_merge_fan_in` · datafusion-execution 55.1.0

```rust
fn with_max_spill_merge_fan_in(self, value: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e57df3dfab95f00bc6cae16"></a>
## with_max_temp_directory_size

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::with_max_temp_directory_size` · datafusion-execution 55.1.0

```rust
fn with_max_temp_directory_size(self, value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c96d8756fedb0abcb84210e2"></a>
## with_mode

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::with_mode` · datafusion-execution 55.1.0

```rust
fn with_mode(self, mode: DiskManagerMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75649c54c318292a968bc6a4"></a>
## with_temp_file_factory

`function` · `datafusion_execution::disk_manager::DiskManagerBuilder::with_temp_file_factory` · datafusion-execution 55.1.0

```rust
fn with_temp_file_factory(self, temp_file_factory: Arc<dyn TempFileFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerBuilder", "path": "DiskManagerBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [158, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Configure a custom factory for creating temporary spill files.

See details on [`Self::set_temp_file_factory`](../operations/datafusion_execution.disk_manager.DiskManagerBuilder.md#op-adfeecb5fd02cd58bb1a0ff9).
