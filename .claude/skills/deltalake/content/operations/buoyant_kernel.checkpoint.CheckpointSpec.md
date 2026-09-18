# `buoyant_kernel::checkpoint::CheckpointSpec`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.checkpoint.CheckpointSpec.json).

<a id="op-44c5739f08f77a821b0f5ded"></a>
## CheckpointSpec

`enum` · `buoyant_kernel::checkpoint::CheckpointSpec` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CheckpointSpec
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L250).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:250`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Specifies the checkpoint format and behavior.

<a id="op-c9141b61eb13778b0b75a021"></a>
## V1

`variant` · `buoyant_kernel::checkpoint::CheckpointSpec::V1` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
V1
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L255).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:255`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write a checkpoint following the V1 spec, the original checkpoint format, without
sidecar files or checkpoint metadata. See [V1 spec] for more details.

[V1 spec]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#v1-spec

<a id="op-9983c3edc314a30e772e3f1a"></a>
## V2

`variant` · `buoyant_kernel::checkpoint::CheckpointSpec::V2` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
V2
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L261).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:261`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write a checkpoint following the V2 spec, which allows putting file actions (`add`
and `remove`) in sidecar files. Requires the `v2Checkpoint` reader/writer feature.
See [V2 spec] and [`V2CheckpointConfig::WithSidecar`](../operations/buoyant_kernel.checkpoint.V2CheckpointConfig.md#op-0c47e3a31b650975f1c16bb0) for more details.

[V2 spec]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#v2-spec

<a id="op-b793f6902a4c94434d1e4d2a"></a>
## fmt

`function` · `buoyant_kernel::checkpoint::CheckpointSpec::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/checkpoint/mod.rs#L249).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::checkpoint::CheckpointSpec", "path": "CheckpointSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [249, 10], "end": [249, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/checkpoint/mod.rs:249`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
