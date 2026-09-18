# `buoyant_kernel::history_manager::error::NearestTimestamp`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.error.NearestTimestamp.json).

<a id="op-368895193901f0d9e7d18910"></a>
## NearestTimestamp

`enum` · `buoyant_kernel::history_manager::error::NearestTimestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum NearestTimestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L18).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The nearest retained timestamp on the side of the search bound that an
out-of-range search failed against. Engines surface this to users so the
error message can point at a valid timestamp.

`Earliest` and `Latest` carry the boundary commit's timestamp. `Unknown`
means no boundary timestamp was available, e.g. empty log or a failure
reading the boundary commit's In-Commit Timestamp at the error site.

<a id="op-65c2180ad83e6664357ac406"></a>
## Earliest

`variant` · `buoyant_kernel::history_manager::error::NearestTimestamp::Earliest` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Earliest
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L21).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The earliest retained commit's timestamp. Returned for a
`GreatestLower` search whose input fell below the retained range.

<a id="op-8bbb87c319c7181ee6ea5fb1"></a>
## Latest

`variant` · `buoyant_kernel::history_manager::error::NearestTimestamp::Latest` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Latest
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The latest retained commit's timestamp. Returned for a `LeastUpper`
search whose input fell above the retained range.

<a id="op-14e8c8f4bbbaafdeac92d56b"></a>
## Unknown

`variant` · `buoyant_kernel::history_manager::error::NearestTimestamp::Unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unknown
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No boundary timestamp could be determined.

<a id="op-47a50552a47f9257037d0ac9"></a>
## clone

`function` · `buoyant_kernel::history_manager::error::NearestTimestamp::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> NearestTimestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::NearestTimestamp", "path": "NearestTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 17], "end": [16, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0020cd74dfb8be8ce5064f2"></a>
## eq

`function` · `buoyant_kernel::history_manager::error::NearestTimestamp::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &NearestTimestamp) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::NearestTimestamp", "path": "NearestTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 30], "end": [16, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5461c3df57673dd82bec285e"></a>
## fmt

`function` · `buoyant_kernel::history_manager::error::NearestTimestamp::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::error::NearestTimestamp", "path": "NearestTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 10], "end": [16, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
