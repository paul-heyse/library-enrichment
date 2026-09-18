# `buoyant_kernel::checkpoint::V2CheckpointConfig`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.V2CheckpointConfig.json).

<a id="op-374dcaad1e91c9150193632d"></a>
## V2CheckpointConfig

`enum` · `buoyant_kernel::checkpoint::V2CheckpointConfig` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum V2CheckpointConfig
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L269).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:269`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Configuration for V2 checkpoints.

Note: "File actions" here means `add` and `remove` actions. "Non-file actions" means
the rest (`protocol`, `metaData`, `txn`, etc.).

<a id="op-1db1a400692a2d2f1b2a4bae"></a>
## NoSidecar

`variant` · `buoyant_kernel::checkpoint::V2CheckpointConfig::NoSidecar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NoSidecar
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L271).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:271`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write a V2 checkpoint without sidecar files.

<a id="op-0c47e3a31b650975f1c16bb0"></a>
## WithSidecar

`variant` · `buoyant_kernel::checkpoint::V2CheckpointConfig::WithSidecar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
WithSidecar
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L284).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:284`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write a V2 checkpoint with file actions split into [Sidecar Files]. A main
checkpoint file is written, with one `sidecar` action pointing to each sidecar file.

# Benefits of Sidecars
- **Read parallelism**: readers can fetch sidecars in parallel.
- **Smaller main checkpoint**: callers that only need non-file actions (e.g. `protocol`,
  `metaData`) can skip the sidecars entirely.

# Note
Sidecars add extra write cost (one parquet file per sidecar).

[Sidecar Files]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#sidecar-files

<a id="op-d6f849a6eb75c3cd30bd771b"></a>
## fmt

`function` · `buoyant_kernel::checkpoint::V2CheckpointConfig::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L268).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::V2CheckpointConfig", "path": "V2CheckpointConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 10], "end": [268, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:268`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
