# `datafusion_execution::disk_manager::DiskManager`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.disk_manager.DiskManager.json).

<a id="op-e778066e9d4969684552adc1"></a>
## DiskManager

`struct` · `datafusion_execution::disk_manager::DiskManager` · datafusion-execution 55.1.0

```rust
struct DiskManager
```

Source: `src/disk_manager.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Manages files generated during query execution, e.g. spill files generated
while processing dataset larger than available memory.

<a id="op-e691e75bca9a884b2042ea79"></a>
## builder

`function` · `datafusion_execution::disk_manager::DiskManager::builder` · datafusion-execution 55.1.0

```rust
fn builder() -> DiskManagerBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Creates a builder for [DiskManager](../operations/datafusion_execution.disk_manager.DiskManager.md#op-e778066e9d4969684552adc1)

<a id="op-639763c343e5372c96c8409a"></a>
## create_tmp_file

`function` · `datafusion_execution::disk_manager::DiskManager::create_tmp_file` · datafusion-execution 55.1.0

```rust
fn create_tmp_file(&Arc<self>, request_description: &str) -> Result<Arc<dyn SpillFile>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return a temporary file from a randomized choice in the configured locations

If the file can not be created for some reason, returns an
error message referencing the request description

<a id="op-ba2dc68baad7de51b02f9636"></a>
## fmt

`function` · `datafusion_execution::disk_manager::DiskManager::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 1], "end": [223, 2], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/disk_manager.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c86d298a8265e920538b0b21"></a>
## max_spill_merge_fan_in

`function` · `datafusion_execution::disk_manager::DiskManager::max_spill_merge_fan_in` · datafusion-execution 55.1.0

```rust
fn max_spill_merge_fan_in(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the maximum number of spill files opened by one merge pass.

A value of 0 means unlimited.

<a id="op-4cdd6c2f99662185cde1430e"></a>
## max_temp_directory_size

`function` · `datafusion_execution::disk_manager::DiskManager::max_temp_directory_size` · datafusion-execution 55.1.0

```rust
fn max_temp_directory_size(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the maximum temporary directory size in bytes

<a id="op-bb3cd72dc31581019a308f34"></a>
## set_arc_max_temp_directory_size

`function` · `datafusion_execution::disk_manager::DiskManager::set_arc_max_temp_directory_size` · datafusion-execution 55.1.0

```rust
fn set_arc_max_temp_directory_size(this: &Arc<Self>, max_temp_directory_size: u64) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f00b5617c22b2d26a9d6ca"></a>
## set_max_spill_merge_fan_in

`function` · `datafusion_execution::disk_manager::DiskManager::set_max_spill_merge_fan_in` · datafusion-execution 55.1.0

```rust
fn set_max_spill_merge_fan_in(&self, max_spill_merge_fan_in: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Atomically set the maximum spill merge fan-in.

A value of 0 disables the cap. Values of 1 are accepted but external
merge code will still merge at least two spill streams to make progress.

<a id="op-4486e43112034464fa192be6"></a>
## set_max_temp_directory_size

`function` · `datafusion_execution::disk_manager::DiskManager::set_max_temp_directory_size` · datafusion-execution 55.1.0

```rust
fn set_max_temp_directory_size(&self, max_temp_directory_size: u64) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Atomically set the max temp directory size at runtime.

Takes `&self`, so it works through `Arc<DiskManager>` without requiring
exclusive access. Takes effect immediately for subsequent spill writes.

Use this when you need to adjust the limit dynamically while queries
are running (e.g., adapting to available disk space).

<a id="op-007d099d1dcc1b2a1390ca88"></a>
## spilling_progress

`function` · `datafusion_execution::disk_manager::DiskManager::spilling_progress` · datafusion-execution 55.1.0

```rust
fn spilling_progress(&self) -> SpillingProgress
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the current spilling progress

<a id="op-a2904cfae1b8733ecde83166"></a>
## temp_dir_paths

`function` · `datafusion_execution::disk_manager::DiskManager::temp_dir_paths` · datafusion-execution 55.1.0

```rust
fn temp_dir_paths(&self) -> Vec<PathBuf>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the temporary directory paths

<a id="op-d2fa61d61911e040a6e0ce28"></a>
## tmp_files_enabled

`function` · `datafusion_execution::disk_manager::DiskManager::tmp_files_enabled` · datafusion-execution 55.1.0

```rust
fn tmp_files_enabled(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return true if this disk manager supports creating temporary
files. If this returns false, any call to `create_tmp_file`
will error.

<a id="op-05f94341cb42558735330218"></a>
## used_disk_space

`function` · `datafusion_execution::disk_manager::DiskManager::used_disk_space` · datafusion-execution 55.1.0

```rust
fn used_disk_space(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6da7d6efba6097e3366b4950"></a>
## with_max_temp_directory_size

`function` · `datafusion_execution::disk_manager::DiskManager::with_max_temp_directory_size` · datafusion-execution 55.1.0

```rust
fn with_max_temp_directory_size(self, max_temp_directory_size: u64) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::DiskManager", "path": "DiskManager"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [383, 2], "filename": "src/disk_manager.rs"}, "trait": null, "trait_path": null}`

Source: `src/disk_manager.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
