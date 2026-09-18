# `buoyant_kernel::incremental_scan`

Crate `buoyant_kernel` · 6 public items · structured records in [`model/buoyant_kernel.incremental_scan.json`](../model/buoyant_kernel.incremental_scan.json)

## IncrementalListing

`struct` · `buoyant_kernel::incremental_scan::IncrementalListing`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.incremental_scan.IncrementalListing.md)

Also reachable as `delta_kernel::incremental_scan::IncrementalListing`

```rust
struct IncrementalListing
```

**Fields**: `summary`, `add_files`

**Derives**: Debug

Eager output of [`IncrementalScanStream::into_listing`]: the buffered Add
batches plus the summary (no cross-snapshot classification).

---

## IncrementalListingAgainstBase

`struct` · `buoyant_kernel::incremental_scan::IncrementalListingAgainstBase`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.incremental_scan.IncrementalListingAgainstBase.md)

Also reachable as `delta_kernel::incremental_scan::IncrementalListingAgainstBase`

```rust
struct IncrementalListingAgainstBase
```

**Fields**: `summary`, `add_files`

**Derives**: Debug

Eager output of [`IncrementalScanStream::into_listing_against_base_iter`] (or its
closure variant): the buffered
Add batches plus the classified summary.

---

## IncrementalScanBuilder

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.incremental_scan.IncrementalScanBuilder.md)

Also reachable as `delta_kernel::incremental_scan::IncrementalScanBuilder`

```rust
struct IncrementalScanBuilder
```

**Derives**: Debug

**Methods** (1)

```rust
fn build(self, engine: &dyn Engine) -> DeltaResult<Option<IncrementalScanStream>>
```

Builder for an incremental scan over `(base_version, target_version]`. Construct via
[`crate::Snapshot::incremental_scan_builder`] and drive with
[`IncrementalScanBuilder::build`].

---

## IncrementalScanStream

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanStream`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.incremental_scan.IncrementalScanStream.md)

Also reachable as `delta_kernel::incremental_scan::IncrementalScanStream`

```rust
struct IncrementalScanStream
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (6)

```rust
fn into_listing(self) -> DeltaResult<IncrementalListing>
fn into_listing_against_base_closure(self, base_contains: impl Fn(&FileActionKey) -> bool) -> DeltaResult<IncrementalListingAgainstBase>
fn into_listing_against_base_iter<'a>(self, base_keys: impl IntoIterator<Item = &'a FileActionKey>) -> DeltaResult<IncrementalListingAgainstBase>
fn into_summary(self) -> DeltaResult<IncrementalScanSummary>
fn into_summary_against_base_closure(self, base_contains: impl Fn(&FileActionKey) -> bool) -> DeltaResult<IncrementalScanSummaryAgainstBase>
fn into_summary_against_base_iter<'a>(self, base_keys: impl IntoIterator<Item = &'a FileActionKey>) -> DeltaResult<IncrementalScanSummaryAgainstBase>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Streaming output of an incremental scan over `(base_version, target_version]`.
Yields live Add batches as [`FilteredEngineData`] in newest-first order via
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

---

## IncrementalScanSummary

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanSummary`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.incremental_scan.IncrementalScanSummary.md)

Also reachable as `delta_kernel::incremental_scan::IncrementalScanSummary`

```rust
struct IncrementalScanSummary
```

**Fields**: `base_version`, `target_version`, `live_adds`, `removes`

**Derives**: Debug

Live file-key sets without cross-snapshot classification, returned by
[`IncrementalScanStream::into_summary`].

Each set element is a [`FileActionKey`] of `(path, dv_unique_id)`. Consumers should match
on the full key rather than just the path: the same path with different DV ids (e.g. an
`add(P, dv=new) + remove(P, dv=old)` DV-replacement pair) refers to distinct logical
files, and collapsing to path alone loses that distinction.

---

## IncrementalScanSummaryAgainstBase

`struct` · `buoyant_kernel::incremental_scan::IncrementalScanSummaryAgainstBase`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.incremental_scan.IncrementalScanSummaryAgainstBase.md)

Also reachable as `delta_kernel::incremental_scan::IncrementalScanSummaryAgainstBase`

```rust
struct IncrementalScanSummaryAgainstBase
```

**Fields**: `base_version`, `target_version`, `duplicate_adds`, `removes`

**Derives**: Debug

Cross-snapshot-classified file-key sets, returned by
[`IncrementalScanStream::into_summary_against_base_iter`] (or its closure variant).

To advance a delta-on-base file listing cache, append the streamed Add batches to
the delta layer and use the union `removes U duplicate_adds` as the remove-mask
against the base. Both sets are required: `removes` masks files that left the table;
`duplicate_adds` masks the stale base entry of each file the range re-added with new
metadata. Matching against the full `(path, dv_unique_id)` key (not path alone) keeps
distinct DV-revision entries separate.

---
