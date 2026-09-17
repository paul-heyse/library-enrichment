# Using both tools together

**How do I combine lexical and structural search?**

The two tools compose in one direction far more often than the other: **ripgrep narrows, ast-grep confirms**. ripgrep is faster and needs no parse, so it is the right way to reduce thousands of files to the few worth parsing. ast-grep is precise, so it is the right way to decide whether a candidate is really the construct you meant.

Probe A013 is the argument in miniature. On a file where `helper` appears both as a call and inside a comment, `rg -c helper` and `ast-grep -p 'helper($X)'` return different counts. Neither is wrong; they answer different questions. Composing them gives you ripgrep's speed over the corpus and ast-grep's precision over the survivors.

The reverse direction has one good use: `outline` to locate the structural region, then ripgrep inside it for the things outline cannot see -- comments, string literals, configuration keys, dynamically built names.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| A013 | confirmed | Does ast-grep ignore an occurrence that lives inside a comment? | `ast-grep run -l rust -p helper($X) src/main.rs` |

## Decision rules

- Narrow with `rg -l`, then pipe the file list into ast-grep. NUL-delimit with `-0` and `xargs -0` so unusual filenames survive.
- A count that differs between the two tools is information, not a bug: the difference is comments, strings and generated text.
- After both, a semantic tool if the conclusion depends on identity rather than occurrence.
- Structural first, lexical second, when the question is 'what else is in this function' rather than 'where is this token'.

## Anti-patterns

- Running ast-grep over an entire monorepo when ripgrep could have cut it to twenty files first.
- Reporting a ripgrep count as the number of call sites.
- Stopping at syntax when the question was about meaning.

## Checklist

- Has the file set been narrowed before parsing?
- Are filenames NUL-delimited through the pipeline?
- Is the final claim within what syntax can establish?
