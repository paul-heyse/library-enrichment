# Retained assessment evidence

Historical planning checkpoint. Current implemented probes, independent evaluation and portable
release receipts are indexed in [implementation/README.md](implementation/README.md).

Created 2026-09-18. These are reproducible planning investigations, not a replacement for the active
skill's build system. Run from a directory containing the copied skill, adjusting the skill path
as needed. New source, commands, controls and results are retained here; compiler outputs are ignored.

| Artifact | Establishes | Does not establish |
|---|---|---|
| `source-manifest.json`, `sources/delta-rs/` | 98 selected files equal exact commit git objects; acquired artifacts match their recorded digests; source URLs/license retained | Complete source checkout or all cloud/runtime behavior |
| `acquisition.Cargo.lock` | Bytes match the original capture's lock SHA-256 | A moving branch can recreate that lock by fresh resolution |
| `rustdoc-fragments.json` | Full selected docs and associated outputs, with artifact-scoped IDs and original spans | IDs stable across public/private artifacts or future captures |
| `inventory.json` | Current model fields/counts and full selected foreign-impl rows | Suitability from counts; all items publicly importable |
| `baseline-files.json` | Hashes of active skill files outside this assessment/cache | A commit or reset of user work |
| `baseline-verification.json`, `logs/baseline-verification.log` | Existing checks pass with `--skip-rebuild` | Actual regeneration or copied-bundle portability |
| `context7.json` | Discovery response with provider/CDF leads | Exact commit or exhaustive coverage |
| `query-runs.json`, `queries/`, `query-fixture.rs` | Executed structural/text searches and same-name/default-properties controls | Receiver identity or semantic defect proof |
| `probes/`, `runtime-profile.json`, `runtime-receipt.json` | Locked executed local tests, exact feature closure, source hashes and compiler log | Concurrent exactly-once, cloud behavior, performance or crash durability |
| `initial-receipt.json`, `logs/initial-*` | Earlier four-test run before adding the provider-version-precedence assertion | The later assertion or subsequently formatted source bytes |
| `validation.json` | Final document/evidence checks and preservation result | Proposed implementation/release gates |

## Re-run the assessment extraction

Requires Python 3.14 (`compression.zstd`), git, and an exact existing checkout with the original
acquisition lock. It reads that checkout and writes selected evidence only below this directory.
It refuses different commits, source bytes or acquisition/lock digests.

```bash
uv run --no-project python .claude/skills/deltalake/skill_improvement/evidence/audit_reference.py \
  --source /path/to/delta-rs-at-58f07cd6
uv run --no-project python .claude/skills/deltalake/skill_improvement/evidence/run_queries.py
```

The retained query script includes the full structural patterns and rg expressions. The queried
hint is frozen in `queries/baseline-write-rule.yml` for replay after the active skill changes.
ast-grep matches are source candidates; queries intentionally do not claim name/type resolution. The
fixture is syntax-only and is not included in the compilable Rust crate.

## Re-run the Rust controls

Use stable Rust 1.98.1 and the committed probe Cargo.lock. The facade is a git dependency at the
exact delta-rs commit. The lock preserves its transitive kernel branch dependency at the exact
kernel commit; the runner asserts those identities and the single DataFusion/Arrow/object_store
versions before tests. There are no upstream source patches or path substitutions.

```bash
uv run --no-project python .claude/skills/deltalake/skill_improvement/evidence/run_probes.py
```

The runner uses Cargo `--locked --offline`, stores compiler targets under `.build-target`, and
records four assertion tests with `--show-output --test-threads=1`. Its first `--resolve --label initial`
run seeded resolution from the retained acquisition lock; ordinary replay does not regenerate it.
On a fresh machine, fetch the locked probe dependencies explicitly before using the offline runner:

```bash
cargo +1.98.1 fetch --locked \
  --manifest-path .claude/skills/deltalake/skill_improvement/evidence/probes/Cargo.toml
```

Cargo cache/registry/git data and the compiler are external prerequisites, not secretly included
in these planning files. The complete future research bundle and offline compile guarantee are
separate proposed deliverables. Historic logs may contain source-machine absolute paths as
provenance; scripts derive their paths from their location rather than importing the host project.

## Scope of the controls

The tests use new in-memory tables and a disposable temporary local directory. They demonstrate:
snapshot/provider freshness and snapshot-versus-version-option precedence; the callable inferred
LoadBuilder; duplicate sequential writes with the same transaction marker; visible publication
despite a post-commit error; and logical Delta deletion versus raw Parquet directory contents.
All counts describe these fixtures only. Negative import, concurrency, session-fallback, CDF,
protocol, retention and cloud controls are in the implementation backlog.
