# Assessment of the current Delta Lake skill

Verified 2026-09-18 against the existing reader, its acquired rustdoc, exact source and four
executed local Rust probes. This assessment proposes changes; the active skill has not been edited.

## Main conclusion

Retain the existing acquisition and broad inventory, then add a reviewed decision layer and repair
contract loss in the renderer. The highest-value Delta-specific improvement is to characterize
**table/snapshot state and durable effects alongside Rust inputs and outputs**. Correct names and
builder lists alone do not explain snapshot freshness, transaction replay, post-commit errors,
protocol admission or the difference between logical deletion and physical object removal.

The [target design](TARGET_DESIGN.md) adapts the DataFusion work, and the
[implementation plan](IMPLEMENTATION_PLAN.md) makes the required corrections testable.

## What is already useful

- Exact delta-rs and kernel commits; local public/private rustdoc captures, hashes and resolved
  feature envelopes. The kernel's branch dependency is recognized as distinct from its actual pin.
- Canonical symbols, aliases, methods, foreign impls, unnameable types, feature exclusions, upstream
  examples/tests/guides and protocol material. These support broad discovery without an LLM index.
- Fifteen topic axes and concrete source-oriented query recipes. Topic explanations often identify
  useful seams, even where advice needs qualification or stronger evidence.
- Existing checks passed: 587 recorded content hashes, integrity for 804 symbols/211 API pages,
  15 structural rule groups and 27 navigation checks. The command used `--skip-rebuild`: its
  “determinism” label means existing hashes matched, not that a rebuild was run. Its transfer check
  is a text scan of builder modules, not execution of a copied bundle.

Evidence: [baseline check](evidence/baseline-verification.json),
[log](evidence/logs/baseline-verification.log), [inventory](evidence/inventory.json).

## Exact profile, not a published-version substitution

| Component | Assessed identity |
|---|---|
| delta-rs | `58f07cd62bfbce3649a7e1c87c696288068ae184`, committed 2026-09-15 |
| buoyant kernel / engine / derive | `8ba063f8f84fec222000f66d40d70911d7c79675` |
| Documentation producer | `nightly-2026-09-13`, rustc `1.100.0-nightly (809936eac)`, rustdoc format 61, Linux x86_64 |
| Locked common dependencies | DataFusion 55.1.0; Arrow/Parquet 59.3.0; object_store 0.13.2 |
| Runtime probe compiler | Stable 1.98.1; exact details in the retained compiler log |
| Documentation scope | 13 acquired packages, broad cloud/catalog/nanosecond profile; mutually exclusive features explicitly excluded |
| Runtime scope | Facade `datafusion` plus default `rustls`, local memory/filesystem; resolved transitive feature profile retained |

The [source manifest](evidence/source-manifest.json) verifies selected source bytes against git
objects at the exact commit and checks every acquired artifact hash. It retains 98 source files
with commit URLs and upstream license. The recovered [acquisition lock](evidence/acquisition.Cargo.lock)
matches the original recorded SHA-256. The probe has its own
[Cargo.lock](evidence/probes/Cargo.lock) and [resolved profile](evidence/runtime-profile.json).

The upstream [commit manifest](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/Cargo.toml)
declares the compatible dependency families. Context7 returned useful provider/CDF discovery
examples, but links to `main` do not identify this capture; the
[retained response](evidence/context7.json) is not exact-commit proof. No dependency on the newest
published release, a moving branch tip, or a stale upgrade guide is proposed.

## Concrete gaps and their implications

