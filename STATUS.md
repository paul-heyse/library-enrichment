# Implementation handoff

Updated 2026-09-14. **[Plan 12](docs/plans/12-architecture-first-completion.md) is functionally
complete and validated.** The user explicitly stopped the redundant independent replay after a
test-registry mapping correction. No further test campaign is pending. Performance tuning remains
deferred. See the [execution ledger](docs/plans/12-architecture-first-execution-ledger.md) and
[accepted scoped review](docs/design_review/reviews/design_review_plan12-integrated-completion_2026-09-14.md).
Do not restart Plans 09–11 or treat their historical gaps as current.

## Completed scope

Rust owns typed Arrow/Parquet evidence, relational catalogs, bounded DataFusion assembly/query,
identities, jobs, policy and publication. The mandatory native worker handles raw Rustdoc and
Parquet admission. Python remains the thin MCP adapter and separate static worker. Legacy
readers/writers, whole-corpus normalizers, parallel query paths and old installer/client harness
logic are removed. ADR-0028–ADR-0035 are accepted with scoped reviews.

The development cutover removed 340 owned payload roots/files across 36 physical roots, with
2,818 entries and 43 reconciled ownership records; five confirmed exited containers were removed.
No historical reader or migration remains. Future unchanged exact-version and qualified-context
evidence remains reusable without age expiry; registry revalidation and explicit cleanup are separate.

Functional scope includes durable comparison, semantic/runtime inspection and qualified reuse,
stable/nightly comparison, six Rust/Python canary cases, precommitted delivery and crash recovery,
export closure, generational installer recovery, two-adapter survivors and bounded diagnostics.
Exact definition selection resolves same-path ambiguity; overview retains available documentation.

## Executed validation — 2026-09-14

- Final functional source: `4b434e3f7fd604068cf5e59205d7a0a54de09b442ce1a0d3267913babfd0db0a`.
  `CARGO_INCREMENTAL=0 LIBENR_EXECUTION_ROOT=/home/paul/library-enrichment/.dev-state/p4p just ci`
  passed **390 Rust and 189 Python tests**, Clippy, Ruff, ty, formatting, schemas, dependency policy,
  rules, provenance and state checks. Log: `.dev-state/plan12-ci-final.log`.
- `CARGO_INCREMENTAL=0 LIBENR_EXECUTION_ROOT=/home/paul/library-enrichment/.dev-state/p4p just test-execution`
  passed **12 real contained cases**; `.dev-state/plan12-execution-final.log`.
- `CARGO_INCREMENTAL=0 just test-live` passed **2 live cases** on final functional source;
  `.dev-state/plan12-live-final-source.log`.
- Refreshed authenticated `just test-client` passed **11 checks covering all 10 actual client
  scenarios**; `.dev-state/plan12-client-current.log`. All outputs were reviewed. This run preceded
  the final overview correction, subsequently covered by its deterministic native regression,
  strengthened real Python/MCP journey and full CI. Real user configuration remained unchanged.

Only P03/C17 registry references changed afterward; IDs and assertions did not. The final source
fingerprint is `6f5a3646a3fbb6ac5c9faca18a579b8364ac4cd506d317b3811e4a547f2886c8`.
The independent auditor found no functional blocker and replayed **390 Rust tests successfully**.
The user then stopped redundant validation during the Python tier. That interrupted tier is not a
pass or a source failure; execution/live/client replays were not started.

The regenerated source-bound report records **7 passed / 0 failed / 0 blocked / 41 not_run** out
of 48 IDs, including retired P08. Earlier receipts remain preserved and were not rewritten after
the registry changed. This is not a full current-registry certification; the remaining `not_run`
entries do not overturn the completed functional runs above. No repeat campaign is requested.
Audit and cleanup evidence: `.dev-state/plan12-independent-audit/AUDIT.md`, `stop.json`,
`cleanup.json`, `ci.log` and `prior-logs/`. All nine recorded processes exited; the selected
execution root has zero containers and zero ownership records. No cleanup obligation remains.

## Remaining work

No functional Plan 12 work remains. Performance optimization and threshold enforcement are
explicitly deferred: the existing five-run optimized workload passed output checks with median
2.20897 seconds against the older 1.5-second target. No target was changed or performance pass
claimed. Do not restart tuning or certification without new work or user direction.

## Environment and ownership

All work remains uncommitted over `cd6d9490a6433ae355b04a465538062e1320544c`, including substantial
inherited changes. Do not reset, clean, stash or overwrite the shared tree.

Verified 2026-09-14: Rust 1.98.1, Python 3.14.7, uv 0.12.13, ty 0.0.80, Griffe 2.3.0,
rust-analyzer 1.98.1; DataFusion 55.1.0, Arrow/Parquet 59.3.0, object_store 0.13.2.
Clients: Codex 0.154.0, Claude 2.1.270. All hard doctor prerequisites are present.
The workstation default is nightly; retain `rust-toolchain.toml` and the producer's dated
`nightly-2026-09-13`. Use `CARGO_INCREMENTAL=0` after the earlier incremental compiler ICE.

Build/deploy all three binaries: `library-enrichmentd`, `library-enrichment-executor`,
`library-enrichment-native-worker`. Selected execution root is `.dev-state/p4p`; its receipt
binds current helper/broker/configuration and the pinned Rust/Python images. The systemd user-bus
repair is complete; see the [environment report](docs/reports/user-session-environment-2026-09-14.md).
Do not repeat the repair or alter unrelated Podman state. The
[operations guide](docs/operations/README.md) documents setup, qualification, retention,
export, explicit cleanup and recovery.
