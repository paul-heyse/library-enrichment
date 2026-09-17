# What this index refuses to claim

Stated as boundaries rather than left as gaps, because silence in an index is not
evidence and a reader cannot tell an omission from an absence.

This page covers behaviour that was considered for a probe and deliberately not probed.
`reference.md` carries the full known-limits list, including what is excluded from the
API index and why.

| Behaviour | Why no probe | Where to look instead |
|---|---|---|
| `Progress`, `Live`, `Status` animation | Output depends on elapsed time and refresh rate; there is no construction that produces stable bytes, and a normalised capture would be a fiction dressed as evidence. | `content/corpus/rich/tests/test_progress.py` -- upstream's own executable statement. Probe `R003` covers the deterministic `disable=True` path. |
| `Spinner` frames | The frame index is a function of elapsed time. | `rich/_spinners.py` is the data table; read the frames rather than a capture of one. |
| Terminal capability negotiation on a real TTY | Every probe writes to a `StringIO`, which is not a terminal. What a real TTY negotiates is indexed from source and never asserted. | `catalogs/env-vars.md` for the variables, and `rich/docs/source/console.rst` in the corpus for upstream's account. |
| `Syntax` token colours | Lexers and token boundaries belong to Pygments, not Rich. A captured highlight would be a claim about Pygments wearing Rich's name. | The frame -- gutter, line numbers, padding, background -- is stable and indexed. Pygments is pinned at 2.21.0 in `resolved.json`. |
| `Traceback` bytes | A real traceback carries absolute paths, an interpreter version and frame counts that differ per machine -- and the paths are a transferability leak into `content/`, not merely a determinism one. | Probe `R002` captures the structure under a synthetic filename, with the normalisation recorded on the row. |
| Windows console paths | `_win32_console.py`, `_windows_renderer.py`, `legacy_windows=True` and Typer's conditional `colorama` are never executed. Every probe passes `legacy_windows=False` explicitly, which is part of why they are deterministic. | Indexed from source. A box glyph shown here may be substituted on a legacy Windows console. |
| Jupyter rendering | No probe imports IPython, and `is_jupyter()` is False in all of them. | `content/seams/jupyter.md`, marked `supported: observed-only` for exactly this reason. |
| Shell completion installation | `shellingham` inspects the parent process and `--install-completion` writes to the user's home directory. Neither is safe or stable to execute in a build. | Indexed from source, plus `typer/docs/tutorial/options-autocompletion.md` in the corpus. |

## The general rule

A capture is a fact about a construction. Where no construction produces stable bytes --
anything driven by elapsed time, by a real terminal's negotiation, or by a library this
index does not pin -- a normalised capture would be a fiction dressed as evidence, and
upstream's own tests are the better answer. They are in the corpus.
