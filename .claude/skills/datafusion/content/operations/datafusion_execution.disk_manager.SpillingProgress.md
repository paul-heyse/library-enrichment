# `datafusion_execution::disk_manager::SpillingProgress`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.disk_manager.SpillingProgress.json).

<a id="op-79c8a95dbd65b45c3c28ca8b"></a>
## SpillingProgress

`struct` · `datafusion_execution::disk_manager::SpillingProgress` · datafusion-execution 55.1.0

```rust
struct SpillingProgress
```

Source: `src/disk_manager.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Information about the current disk usage for spilling

<a id="op-fcf43181b5ec37819c6366e6"></a>
## active_files_count

`struct_field` · `datafusion_execution::disk_manager::SpillingProgress::active_files_count` · datafusion-execution 55.1.0

```rust
active_files_count: usize
```

Source: `src/disk_manager.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Total number of active spill files

<a id="op-d6e6a0cb8a289508a3416149"></a>
## clone

`function` · `datafusion_execution::disk_manager::SpillingProgress::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> SpillingProgress
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::SpillingProgress", "path": "SpillingProgress"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 17], "end": [225, 22], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/disk_manager.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06f9afab3367f23970e156cd"></a>
## current_bytes

`struct_field` · `datafusion_execution::disk_manager::SpillingProgress::current_bytes` · datafusion-execution 55.1.0

```rust
current_bytes: u64
```

Source: `src/disk_manager.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Total bytes currently used on disk for spilling

<a id="op-69cefecf7eed49f606be98fe"></a>
## fmt

`function` · `datafusion_execution::disk_manager::SpillingProgress::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::disk_manager::SpillingProgress", "path": "SpillingProgress"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 10], "end": [225, 15], "filename": "src/disk_manager.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/disk_manager.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
