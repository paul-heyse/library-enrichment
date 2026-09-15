---
name: datafusion
description: Find what Apache DataFusion and the Arrow Rust family can actually do, at the pinned release, from a prebuilt index of the full API surface, the upstream guides and the runnable examples. Use when implementing against DataFusion, Arrow, Parquet or object_store, when choosing between approaches, when a capability might exist but you are not sure, and to check whether code already written is leaving capability on the table. Do not use for general Rust questions unrelated to these crates.
---

# DataFusion capability repository

DataFusion's real API is not in the `datafusion` crate. That facade documents 1,376 items; the
surface it re-exports spans 60 crates and 7,134 public items with 21,047 methods. Types you think
you know carry far more than you remember — `SessionConfig` has 64 methods, `Expr` 101,
`ScalarValue` 107 — and `TableProvider` alone has 3 required methods against 12 provided ones that
include `merge_into`, `truncate`, `update` and `supports_filters_pushdown`.

So the failure this skill exists to prevent is not "I could not find the answer". It is
**believing you already knew the answer**. Check the index before deciding a capability is absent.

`content/` is prebuilt and pinned. Nothing here queries the network or a service.

## Escalation ladder

Stop at the first rung that answers the question.

0. **Orienting in an unfamiliar area?** Start at `content/topics/00-map.md` and read the one page
   for that capability. Each maps the capability to its entry points, extension points, settings
   and examples, so it replaces a dozen index lookups. Skip this rung when you already know the
   symbol you want.
1. **Does it exist?** `rg` over `content/index/*.tsv`.
   ```
   rg -i 'pushdown' content/index/symbols.tsv
   rg -iP '\tspill|\tmemory_pool' content/index/methods.tsv
   ```
2. **A structured question over the model** — a shipped rule, not an invented pattern.
   ```
   ast-grep scan -c queries/sgconfig.yml --filter '^model-' content/model
   ```
3. **Full prose for a resolved item.** Read `content/api/<module>.md`. The path is a rule, not a
   lookup: take the canonical path's module part and replace `::` with `.`.
   `datafusion_expr::expr::Expr` → `content/api/datafusion_expr.expr.md`.
4. **Can I plug into it?** Read `content/traits/<Trait>.md` — required versus provided methods,
   every in-tree implementor, and the examples that implement it.
5. **How is it actually used?** Structural search over the 79 runnable examples.
   ```
   ast-grep run -l rust -p 'impl $TRAIT for $TYPE { $$$ }' content/corpus/examples
   ast-grep outline --no-default-outline-rules --outline-rules queries/outline/datafusion.yml content/corpus/examples
   ```
6. **What is my own code missing?** Run the capability-gap rules against the working repository.
   ```
   ast-grep scan -c queries/sgconfig.yml --filter '^project-' <path to the repo you are editing>
   ```

Two catalogs sit outside the ladder because they answer a lookup directly:
`content/catalogs/config-options.md` (155 settings, each joined to the Rust builder method that
sets it) and `content/catalogs/sql-functions.md` (332 SQL functions and operators, categorized).
`content/catalogs/crate-map.md` says which of the 60 crates owns what.

Rung 6 is the one to reach for unprompted after writing DataFusion code. It is cheap and it is the
only rung that finds capability you did not think to look for.

## Rules that keep answers correct

**`datafusion::…` is almost never where something is defined.** It is an access path. Resolve it
through `content/index/aliases.tsv` before concluding anything about which crate owns an item, and
quote the canonical path when you report a finding.

**Absence from the index is evidence about this pinned release only** — DataFusion 55.1.0, Arrow
59.3.0, object_store 0.13.2, sqlparser 0.62.0. It is not evidence about a newer release and not
evidence about a crate outside the pinned set. Say which you mean.

**Feature gating comes from `content/index/features.tsv` and nowhere else.** rustdoc JSON records
no per-item `cfg(feature = …)` at these format versions, so a signature can never tell you whether
an item is behind a feature. Do not infer it from one. `content/PROVENANCE.json` also records which
crates were documented with `all-features`; four were not, and their surface is default-features
only.

**Prefer a shipped rule to an invented pattern, and a `kind`-anchored rule to `--pattern`.**
Patterns are formatting-sensitive: a pattern for a method declaration silently misses the same
method written across several lines. Every shipped rule has `ast-grep test` fixtures for this
reason.

**ast-grep establishes syntax, not semantics.** It resolves no imports, types or dispatch. A trait
implemented in an example is syntax-confirmed; which crate a bare identifier refers to is not
established by ast-grep at all. Report the difference rather than collapsing it.

## Reporting

Cite the canonical path and the file you read it in. When a capability exists but you are not
recommending it, say so explicitly — the point of this repository is that the caller learns the
option existed. When the index is silent, report silence rather than absence.

## Additional references

Read `reference.md` for the full layout, the record and column schemas, the rule inventory, and
runnable query recipes. Read `queries/README.md` before writing a new rule or changing one.
