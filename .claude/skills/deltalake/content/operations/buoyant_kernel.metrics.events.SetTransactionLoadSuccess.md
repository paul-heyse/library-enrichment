# `buoyant_kernel::metrics::events::SetTransactionLoadSuccess`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.events.SetTransactionLoadSuccess.json).

<a id="op-34ce5a03a04c488e5cc7f4ec"></a>
## SetTransactionLoadSuccess

`struct` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SetTransactionLoadSuccess
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L850).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:850`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emitted once per `SetTransaction` (app id) load, whether served from the CRC cache
(`from_cache`) or from a log replay. `found` is true when the app id has a committed
transaction version, false when none exists or the existing one is expired.

<a id="op-82ddcafecac508c5217125dc"></a>
## clone

`function` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> SetTransactionLoadSuccess
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L849).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::SetTransactionLoadSuccess", "path": "SetTransactionLoadSuccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 17], "end": [849, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:849`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac97546f7abeb68609e4c97f"></a>
## duration

`struct_field` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess::duration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
duration: std::time::Duration
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L856).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:856`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10e0d727183f267b1d4ab45d"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L849).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::SetTransactionLoadSuccess", "path": "SetTransactionLoadSuccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 10], "end": [849, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:849`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec7fe3846bc5792f0baad0a4"></a>
## fmt

`function` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L885).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::metrics::events::SetTransactionLoadSuccess", "path": "SetTransactionLoadSuccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [896, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:885`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79f525cb6de9b6508f5a7df5"></a>
## found

`struct_field` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess::found` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
found: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L853).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:853`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-094c6d1cd861dd3b482bf73d"></a>
## from_cache

`struct_field` · `buoyant_kernel::metrics::events::SetTransactionLoadSuccess::from_cache` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
from_cache: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/events.rs#L852).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/events.rs:852`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
