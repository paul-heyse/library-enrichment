# Maintain and transfer the reference

The reader needs Python 3.11+ standard library. `rg` and `ast-grep` are optional for direct and
structural search. Research builds use Python 3.14 zstd (or the `zstd` CLI), `ast-grep`, cached exact
upstream inputs and the pinned Cargo probe workspace. No host application imports or services are
required. Use the host's preferred Python launcher, for example `uv run python` where uv is required.

## Authoring and evidence

Edit `authoring/capabilities/*.json`, `authoring/crate-roles.tsv`, `build/topics.json`, or builder code;
regenerate `content/`. Never hand-edit generated contracts. Capability records bind claims to
evidence classes and exact operation paths. Conditional recommendations are authored judgments;
runtime observations are limited to named tests in a successful receipt. New observations need
source, lockfile/profile, controls and a command log, not a claim of success inferred from compilation.

```bash
python3 scripts/run_probes.py
python3 build/build.py
python3 build/test_contracts.py
python3 build/verify.py
```

`run_probes.py` uses `cargo +1.98.1 --locked --offline` in the isolated workspace at
`skill_improvement/evidence/implementation/probes`. On a clean research installation, first use
`cargo +1.98.1 fetch --locked --manifest-path <skill>/skill_improvement/evidence/implementation/probes/Cargo.toml`
to acquire that dependency closure. The research bundle includes upstream reference inputs, not
the workstation's Cargo registry or compiler binaries. The profile records resolved features,
versions, target/toolchain and relevant compiler flags. Build products are disposable and excluded
from bundles. No root workspace manifest is involved.

`test_contracts.py` compares every preserved record's docs, type tree, links and span to its exact
cached rustdoc item. `--reader-only` checks aliases, task routes, contract links and lossless bounded
continuation without raw inputs. `verify.py` separately checks deterministic rebuilds, old index
integrity, structural fixtures and navigation. `--skip-rebuild` checks recorded hashes only.

For a source question not answered by the retained excerpts:

```bash
python3 scripts/source_evidence.py --package datafusion-functions --version 55.1.0 \
  --file src/core/coalesce.rs
```

The adapter reads the exact `.crate` cache, writes selected files, Cargo/VCS metadata and available
license/notices under `skill_improvement/evidence/implementation/source-exports`, and records archive
and file digests. It never substitutes a current-branch file. Pass `--cache` for another archive
directory. The source span in an operation record identifies what to inspect. Structural queries
and controls remain in `queries/` and `skill_improvement/evidence/queries/`; their matches are
syntax evidence, not semantic identity proof.

## Review changes at the right scope

Run `scripts/invalidation.py` after changing upstream inputs or authoring. The generated dependency
map binds each capability to its authoring, complete operation records and runtime receipt.
Crate-surface changes trigger candidate-set review even if an old selected method is unchanged.
Regenerate, rerun affected probes, and review alternative suitability before accepting a new pin.
Unknown features or newly added crates require explicit discovery review; lack of a prior edge is
not proof that a capability cannot be affected.

The runtime registry is one observed SessionContext profile. Refresh it when features, defaults,
registrations or versions change. Config builder associations are heuristic name matches; resolve
the method body/typed configuration field for a consequential setting. Keep the consumer Cargo.lock,
hosted rustdoc profile and executable probe profile distinct.

## Portable bundles

```bash
python3 scripts/package.py reader --output skill_improvement/evidence/implementation/bundles/reader.tar.gz
python3 scripts/package.py research --output skill_improvement/evidence/implementation/bundles/research.tar.gz
```

Reader includes generated routes/contracts, scripts, queries and retained supporting evidence.
Research additionally includes exact cached rustdoc/crate/repository inputs, assessment artifacts,
evaluation responses/judgment, and compressed baseline/candidate pilot snapshots.
Neither includes compiler targets, extracted evaluation workspaces or host files.
Each tarball has a SHA-256 manifest; tar metadata and compression headers are deterministic.

Extract to any directory, change to an unrelated working directory, and run:

```bash
python3 /path/to/copied-skill/scripts/verify_bundle.py
```

This verifies file integrity and actual offline lookups. A raw directory copy remains usable without
a bundle manifest; the manifest check applies to packaged distributions. Exact-source regeneration
and decision quality are separate from portability and file integrity.

On Linux, `scripts/qualify_transfer.py` additionally uses `strace` to verify the reader from `/`
with a minimal environment. It records Internet socket use, original-project file access, and
positive invalidation controls. Python/system-library access and local system name-cache attempts
are distinguished from application or Internet dependencies.
