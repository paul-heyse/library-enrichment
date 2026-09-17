# FP05 native maintenance upstream verification

## Product follow-up: checked descriptor admission

Implemented and tested 2026-09-16 after the upstream inspection below. The vendor vacuum route now
calls `FileView::checked_deletion_vector_descriptor`, which reads the native Arrow view with checked
field/type/null handling and constructs the kernel descriptor with its `try_new`. It does not call
the upstream adapter getter's `parse().unwrap()`. Native `absolute_path` and the shared active-file
protection set still own path resolution and protection; the underlying vacuum selection algorithm
is unchanged. The existing upstream adapter method remains unmodified for other upstream consumers.

`resumed-vacuum-dv-oracle.log`: one actual Delta maintenance test passed in 0.74 s. It includes
current/historical vectors, relative/absolute encoded paths, foreign-root refusal, and a malformed
storage tag in the persisted Add action. The malformed case returns an error while the orphan and
DV objects remain present. This is focused maintenance evidence, not service-wide GC qualification.
`vendor/delta-rs/PROVENANCE.json` records both changed source files against the same upstream pin.

## Original upstream inspection

Retrieved 2026-09-16 from exact local primary sources. Source inspection only;
no builds or tests. This describes unmodified upstream checkouts, not the concurrent
product/vendor maintenance patch. Delta and DataFusion skills supplied API discovery.
The frozen source guide was consulted; exact commits below are the operative evidence.

## Source identity

- Delta source root: `/home/paul/.cargo/git/checkouts/delta-rs-dcb716bfdc369320/58f07cd/`,
  commit `58f07cd62bfbce3649a7e1c87c696288068ae184`.
