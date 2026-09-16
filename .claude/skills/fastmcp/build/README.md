# Building and re-pinning

Two stages, deliberately separated.

```bash
python3 acquire.py            # network + uv + griffe + ty. Run once per pin.
python3 build.py              # offline: standard library + ast-grep only.
python3 verify.py             # seven checks, including a rebuild.
python3 acquire.py --check    # has PyPI moved? Reports; never re-pins.
```

`acquire.py` is unreachable from `build.py` on purpose. `verify.py` re-runs `build.py` to prove
the output is byte-for-byte reproducible, and if acquisition were reachable that check would
start installing packages.

## What acquisition does

0. **Assert tools.** Griffe outside the manifest's accepted set aborts immediately: its
   canonical-path inference changes between versions, so that is a decision, not a drift.
1. **Build a capsule.** A virtualenv under `${PY_SKILL_ACQUIRE_CAPSULE:-${XDG_CACHE_HOME:-~/.cache}/python-skill-acquire}`,
   never the calling repository's own. The index must describe a pinned environment, not one
   developer's venv, and no repository under study is ever an install target.
2. **Assert the resolved graph** against `manifests/resolved.json`. This is the step it is
   tempting to skip. `fastmcp==4.0.3` is a metapackage; the `fastmcp-slim` that owns the code
   depends on `mcp` by *range*, so the declared pins are not a pin. A mismatch aborts naming
   both sides.
3. **Extract** each distribution with Griffe into our own normalized schema, then supplement
   with `ty` hover for the definition sites where the source states no type.
4. **Collect**, digesting every file. If any expected document is missing, `ACQUISITION.json`
   is not written at all, so `build.py` sees a clean cache miss rather than an index that is
   quietly one distribution short.

## Why `acquired/` is committed and `.cache/` is not

`acquired/` holds bytes that exist only because someone ran Griffe and a language server
against a specific capsule. Nobody can re-serve them, so they ship. `.cache/` holds upstream
tarballs, which GitHub can always re-serve.

## Re-pinning

1. `python3 acquire.py --check` to see what moved.
2. Edit `manifests/fastmcp.json` — the `install` line and the version table.
3. Regenerate `manifests/resolved.json` from a fresh resolve.
4. `rm -rf acquired && python3 acquire.py && python3 build.py && python3 verify.py`.
5. Expect probes to fail. That is the point: several are deliberately brittle, and a probe that
   keeps passing while its subject changes is certifying a route that no longer leads anywhere.
   Read each failure before editing it.

Re-check `catalogs/registration.md` by hand on any major bump. It is the one page carrying a
claim no static analysis produced — what the decorators return at runtime — and `verify.py`'s
tripwire only catches the specific way it is known to go stale.

## The corpus

Five corpora are declared in `manifests/fastmcp.json` and fetched by `acquire.py` phase 3c, not
by `build.py`. The sibling skills fetch inside the build, which means their "offline by
construction" build reaches for a network on a cold cache. Keeping it in acquisition is what
makes that claim literally true here.

`fetch.repo_files` **raises on an empty result** rather than returning `{}`, and `check_corpus`
fails the build if any declared corpus is missing, empty, or does not hash to what acquisition
recorded. Both exist for the same reason: a reader cannot tell a corpus that was never fetched
from a corpus with no matches, and the second answers "this capability does not exist".

Note the tag spellings differ by project. `jlowin/fastmcp` tags carry a leading `v` (`v4.0.3`);
the analysis toolchain's do not. A wrong ref is a 404, not an empty set.

The two MCP specification corpora are pinned to a **commit**, not to `main`. They were declared
against the branch, which is not a pin: the determinism check would have broken the first time
upstream pushed.

## Re-pinning

`envelope_key` covers the Python version, the install line, the resolved graph, the distributions
and the Griffe versions. It deliberately does **not** cover the corpora, so amending a corpus
does not force a capsule rebuild. The corpus digests in `ACQUISITION.json` and `PROVENANCE.json`
are what make such a change visible instead.
