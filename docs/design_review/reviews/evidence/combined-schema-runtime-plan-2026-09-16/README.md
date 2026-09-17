# Combined schema/runtime plan: library evidence

Recorded **2026-09-16** for [Plan 17](../../../../plans/17-schema-governed-unified-runtime-hard-pivot.md).
This is planning evidence, not product acceptance. The source review used the
[DataFusion](../../../../../.claude/skills/datafusion/SKILL.md),
[Delta Lake](../../../../../.claude/skills/deltalake/SKILL.md) and
[FastMCP](../../../../../.claude/skills/fastmcp/SKILL.md) skills, then exact installed/check-out source.
Context7 was not used. No product code, dependency pin, installation or production state changed.

## Artifacts and exact basis

- [schema_probe.rs](schema_probe.rs): complete standalone probe source.
- [compile-command.json](compile-command.json): actual compiler argv and linked library hashes.
- [probe-run.log](probe-run.log): complete final successful probe output, including expected refusals.
- [delta-actions.json](delta-actions.json): selected native protocol/metadata/add actions from every
  probe table; `add.stats` JSON is decoded for readability, with log versions retained.
- [receipt.json](receipt.json): source/log/lock hashes, compiler, exact pins, run argv, exits and limits.
- [source-findings.md](source-findings.md): exact-source findings and current code consumers.
- [planning-checks.json](planning-checks.json): document links, scope carry-forward, formatting and
  evidence-integrity checks; separate from the unexecuted target product gates.

DataFusion is **55.1.0**, Arrow/Parquet **59.3.0**, object_store **0.13.2**, delta-rs
`58f07cd62bfbce3649a7e1c87c696288068ae184`, Buoyant kernel
`8ba063f8f84fec222000f66d40d70911d7c79675`. Cargo dep-info for the linked DataFusion/Delta libraries
points to those package/check-out directories. This is a cached-rustlib probe, not a fresh full
dependency build. Hashes identify the actual libraries. Rust is 1.98.1; the runtime uses two Tokio
threads and the same SessionContext with DeltaPlanner for the probe operations.

## Executed observations

The planning fixture uses Int64 values with `enrichment.id` metadata to isolate metadata behavior.
It does not qualify the target fixed-binary-ID representation or a registered application analyzer.

| Case | Observed result | Consequence |
|---|---|---|
| Projection / alias | Extension name and symbol-domain metadata retained in logical and collected schemas | These simple cases work; they do not prove arbitrary propagation |
| Same-type CAST | Extension metadata erased | A semantic checker cannot rely only on post-cast field tags |
| Same-domain UNION ALL | Both name and domain retained; two rows | Supported fixture, still require actual schema cases |
| Cross-domain UNION ALL | Logical field retains the extension name but loses domain metadata; collected field reports the definition-domain metadata; two rows | Inspect logical and physical/output contracts and refuse incompatible domains before coercion |
| Cross-domain equality JOIN | Accepted and returned one matching row | The base engine does not enforce the application's identity-domain separation |
| `min(id)` | Extension metadata erased | Define/validate domain-preserving aggregate behavior explicitly |
| Struct reorder | `{b:20,a:10}` → `{a,b}` yields `{a:10,b:20}` | Native cast is name based in this fixture, not positional |
| Struct rename/missing child | `{a:10,c:30}` → `{a,b}` yields `{a:10,b:NULL}` | Reject undeclared shape changes before native null filling |
| Required child, present valid parent | Accepted; data commit version 1 | Basic native nested NOT NULL route exists |
| Required child, present parent, NULL child | Refused by native validation | Keep native validation where its semantics match |
| Required child, NULL optional parent | Refused by native validation | Blindly restoring nested NOT NULL would reject legal optional variants |
| Nullable child plus metadata-only CHECK, NULL parent | Accepted | This alone does not establish constraint enforcement |
| Nullable child plus metadata-only CHECK, present parent/NULL child | Accepted; protocol remains writer version 2 | CHECK text in metadata without the feature is ineffective on this route |
| Nullable child plus native constraint builder, NULL parent | Accepted; protocol writer version 3, data commit version 2 | Parent-aware constraint is viable when actually enabled |
| Nullable child plus native constraint builder, present parent/NULL child | Refused; no data add action | The complete native constraint operation enforces the intended condition |
| Default two indexed top-level columns | Stats include nested `payload.required` and `id` | Nested leaves can be present in log statistics |
| Explicit `payload.required,id` statistics columns | Stats contain `id` only | Dotted explicit selection needs a qualified fix or a proven native alternative |

The condition tested was `payload IS NULL OR payload.required IS NOT NULL`. The `guarded_*` cases
deliberately use `CreateBuilder::with_raise_if_key_not_exists(false)` to permit raw constraint
properties; they illustrate the unsupported shortcut. `feature_guarded_*` calls
`DeltaTable::add_constraint().with_constraint(...).with_session_state(...)` before writing.
This is the required implementation path, subject to the full mutation matrix.

## Reproduction

The exact compile argv is in `compile-command.json`; it uses `rustc --edition=2024`, the existing
`target/debug/deps`, and explicit `--extern` rlib paths. It compiles only this probe and writes its
binary under `.dev-state/plan17/schema-probe/`. No Cargo manifest/lockfile was edited.

Execute the resulting binary with **a new empty service-development scratch root** as its sole
argument. The probe creates eight Delta tables and must not be pointed at an existing runtime,
user state or a repository under study. The recorded run argv names the root actually used.
To repeat after cached rlibs have changed, rebuild the affected locked dependencies using the
project's normal build path, identify the matching artifacts and record new compiler/library hashes;
do not present new output as this receipt or silently select a different dependency revision.

The first compile attempt used a Tokio artifact without the multithread runtime feature; the final
command selects the cached compatible artifact. An intermediate extension of the probe rejected
unknown raw constraint properties, leading to the explicit metadata-only and complete-builder
comparison above. Only the final source and successful complete execution are the attached receipt;
these development corrections changed no production configuration.

## Limits

No file-pruning benchmark, complete collection/nullability matrix, semantic analyzer implementation,
fixed-binary-ID plan, wire precision test, producer journey, MCP client, race, retention or power-loss
qualification ran here. The source review supports proposed APIs; the executed observations support
only their named fixtures. Plan 17's SC and Q oracles remain `not_run` for the target implementation.
