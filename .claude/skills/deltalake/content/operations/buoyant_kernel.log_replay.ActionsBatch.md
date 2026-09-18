# `buoyant_kernel::log_replay::ActionsBatch`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_replay.ActionsBatch.json).

<a id="op-72ca0249e147b15cf59dc008"></a>
## ActionsBatch

`struct` · `buoyant_kernel::log_replay::ActionsBatch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ActionsBatch
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L212).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:212`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e53354cb7e2be4b601f4ca5"></a>
## actions

`function` · `buoyant_kernel::log_replay::ActionsBatch::actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn actions(&self) -> &dyn EngineData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L238).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_replay::ActionsBatch", "path": "ActionsBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [241, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:238`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

HACK: a duplication of the pub(crate) field `actions` to allow us to export as
'internal-api' and let inspect-table example use it.

<a id="op-a592929f816af0d38393695d"></a>
## actions

`struct_field` · `buoyant_kernel::log_replay::ActionsBatch::actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
actions: Box<dyn EngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L214).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:214`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The batch of actions to be processed: each row is an action from the log.

<a id="op-120e44f399396193d9b467ea"></a>
## is_log_batch

`struct_field` · `buoyant_kernel::log_replay::ActionsBatch::is_log_batch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
is_log_batch: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L216).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:216`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether the batch is from a commit log (=true) or a checkpoint/CRC/elsewhere (=false).
