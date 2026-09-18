# Implementation evidence — 2026-09-18

This folder retains the evidence behind the implemented reference. The planning investigations one
level up remain historical records. Current passing receipts apply only to their declared profile,
source bytes and assertions; earlier failed investigations are not silently discarded.

| Artifact | Current meaning |
|---|---|
| `baseline.json`, `baseline.tar.gz` | Original reader snapshot and exact file/archive hashes |
| `probe-results.json`, `runtime-profile.json`, `logs/runtime-*` | Current 24-test result, commands/toolchain, exact dependency features, source/fixture/lock/runner/profile hashes |
| `probes/` | Runnable standalone Cargo project, exact lock, fixture provenance/licenses and five test files |
| `compile-fail-results.json`, `compile-fail/` | Two deliberately invalid callers and the specific expected compiler errors |
| `additional-sources.json` | Supplemental immutable source evidence beyond the planning source manifest |
| `contracts-validation.json`, `unit-tests.log` | Full raw contract preservation, bounded reader, identity/access/replay controls |
| `legacy-validation.json`, `legacy-validation.log` | Existing deterministic build, pointer integrity, ast-grep rules, navigation and transferability |
| `validation.json`, `links.json`, `ruff.log`, `clippy.log` | Dated final focused checks and their logs/identities |
| `evaluation/REPORT.md`, `evaluation/judgment.json` | Independent original 38-response comparisons, eight caller tests per arm and stated evaluation limits |
| `evaluation/candidate/` | Original attempts and separately labeled judge-prompted E05B attempt-06 control |
| `evaluation/revision/` | New reader identity, two affected-case responses/review; no replacement of frozen scores |
| `evaluation/revision/frozen-evidence/`, `evaluation/judge-sources/` | Permanent copies of cited frozen-reader evidence and exact-commit catalog sources; no extracted-directory dependency |
| `qualification.json`, `transfer-qualification.json` | Final distribution identities and copied-reader/clean-rebuild results; external to their archives |
| `*-syscalls.log`, `research-rebuild.log` | File/network tracing and clean research regeneration output |
| `bundles/` | Final reader/research plus frozen candidate archives; excluded from recursive packaging |

`probes-initial` through `probes-sixth`, `boundaries-initial/second/third`, `competing-initial`,
`partition-*-diagnostic`, `writer-roll-initial` and evaluator attempt logs are investigation history.
The early writer observation failed because upload is asynchronous; the retained successful test
waits with a bounded timeout and asserts both staged objects and unchanged Delta visibility.
Neither a failed attempt nor an earlier successful source hash is the current runtime receipt.

From the copied skill root (use `uv run --no-project python` instead of `python3` when required):

```sh
python3 scripts/run_probes.py
python3 scripts/check_access.py
python3 build/test_contracts.py
python3 -m unittest discover -s build -p 'test_*.py'
python3 build/verify.py
python3 scripts/invalidation.py
python3 skill_improvement/evidence/implementation/verify_links.py
python3 scripts/qualify.py
```

Runtime replay needs Rust 1.98.1 and fetched locked dependencies. Regeneration needs the research
bundle, Python 3.14 or zstd decompression support, and ast-grep. Qualification also needs strace.
These are declared external tools, not hidden host-project dependencies. Copied-reader use needs
only standard-library Python and the reader files.

`record_validation.py` verifies and aggregates retained executed logs; it does not rerun tests.
Its `validation.json` keeps command, artifact hash and scope together. Re-run the named commands
to establish new execution evidence before collecting a new aggregate.

No cloud credential/service operation or workload benchmark was run. Source guidance for those
areas is explicitly distinct from the local runtime assertions. Publication visibility tests do not
prove crash durability. Initial planning receipts and frozen evaluator identities stay unchanged
when implementation probes or final release content are revised.
