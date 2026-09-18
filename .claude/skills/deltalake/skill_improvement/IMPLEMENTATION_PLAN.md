# Delta Lake skill improvement implementation plan

Approved planning deliverable, 2026-09-18; implementation is now delivered. See the
[implementation report](IMPLEMENTATION_REPORT.md) for executed results and qualification limits.
The original work specification below is retained as planning provenance; proposed/not_run
statements describe that checkpoint rather than the current implementation.
Scope is the portable `.claude/skills/deltalake` skill; no host-application changes or changes to
the supporting ast-grep-ripgrep/rust-code-model skills are required.

Read [assessment](ASSESSMENT.md), [target design](TARGET_DESIGN.md), and
[worked contracts](CAPABILITY_EXAMPLES.md). The plan's commands and file layout are proposed unless
explicitly identified as executed in [evidence](evidence/README.md).

## Dependency-ordered work

| Stage | Concrete work and deliverables | Acceptance before advancing |
|---|---|---|
| D0. Freeze and preserve | Snapshot the active reader, acquisition and scripts. Preserve existing aliases, useful topic mental models and fixtures. Record exact delta-rs/kernel commits, lock bytes, docs/runtime profiles and current limitations. Save baseline evaluation inputs before corrections. | Full original-file hashes; acquisition hashes verified; captured lock matches original acquisition; profile distinctions explicit; no host changes. Planning evidence supplies an initial baseline, not a future frozen evaluation arm. |
| D1. Restore contracts and identity | Port raw-preservation and stable-identity machinery from DataFusion. Admit full docs/modules/members/associated outputs/fields/variants/type trees/source/links. Model public, inferred-returned and internal access separately; repair cross-capture joins and ambiguous builder links. | Compare every admitted record to its exact raw artifact. ID-renumber controls pass. `LoadBuilder` callable chain compiles while invalid import/internal-trait controls fail for the expected reason. Merge clause types never become standalone DML constructors. Unresolved links/access remain explicit. |
| D2. Build broad Delta routes | Generate task/representation/crate/effect routes, complete operation map, feature matrix and integration map. Cover all 13 acquired packages and exclusion reasons. Include CDF explicit builders, low-level writer flush/commit, clause builders and non-builder APIs. | Every admitted public entrypoint is discoverable by canonical/alias lookup. Each declared family has a route or explicit gap. Protocol enums are not labeled supported execution. Integration rows identify public entrypoint/access category. Queries can find a capability without knowing its crate. |
| D3. Author decisive contracts | Write the 18 initial families in TARGET_DESIGN, with reusable operation cards and conditional comparisons. Start with read/snapshot/session, write/replace/merge/schema, commit/replay, CDF and retention; then storage/catalog/kernel depth. Correct overclaims in topics and project hints from source and probes. | Each recommendation has a qualifying condition, alternative, exact input/output/effect contract, evidence and unknowns. All selected operation outputs resolve. No blanket transaction-ID idempotency, enum-support, cache/session or “defaults are suboptimal” advice. Reviewed-depth coverage is separate from inventory breadth. |
| D4. Extend reproducible probes | Retain the planning probes; add the controls in the matrix below. Build locked standalone consumers at the pinned commits; save commands, source/lock hashes, resolved features, toolchain and assertion-level results. Capture source queries with controls; use semantic tools only for unresolved type identity. | Positive and negative behavioral controls execute in the declared profiles. Errors specify commit phase. Plan/resource claims have appropriate measurements. Cloud and unsupported-platform gates report blocked/not_run, never inferred from local storage. Probe source changes invalidate old results. |
| D5. Add bounded retrieval | Adapt DataFusion `find`/`show`/`compare` to task vocabulary, representations, effects and state conditions. Show alternatives and why they match; expand full contracts/evidence on demand. Keep legacy indexes and direct files usable. | Byte-limited output has lossless continuation. Alias/canonical queries agree. Known difficult cases select the relevant brief rather than an internal implementation. Compare task-call/context costs against direct-file navigation; add FTS only if measured failures justify it. |
| D6. Evaluate and revise | Freeze both reader bundles and run the paired/held-out suite in EVALUATION. Use independent arms/judging when implementation is authorized; compile changed evaluator compositions. Correct retrieval and contract failures, then rerun affected cases with a new candidate identity. | No unresolved consequential contract errors; correct condition-sensitive decisions; executable integration for all claimed task completions. Report quality, uncertainty and retrieval cost separately. A pilot supports only its cases; no general improvement claim from counts or static fixtures. |
| D7. Package and qualify | Produce deterministic reader/research bundles, licenses, digest manifests, minimal cross-skill integration contracts, offline rebuild and copied-reader qualification. Preserve raw acquired artifacts and Cargo.lock for git-based replay. | Same lookups work after copying from an unrelated directory without original-project/sibling/cache access or Internet. Research regeneration reproduces artifacts from explicitly bundled inputs. Offline compile is claimed only if separately proven. Targets and extracted bundles excluded; repeated packages byte-identical. |
| D8. Refresh and release | Add a maintenance guide, replay/refresh separation and targeted invalidation for API/source/profile/protocol/candidate changes. Produce final implementation report and replace the entrypoint only after release gates. | Simulated changed API/default, source hash, profile and newly available alternative each reopen affected claims. Unaffected records stay stable. Report all passed/failed/blocked/not_run gates and remaining semantic gaps with dated receipts. |

