# `buoyant_kernel::scan::StructStats`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.StructStats.json).

<a id="op-a1896c6d6a8bfc4e0cad63bb"></a>
## StructStats

`enum` · `buoyant_kernel::scan::StructStats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum StructStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L118).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:118`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Which struct stats columns appear in `stats_parsed` in scan metadata output.

<a id="op-269ef7fb082008899d973b3a"></a>
## All

`variant` · `buoyant_kernel::scan::StructStats::All` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
All
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L124).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:124`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emit all indexed stats columns.

<a id="op-071fbd84ec4930e3c51927ab"></a>
## Columns

`variant` · `buoyant_kernel::scan::StructStats::Columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Columns
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L126).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:126`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emit only the specified stats columns.

<a id="op-d221f663b5c08c1a89c7e515"></a>
## None

`variant` · `buoyant_kernel::scan::StructStats::None` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
None
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L122).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:122`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Don't emit `stats_parsed`. Kernel still reads predicate-referenced stats for
internal data skipping unless the caller picked [`StatsOptions::none`](../operations/buoyant_kernel.scan.StatsOptions.md#op-2ff98661dc8411512451e897), which
disables stats reading entirely.

<a id="op-99cb6c4e383fb338bb7756b6"></a>
## clone

`function` · `buoyant_kernel::scan::StructStats::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> StructStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L117).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StructStats", "path": "StructStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 10], "end": [117, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:117`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b897cbb4fd1da0f59496bba"></a>
## fmt

`function` · `buoyant_kernel::scan::StructStats::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L117).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StructStats", "path": "StructStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 17], "end": [117, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:117`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
