# Maintain the pinned Delta reference

## Identity and authority

The git manifest, public/private rustdoc captures and retained acquisition Cargo.lock identify
this API. `build/inputs.lock.json` binds all captured input bytes. The protocol document was
originally acquired from `master` without a commit identity; its retained archive SHA-256 is the
honest snapshot identity. It is normative context, not operation-support proof at the delta-rs pin.
The kernel package names, Cargo aliases and exact fork revision all matter. Documentation features
and probe features are distinct in `content/profile.json`; captured model version labels are not
claims about each package's published version.

## Replay without resolving a branch

```sh
python3 build/build.py
python3 build/test_contracts.py
python3 -m unittest discover -s build -p 'test_*.py'
python3 build/verify.py
python3 scripts/invalidation.py
```

Build is offline and verifies captured input hashes before generation. Missing cache entries fail
instead of fetching. Research bundles contain raw public/private artifacts, manifests, lock bytes,
source corpus inputs, authoring, queries and retained probes. They support regenerating the
reference; they do not imply offline compilation of the whole Cargo dependency graph.

`build/acquire.py` is a separate producer. Its default replays retained Cargo.lock bytes, verifies
the acquisition lock digest and each declared git package revision, then fetches that locked graph
and documents with the dated producer nightly. It may need network/cache/toolchain availability;
it never runs as part of a reference rebuild. `--check` reports branch drift without changing pins.

## Deliberate refresh

Use a new manifest/capture identity for a changed delta-rs revision, fork, feature profile or lock
resolution. `build/acquire.py --manifest ... --refresh` explicitly generates a new lock and checks
its declared git dependencies. It refuses to overwrite an existing acquisition with a different
lock digest. Retain the old envelope and full lock, update input hashes deliberately after reviewing
new captures/corpora, and rebuild. Source files and fixtures must carry immutable URLs and hashes.
Do not replace the exact capture with an older Context7/published-crate API example.

## Semantic maintenance

Author reviewed records in `authoring/capabilities/`; generated `content/` is never hand-maintained.
The builder checks operation references, evidence files, unique claim IDs and runtime test names.
Run the retained standalone probe project with `scripts/run_probes.py` after changing its source,
fixtures or lock. Access controls run through `scripts/check_access.py`; retain expected compiler
errors, not merely a nonzero exit. Keep initial failed probes as investigation history, distinct
from the current successful receipt.

`scripts/invalidation.py` compares selected operation docs/types, source hashes, capture/runtime
profiles and neighboring crate candidate surfaces. A new alternative can reopen a decision even
when its previously selected API is unchanged. Treat its output as review hints: unlinked semantic
changes can still matter. Re-evaluate conditions and alternatives, not just signatures.

## Portable distribution

The reader contains decisions, full contracts, legacy indexes, query rules, minimal DataFusion/Arrow
seam contracts and referenced evidence. Research adds rebuild inputs and implementation/evaluation
history. Both exclude compiler targets and extracted qualification trees; archives have fixed tar/gzip
metadata and digest manifests. `scripts/qualify.py` checks repeated byte-identical packaging, runs
copied lookups outside the source tree with isolated Python, and rebuilds the research copy offline.
Recorded file/network tracing distinguishes tested independence from assumed portability.
Research qualification removes generated content before rebuilding and compares every regenerated
byte. Distribution hash/trace receipts are adjacent to the archives, excluded from the archives
because they describe the completed packaging run. Compiler tools and the Cargo graph remain
external prerequisites; authoring inputs and the minimal integration contracts are self-contained.

## Evaluation and release

Freeze a candidate identity before evaluation. Use the same 32 paired and six held-out requests
for independent baseline and candidate readers, then independent judgment. Compile/run the eight
caller compositions per arm; retain initial failures, corrections and final outcomes. A correction
after the freeze is a new candidate, not retroactive evidence for the old one. Report correctness,
uncertainty and retrieval cost separately; uninstrumented token/time metrics stay unknown.

Use passed/failed/blocked/not_run for checks. Local memory/filesystem and fixture assertions do not
qualify cloud services, crash durability, arbitrary concurrency or workload RSS/performance. Report
these limits in the implementation report and in affected capability briefs.
