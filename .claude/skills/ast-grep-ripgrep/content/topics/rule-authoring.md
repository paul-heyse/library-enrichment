# Writing a durable rule

**I need this check to run repeatedly, not once.**

A rule file is a matching predicate plus everything needed to report it: an id, a language, a message, a severity, and optionally a fix. The predicate is the rule object, whose complete field list is in `rule-fields.tsv` -- generated from ast-grep's own JSON Schema, so it cannot drift from what the binary validates.

The field group to understand first is relational: `inside`, `has`, `precedes`, `follows`, each accepting `stopBy` and `field`. `field` is the one that most often makes a rule work, and the one hardest to guess -- which is why `fields.tsv` exists, with 631 field names across 23 languages.

When a later clause uses a metavariable an earlier clause captured, write them under an explicit `all:`. Field order within a rule object is not a promise about evaluation order; `all` is.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| A004 | confirmed | Does --kind accept an ESQuery-style selector? | `ast-grep run -l rust -k function_item src/main.rs` |
| A005 | confirmed | Does the ESQuery direct-child combinator work in --kind? | `ast-grep run -l rust -k function_item > identifier src/lib.rs` |

## Decision rules

- Anchor on a positive `kind` first; add relations second.
- `all:` whenever one clause depends on another's capture.
- Bound `stopBy`. The default neighbour behaviour is usually what you meant.
- Look `field` up in `fields.tsv` rather than guessing.
- Prototype with `scan --inline-rules` before creating a file.

## Anti-patterns

- One rule with an `any:` covering unrelated concerns.
- `stopBy: end` everywhere.
- Rebuilding a parser with `regex:` inside a rule that already has structural context.
- Relying on field order for evaluation order.

## Checklist

- Is there a positive kind anchor?
- Are capture dependencies inside `all:`?
- Do the field names exist in `fields.tsv`?
- Are there valid and invalid test cases?
