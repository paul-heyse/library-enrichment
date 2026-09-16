# The builder

Regenerates `content/` and the generated rules from pinned upstream sources.

```bash
python3 acquire.py                    # once; needs network, git, cargo, the pinned nightly
python3 acquire.py --check            # has a tracked branch moved off its pin?
python3 build.py --manifest manifests/deltalake.json
python3 build.py --stage model        # indexes only, skip pages and corpora
python3 verify.py                     # rebuild, then five checks
python3 verify.py --skip-rebuild      # check recorded digests only
```

## Dependencies

`build.py` and `verify.py`: Python 3.11+ standard library, plus the `ast-grep` binary for the
structural stages. Nothing else — no pip install, no PyYAML, and no import from any host
repository. `verify.py` enforces that last property.

`acquire.py` additionally needs `git`, `cargo`, and the dated nightly the manifest names. It is
a **separate entry point on purpose**: `verify.py` re-runs `build.py` to prove determinism, so
acquisition must never be reachable from there. If anyone wires it in, the determinism check
will start running cargo and the mistake will be loud.

Zstd comes from `compression.zstd` on Python 3.14+, falling back to the `zstd` CLI.

## Why there is an acquisition stage at all

The DataFusion skill downloads rustdoc JSON from docs.rs, keyed by a published version. That
does not work here. The newest published `deltalake` is 0.32.4 (2026-06-07) and pins arrow 58 /
datafusion 53; the arrow-59 / datafusion-55 combination exists only on unreleased `main`. So
the bytes have to be produced locally, and `acquire.py` does it in five phases, each failing
loudly rather than degrading:

0. **Assert the toolchain** is exactly the one the manifest names, by rustc release *and* build
   hash, before anything expensive runs.
1. **Check out** each repository at its pinned commit, into a capsule outside every working
   repository. A fresh blobless clone, not cargo's `~/.cargo/git/checkouts` — cargo owns and
   garbage-collects that tree.
2. **Resolve a lockfile, then assert it.** delta-rs does not commit `Cargo.lock` and takes the
   kernel from a *branch*, so `--frozen` cannot work on the first pass and a branch is not a
   pin. `generate-lockfile` resolves it; the manifest's `lock_deps` is then checked against the
   result, and a moved branch is a hard stop.
3. **Document every crate in ONE `cargo doc` invocation.** Under resolver 3, a per-crate loop
   resolves features N times and emits N mutually inconsistent documents of the same graph.
   A second capture follows with `--document-private-items` (see below).
4. **Collect**, validating `format_version` per file *before* compressing, so a drift to an
   unsupported version fails in seconds rather than after the whole build.

Four guards keep the pinned toolchain in force, because both repositories ship a
`rust-toolchain.toml` naming a stable channel that cannot emit rustdoc JSON at all: cargo runs
from the capsule rather than inside the checkout (rustup reads the file from the CWD upward),
`RUSTUP_TOOLCHAIN` is set, `+<toolchain>` is passed explicitly, and phase 0 asserts the result.
The checkout is never modified — editing it would break the SHA-to-content identity that makes
the pin mean anything.

## `build/acquired/` is tracked, and that is the structural difference

`.cache/` is gitignored because docs.rs, crates.io and GitHub can always re-serve it.
`acquired/` **cannot be re-served by anyone**: it exists only because someone ran cargo against
a specific commit with a specific toolchain. So it is committed. Without that, "copy it
anywhere and rebuild" would be false for this skill — it would mean "copy it anywhere, install
a nightly toolchain, and rebuild for half an hour".

Its directory name is `<repo>@<rev8>-<envelope12>`, where the envelope hash covers the
revision, the pinned git dependencies, the toolchain, the target and the feature set. Every
input to that hash is declared in the manifest, so `build.py` recomputes it without running
anything: a manifest edit produces a different directory, and a miss is the staleness signal.
Each file's sha256 is recorded in `ACQUISITION.json` and checked on read.

## The second capture

