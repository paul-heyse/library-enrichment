# `buoyant_kernel::FileMeta`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.FileMeta.json).

<a id="op-4f91686d2ae38bd425051a59"></a>
## FileMeta

`struct` · `buoyant_kernel::FileMeta` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileMeta
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L219).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:219`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The metadata that describes an object.

<a id="op-cf7128218ac61bae3dbb28fb"></a>
## Error

`assoc_type` · `buoyant_kernel::FileMeta::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L241).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [263, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::DirEntry", "path": "DirEntry"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:241`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3141781d857a7f4d2ebbd8b"></a>
## as_url

`function` · `buoyant_kernel::FileMeta::as_url` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_url(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/path.rs#L96).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "crate::FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [99, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs"}, "trait": {"args": null, "id": "buoyant_kernel::path::AsUrl", "path": "AsUrl"}, "trait_path": "buoyant_kernel::path::AsUrl"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/path.rs:96`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39c6501dc3d9b8540f0f3930"></a>
## clone

`function` · `buoyant_kernel::FileMeta::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> FileMeta
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L218).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 17], "end": [218, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:218`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3241063a5f7c6074fc2f7cd"></a>
## cmp

`function` · `buoyant_kernel::FileMeta::cmp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn cmp(&self, other: &Self) -> Ordering
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L229).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [232, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:229`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57099e8663a280b03a60cfe5"></a>
## eq

`function` · `buoyant_kernel::FileMeta::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &FileMeta) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L218).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 24], "end": [218, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:218`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52a8817d0f73048844e539e9"></a>
## fmt

`function` · `buoyant_kernel::FileMeta::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L218).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 10], "end": [218, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:218`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1b77585b7b734516838b666"></a>
## last_modified

`struct_field` · `buoyant_kernel::FileMeta::last_modified` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
last_modified: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L223).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:223`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The last modified time as milliseconds since unix epoch

<a id="op-24a24076173e17c593dc8615"></a>
## location

`struct_field` · `buoyant_kernel::FileMeta::location` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
location: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L221).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:221`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The fully qualified path to the object

<a id="op-de3e9dc0a8e1d99169d66128"></a>
## new

`function` · `buoyant_kernel::FileMeta::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(location: Url, last_modified: i64, size: u64) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L267).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [280, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:267`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new instance of `FileMeta`

<a id="op-9a7531f81a47c61309cdc371"></a>
## partial_cmp

`function` · `buoyant_kernel::FileMeta::partial_cmp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L235).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [238, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:235`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71ce2818e9705b85d84f4fd9"></a>
## size

`struct_field` · `buoyant_kernel::FileMeta::size` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size: FileSize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L225).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:225`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The size in bytes of the object

<a id="op-89fb1c41dcd41b8d23c6a5f7"></a>
## try_from

`function` · `buoyant_kernel::FileMeta::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(ent: DirEntry) -> DeltaResult<FileMeta>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L243).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::FileMeta", "path": "FileMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [263, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "std::fs::DirEntry", "path": "DirEntry"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:243`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
