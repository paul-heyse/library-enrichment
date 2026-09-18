# `buoyant_kernel::crc::state::DomainMetadataState`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.state.DomainMetadataState.json).

<a id="op-1bdc582d72a8cd8896b7ca99"></a>
## DomainMetadataState

`enum` · `buoyant_kernel::crc::state::DomainMetadataState` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DomainMetadataState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L77).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:77`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The completeness state of cached domain metadata in a CRC.

<a id="op-87a860bc3ce834f605a0d5a5"></a>
## Complete

`variant` · `buoyant_kernel::crc::state::DomainMetadataState::Complete` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Complete
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L81).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:81`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The CRC file had the `domainMetadata` field (possibly as an empty array). The map
is the full set of active (non-removed) domains at this version; a domain not in the
map does not exist.

<a id="op-5d7b2cc9d08e8975059bde22"></a>
## Partial

`variant` · `buoyant_kernel::crc::state::DomainMetadataState::Partial` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Partial
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L87).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The CRC file did not have the `domainMetadata` field. The map starts empty and gets
populated by incremental CRC replay over the JSON commits after the latest stale CRC.
Hits are authoritative (definitely present at this version); misses are NOT
(the domain may exist in older commits), so callers must do a further log scan.

<a id="op-3abef5f5af504fd9cfd18e0e"></a>
## clone

`function` · `buoyant_kernel::crc::state::DomainMetadataState::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DomainMetadataState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::DomainMetadataState", "path": "DomainMetadataState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 17], "end": [76, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae8422e51570bfe4de56a323"></a>
## default

`function` · `buoyant_kernel::crc::state::DomainMetadataState::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L91).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::DomainMetadataState", "path": "DomainMetadataState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [94, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:91`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20d6cd031e7df2f8205c7aed"></a>
## eq

`function` · `buoyant_kernel::crc::state::DomainMetadataState::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DomainMetadataState) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::DomainMetadataState", "path": "DomainMetadataState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 24], "end": [76, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9feebb21e51e269c9723f9ca"></a>
## fmt

`function` · `buoyant_kernel::crc::state::DomainMetadataState::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L76).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::DomainMetadataState", "path": "DomainMetadataState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 10], "end": [76, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
