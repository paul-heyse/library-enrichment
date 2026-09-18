# `buoyant_kernel::committer::publish_types::CatalogCommit`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.publish_types.CatalogCommit.json).

<a id="op-09a679dbf07e9a165d8e189f"></a>
## CatalogCommit

`struct` · `buoyant_kernel::committer::publish_types::CatalogCommit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CatalogCommit
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A catalog commit that has been ratified by the catalog but not yet published to the Delta log.

Catalog commits are staged commits stored in `_delta_log/_staged_commits/` that have been
ratified (accepted) by the catalog but not yet copied to the main delta log as published
commits. This struct provides the information needed to publish a catalog commit.

See [`Committer::publish`] for details on the publish operation.

[`Committer::publish`]: super::Committer::publish

<a id="op-0f0de63ca4add72de9013f9a"></a>
## clone

`function` · `buoyant_kernel::committer::publish_types::CatalogCommit::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CatalogCommit
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L18).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::CatalogCommit", "path": "CatalogCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [18, 17], "end": [18, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c11a5b78ebded801f75a384"></a>
## fmt

`function` · `buoyant_kernel::committer::publish_types::CatalogCommit::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L18).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::CatalogCommit", "path": "CatalogCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [18, 10], "end": [18, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78b87c947ee9f421ca367688"></a>
## location

`function` · `buoyant_kernel::committer::publish_types::CatalogCommit::location` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn location(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::CatalogCommit", "path": "CatalogCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [61, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The location of the staged catalog commit file
(e.g., `s3://bucket/table/_delta_log/_staged_commits/00000000000000000001.uuid.json`).

<a id="op-74626fffdec800263d7ca1ce"></a>
## published_location

`function` · `buoyant_kernel::committer::publish_types::CatalogCommit::published_location` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn published_location(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L58).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::CatalogCommit", "path": "CatalogCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [61, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:58`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The target location where this commit should be published
(e.g., `s3://bucket/table/_delta_log/00000000000000000001.json`).

<a id="op-d30a93295cf87150a9f2cb7b"></a>
## version

`function` · `buoyant_kernel::committer::publish_types::CatalogCommit::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::CatalogCommit", "path": "CatalogCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [61, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version of this catalog commit.

<a id="op-27e2c091266e1aebab92c46b"></a>
## location

`struct_field` · `buoyant_kernel::committer::publish_types::CatalogCommit::location` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
location: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L21).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7ad19a2294ec1d3d947eca3"></a>
## published_location

`struct_field` · `buoyant_kernel::committer::publish_types::CatalogCommit::published_location` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
published_location: url::Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L22).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0b0b7597e3bc11b489837ec"></a>
## version

`struct_field` · `buoyant_kernel::committer::publish_types::CatalogCommit::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
