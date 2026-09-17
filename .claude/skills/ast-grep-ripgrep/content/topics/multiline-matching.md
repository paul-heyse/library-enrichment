# Matching across line boundaries

**My pattern needs to span more than one line.**

ripgrep is line-oriented by default: the haystack handed to the matcher is a single line, so a pattern containing `\n` cannot match. `-U` changes what the haystack *is*, letting it span lines.

What `-U` does not change is what `^` and `$` mean. They remain **line** anchors even in multiline mode. Probe P020 measures this: under `-U`, `^end block` matches, and `\Aend block` does not. If you want the start of the whole haystack, `\A` and `\z` are the anchors. Nearly everyone assumes `-U` turns `^` into a haystack anchor, and it does not.

`.` also still stops at a newline under `-U`; `--multiline-dotall` is the separate switch for that (P021). Three independent concepts -- haystack extent, anchor meaning, dot behaviour -- and one flag only moves the first.

## Constructs

`default` is ripgrep's Rust engine; `reachable` says whether the ripgrep CLI can actually get at it, which is narrower than whether PCRE2 supports it. `observed` is the verdict of an executed probe, not a reading of a version string.

| construct | syntax | default | reachable | since pcre2 | observed |
|---|---|---|---|---|---|
| anchor-line | `^ $` | yes | yes | - | confirmed |
| anchor-haystack | `\A \z` | yes | yes | - | confirmed |
| dot | `.` | yes | yes | - | not-probed |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P019 | confirmed | Does -U let a pattern cross a line boundary? | `rg -U -c start block\n  inner one docs/multiline.txt` |
| P020 | confirmed | Under -U, does ^ still anchor to a line rather than to the whole haystack? | `rg -U -c ^end block docs/multiline.txt` |
| P021 | confirmed | Does --multiline-dotall let . cross newlines? | `rg -U --multiline-dotall -c start block.*end block docs/multiline.txt` |

## Decision rules

- `-U` widens the haystack. `--multiline-dotall` widens `.`. `\A` / `\z` are the haystack anchors. Reach for whichever of the three you actually need.
- Multiline search buffers more, so bound it by path and type first.
- For a construct that nests -- braces, parentheses, tags -- a multiline regex is the wrong instrument. Use ast-grep, which parses.

## Anti-patterns

- Expecting `-U` to make `^` mean start-of-file.
- Repository-wide `-U --multiline-dotall` with a `.*` between two anchors. It will match nearly everything, slowly.
- Matching a nested structure by counting delimiters in a regex.

## Checklist

- Is `-U` needed, or only `--multiline-dotall`?
- Should the anchors be `^`/`$` or `\A`/`\z`?
- Would ast-grep express this as one node?