- Kernel source root: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/`,
  commit `8ba063f8f84fec222000f66d40d70911d7c79675`.
- Object-store source root:
  `/home/paul/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/object_store-0.13.2/`.

## Vacuum: verified protection gap and native repair seam

`crates/core/src/operations/vacuum.rs:72–88` collects only
`.map_ok(|file| file.object_store_path())`. Both retained snapshots
(`collect_keep_version_paths`, lines 97–130) and the current snapshot (line 398)
use this helper. Neither contributes external deletion-vector objects to its
protection set. This proves the missing protection, not that every vacuum mode
necessarily selects every omitted DV file for deletion.
[Pinned source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L72).

The narrow repair is to insert each active Add's data path and, when present,
its resolved DV object's table-relative path into the same set. Existing
current-version and `keep_versions` callers then share the correction. No bitmap
read is necessary: retaining the containing object protects all offset slices,
and a set deduplicates shared DV objects. Propagate descriptor/path failures before
deletion rather than silently omitting a protection.

Native APIs:

- `FileView::deletion_vector_descriptor(&self) -> Option<DeletionVectorDescriptor>`
  in `crates/core/src/kernel/snapshot/iterators.rs:320–327` is public. Its documentation
  warns that it may be removed without deprecation; the exact pin matters.
  This returns **delta-rs's adapter descriptor**, defined in
  `crates/core/src/kernel/models/actions.rs:903–932` and re-exported through
  `crate::kernel`. It is not the kernel descriptor below and has no `absolute_path`
  method. The import at `iterators.rs:27` establishes this identity.
  [Source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L320).
- `DeletionVectorDescriptor::absolute_path(&self, parent: &Url) -> DeltaResult<Option<Url>>`
  in kernel `kernel/src/actions/deletion_vector.rs:257–284` decodes the UUID suffix
  and joins relative persisted DVs to `parent`; absolute persisted DVs are parsed
  as URLs without containment checks; inline DVs return `Ok(None)`.
  [Source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L257).
- The required bridge is the kernel descriptor's public checked constructor
  `try_new(storage_type: DeletionVectorStorageType, path_or_inline_dv: impl Into<String>,
  offset: Option<i32>, size_in_bytes: i32, cardinality: i64) -> DeltaResult<Self>`
  (`kernel/src/actions/deletion_vector.rs:178–235`). In delta-rs the renamed dependency
  is `delta_kernel`; its package is `buoyant_kernel`. The current vendor vacuum patch
  uses this checked conversion before calling `absolute_path`:

  ```rust
  let descriptor =
      delta_kernel::actions::deletion_vector::DeletionVectorDescriptor::try_new(
          descriptor.storage_type.as_ref().parse()?,
          descriptor.path_or_inline_dv,
          descriptor.offset,
          descriptor.size_in_bytes,
          descriptor.cardinality,
      )?;
  ```

  Adapter `StorageType::as_ref` returns the protocol tags `u`, `i`, `p`
  (`models/actions.rs:884–892`); parsing creates the distinct kernel enum.
  `try_new` checks nonnegative sizes/cardinality/offset, forbids inline offsets,
  validates the relative UUID's z85 suffix, and validates absolute URL syntax.
  Inline bitmap framing remains unchecked until bitmap reading, which maintenance
  path protection does not perform.
  [Adapter types](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L856),
  [Checked kernel constructor](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/deletion_vector.rs#L178).
- Public `LogStore::root_url(&self) -> &Url`,
  `crates/core/src/logstore/mod.rs:426–429`, exposes the configured table URL.
  The **crate-private** `LogStoreExt::table_root_url(&self) -> Url`, lines 496–507,
  guarantees its path ends with `/`, suitable for descriptor joining. The vendor
  patch can use this native helper; external callers cannot name this trait.
  [Source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L496).

**Getter qualification gap:** before this checked bridge runs, the existing
`DeletionVectorView::descriptor` in `iterators.rs:491–498` executes
`storage_type: self.storage_type().parse().unwrap()`. This invokes the adapter's
`StorageType::from_str`; unknown tags return an error which is then unwrapped.
The underlying storage-type accessor also unwraps its string value (lines 503–510).
Therefore malformed storage tags are not yet proven to return a fail-closed typed
error at this access point: they can panic. The checked kernel constructor cannot
repair a failure that occurs before it is called. The malformed-input oracle below
remains open pending qualification or a fallible adapter access path; this note
does not claim the current vendor bridge closes it.
[Exact getter](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/iterators.rs#L491).

### Exact URL-to-object-path composition

Inside delta-rs, `crate::logstore::object_store_path(&Url) -> DeltaResult<Path>`
(`logstore/mod.rs:633–638`) first calls public
`object_store::ObjectStoreScheme::parse(&Url) -> Result<(ObjectStoreScheme, Path), Error>`;
its fallback is `Path::parse(url.path())`. The helper is crate-private.
[Delta helper](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L633),
[object_store 0.13.2 source](https://docs.rs/object_store/0.13.2/src/object_store/parse.rs.html).

`object_store/src/parse.rs:105–139` handles `file:` by taking the URL path;
S3/GS/Azure schemes use their scheme-specific paths. Some HTTPS cloud endpoints
strip the first URL path component (bucket/container). Every successful branch
then calls `Path::from_url_path(path)`. For supported local file and cloud URLs,
prefer this fallible native conversion for both root and resolved DV; do not
invent filesystem-string or URL-string slicing. The Delta fallback preserves
URL percent spelling rather than decoding it, so it is not equivalent for an
unrecognized scheme. A strict maintenance policy can reject unrecognized schemes.

`Path::from_url_path` (`path/mod.rs:260–270`) percent-decodes once, requires UTF-8,
then validates using `Path::parse`. Thus spaces/Unicode and literal percent
characters need URL-appropriate encoding; malformed/non-Unicode decoded paths
must error. Do not run another decode on the returned Path. Existing Add paths
use their own native `FileView::object_store_path` semantics and should retain
that existing route.
[Path source](https://docs.rs/object_store/0.13.2/src/object_store/path/mod.rs.html).

After checking URL namespace identity, convert root and DV through the same
scheme-aware API, call `dv_path.prefix_match(&root_path)`, and collect the returned
`PathPart` iterator into `Path` (`FromIterator`, `path/mod.rs:416–424`).
`prefix_match` (lines 349–355) requires a `/` boundary: root `table` cannot match
`table-other`. Reject a missing prefix or an empty relative result.

Namespace checks must precede stripping: compare scheme, host, port and URL
credentials, reject unexpected query/fragment, and establish containment of the
decoded URL path at component boundaries as well. Checking only the converted
object-store path is insufficient for HTTPS endpoints that strip bucket/container:
different containers can otherwise produce identical converted suffixes. For
local files, this is URL/object-key containment, not a filesystem symlink security
proof; retain the existing admitted-root policy.

**Inline:** `None` means no external object to retain. **Outside-root persisted
DV:** it must never be rebased onto a local suffix or treated as an unreferenced
local object. For this service's strict root-scoped maintenance, fail closed before
deletion with the unsupported external reference identified. Upstream does permit
absolute URLs; the root restriction is an application policy, not a Delta protocol
rule. A more permissive implementation could explicitly leave foreign-root objects
untouched, but that cannot claim to protect them from another root's maintenance.

## Log cleanup: verified unbounded inventory and swallowed errors

`crates/core/src/protocol/checkpoints.rs::cleanup_expired_logs_for`, lines 114–215:

- Lines 123–126 collect `Vec<Result<ObjectMeta, _>>` for the entire listing.
- Lines 137 and 159 discard errors when deriving retention and checkpoint bounds.
- Lines 183–192 log listing errors and omit those entries from deletion.
- Lines 207–213 collect another vector of successfully deleted paths to count them.

Consequently upstream can return success after incomplete inventory and uses
memory proportional to inventory plus deletion results.
[Pinned source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/checkpoints.rs#L114).

Smallest bounded, fail-closed repair: consume the original listing with `try_next`,
admit a finite entry count and retained path-byte budget, and propagate any listing
error or budget breach **before invoking delete**. Count every listed entry toward
work admission, even if irrelevant; retain only bounded metadata needed by the
existing algorithm. Derive its minimum retained version and safe checkpoint from
that completed inventory, then stream selected paths to `delete_stream`; count
results with a checked fold rather than a vector. Validate a timestamp before the
existing debug `timestamp_millis_opt(...).unwrap()` can panic. An explicit bounded
entry point plus a conservative bounded default wrapper can retain call compatibility.

Preserve the actual upstream selection algorithm: checkpoint version is at most
the effective keep version; deletion requires version strictly below that checkpoint
and timestamp at most the cutoff. The prose above the function differs from the
implementation about checkpoint timestamp eligibility; this patch should not
silently change retention semantics based on that prose.

This is fail-closed **inventory admission**, not atomic deletion. A deletion-stream
error can follow successful earlier deletions and must remain an error. Listing is
not a transactional snapshot; existing maintenance leases/retained-version coordination
remain necessary. Filename matching alone is not proof that a multipart checkpoint
is complete. These limits are not fixed by a bounded vector or a second listing pass.

## Focused qualification oracles (proposed; not run)

1. Current and explicitly retained historical snapshots reference persisted relative
   and absolute same-root DV files with old mtimes. Full vacuum keeps these objects
   while deleting an unrelated eligible object. Read the protected snapshot afterward.
   Include shared-file DV offsets and duplicate references.
2. Inline DV contributes no fabricated file; malformed descriptor and foreign-root
   absolute DV fail before any delete call. Include sibling path prefixes, different
   HTTPS containers, percent characters, spaces and Unicode to verify exact keys.
3. Listing yields eligible files then an injected error: cleanup returns the error
   with zero delete calls. At and one beyond both inventory budgets verify acceptance
   versus zero-delete rejection; include irrelevant objects in the count budget.
4. With complete bounded inventory, check no-checkpoint no-op, non-monotonic commit
   timestamps, retained-version boundary and preservation of the chosen checkpoint.
   Compare deleted keys with the existing algorithm on the same successful inventory.
5. Inject deletion failure after one successful deletion: report failure with observable
   partial work; do not claim rollback. Verify empty inventory and invalid timestamp.

Disposition: a narrow delta-rs patch at the two existing helpers is justified;
the required descriptor/path APIs already exist at these pins. No kernel or
DataFusion change is required for these corrections. Product patch correctness
and the proposed oracles remain separately unqualified by this source-only note.

## Follow-up: maintenance CommitProperties propagation

Retrieved 2026-09-16 from the same unmodified upstream checkout; no tests/builds.

`CommitProperties` is `Clone` and contains application metadata, a vector of
application transactions, retry count, checkpoint enablement and optional log-cleanup
override (`crates/core/src/kernel/transaction/mod.rs:547–568`). Its default restores
`DEFAULT_RETRIES`, enables checkpoints, selects table-configured cleanup (`None`),
and empties transactions. `CommitBuilder::from` transfers all five fields
(lines 611–624). Thus copying metadata alone loses consequential policy.
[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L547).

### VACUUM start and end

`operations/vacuum.rs:651–665` constructs default start properties and copies only
metadata. In contrast, lines 687–696 add end metrics to the original properties and
build VACUUM END from them. An explicit `cleanup_expired_logs = Some(false)`, disabled
checkpoint creation, retry policy and transaction markers therefore reach END but
not START. END uses the successful START snapshot as its baseline.
[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L650).

Smallest uniform property-propagation correction: initialize
`let mut start_props = commit_properties.clone();` and retain the existing metrics
insertion. END already carries original properties; no reconstruction is needed.
This prevents the START hook from unexpectedly invoking cleanup or checkpointing
that the caller disabled. `transaction/mod.rs:1064–1105` confirms the cleanup override
controls the post-commit cleanup call; `None` delegates to table configuration.
[Post-commit hook](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1064).

### OPTIMIZE incremental commits and retry policy

`operations/optimize.rs:944–996` maintains a mutable snapshot. Each commit constructs
default properties, copies only metadata, adds readVersion/metrics, then explicitly
sets `.with_max_retries(DEFAULT_RETRIES + commits_made)`. After success it assigns
`snapshot = commit.snapshot().snapshot`. This both discards caller property fields
and overrides any caller retry bound.
[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L944).

Smallest correction: clone the supplied properties for each commit, insert the
existing operation metadata, remove the retry override and now-unused
`commits_made` counter/`DEFAULT_RETRIES` import. Preserve snapshot advancement.

**No additional retries are needed merely because OPTIMIZE made earlier commits.**
Their versions are already included in the next commit's updated snapshot.
The native transaction loop queries latest relative to that baseline, checks all
intervening concurrent versions when retries are enabled, and retries an atomic
version collision. `max_retries == 0` deliberately rejects a snapshot that has
fallen behind; this is caller policy, not a reason to raise the bound. The source
counts attempts, not one retry for each previous OPTIMIZE commit
(`transaction/mod.rs:869–918, 940–1006`). Concurrent external writers can still
cause conflicts/failure; propagating policy does not promise eventual success.
[Transaction loop](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L869).

### Application-transaction meaning remains explicit

Cloning properties also places caller transaction markers in START and each partial
OPTIMIZE commit. The commit builder serializes these as transaction actions
(`transaction/mod.rs:479–481`); that is low-level commit propagation, **not** a promise
of whole-maintenance-operation atomicity or completion. A crash after START or an
early OPTIMIZE commit can leave that marker durable before the operation finishes.
Do not use one propagated marker as proof of whole-operation completion without a
separate completion contract. If the intended API promises a completion marker only,
preserve all policy fields by cloning but explicitly stage transaction actions at
the completion boundary; this is a separate semantic decision, not an accidental
return to default properties. The service may instead keep maintenance transaction
vectors empty while retaining its existing explicit operation outcome records.
[Transaction actions](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L479).

Focused oracles: disable cleanup/checkpointing on a table whose defaults enable
them and observe neither START nor any partial OPTIMIZE commit invoking those hooks;
force multiple incremental commits with zero configured retries and no competitor
(must succeed without manufactured extra retries); inject a genuine concurrent
commit/collision and verify the configured bound; inspect all native commit JSONs
for metadata/transaction propagation under the selected marker contract. Retain
separate assertions for VACUUM START and END and for intermediate/final OPTIMIZE
commits. These oracles are proposed, not executed here.