| Finding | Current evidence | Why it changes the design |
|---|---|---|
| Member contracts are reduced to summaries | Current method records have only name/signature/summary/via_trait; summaries top out at 240 characters. Raw core rustdoc includes 64 associated-type records, 332 fields, 210 variants and 70 modules, whose full structure is not available as the proposed reader contracts. | Preserve docs/raw structure before writing more summaries. Include `IntoFuture::Output`, fallibility, fields/variant payloads, source and resolved doc links. Counts alone do not imply all these records belong in the public surface. |
| Awaited results are hard to discover | `LoadBuilder` page lists `into_future → Self::IntoFuture`, but the source says `Result<(DeltaTable, SendableRecordBatchStream)>`; write, merge and vacuum return different shapes. | Index operation construction and actual async output separately; compile small compositions. |
| “24 operations” is a structural subset | `CdfLoadBuilder` is indexed but absent from the operation catalog because it does not implement IntoFuture. Public `build` returns an execution plan; provider construction is another entrypoint. | Derive operation candidates from more than one trait; include explicit build/execute/flush/commit and clause methods. |
| Merge clause methods link to standalone DML builders | Catalog constructor inference splits rendered signature text at the last `->`, then matches leaf names. `when_matched_update` has `FnOnce(UpdateBuilder) -> UpdateBuilder` but returns `DeltaResult<MergeBuilder>`. | Follow structured nominal return types. Do not infer semantic ownership from leaf-name or formatted-signature matches. |
| Private supplement access needs another distinction | LoadBuilder's page includes methods via the crate-private `Operation` trait; the trait itself is `pub(crate)`. Internal trait implementations and public returned types are different access categories. | Preserve internal evidence without advertising inaccessible methods as caller APIs. Add expected compile-failure controls. |
| Feature enumeration overstates support | `catalogs/table-features.md` calls enum variants “what this pin can honour.” The source's protocol admission sets are narrower and feature-gated; e.g. IdentityColumns is recognized but absent from the writer insertion set. | Separate recognition, admission, operation support and executed fixtures. No universal support boolean. |
| Transaction marker advice overclaims | Topic and project hint imply that supplying an application transaction ID makes retries idempotent. The sequential duplicate-marker probe publishes two rows and two versions. | Describe marker persistence, lookup, caller policy and conflicts independently. A property setter is not an end-to-end replay guarantee. |
| Error versus publication is not modeled | Injected `before_post_commit_hook` error reaches the caller after the version is visible to a refreshed observer. | Include failure phase and possible effects in operation contracts; re-observe state before designing recovery. |
| Some recommendations are unconditional | “Reuse one SessionContext,” “write schema explicitly,” and defaults framed as chosen against the workload lack condition-sensitive alternatives. | Carry over the DataFusion lesson: qualify reuse by isolation/policy/lifetime, distinguish intentional inference, and evaluate defaults rather than treating them as defects. |
| Commit mechanism is described too narrowly | Transaction topic calls every commit an atomic rename; the implementation delegates publication to `LogStore::write_commit_entry` and has backend-dependent paths. | Explain atomic log-version publication and the selected backend's implementation separately. Local tests do not certify cloud behavior. |
| Capture replay has an avoidable dependency | Acquired artifacts record the lock hash, but acquisition currently regenerates a lock and checks the kernel branch resolution. The tracked acquired directory does not retain the matching Cargo.lock. | Store lock bytes, add replay versus refresh modes, and bind resolved closure to capture identity. Branch movement should not destroy replay of an existing capture. |

Specific raw member and associated-output evidence is preserved in
[rustdoc fragments](evidence/rustdoc-fragments.json). Source references for the above are
[catalog construction](../build/catalogs.py),
[load builder](evidence/sources/delta-rs/crates/core/src/operations/load.rs),
[merge clauses](evidence/sources/delta-rs/crates/core/src/operations/merge/mod.rs),
[operation visibility](evidence/sources/delta-rs/crates/core/src/operations/mod.rs),
[protocol admission](evidence/sources/delta-rs/crates/core/src/kernel/transaction/protocol.rs),
[session policy](evidence/sources/delta-rs/crates/core/src/delta_datafusion/session.rs) and
[transaction phases](evidence/sources/delta-rs/crates/core/src/kernel/transaction/mod.rs).

