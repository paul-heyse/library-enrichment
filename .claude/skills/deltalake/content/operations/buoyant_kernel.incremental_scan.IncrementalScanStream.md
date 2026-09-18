# `buoyant_kernel::incremental_scan::IncrementalScanStream`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.incremental_scan.IncrementalScanStream.json).

<a id="op-b02b09e5e0c4215366522d6e"></a>
## IncrementalScanStream

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanStream` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct IncrementalScanStream
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L151).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:151`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Streaming output of an incremental scan over `(base_version, target_version]`.
Yields live Add batches as [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab) in newest-first order via
[`Iterator::next`]; call [`into_summary`] or [`into_listing`] to terminate and recover
the live file-key sets.

An Add is "live" at the target if, walking commits newest-first and keying by
`(path, dv_unique_id)`, it is the first occurrence of that key (first-seen-wins dedup)
and no later commit in the range contains a Remove for the same key — i.e. the file is
still in the table at `target_version`.

On error, `next()` yields `Some(Err(_))` once and returns `None` on every subsequent
call; the stream's dedup state is incomplete, so terminal methods then return `Err`
rather than producing a partial summary.

[`into_summary`]: Self::into_summary
[`into_listing`]: Self::into_listing

Unresolved upstream links (retained, not inferred): ``Iterator::next``.

<a id="op-7fa0df243fb63eba85606b63"></a>
## Item

