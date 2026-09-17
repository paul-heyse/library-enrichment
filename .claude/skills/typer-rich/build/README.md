# Building and re-pinning

Three stages, deliberately separated.

```bash
python3 acquire.py            # network + uv + griffe + ty + pyrefly + the probe run
python3 build.py              # offline: standard library and the ast-grep binary only
python3 verify.py             # 15 checks, including a rebuild
python3 acquire.py --check    # has PyPI moved? Reports; never re-pins.
```

`acquire.py` is unreachable from `build.py` on purpose. `verify.py` re-runs `build.py` to prove
the output is byte-for-byte reproducible, and if acquisition were reachable that check would
start installing packages and executing subjects.

## What acquisition does

0. **Assert tools.** Griffe, griffelib and **ast-grep** outside the manifest's accepted set all
   abort. ast-grep is a hard gate here, unlike in the sibling repositories where a mismatch only
   warns: `link.py` reads the corpus with it and `check_rule_tests` runs against it, so a
   silently different parser changes what the index contains without changing anything that
   would fail.
1. **Build a capsule** under `${PY_SKILL_ACQUIRE_CAPSULE:-${XDG_CACHE_HOME:-~/.cache}/python-skill-acquire}/typer-rich`,
   never the calling repository's own venv. No repository under study is ever an install target.
2. **Assert the resolved graph** against `manifests/resolved.json`. The step it is tempting to
   skip, and the reason it is not: three of the seven distributions arrive through ranges, and
   `Syntax` output depends on which Pygments resolves.
3. **Extract** each distribution with Griffe into our own schema, then supplement with `ty` hover
   where the source states no type, then run the pyrefly batch reports.
   3c. **Vendor the corpora** from pinned GitHub tarballs.
   3e. **Run the probes** against the capsule and write `PROBES.json`.
4. **Collect**, digesting every file. If any expected document is missing, `ACQUISITION.json` is
   not written at all, so `build.py` sees a clean cache miss rather than an index quietly one
   distribution short.

## Why probes live in acquisition

For the same reason the corpus does. `build.py` must stay on the standard library plus
`ast-grep`, because `verify.py` re-runs it and demands identical bytes; probes spawn the capsule
interpreter, which is emphatically not that. The build **replays** `PROBES.json` and executes
nothing.

A probe writes a `HOME` of its own so `typer.get_app_dir()` is deterministic. That scratch tree
lives under the capsule, never under a working tree.

## Why `acquired/` is committed and `.cache/` is not

`acquired/` holds bytes that exist only because someone ran Griffe, a language server and the
subjects themselves against a specific capsule. Nobody can re-serve them, so they ship.
`.cache/` holds upstream tarballs, which GitHub can always re-serve.

## Re-pinning

1. `python3 acquire.py --check` to see what moved.
2. Edit `manifests/typer-rich.json` — the `install` line, and the version and date columns in
   `catalogs.json` if a new breaking change landed.
3. Regenerate `manifests/resolved.json` from a fresh resolve.
4. `rm -rf acquired && python3 acquire.py && python3 build.py && python3 verify.py`.
5. **Expect probes and tripwires to fail.** That is the point. Several are deliberately brittle,
   and a probe that keeps passing while its subject changes is certifying behaviour that no
   longer happens. Read each failure before editing it.

`envelope_key` covers the Python version, the install line, the resolved graph, the
distributions and the Griffe versions. It deliberately does **not** cover the corpora, so
amending a corpus does not force a capsule rebuild; the corpus digests make that change visible
instead.

A Rich-only bump changes the key and invalidates Typer's captures. That is correct rather than
annoying: bumping Rich changes what Typer's `--help` renders.

## The corpus

Eight corpora, declared in the manifest and fetched by acquisition. Each declares
`expected_files`, asserted after the fetch — `fetch.repo_files` already raises on an empty
result, but it cannot catch a corpus that silently halved when an exclude pattern grew.

Tag spellings differ by project and a wrong ref is a 404, not an empty set: `fastapi/typer` tags
are bare (`0.27.2`), `Textualize/rich` tags carry a leading `v` (`v15.0.0`).

## Things that were measured and are easy to undo

- **Neither library uses `__all__`.** Rich declares it in 2 modules of 77, Typer in 1 of 32 and
  that one is private. `export_signals` in the manifest adds two more signals. Turning them off
  leaves the preferred-spelling ranking with a constant term and degrades it to shortest-path.
- **`rich.Console` is not importable.** It exists only under `if TYPE_CHECKING:`, and both `ty`
  and `pyrefly` accept the failing import. `_preferred_spelling` excludes guarded spellings; a
  tripwire asserts it stays that way.
- **Rich's two central protocols declare nothing but a dunder.** Without the `dunder_protocols`
  allowlist, `satisfies.tsv` ships with zero rows for `ConsoleRenderable` and `RichCast` and
  every other check stays green. `check_assignability_honesty` has a floor for exactly this.
- **`Traceback(show_locals=True)` prints the environment.** Which is why Typer 0.23.0 stopped
  doing it by default, and why probe R002 declares the `home` normaliser.
