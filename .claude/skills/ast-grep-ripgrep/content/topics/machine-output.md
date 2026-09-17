# Consuming results from a program

**Another program has to read these results.**

Both tools emit structured output, and neither's human output is an API. ripgrep's `--json` is JSON Lines with `begin` / `match` / `context` / `end` / `summary` records; ast-grep's `--json` takes a style with `pretty`, `compact` or `stream`.

ast-grep's style argument must be attached with `=`. Written with a space, `stream` is consumed as a **path** -- the command succeeds, searches the wrong thing, and reports no error. Probe A010 records both halves because the failure is silent rather than loud.

Coordinates are machine values and should be carried, not re-derived. ast-grep reports zero-based line and column plus byte offsets; ripgrep reports byte offsets and one-based line numbers. Converting once at the boundary where a human sees them is correct; converting in the middle of a pipeline is how off-by-one errors are born.

## Flags

Every flag upstream files under OUTPUT MODES, LOGGING OPTIONS. The grouping is ripgrep's own, taken from `rg --help`.

| tool | long | short | what it does |
|---|---|---|---|
| rg | --count | -c | This flag suppresses normal output and shows the number of lines that |
| rg | --count-matches |  | This flag suppresses normal output and shows the number of individual |
| rg | --debug |  | Show debug messages. |
| rg | --files-with-matches | -l | Print only the paths with at least one match and suppress match |
| rg | --files-without-match |  | Print the paths that contain zero matches and suppress match contents. |
| rg | --json |  | Enable printing results in a JSON Lines format. |
| rg | --no-ignore-messages |  | When this flag is enabled, all error messages related to parsing ignore |
| rg | --no-messages |  | This flag suppresses some error messages. |
| rg | --stats |  | When enabled, ripgrep will print aggregate statistics about the search. |
| rg | --trace |  | Show trace messages. |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P030 | recorded | What shape does --json emit? | `rg --json helper src/main.rs` |
| P034 | confirmed | What exit code distinguishes no-match from error? | `rg -q definitely-not-present-anywhere .` |
| A010 | recorded | Must --json take its style with = rather than a space? | `ast-grep run -l rust -p helper($$$A) --json=compact src/main.rs` |

## Decision rules

- `rg --json` over parsing `path:line:text`. Paths contain colons.
- `ast-grep --json=stream` -- with the `=` -- for large result sets.
- Exit codes differ between the tools and are not interchangeable. `exit-codes.tsv` has the table.
- Preserve byte offsets end to end; convert to display coordinates once, at the edge.

## Anti-patterns

- `--json stream` with a space (A010).
- Treating every nonzero exit as 'no matches'. For ripgrep 1 and 2 are different answers (P034).
- Assuming ast-grep uses ripgrep's exit vocabulary. An unparseable kind exits 8.
- Depending on human output field order or colour.

## Checklist

- Is `=` used for ast-grep's JSON style?
- Are exit 1 and exit 2 distinguished?
- Are byte offsets preserved rather than recomputed?
