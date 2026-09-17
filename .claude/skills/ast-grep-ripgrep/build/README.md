# The generator

Standard library only, plus the two pinned binaries. No pip install, no extraction toolchain to
pin separately — the bindings are parsed with `ast-grep outline`, which is to say the subject
indexes itself.

## Two entry points, and why they cannot be one

```
python3 acquire.py    the only stage that touches the network
python3 build.py      offline; reads acquired/ and the installed binaries
python3 verify.py     seven checks; exit 1 on any failure
```

`build.py` never imports `acquire`. That is load-bearing rather than tidy: `verify.py` proves
determinism by rebuilding and comparing digests, and if acquisition could run inside the build, a
rebuild could quietly fetch different bytes and still compare equal to itself.

`.cache/` is gitignored and holds raw downloads keyed by exact pin, never revalidated, because
the key *is* the pin. `acquired/` is committed and holds the extracted inputs the build reads.
The criterion for which is which is whether anyone else could re-serve the bytes.

## Stages

| Module | Does |
|---|---|
| `acquire.py` | GitHub tarballs at tags, docs.rs rustdoc JSON, npm and PyPI binding declarations |
| `oracles.py` | Makes the installed binaries describe themselves |
| `probes.py` | Executes the binaries against a fixture tree and records what happened |
| `surfaces.py` | Flags, kinds, fields, rule surface, languages, exit codes |
| `regexmap.py` | The construct matrix and the negative-space table, joined to probe verdicts |
| `api.py` | Library crates via rustdoc, bindings via `ast-grep outline` |
| `pages.py` | Topic pages, construct pages, catalogues |
| `model.py` `render.py` `emit.py` `link.py` | Shared with the sibling capability repositories, unmodified |
| `build.py` | Orchestrates, writes the indexes and `PROVENANCE.json` |
| `verify.py` | The gate |

## Four things that were easy to get wrong

**The two shipped kind catalogues disagree.** `schemas/languages.json` and each
`schemas/<lang>_rule.json` both enumerate node kinds, and neither is a superset — for Rust they
differ in both directions. Picking one silently would publish a catalogue wrong in a way no
reader could detect. So disputed kinds are adjudicated by running them: `ast-grep run -k <kind>`
exits 8 for a kind it cannot parse and 1 for a valid kind that matched nothing. Three published
kinds turn out to be rejected.

**A probe without a control is not evidence.** A pattern that matches proves only that something
matched. Every feature probe carries a second command, identical except for the feature under
test, which must come out the other way. If either half surprises us the probe records
`inconclusive` — never a pass. This is not hypothetical: writing the probes surfaced that the
default regex engine accepts `(?R)` and silently returns a different answer, which no amount of
reading would have revealed.

**The fixture tree is a manifest, not a directory.** Two of its files must be named `.gitignore`
and `.ignore`. Committed under those names they would be obeyed by the host repository's git and
by any ripgrep run that walked this skill. Several probes also exist to prove hidden files are
skipped by default, which cannot be demonstrated if the checkout already hid them. So
`fixtures/tree.json` is materialised into a scratch directory per run and discarded.

**Crate versions come from ripgrep's lock file, not crates.io.** ripgrep 15.2.0 links `ignore`
0.4.29 while crates.io offers 0.4.33. Indexing the latter would document code nobody is running
and would quietly contradict every probe here.

## Re-pinning

Edit `manifests/ast-grep-ripgrep.json`, install the matching binaries, then run all three stages.
`build.py` refuses to run if the installed binaries do not match the manifest, because a
repository built from one version while claiming another is the exact failure this skill exists
to prevent.

Expect `verify.py` to fail after a re-pin until the probes are re-examined. That is correct: a
behaviour that changed upstream should break the build rather than be quietly overwritten.
