# `buoyant_kernel::path::ParsedLogPath`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.path.ParsedLogPath.json).

<a id="op-d227cf1017d1495bda5b49a7"></a>
## ParsedLogPath

`struct` · `buoyant_kernel::path::ParsedLogPath` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ParsedLogPath<Location: AsUrl = FileMeta>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L68).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A ParsedLogPath is a well-understood path to a file in the _delta_log directory.

Note this includes things like checkpoints and commits (containing current table state), but
also files used for various optimizations like CRC, compaction, etc.

Every parsed log path has a version. And additionally, we implement a 'should_list' method
which controls whether or not we include this file in our listing. For example, when we list
the _delta_log we may see _staged_commits/00000000000000000000.{uuid}.json, but we MUST NOT
include those in listing, as only the catalog can tell us which are valid commits.

<a id="op-1f4bfce4fcebc18f8d8a5ca3"></a>
## clone

`function` · `buoyant_kernel::path::ParsedLogPath::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ParsedLogPath<Location>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Location"}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}}}], "default": null, "is_synthetic": false}}, "name": "Location"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 17], "end": [66, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fdfa0da0a734e8457d058a0"></a>
## eq

`function` · `buoyant_kernel::path::ParsedLogPath::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ParsedLogPath<Location>) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Location"}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}}}], "default": null, "is_synthetic": false}}, "name": "Location"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 24], "end": [66, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75767d4ad27e3af13f6d510d"></a>
## extension

`struct_field` · `buoyant_kernel::path::ParsedLogPath::extension` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
extension: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L73).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:73`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0323dae62892e2c0c88f80a6"></a>
## file_type

`struct_field` · `buoyant_kernel::path::ParsedLogPath::file_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
file_type: LogPathFileType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L75).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e360596c065e90f91d5eda8"></a>
## filename

`struct_field` · `buoyant_kernel::path::ParsedLogPath::filename` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
filename: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L71).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71246b6b98e3d9472b9000b6"></a>
## fmt

`function` · `buoyant_kernel::path::ParsedLogPath::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Location"}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}}}], "default": null, "is_synthetic": false}}, "name": "Location"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 10], "end": [66, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a5f07b553e2b269d3ed60c0"></a>
## from

`function` · `buoyant_kernel::path::ParsedLogPath::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(p: LogPath) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_path.rs#L18).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::path::ParsedLogPath", "path": "crate::path::ParsedLogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [17, 1], "end": [21, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::log_path::LogPath", "path": "LogPath"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_path.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ebc52805b6e3fa34ffe1c1a"></a>
## is_checkpoint

`function` · `buoyant_kernel::path::ParsedLogPath::is_checkpoint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_checkpoint(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L290).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Location"}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}}}], "default": null, "is_synthetic": false}}, "name": "Location"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [304, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:290`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c95702067a00381f358bf7d"></a>
## is_commit

`function` · `buoyant_kernel::path::ParsedLogPath::is_commit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_commit(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L282).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Location"}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}}}], "default": null, "is_synthetic": false}}, "name": "Location"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [304, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:282`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1ec9e4a62bfb075ab513c5a"></a>
## is_unknown

`function` · `buoyant_kernel::path::ParsedLogPath::is_unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_unknown(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L301).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Location"}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}}}], "default": null, "is_synthetic": false}}, "name": "Location"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [304, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:301`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dde00430d6325322266224a"></a>
## location

`struct_field` · `buoyant_kernel::path::ParsedLogPath::location` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
location: Location
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L69).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:69`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dac9f24d74bf5c8d80eb7390"></a>
## new_crc

`function` · `buoyant_kernel::path::ParsedLogPath::new_crc` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_crc(table_root: &Url, version: Version) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L405).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "url::Url", "path": "url::Url"}}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [433, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:405`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new `ParsedLogPath<Url>` for a version checksum (CRC) file.

<a id="op-edf4a2474e1a99332ccca33e"></a>
## try_from

`function` · `buoyant_kernel::path::ParsedLogPath::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(location: Location) -> DeltaResult<Option<ParsedLogPath<Location>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L140).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Location"}}], "constraints": []}}, "id": "buoyant_kernel::path::ParsedLogPath", "path": "ParsedLogPath"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}}}], "default": null, "is_synthetic": false}}, "name": "Location"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [304, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:140`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c50fa17e08fe6f644253c7fa"></a>
## version

`struct_field` · `buoyant_kernel::path::ParsedLogPath::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
