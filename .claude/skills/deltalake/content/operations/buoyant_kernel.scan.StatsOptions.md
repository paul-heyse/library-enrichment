# `buoyant_kernel::scan::StatsOptions`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.StatsOptions.json).

<a id="op-df28d0423c58aeff877d5915"></a>
## StatsOptions

`struct` · `buoyant_kernel::scan::StatsOptions` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StatsOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L101).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:101`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Engine-facing stats options. Pass to [`ScanBuilder::with_stats`](../operations/buoyant_kernel.scan.ScanBuilder.md#op-22528da5a54fb36bd4f6afa5) to declare
what stats data the engine wants in scan metadata output. Two orthogonal axes:
JSON stats (`add.stats`) and struct stats (`add.stats_parsed`).

Most consumers should pick one of the named constructors:
- [`Self::json_only`](../operations/buoyant_kernel.scan.StatsOptions.md#op-9324ed29670118c3218d6350) (default) -- JSON stats only.
- [`Self::all_struct`](../operations/buoyant_kernel.scan.StatsOptions.md#op-1f3dfe4a8f9225d84366d85b) -- all struct stats, no JSON. Cheap path when the engine consumes
  `stats_parsed` directly; avoids the per-batch `ToJson` cost.
- [`Self::struct_columns`](../operations/buoyant_kernel.scan.StatsOptions.md#op-b537af9d370ee2c57d0cbc45) -- struct stats projected to a subset of columns, no JSON.
- [`Self::all`](../operations/buoyant_kernel.scan.StatsOptions.md#op-ba5ca0af032bd192c84f8c7e) -- both representations.
- [`Self::none`](../operations/buoyant_kernel.scan.StatsOptions.md#op-2ff98661dc8411512451e897) -- neither, AND disables internal data skipping. Unlike the other four
  constructors, this is the only one that stops kernel from reading stats from parquet at all.

<a id="op-ba5ca0af032bd192c84f8c7e"></a>
## all

`function` · `buoyant_kernel::scan::StatsOptions::all` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn all() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L165).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [184, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:165`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Both JSON and struct stats. Pays for both representations.

<a id="op-1f3dfe4a8f9225d84366d85b"></a>
## all_struct

`function` · `buoyant_kernel::scan::StatsOptions::all_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn all_struct() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L148).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [184, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:148`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

All struct stats, no JSON. Cheap path for engines that consume
`stats_parsed` directly: avoids the per-batch `ToJson` cost on
parsed-stats checkpoints.

<a id="op-47601b4cc479257dc47b222b"></a>
## clone

`function` · `buoyant_kernel::scan::StatsOptions::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> StatsOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 10], "end": [100, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2323dd940c219070dabd5c0"></a>
## default

`function` · `buoyant_kernel::scan::StatsOptions::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L131).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [137, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:131`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

JSON only, no struct stats.

<a id="op-8a68518cc058bf83d7d0690c"></a>
## fmt

`function` · `buoyant_kernel::scan::StatsOptions::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 17], "end": [100, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9324ed29670118c3218d6350"></a>
## json_only

`function` · `buoyant_kernel::scan::StatsOptions::json_only` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn json_only() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L141).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [184, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:141`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

JSON only. Equivalent to [`Default::default`].

Unresolved upstream links (retained, not inferred): ``Default::default``.

<a id="op-2ff98661dc8411512451e897"></a>
## none

`function` · `buoyant_kernel::scan::StatsOptions::none` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn none() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L178).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [184, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:178`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

**Disables all stats work**: no stats output, no internal data skipping (even
when a predicate is set). Kernel reads no stats columns from parquet at all.
Use when the engine handles its own pruning.

To get internal predicate-based skipping without `stats_parsed` output, use
[`StatsOptions::default`](../operations/buoyant_kernel.scan.StatsOptions.md#op-e2323dd940c219070dabd5c0) (JSON only) or set `struct_stats` to `All`/`Columns(_)`.

<a id="op-b537af9d370ee2c57d0cbc45"></a>
## struct_columns

`function` · `buoyant_kernel::scan::StatsOptions::struct_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn struct_columns(cols: Vec<ColumnName>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L157).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::scan::StatsOptions", "path": "StatsOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [184, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:157`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Struct stats projected to the specified columns, no JSON. Like
[`Self::all_struct`](../operations/buoyant_kernel.scan.StatsOptions.md#op-1f3dfe4a8f9225d84366d85b) but narrowed to a subset of indexed columns.

<a id="op-0b065139d9e053f473850f25"></a>
## struct_stats

`struct_field` · `buoyant_kernel::scan::StatsOptions::struct_stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
struct_stats: StructStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L113).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Which struct stats columns to emit in `stats_parsed`.

<a id="op-208165a6e7e4389f320335be"></a>
## synthesize_json

`struct_field` · `buoyant_kernel::scan::StatsOptions::synthesize_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
synthesize_json: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/mod.rs#L110).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/mod.rs:110`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether to surface JSON stats on parsed-stats checkpoints (where the
checkpoint writes stats only as a struct, not as JSON). When true, kernel
re-serializes the struct stats into JSON so engines that read JSON stats
see a populated value; when false, JSON stats are left null on such
checkpoints and the engine consumes the struct stats directly.

No effect on tables that write JSON stats directly, or on commit JSON --
the existing JSON is passed through regardless.
