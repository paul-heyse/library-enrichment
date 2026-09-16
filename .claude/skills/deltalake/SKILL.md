---
name: deltalake
description: Find what delta-rs and the Delta kernel can actually do, at a pinned commit, from a prebuilt index of the full API surface, the upstream guides, the integration tests and the Delta protocol specification. Use when implementing against deltalake, when choosing between operations or storage backends, when a capability might exist but you are not sure, and to check whether code already written is leaving capability on the table. Do not use for general Rust questions unrelated to Delta Lake.
---

# delta-rs capability repository

This library moves faster than anything written about it. `DeltaOps` — the entry point every
blog post, every older reference and most of your training data teaches — **does not exist at
this pin**. The operations are inherent methods on `DeltaTable`: `table.delete()`, not
`DeltaOps(table).delete()`. It was still the documented entry point on 2026-08-23 and was gone
by 2026-09-11, and it is not the only change of that kind.

So the failure this skill exists to prevent is not "I could not find the answer". It is
**confidently writing the previous API**. Check the index before deciding how something is
spelled, and before deciding a capability is absent.

`content/` is prebuilt and pinned. Nothing here queries the network or a service.

## What is pinned

delta-rs **`58f07cd6`** on `main` (2026-09-15), because the newest published release — 0.32.4,
June 2026 — pins arrow 58 and datafusion 53. Only unreleased `main` is on arrow 59 /
datafusion 55 / object_store 0.13.2. The Delta kernel is pinned separately at
`buoyant-data/delta-kernel-rs@8ba063f8`, because delta-rs tracks it by *branch*, and a branch
is not a pin.

## Escalation ladder

Stop at the first rung that answers the question.

0. **Orienting in an unfamiliar area?** Start at `content/topics/00-map.md` and read the one
   page for that capability. Each maps a capability to its entry points, extension points and
   examples, so it replaces a dozen index lookups. Skip this rung when you already know the
   symbol you want.
1. **Does it exist, and how is it spelled?** `rg` over `content/index/*.tsv`.
   ```
   rg -i 'vacuum|retention' content/index/symbols.tsv
   rg -P '^deltalake_core::table::DeltaTable\t' content/index/methods.tsv | cut -f2,4
   ```
2. **Which operation, and what can I configure?** `content/catalogs/operations.md` — all 24
   builders, the method that constructs each, and every `with_*` it accepts.
3. **Full prose for a resolved item.** Read `content/api/<module>.md`. The path is a rule, not
   a lookup: take the canonical path's module part and replace `::` with `.`.
   `deltalake_core::table::DeltaTable` → `content/api/deltalake_core.table.md`.
4. **Can I plug into it?** Read `content/traits/<Trait>.md` — required versus provided methods,
   every in-tree implementor, and the upstream code that implements it.
5. **How is it actually used?** Structural search over the corpus.
   ```
   ast-grep run -l rust -p 'impl $TRAIT for $TYPE { $$$ }' content/corpus
   ast-grep scan -c queries/sgconfig.yml --filter '^corpus-' content/corpus
   ```
6. **What is my own code missing?** Run the capability-gap rules against the working repository.
   ```
   ast-grep scan -c queries/sgconfig.yml --filter '^project-' <path to the repo you are editing>
   ```

Rung 6 is the one to reach for unprompted after writing delta-rs code. It is cheap, and
`project-deltaops-entry-point` alone catches the single most likely thing to be written from
memory.

Four catalogs sit outside the ladder because they answer a lookup directly:
`content/catalogs/table-properties.md`, `table-features.md`, `errors.md` (the variants a retry
policy must branch on), and `foreign-impls.md` (every DataFusion, Arrow, object_store and
kernel trait this library implements, split into implementors you can name and implementors you
cannot — most of the physical-plan seam is the second kind, so read the column before treating
one as something you can substitute). `crate-map.md` says which of the 13 crates owns what.

## Rules that keep answers correct

**The operations live on `DeltaTable`.** `DeltaOps` is gone. If you are about to write
`DeltaOps(table)`, read `content/catalogs/operations.md` instead.

**The data model is the kernel's, and the crate is renamed.** `Schema` and `SchemaRef` are
`buoyant_kernel::schema::StructType`. Cargo renames that crate to `delta_kernel`, so source and
documentation write `delta_kernel::schema::StructType` while the index's canonical path says
`buoyant_kernel::…`. Both resolve through `content/index/aliases.tsv`; quote the canonical path
when you report a finding.

**Absence from `symbols.tsv` is not absence from the library.** `--all-features` is impossible
here — `rustls` and `native-tls` are mutually exclusive — so this index was built under one
explicit feature envelope. `content/index/coverage.tsv` lists every feature of every crate as
`on` or `off`, with a reason for each exclusion. Check it before reporting that something does
not exist, and say which you mean.

**Some types exist but cannot be named.** A type declared in a private module and returned by a
public method is real, callable and awaitable — and impossible to `use`, or to write in a
signature. `content/index/unnameable.tsv` lists them. rustdoc also drops such a type's impls,
so this repository documents them from a second capture; without that, `LoadBuilder` would
appear to have no methods at all.

**Re-exports that leave the indexed set are written down, not swallowed.**
`content/index/unresolved.tsv` is why `deltalake::arrow::…` and `deltalake::datafusion::…`
resolve to nothing here: those are re-exports of crates this repository does not index. That is
a boundary, not a gap — use the DataFusion or Arrow reference for them.

**ast-grep establishes syntax, not semantics.** It resolves no imports, types or dispatch. A
trait implemented in the corpus is syntax-confirmed; which crate a bare identifier refers to is
not established by ast-grep at all. Report the difference rather than collapsing it.

## Reporting

Cite the canonical path and the file you read it in. When a capability exists but you are not
recommending it, say so explicitly — the point of this repository is that the caller learns the
option existed. When the index is silent, report silence rather than absence, and name which of
the three reasons applies: not in this feature envelope, not in an indexed crate, or genuinely
not present at this commit.

## Additional references

Read `reference.md` for the full layout, the record and column schemas, the rule inventory, the
known limits, and runnable query recipes. Read `queries/README.md` before writing a new rule or
changing one. Read `build/README.md` before rebuilding or re-pinning.
