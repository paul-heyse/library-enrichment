# Why did this not match?

**I expected results and got none, or got the wrong ones.**

Classify the failure before changing the query, because the two causes need opposite fixes. Either the file was never searched, or it was searched and the pattern did not match.

Settle the first question first, and cheaply. `rg --files` lists the candidate set; `rg --debug` says why a path was skipped; `ast-grep --inspect` reports config discovery and file routing. If the file is not in the candidate set, no amount of pattern work will help -- and this is the more common cause of the two.

Only then look at the pattern. For ast-grep, `--debug-query=cst` shows what the grammar produced, which usually reveals that the root node is not the one you assumed. For ripgrep, the question is normally which engine is running and whether a construct is silently degrading (P012).

## Flags

Every flag upstream files under LOGGING OPTIONS. The grouping is ripgrep's own, taken from `rg --help`.

| tool | long | short | what it does |
|---|---|---|---|
| rg | --debug |  | Show debug messages. |
| rg | --no-ignore-messages |  | When this flag is enabled, all error messages related to parsing ignore |
| rg | --no-messages |  | This flag suppresses some error messages. |
| rg | --stats |  | When enabled, ripgrep will print aggregate statistics about the search. |
| rg | --trace |  | Show trace messages. |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| A011 | recorded | Does --debug-query show how the pattern parsed? | `ast-grep run -l rust -p helper($X) --debug-query=cst src/main.rs` |
| P012 | confirmed | Does whole-pattern recursion match balanced delimiters, and what does the default engine do with the same pattern? | `rg -oP \((?:[^()]\|(?R))*\) docs/nested.txt` |
| P022 | confirmed | Are .gitignore'd paths skipped by default? | `rg -l generated .` |
| P023 | confirmed | Is .ignore a separate layer from .gitignore? | `rg -l --no-ignore-vcs 8080 .` |

## Decision rules

- File set before pattern, always.
- `--debug-query=cst` before adding a relational clause.
- If an unsupported construct is suspected, run the control: a pattern that *must* fail. Silence proves nothing on its own.
- Test a pattern against a two-line fixture before running it over a repository.

## Anti-patterns

- Making a pattern more complex to fix a file-set problem.
- Assuming an empty result means absence.
- Trusting a playground or a doc example over the installed binary.

## Checklist

- Was the file a candidate?
- Which ignore layer, if not?
- What did the query actually parse to?
- Is there a control proving the query can match anything at all?
