# Getting a value out, not a line

**I want the matched value itself, or a count, or context around it.**

By default ripgrep prints the whole line a match occurs on. When the goal is a *value* rather than evidence of a line, `-o` narrows the output to the match itself, and `-r` rewrites that output using capture groups.

`-r` is the one to be careful with, because it looks like `sed -i` and is not. It rewrites **printed output only** and never touches the file -- probe P031 runs the replacement and then re-reads the original text to prove it. ripgrep has no in-place edit mode at all, by design. If a file must change, that is ast-grep's `-U` for a structural rewrite, or a different tool for a textual one.

For extraction, `\K` under `-P` is often cleaner than a capture group: it drops the prefix from the reported span, so `-o` prints only the part you wanted without a second processing step.

## Flags

Every flag upstream files under OUTPUT OPTIONS, OUTPUT MODES. The grouping is ripgrep's own, taken from `rg --help`.

| tool | long | short | what it does |
|---|---|---|---|
| rg | --after-context | -A | Show NUM lines after each match. |
| rg | --before-context | -B | Show NUM lines before each match. |
| rg | --block-buffered |  | When enabled, ripgrep will use block buffering. |
| rg | --byte-offset | -b | Print the 0-based byte offset within the input file before each line of |
| rg | --color |  | This flag controls when to use colors. |
| rg | --colors |  | This flag specifies color settings for use in the output. |
| rg | --column |  | Show column numbers (1-based). |
| rg | --context | -C | Show NUM lines before and after each match. |
| rg | --context-separator |  | The string used to separate non-contiguous context lines in the output. |
| rg | --count | -c | This flag suppresses normal output and shows the number of lines that |
| rg | --count-matches |  | This flag suppresses normal output and shows the number of individual |
| rg | --field-context-separator |  | Set the field context separator. |
| rg | --field-match-separator |  | Set the field match separator. |
| rg | --files-with-matches | -l | Print only the paths with at least one match and suppress match |
| rg | --files-without-match |  | Print the paths that contain zero matches and suppress match contents. |
| rg | --heading |  | This flag prints the file path above clusters of matches from each file |
| rg | --help | -h | This flag prints the help output for ripgrep. |
| rg | --hostname-bin |  | This flag controls how ripgrep determines this system's hostname. |
| rg | --hyperlink-format |  | Set the format of hyperlinks to use when printing results. |
| rg | --include-zero |  | When used with -c/--count or --count-matches, this causes ripgrep to |
| rg | --json |  | Enable printing results in a JSON Lines format. |
| rg | --line-buffered |  | When enabled, ripgrep will always use line buffering. |
| rg | --line-number | -n | Show line numbers (1-based). |
| rg | --max-columns | -M | When given, ripgrep will omit lines longer than this limit in bytes. |
| rg | --max-columns-preview |  | Prints a preview for lines exceeding the configured max column limit. |
| rg | --no-filename | -I | This flag instructs ripgrep to never print the file path with each |
| rg | --no-line-number | -N | Suppress line numbers. |
| rg | --null | -0 | Whenever a file path is printed, follow it with a NUL byte. |
| rg | --only-matching | -o | Print only the matched (non-empty) parts of a matching line, with each |
| rg | --passthru |  | Print both matching and non-matching lines. |
| rg | --path-separator |  | Set the path separator to use when printing file paths. |
| rg | --pretty | -p | This is a convenience alias for --color=always --heading --line-number. |
| rg | --quiet | -q | Do not print anything to stdout. |
| rg | --replace | -r | Replaces every match with the text given when printing results. |
| rg | --sort |  | This flag enables sorting of results in ascending order. |
| rg | --sort-files |  | DEPRECATED. |
| rg | --sortr |  | This flag enables sorting of results in descending order. |
| rg | --trim |  | When set, all ASCII whitespace at the beginning of each line printed |
| rg | --vimgrep |  | This flag instructs ripgrep to print results with every match on its |
| rg | --with-filename | -H | This flag instructs ripgrep to print the file path for each matching |

## Constructs

`default` is ripgrep's Rust engine; `reachable` says whether the ripgrep CLI can actually get at it, which is narrower than whether PCRE2 supports it. `observed` is the verdict of an executed probe, not a reading of a version string.

| construct | syntax | default | reachable | since pcre2 | observed |
|---|---|---|---|---|---|
| match-reset | `\K` | no | requires -P | - | confirmed |
| group-capturing | `(...)` | yes | yes | - | not-probed |
| group-named | `(?<n>...)` | yes | yes | - | confirmed |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P007 | confirmed | Does \K drop the prefix from the reported match? | `rg -oP said \K"hello" docs/quotes.txt` |
| P031 | confirmed | Does -r rewrite the file or only the printed output? | `rg -r REPLACED -N hello src/main.rs` |

## Decision rules

- `-o` for the value, plain output for the evidence, `-c` for how many, `-l` for which files.
- `\K` or `-o` with a lookahead beats post-processing with `cut`.
- `--json` rather than parsing the text output, whenever another program reads the result.
- `-A` / `-B` / `-C` are for humans. A machine consumer should take ranges from `--json`.

## Anti-patterns

- Expecting `-r` to edit files (P031).
- Parsing `path:line:text` with a naive split on `:`. Paths contain colons; use `--json` or `-0`.
- Using `-C` in a pipeline where the context lines are then discarded.

## Checklist

- Is the value wanted, or the line?
- Will a program read this? Then `--json`.
- Does anything here imply a file should change? ripgrep will not do it.
