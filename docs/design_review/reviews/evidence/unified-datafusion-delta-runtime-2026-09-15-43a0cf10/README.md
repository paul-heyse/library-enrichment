# Historical August-revision probe evidence

Verified 2026-09-15. These are isolated native mechanism probes supporting the
[design review](../../design_review_unified-datafusion-delta-runtime_2026-09-15.md).
They do not implement or qualify the proposed service architecture.

This is the earlier probe against the August 23 Delta revision, preserved after the review
was updated to current upstream HEAD. Use the neighboring current evidence directory for
current-revision conclusions. The reproduction manifest path below was adjusted for archival.

## Exact inputs

- DataFusion 55.1.0, Arrow and Parquet 59.3.0, object_store 0.13.2.
- delta-rs revision `43a0cf10a313e5077c48637ad786a05359136bbb` (package version 1.0.0).
- Rust/Cargo 1.98.1; the lockfile and receipt record the resolved dependency universe.
- Service source digest: `5b2640ca15e4e1e6bb3355eda84edb89d4bcdab3e390897513648c0581d4026a`.

`probe-receipt.json` records commands, outcomes, artifact hashes, setup failures and
limitations. `context7-research.json` retains discovery results; exact source and native
execution, rather than Context7 version labels, support consequential API claims.

## Reproduce

Run from the repository root. Each execution needs a **new, absolute scratch directory**
outside any repository under study. The supplied commands use the development state area
and a unique directory. Building uses the existing target cache without modifying the
service manifest or lockfile. Remove `--offline` only when fetching the locked dependencies
is necessary.

```bash
CARGO_TARGET_DIR="$PWD/target" cargo build --locked --offline \
  --manifest-path docs/design_review/reviews/evidence/unified-datafusion-delta-runtime-2026-09-15-43a0cf10/Cargo.toml -j 8
probe_state_dir=$(mktemp -d "$PWD/.dev-state/unified-datafusion-review/reproduction.XXXXXX")
target/debug/unified-datafusion-review "$probe_state_dir"
```

The program asserts the observations, including the intentionally demonstrated validation
and replay gaps. Exit zero means those bounded observations were reproduced; it does not
mean the proposed application protocol passed acceptance.

## Results and limits

| Mechanism | Observed result | Limit |
|---|---|---|
| Recursive CTE / arrays / UNNEST | Three-node cycle terminates; nested filter yields two rows | No full normalizer equivalence or scale qualification |
| Volatile scalar | Dead branch invokes zero times; repeated frame execution invokes twice | No real producer effects in this probe |
| MemTable key metadata | Duplicate SQL insertion accepted despite advertised PK | Metadata is not an admission mechanism |
| Delta provider registration | Pinned provider retains one row; newly loaded provider sees two | No multi-table publication or retention race tested |
| Delta CHECK write routes | WriteBuilder rejects negative ID; direct provider SQL INSERT accepts it | Applies to the exact checked revision and routes |
| Delta transaction action | Same-key stale writers yield one success and one conflict; fresh duplicate sequence is accepted | Full claims, fences, context-head CAS and recovery remain proposed |
| Delta planner | Plain state fails to plan MetricObserver; composed Delta planner permits the checked writes | Service retention/planner composition is still required |

`probe-setup-failure.log` preserves the failed plain-session run. That run's initial
any-error assertion was insufficient to prove CHECK enforcement. The final source installs
DeltaPlanner and checks a validation-specific diagnostic; only `probe-run.log` supplies the
successful final evidence. The build warning for `proc-macro-error2` concerns the scratch
dependency graph, not a production dependency change.

`capability-gap.log` is the DataFusion skill's heuristic source scan. In particular, its
lease-wrapper statistics warning is a false positive after inspecting the actual forwarding
methods. It is not a product acceptance report.

Not run: full service or installed MCP acceptance, external producer execution, full
Arrow/Delta schema round trips, control/publication recovery, active-reader vacuum races,
power-loss durability, production-scale recursion and performance measurements.
