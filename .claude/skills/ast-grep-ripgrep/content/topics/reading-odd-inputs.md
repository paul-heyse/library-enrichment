# Archives, binaries, and generated input

**The content is compressed, binary, or has to be produced first.**

`-z` searches inside gzip, bzip2, xz and zstd without unpacking them first. `--pre` is the general form: run a command per file and search its output, which handles PDFs, databases and anything else with a text projection.

`--pre` executes an arbitrary program for every file in the search. Enabling it is a decision about trust, not about convenience, and `--pre-glob` should bound which files reach it.

Binary handling is two rules that look like one. A file named explicitly on the command line is searched; the same file reached by directory traversal is skipped. Probe P029 measures both halves, because testing on an explicit path and generalising to a recursive search gets it backwards in the direction that silently loses results.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P029 | confirmed | Is a binary file searched when named explicitly but skipped when found by traversal? | `rg -c needle data/blob.bin` |

## Decision rules

- `-z` for ordinary compressed logs.
- `--pre` plus `--pre-glob` for anything needing a real conversion.
- `-a` to search binary content deliberately; `--binary` to search it but keep the output safe.
- `--null-data` for NUL-delimited records rather than lines.

## Anti-patterns

- Enabling `--pre` repository-wide without a glob.
- Generalising binary behaviour from an explicit path to a traversal (P029).
- Piping binary output to a terminal.

## Checklist

- Is the file set for `--pre` bounded?
- Explicit path, or traversal?
- Is the output safe to print?