## Integration inventory: preserve the correction already present

The current foreign-impl index contains 8 ExecutionPlan, 9 DisplayAs, 6 ExtensionPlanner,
3 PruningStatistics, 4 UserDefinedLogicalNodeCore and 3 ScalarUDFImpl rows. It already distinguishes
nameable and internal implementations. In particular, the UDF rows are ZOrderUDF, MakeParquetArray
and ToJson—not only ZOrderUDF. The earlier undercount concern is not a reason to claim these
implementations are now missing.

The remaining problem is the next question: **which public operation reaches this implementation,
under what conditions, and is it something a caller should use directly?** Add entrypoint/task and
registration links, not another unspecific list of trait names. The crate map also counts 12
defining packages while acquisition counts 13, because the facade defines no canonical items;
route the facade explicitly instead of presenting these as contradictory package totals.

## Executed behavioral evidence

All four tests passed in the declared local profile. See
[receipt](evidence/runtime-receipt.json), [source](evidence/probes/tests/contracts.rs), and
[full output](evidence/logs/runtime-2.log).

| Probe | Observation | Boundary |
|---|---|---|
| Provider freshness and LoadBuilder | Old provider returns 2 rows after an append; refreshed table and newly built provider return 3. A loaded v1 snapshot wins over provider `with_table_version(0)`; explicit `load_version(0)` returns 2. Inferred LoadBuilder compiles and returns table/stream. | No claim about external catalog refresh or inaccessible import compile control. |
| Repeated marker | Two sequential appends with the same `(app_id, version=7)` produce 2 rows and table version 1; marker lookup returns 7. | Caller lookup is a replay building block; concurrent exactly-once behavior was not tested. |
| Post-commit error | An injected hook error is returned, while an observer refresh sees version 1 and both rows. | Log publication observed in local memory storage; no process-crash durability or remote fault injection. |
| Delta versus Parquet | Delta delete returns 0 logical rows; raw local Parquet directory scan still returns the 2 obsolete rows before vacuum. | Tombstones only; partition reconstruction, column mapping and deletion vectors remain future fixtures. |

The retained structural-query controls also show that the existing write hint matches an unrelated
same-name call, while a write with default commit properties escapes it without a transaction
marker. This is expected of syntax-only matching, but the hint's wording must not promote those
matches or omissions to correctness judgments. [Query definitions/results](evidence/query-runs.json).

## Lessons carried forward from DataFusion

1. Full upstream contracts must survive normalization. A reviewed brief should add meaning,
   not replace source/doc authority with a short paraphrase.
2. Broad inventory and reviewed depth must be separate coverage dimensions. The small decisive
   alternatives usually matter more than another large symbol list.
3. Conditional claims need executable counterexamples. These four Delta probes already reveal
   decisions that a navigation regex cannot test.
4. Bounded retrieval must earn its complexity. The DataFusion pilot improved contract detail but
   took more recorded commands/time; no automatic efficiency gain follows from better structure.
5. Portability needs an executed copied-bundle test. Regexes excluding host imports cannot catch
   interpreter site hooks or hidden source-cache dependencies.
6. Evaluation answers and the final corrected candidate must have distinct identities. Preserve
   actual agent compositions and compile them before claiming integration improvement.

Those lessons are recorded in the existing
[DataFusion implementation report](../../datafusion/skill_improvement/IMPLEMENTATION_REPORT.md).
This plan borrows its verified mechanisms without requiring that sibling skill at runtime.

## Remaining uncertainty

No comparative Delta agent evaluation, replacement reader, full contract normalization, copied
bundle qualification, cloud tests, concurrency/fault matrix or exhaustive protocol support tests
were executed here. Proposed gates remain `not_run`. The planning probes establish only their
named assertions. They are sufficient to motivate the design and prioritize implementation,
not to label every proposed contract reviewed or every Delta capability supported.
