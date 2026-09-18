# `buoyant_kernel::committer::commit_types::CommitMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.commit_types.CommitMetadata.json).

<a id="op-f0a4d85b92f1e66219725449"></a>
## CommitMetadata

`struct` · `buoyant_kernel::committer::commit_types::CommitMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L111).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:111`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`CommitMetadata` bundles the metadata about a commit operation. This includes the commit path,
version, and protocol/metadata state of the table being committed to. Catalog committers can
use the protocol and metadata getters to validate or inspect the commit.

Note that this struct cannot be constructed. It is handed to the [`Committer`] (in the
[`commit`] method) by the kernel when a transaction is being committed.

See the [module-level documentation] for more details.

[`Committer`]: super::Committer
[`commit`]: super::Committer::commit
[module-level documentation]: crate::committer

<a id="op-d5db08a38bec1260c88420c1"></a>
## commit_type

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::commit_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commit_type(&self) -> CommitType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L166).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:166`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The type of commit operation being performed.

<a id="op-d175041b15032c674371d9ba"></a>
## fmt

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L110).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 10], "end": [110, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:110`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5317a957066fe9598e48471c"></a>
## has_domain_metadata_change

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::has_domain_metadata_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_domain_metadata_change(&self, domain: &str) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L247).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:247`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if this commit includes a domain metadata change for the given domain name.

<a id="op-5d5bf62545709dfc25625ba6"></a>
## has_metadata_change

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::has_metadata_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_metadata_change(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L242).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:242`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if this commit changes the table's metadata.

<a id="op-971f5e204b22b9c75daa483f"></a>
## has_protocol_change

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::has_protocol_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_protocol_change(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L237).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:237`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if this commit changes the table's protocol.

<a id="op-621e4162883a7254b853da59"></a>
## has_reader_feature

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::has_reader_feature` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_reader_feature(&self, feature_name: &str) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L223).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:223`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Check if the effective protocol has a specific reader feature by name.

<a id="op-3081b2ab146cf9f09e6e6efc"></a>
## has_writer_feature

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::has_writer_feature` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_writer_feature(&self, feature_name: &str) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:215`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Check if the effective protocol has a specific writer feature by name.

<a id="op-dc350e12224e881cc4f32f0c"></a>
## in_commit_timestamp

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::in_commit_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn in_commit_timestamp(&self) -> i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L172).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:172`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The in-commit timestamp for the commit. Note that this may differ from the actual commit
file modification time.

<a id="op-567fc87528a61e8002fbd27c"></a>
## max_published_version

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::max_published_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn max_published_version(&self) -> Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L177).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:177`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The maximum published version of the table.

<a id="op-429dc1f98c5f3334b57c3c47"></a>
## metadata_configuration

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::metadata_configuration` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata_configuration(&self) -> Option<&HashMap<String, String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L232).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:232`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the raw metadata configuration for the effective metadata. Returns `None` if no
metadata is set.

<a id="op-e32df32b7273c40c78b3d770"></a>
## published_commit_path

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::published_commit_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn published_commit_path(&self) -> DeltaResult<Url>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L146).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:146`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The commit path is the absolute path (e.g. s3://bucket/table/_delta_log/{version}.json) to
the published delta file for this commit.

<a id="op-32519e4ae36550c6f41f79bc"></a>
## staged_commit_path

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::staged_commit_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn staged_commit_path(&self) -> DeltaResult<Url>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L154).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The staged commit path is the absolute path (e.g.
s3://bucket/table/_delta_log/{version}.{uuid}.json) to the staged commit file.

<a id="op-aa671a1d4167d33a7c65fbac"></a>
## table_root

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::table_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_root(&self) -> &Url
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L182).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:182`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The root URL of the table being committed to.

<a id="op-ffc5fefc6f9d1e05155c596f"></a>
## version

`function` · `buoyant_kernel::committer::commit_types::CommitMetadata::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L161).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitMetadata", "path": "CommitMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [316, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:161`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version to which the transaction is being committed.

<a id="op-1c50bd80d77b8d2e31f987a2"></a>
## commit_type

`struct_field` · `buoyant_kernel::committer::commit_types::CommitMetadata::commit_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_type: CommitType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L114).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6094c28b3635d307151b4ea"></a>
## domain_metadata_changes

`struct_field` · `buoyant_kernel::committer::commit_types::CommitMetadata::domain_metadata_changes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
domain_metadata_changes: Vec<actions::DomainMetadata>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L120).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:120`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Domain metadata actions in this commit (additions and removals).

<a id="op-112491911c7e2ff3f524661f"></a>
## in_commit_timestamp

`struct_field` · `buoyant_kernel::committer::commit_types::CommitMetadata::in_commit_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
in_commit_timestamp: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L115).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc6d9d0482fed87d3bedb71e"></a>
## log_root

`struct_field` · `buoyant_kernel::committer::commit_types::CommitMetadata::log_root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_root: path::LogRoot
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L112).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:112`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-611b7bf2119aeaf253fd37e6"></a>
## max_published_version

`struct_field` · `buoyant_kernel::committer::commit_types::CommitMetadata::max_published_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_published_version: Option<Version>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L116).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:116`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8e62d661492fb1b34cab3b7"></a>
## protocol_metadata

`struct_field` · `buoyant_kernel::committer::commit_types::CommitMetadata::protocol_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
protocol_metadata: CommitProtocolMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L118).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:118`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Protocol and metadata state for this commit.

<a id="op-c2b4938b07d1d089e4ff2075"></a>
## version

`struct_field` · `buoyant_kernel::committer::commit_types::CommitMetadata::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L113).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