D0 → D1 → D2 establishes a trustworthy discovery surface. D3 can author already-grounded records
alongside D2/D4 work. D5 consumes that reviewed model; D6 needs a frozen usable candidate. D7 can
be prototyped early, but final bundles follow corrections; D8 publishes only qualified outputs.
Do not wait for a large extraction framework before writing useful reviewed comparisons.

## Work partition and reuse

| Mechanism | Reuse from DataFusion | Delta-specific work |
|---|---|---|
| Full contracts | Raw tree/doc preservation, member records, scoped IDs, link diagnostics | Associated future outputs, callable unnameable types, private-trait accessibility, aliases across kernel packages |
| Reviewed records | Conditional alternatives, typed operation references, claims/evidence | Durable effects, failure phase, snapshot lifetime, protocol/table-property prerequisites |
| Discovery | Lexical/facet/alias reader and route generation | Complete operations beyond IntoFuture; clause builders; read/write/maintenance feature matrix; backend/catalog/engine roles |
| Evidence | Locked probe receipts, syntax controls, stable identity tests | Retained git capture lock, immutable source replay, Delta table fixtures and transaction outcomes |
| Delivery | Deterministic reader/research bundles, transfer tracing, licenses | Public/private captures, kernel fork identity, standalone minimal DataFusion seam, source/archive replay |
| Evaluation | Paired task design, independent judgment, explicit efficiency limits | Delta state/effects controls and executable caller compositions; held-out source/feature conditions |

Port only mechanisms needed for these deliverables. Initially use standard-library Python,
existing `rg`/`ast-grep`, Cargo and exact source/rustdoc. No additional runtime service, database,
embedding model or host environment change is required. A published crate's docs.rs JSON cannot
replace locally acquired rustdoc for this unpublished commit.

## Probe backlog and exact oracles

The four planning tests cover a narrow local profile. The following are implementation work,
not additional successful tests already claimed.

| Priority | Contract | Positive/control fixtures and oracle |
|---|---|---|
| P0 | Public access and async results | Inferred LoadBuilder, explicit return annotation on its awaited tuple; inaccessible direct import/private trait; CDF explicit build path; compare raw associated types |
| P0 | Snapshot/provider freshness | Existing planning test; add provider built from unloaded handle, explicit version, newer commit and stale/fresh plans; compare exact rows and versions |
| P0 | Session policy | Real SessionState versus trait wrapper with unique UDF/runtime configuration; InternalDefaults/DeriveFromTrait/RequireSessionState; assert preserved/lost/rejected capability |
| P0 | Delta versus raw files | Existing tombstone test; add partition columns, logical/physical names and deletion-vector fixture; compare exact logical rows to independent expected data |
| P0 | Write and replacement | Append/error/ignore/overwrite; valid and out-of-predicate batches; schema-mode and partition changes; validate rows, version, log actions and expected error |
| P0 | Merge | Null-key and duplicate-source cases, clause order, update/delete/insert/by-source, aliases, schema evolution; explicit expected multiplicities and metrics |
| P0 | Commit/replay | Existing sequential marker and post-commit-error controls; add competing writers/conflicting markers, max retries, reload/lookup recovery and a pre-commit validation failure; assert durable log state rather than exception class alone |
| P0 | CDF | Enablement before/after requested interval, inclusive version bounds, timestamps, update pre/post images, delete/insert, residual filter, beyond-head policy; assert rows/metadata/schema and resume checkpoint consequences |
| P0 | Protocol | Recognized-but-not-admitted feature, feature-excluded profile, admitted read path, affected DML and maintenance; test separate cells rather than one universal support flag |
| P0 | Retention | Controlled clock; dry-run versus actual removal, full/lite, kept versions, low-retention guard; read historical version before/after vacuum and distinguish restore from time travel |
| P1 | Schema fidelity | Nullable/nested fields, decimals, unsigned input, timestamp units/timezones/nanoseconds, mapping metadata; assert types as well as values and admission changes |
| P1 | Writers/resources | Batches/plan/RecordBatchWriter path; visibility before/after flush/commit; target size versus actual files, bounded pool/spill and concurrency; do not infer RSS from stream shape |
| P1 | Optimize/checkpoint | Compact/Z-order selection and metrics, repeated/no-op run, commit count; checksum rows and inspect versions/files/checkpoint artifacts |
| P1 | Storage/catalog/kernel | Local custom-store registration and existing mapping; explicit backend feature profiles; catalog/engine adapter compile controls; live cloud gates only with appropriate configured environments |

Use fixtures confined to disposable local tables. A cloud API compiling is not proof of remote
atomicity, credentials or backend operation. Retain explicit input data/fixture provenance,
especially for protocol features that cannot be generated by the selected writer.

## Acceptance and truthful release reporting

Use four gate states: `passed`, `failed`, `blocked`, `not_run`. A missing fixture or credential is
not a successful empty test. Within reference claims, separately retain evidence strength and
unknowns: a source observation is not a failed runtime test, and a proposed interpretation is not
an executed result.

Release qualification consists of contract preservation/access, broad routing, reviewed semantic
depth, executed compositions, evidence receipts, bounded retrieval, paired decisions, copied
reader/research qualification and invalidation controls. Do not replace any of those with a single
test percentage. The active skill's current 27 navigation checks and 15 structural test groups
remain useful regression checks, but cannot establish those new properties.

Final reporting should identify baseline/candidate hashes, exact tested profile, records reviewed,
task outcomes and limitations, bundle sizes and paths, and any unrun remote/resource conditions.
The DataFusion pilot's final corrections were not retroactively part of its frozen candidate;
preserve that distinction here too.