rustdoc emits a type declared in a **private** module without any of its impls. The type is
real — a public method returns it, you can call and await it — but it arrives here with zero
methods and no traits, indistinguishable from a type that genuinely has none. delta-rs hits
this on `LoadBuilder`, which `DeltaTable::scan_table` returns: 34 impls including `IntoFuture`,
none of them in the public document.

So a second `cargo doc --document-private-items` runs into its own target directory, and is
used for exactly one thing: filling in impls for items the **public** document already
admitted. It never adds items — its index is twice the size, and letting it decide membership
would publish private types as API. Methods that are not `pub` are filtered out, and the merge
is by canonical path because ids are per-document.

`content/index/unnameable.tsv` then records what remains true: these types exist and have
methods, and you still cannot name them.

## Stages

| Module | Does |
|---|---|
| `acquire.py` | Git checkout, lockfile assertion, local rustdoc JSON. The only stage needing a toolchain. |
| `fetch.py` | Reads acquired documents, or docs.rs for a `docs_rs` crate set; GitHub tarballs for corpora. |
| `model.py` | Walks each crate into a canonical item table, then stitches re-exports across all of them. |
| `render.py` | rustdoc `Type` trees back into Rust signature text. |
| `link.py` | Runs ast-grep over the corpus to derive symbol-to-usage edges structurally. |
| `emit.py` | Writes `model/`, `api/`, and `PROVENANCE.json`. |
| `traits.py` | Writes one page per extension point. |
| `topics.py` | Writes one page per capability axis, from the seeds in `topics.json`. |
| `catalogs.py` | Operations, table properties, table features, errors, foreign impls, crate map. |
| `queries.py` | Generates rules from the model, plus their tests. |
| `build.py` | Orchestrates, and writes `index/*.tsv`. |
| `verify.py` | Determinism, integrity, rule tests, navigation probes, transferability. |

## Things that are easy to get wrong here

**Enumerate items from the index, not from the module walk.** A crate that defines a type in a
private module and re-exports it never surfaces that type as a child of a public module.
`model.py` enumerates from `index` filtered to `crate_id == 0`, and uses the walk only to learn
access paths.

**Being reachable by the walk is not being nameable by a caller.** rustdoc lists a private
module among its parent's items, so the walk descends into it. Minting an import path through
one produces a path that does not compile — 107 of them, before `public_modules` was tracked
from rustdoc's own `visibility` field. A glob rewrite is therefore applied to access paths,
which are known valid, and to canonical paths only when every module in them is public.

**`pub use private_mod::*` has no resolvable target.** rustdoc records no canonical path for a
private module, so the glob cannot be followed by path — but the module is in `index`, because
its contents are re-exported. Walking it under the *accessing* path is what the glob means.
Skipping it silently drops whatever it holds, and here that is `kernel::schema::schema`,
`kernel::models::actions` and `kernel::snapshot`: the schema model, the log actions and the
snapshot API.

**A crate rename is invisible to rustdoc.** The kernel's `[lib] name` is `buoyant_kernel`;
delta-rs takes it as `delta_kernel`, and that is the spelling in its source, its examples and
its documentation. `crate_aliases` in the manifest emits the second spelling, without which the
name every reader will type resolves to nothing.

**A topic seed that does not resolve must fail the build.** `topics.py` raises rather than
skipping, because a page that quietly drops a type it can no longer find asserts by omission
that the capability does not exist — the exact failure this repository exists to prevent.
Leaf names are ambiguous (`UpdateBuilder` and `DeleteBuilder` each exist twice; `Snapshot`,
`Scan` and `Transaction` exist in both the kernel and delta-rs), so a seed is either unique or
qualified, and ambiguity is an error too.

## Pointing it at something else

`manifests/deltalake.json` names the crate sets, revisions, feature envelopes and corpora. A
different library needs a different manifest, not a different builder. A crate set with no
`source` field is fetched from docs.rs exactly as before, so the same builder still builds a
published crate family; `source: "git"` selects the acquisition path described above.
