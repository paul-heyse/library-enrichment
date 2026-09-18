# Blind baseline evaluation — 2026-09-18

Completed 38 decision responses; all eight required consumer runtime tests pass.
This is an evaluation of the frozen baseline reference, not project acceptance certification.

- `responses.json`: candidates, selected conditions, input/output/effect contracts, sources and unknowns for each task.
- `source-map.json`: per-task source paths/digests and consumer test locations.
- `materials.json`: consulted material and supplied-input digests; Cargo-cache source use is explicit.
- `consumer/tests/decisions.rs`: eight executable runtime tests.
- `commands.jsonl`: exact Cargo invocations, durations, exit codes, source/log digests; capability-gap scan invocation.
- `cargo-test-01.log`: initial compile failure (private trait and redundant provider Arc).
- `cargo-test-02.log`: six passes and two runtime failures (Delta planner requirement and CDF metadata representation).
- `cargo-test-03.log`: final eight passes.
- `metrics.json`: truthful counters, failure history and measured timing interval. Total elapsed and context tokens were not instrumented.
- `capability-gap.log`: 36 help-level syntax suggestions retained for review, not treated as acceptance failures.

The final tests prove refreshed-provider behavior, incremental projection, caller-plan execution with retained UDF/store environment, predicate overwrite validation, serialized replay guarding and marker-only duplicate behavior, CDF row changes, restore history, and explicit stale-store replacement. They do not prove concurrent exactly-once delivery, cloud integration, every table feature, every schema conversion, or vacuum safety.

Reproduce from `consumer/`:

```sh
cargo +1.98.1 test --locked --offline --target-dir /home/paul/library-enrichment/.claude/skills/deltalake/skill_improvement/evidence/.build-target -j 4 -- --show-output --test-threads=1
```

All Delta data used by runtime tests was in memory. Only the assigned output directory and the explicitly authorized shared build target were written. The active skill, planning/assessment/design evidence, sibling skills, other evaluation responses, and expected answers were not consulted. No network or live cloud operation was used.
