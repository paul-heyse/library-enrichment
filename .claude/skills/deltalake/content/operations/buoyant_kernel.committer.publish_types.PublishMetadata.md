# `buoyant_kernel::committer::publish_types::PublishMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.publish_types.PublishMetadata.json).

<a id="op-c9fe0cbc7fc4e65976b1d6ab"></a>
## PublishMetadata

`struct` · `buoyant_kernel::committer::publish_types::PublishMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PublishMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L90).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:90`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metadata required for publishing catalog commits to the Delta log.

`PublishMetadata` bundles all the information needed to publish catalog commits: the version up
to which commits should be published, and the list of catalog commits themselves.

# Invariants

The following invariants are enforced at construction time:
- `commits_to_publish` must be non-empty
- `commits_to_publish` must be contiguous (no version gaps) in ascending order of version
- The last catalog commit version must equal `publish_to_version`

See [`Committer::publish`] for details on the publish operation.

[`Committer::publish`]: super::Committer::publish

<a id="op-4be023569e53be43991965d7"></a>
## commits_to_publish

`function` · `buoyant_kernel::committer::publish_types::PublishMetadata::commits_to_publish` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commits_to_publish(&self) -> &[CatalogCommit]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L116).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::PublishMetadata", "path": "PublishMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [150, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:116`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The list of contiguous catalog commits to be published, in ascending order of version.

<a id="op-cf9429bd10c6fdeca5a6677e"></a>
## publish_version

`function` · `buoyant_kernel::committer::publish_types::PublishMetadata::publish_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn publish_version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L111).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::PublishMetadata", "path": "PublishMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [150, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:111`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The snapshot version up to which all catalog commits must be published.

<a id="op-48c27553110b43d773d5ea95"></a>
## try_new

`function` · `buoyant_kernel::committer::publish_types::PublishMetadata::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(publish_to_version: Version, commits_to_publish: Vec<CatalogCommit>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::publish_types::PublishMetadata", "path": "PublishMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [150, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:98`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new `PublishMetadata` with the given publish to version and catalog commits.

<a id="op-dfcf5bfbc19fdebc89036a31"></a>
## commits_to_publish

`struct_field` · `buoyant_kernel::committer::publish_types::PublishMetadata::commits_to_publish` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commits_to_publish: Vec<CatalogCommit>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L92).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:92`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3324cd47a632d8a4401920c"></a>
## publish_to_version

`struct_field` · `buoyant_kernel::committer::publish_types::PublishMetadata::publish_to_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
publish_to_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/publish_types.rs#L91).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/publish_types.rs:91`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