`assoc_type` · `buoyant_kernel::incremental_scan::IncrementalScanStream::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = Result<FilteredEngineData, Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L162).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [190, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:162`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fe33e05ebc69b707fd11269"></a>
## fmt

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L372).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [371, 1], "end": [381, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:372`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3a7df95276505f9561d2219"></a>
## into_listing

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::into_listing` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_listing(self) -> DeltaResult<IncrementalListing>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L242).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [369, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:242`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Eager helper: collect every live Add batch into a [`Vec`], then call
[`into_summary`] to recover the file-key sets. Use this when the diff fits in memory
and the consumer prefers the simpler eager shape over the iterator pattern.

The returned `Vec` may contain multiple batches per source commit: the engine is
free to split a commit's rows across [`ActionsBatch`] yields (e.g. by JSON-reader
batch-size limits). One yielded batch produces at most one `Vec` entry, and a
commit whose Adds were all cancelled by later Removes produces no entry at all.

# Errors
See [`into_summary`].

[`into_summary`]: Self::into_summary
[`ActionsBatch`]: crate::log_replay::ActionsBatch

Unresolved upstream links (retained, not inferred): ``Vec``.

<a id="op-91c0bbcf5536d134a2183a60"></a>
## into_listing_against_base_closure

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::into_listing_against_base_closure` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_listing_against_base_closure(self, base_contains: impl Fn(&FileActionKey) -> bool) -> DeltaResult<IncrementalListingAgainstBase>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L358).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [369, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:358`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Predicate form of [`into_listing_against_base_iter`]; see
[`into_summary_against_base_closure`] for the rationale.

# Errors
See [`into_summary`].

[`into_summary`]: Self::into_summary
[`into_listing_against_base_iter`]: Self::into_listing_against_base_iter
[`into_summary_against_base_closure`]: Self::into_summary_against_base_closure

<a id="op-7afed1cd4e74bd33ef2ded5b"></a>
## into_listing_against_base_iter

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::into_listing_against_base_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_listing_against_base_iter<'a>(self, base_keys: impl IntoIterator<Item = &'a FileActionKey>) -> DeltaResult<IncrementalListingAgainstBase>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L337).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [369, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:337`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Eager classified helper: collect every live Add batch and call
[`into_summary_against_base_iter`] against `base_keys`. Returns an
[`IncrementalListingAgainstBase`](../operations/buoyant_kernel.incremental_scan.IncrementalListingAgainstBase.md#op-7b6d50d0ae6015aa96ee0b19) with the classified summary.

Prefer [`into_listing_against_base_closure`] when your base supports `contains`-style
lookup — see [`into_summary_against_base_closure`] for the rationale.

# Errors
See [`into_summary`].

[`into_summary`]: Self::into_summary
[`into_summary_against_base_iter`]: Self::into_summary_against_base_iter
[`into_summary_against_base_closure`]: Self::into_summary_against_base_closure
[`into_listing_against_base_closure`]: Self::into_listing_against_base_closure

<a id="op-b575b2704177d751b5713872"></a>
## into_summary

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::into_summary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_summary(self) -> DeltaResult<IncrementalScanSummary>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L209).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [369, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:209`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Drain any unread batches, then return the live file-key sets without applying
cross-snapshot dedup. Consumers can intersect these against whatever base
data structure they prefer.

# Errors
- [`Error::IOError`](../operations/buoyant_kernel.error.Error.md#op-93adf5fd1d842c00848b2aa6), [`Error::ObjectStore`](../operations/buoyant_kernel.error.Error.md#op-27222d9c0d00af0d33fdc816), or [`Error::Reqwest`](../operations/buoyant_kernel.error.Error.md#op-5a61a4cc1945adaf549f656a) on transient I/O while
  reading commit JSONs. Retryable by rebuilding the stream.
- [`Error::FileNotFound`](../operations/buoyant_kernel.error.Error.md#op-f58dcfa1d09ba8ac4cbba335) if a commit was vacuumed between [`IncrementalScanBuilder::build`](../operations/buoyant_kernel.incremental_scan.IncrementalScanBuilder.md#op-3a54fd61804755c3505b7c49)
  and stream consumption. Rebuilding will likely return `Ok(None)` (commits unavailable);
  fall back to [`crate::Snapshot::scan_builder`](../operations/buoyant_kernel.snapshot.Snapshot.md#op-c9c6b7e23f7231ad4b5e40d8).
- [`Error::MalformedJson`](../operations/buoyant_kernel.error.Error.md#op-04360595e83972213a8f7b40) or [`Error::Arrow`](../operations/buoyant_kernel.error.Error.md#op-1dfd346043afb342350a8eba) (default-engine) on commit JSON corruption.
  Not retryable.
- [`Error::Generic`](../operations/buoyant_kernel.error.Error.md#op-5448ac938c976df2479cb9fb) on malformed `deletionVector` fields in a commit row, or on "cannot
  finish a stream that previously errored" when a terminal method is called after a prior
  `next()` returned `Err`. Rebuild to retry the latter; the former indicates table
  corruption.

<a id="op-1d7b630af20a2932bb6cb1f9"></a>
## into_summary_against_base_closure

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::into_summary_against_base_closure` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_summary_against_base_closure(self, base_contains: impl Fn(&FileActionKey) -> bool) -> DeltaResult<IncrementalScanSummaryAgainstBase>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L304).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [369, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:304`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Predicate form of [`into_summary_against_base_iter`]: pass a closure that answers
"is this key in your base?" Iterates the live-Add set (typically much smaller than
the base) and calls `base_contains` once per live-Add key.

Works with any base shape that supports membership lookup:
- `HashMap<FileActionKey, _>` -> `|k| map.contains_key(k)`
- `HashSet<FileActionKey>` -> `|k| set.contains(k)`
- `BTreeMap<FileActionKey, _>` -> `|k| map.contains_key(k)`
- Sorted `Vec<FileActionKey>` -> `|k| vec.binary_search(k).is_ok()`

# Errors
See [`into_summary`].

[`into_summary`]: Self::into_summary
[`into_summary_against_base_iter`]: Self::into_summary_against_base_iter

<a id="op-f583ebb1d9cacd91a9711db1"></a>
## into_summary_against_base_iter

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::into_summary_against_base_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_summary_against_base_iter<'a>(self, base_keys: impl IntoIterator<Item = &'a FileActionKey>) -> DeltaResult<IncrementalScanSummaryAgainstBase>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L271).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [369, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:271`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Drain any unread batches, then intersect `base_keys` against the live-Add file-key
set to compute `duplicate_adds` (file keys in the consumer's base listing that the
range re-adds with new metadata, e.g. OPTIMIZE / liquid clustering re-tag).

Streaming form: accepts borrowed keys from any source (`&HashSet`, `&Vec`, `&[…]`,
`map.keys()`, custom iterator). Kernel streams `base_keys` once and probes the
live-Add set per element. Work is `O(|base|)` with no call-site clones. Use this
when your cache holds keys in a streamable shape and you don't have a faster
`contains` lookup.

Prefer [`into_summary_against_base_closure`] when your cache supports
`contains`-style lookup (HashMap, HashSet, BTreeMap, custom index). That form
iterates the (typically much smaller) live-Add set and probes your base via the
supplied closure, dropping work from `O(|base|)` to `O(|live_adds|)`.

# Errors
See [`into_summary`].

[`into_summary`]: Self::into_summary
[`into_summary_against_base_closure`]: Self::into_summary_against_base_closure

<a id="op-8cd434ff19cb651793e1a145"></a>
## next

`function` · `buoyant_kernel::incremental_scan::IncrementalScanStream::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L164).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::incremental_scan::IncrementalScanStream", "path": "IncrementalScanStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [190, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:164`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9032b679a2e3cf95fb5cb823"></a>
## actions

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanStream::actions` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
actions: FileDataReadResultIterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L154).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-038dd762f0dfc295886bccb4"></a>
## base_version

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanStream::base_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
base_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L152).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:152`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c1463b0ba2914aec1883d6a"></a>
## errored

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanStream::errored` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
errored: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L158).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:158`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48fd2ae179937a4b86f32a2c"></a>
## live_adds

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanStream::live_adds` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
live_adds: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L156).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:156`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66f0d091cda8f478548926b2"></a>
## removes

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanStream::removes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
removes: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L157).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:157`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fcc2cfa1760fe8208883d6b"></a>
## seen_file_keys

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanStream::seen_file_keys` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
seen_file_keys: std::collections::HashSet<log_replay::FileActionKey>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L155).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:155`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b9e8d51983e4703e7b82da9"></a>
## target_version

`struct_field` · `buoyant_kernel::incremental_scan::IncrementalScanStream::target_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
target_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/incremental_scan/mod.rs#L153).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/incremental_scan/mod.rs:153`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
