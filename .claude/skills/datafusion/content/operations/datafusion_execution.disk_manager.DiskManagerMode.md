# `datafusion_execution::disk_manager::DiskManagerMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.disk_manager.DiskManagerMode.json).

<a id="op-d3442ee6e0a7e1524353bbac"></a>
## DiskManagerMode

`enum` · `datafusion_execution::disk_manager::DiskManagerMode` · datafusion-execution 55.1.0

```rust
enum DiskManagerMode
```

Source: `src/disk_manager.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f713d022dce22fb17a0efafd"></a>
## Custom

`variant` · `datafusion_execution::disk_manager::DiskManagerMode::Custom` · datafusion-execution 55.1.0

```rust
Custom
```

Source: `src/disk_manager.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new [DiskManager](../operations/datafusion_execution.disk_manager.DiskManager.md#op-e778066e9d4969684552adc1) with a cutstom backend

<a id="op-edbe130b494fc4a83644007e"></a>
## Directories

`variant` · `datafusion_execution::disk_manager::DiskManagerMode::Directories` · datafusion-execution 55.1.0

```rust
Directories
```

Source: `src/disk_manager.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new [DiskManager](../operations/datafusion_execution.disk_manager.DiskManager.md#op-e778066e9d4969684552adc1) that creates temporary files within
the specified directories. One of the directories will be chosen
at random for each temporary file created.

<a id="op-22948741cfaa5c0e5181901c"></a>
## Disabled

`variant` · `datafusion_execution::disk_manager::DiskManagerMode::Disabled` · datafusion-execution 55.1.0

```rust
Disabled
```

Source: `src/disk_manager.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Disable disk manager, attempts to create temporary files will error

<a id="op-973a461b32652691bfd7affa"></a>
## OsTmpDirectory

`variant` · `datafusion_execution::disk_manager::DiskManagerMode::OsTmpDirectory` · datafusion-execution 55.1.0

```rust
OsTmpDirectory
```

Source: `src/disk_manager.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new [DiskManager](../operations/datafusion_execution.disk_manager.DiskManager.md#op-e778066e9d4969684552adc1) that creates temporary files within
a temporary directory chosen by the OS

<a id="op-c16e941b3345c59a61b3f702"></a>
## clone

`function` · `datafusion_execution::disk_manager::DiskManagerMode::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> DiskManagerMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerMode", "path": "DiskManagerMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 10], "end": [160, 15], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/disk_manager.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76cac1fbb5ae65918ca24a9a"></a>
## default

`function` · `datafusion_execution::disk_manager::DiskManagerMode::default` · datafusion-execution 55.1.0

```rust
fn default() -> DiskManagerMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerMode", "path": "DiskManagerMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 17], "end": [160, 24], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/disk_manager.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f52d6e70c8a6c5dc8bfc5b1a"></a>
## fmt

`function` · `datafusion_execution::disk_manager::DiskManagerMode::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManagerMode", "path": "DiskManagerMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [188, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/disk_manager.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
