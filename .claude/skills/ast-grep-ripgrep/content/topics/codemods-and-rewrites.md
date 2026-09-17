# Changing code at scale

**I need to rewrite this construct across a repository.**

ast-grep rewrites what it matched, so a rewrite is only as safe as the match set. The order that keeps this safe is not negotiable: enumerate, preview, review, apply, verify. `-U` applies everything, and it belongs after a clean VCS state and a tested rule, never before.

`fix` is a text template with the captured metavariables interpolated. `transform` derives new values from captures -- case conversion, substring, regex replacement -- before they reach the template. `rewriters` handle the recursive case, where each element of a matched list needs its own rewrite.

The thing syntax cannot tell you is whether the rewrite preserved behaviour. A rewrite that produced exactly the intended edit is edit-confirmed; that it is correct needs a compiler and a test suite. Keep the two claims apart when reporting.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P031 | confirmed | Does -r rewrite the file or only the printed output? | `rg -r REPLACED -N hello src/main.rs` |

## Decision rules

- Enumerate, preview, review interactively, then apply.
- `-U` only from a clean working tree, with a rule that has tests.
- `transform` rather than encoding a case convention into the pattern.
- Re-parse the result: a rewrite that produces invalid syntax should fail loudly.
- ripgrep cannot do this. `-r` rewrites output only (P031).

## Anti-patterns

- `-U` before seeing the match set.
- A rewrite whose correctness depends on an import the tool never resolved.
- Claiming behaviour preservation from a syntactic edit.
- Hand-editing the output of a codemod instead of fixing the rule.

## Checklist

- Has the match set been reviewed?
- Is the working tree clean?
- Does the rule have fixture tests?
- Has a formatter and a type check run afterwards?
