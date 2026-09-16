---
name: pyrefly-ruff
description: Find what ruff and pyrefly can actually do to Python code, at pinned releases, from a prebuilt index of 42 Rust crates plus every catalog the two tools emit about themselves. Use when you need cross-references, call graphs, an import map, inferred types or annotations out of a Python codebase; when choosing which tool answers a question; when embedding the ruff crates in Rust; and when writing or selecting lint rules. Do not use for general Python questions unrelated to static analysis.
---

# ruff and pyrefly capability repository

Most wrong answers about these two tools are not "I could not find it". They are **reaching for
the wrong instrument**, and the four commonest are all invisible until the output is already
wrong:

- Driving a language server in a loop to build a project-wide index, when one command emits the
  whole cross-reference and call graph as JSON.
- Treating `ruff analyze graph` as a module graph. It is **file-level**, it has no
  `--output-format` flag, and a path that does not exist returns `{}` with **exit status 0**.
- Parsing human output. Both tools emit structured forms for everything that matters, and both
  say the human formats are unstable.
- Writing `ruff_python_ast = "0.16.7"` into a `Cargo.toml`. That version does not exist — the
  library crates are on a separate `0.0.x` line from the product.

`content/` is prebuilt and pinned. Nothing here queries the network or a service.

## What is pinned

`ruff` **0.16.7** with its library crates at **0.0.13** — one release, two version lines, verified
by `ruff_linter 0.16.7` depending on all 18 library crates at `^0.0.13`. And **pyrefly 1.3.1**,
whose crates are not published at all (crates.io carries a name squat at `0.0.1`), so its rustdoc
JSON is produced locally from tag `1.3.1` on a dated nightly.

3,029 items · 12,520 methods · 1,358 trait impls · 586 module pages · 970 lint rules ·
497 CLI flags · 182 config keys · 144 error kinds · 28 extension points · 13 topics ·
10 catalogs · 42 crates.

## Escalation ladder

Stop at the first rung that answers the question.

0. **Which tool answers this at all?** `content/topics/00-map.md`. This is the rung to start on,
   because the expensive mistake here is instrument choice, not lookup.
1. **Who calls this? Where is this referenced?**
   `content/catalogs/cross-references.md`. `pyrefly check --report-glean <dir>` emits
   `python.CalleeToCaller`, `python.XRefsViaNameByTarget`, `DeclarationLocation` and
   `DefinitionLocation` as JSON, per module. This is the only route either toolchain offers to
   whole-project references and call edges. `--report-pysa` is **not** an alternative: despite
   its help text promising JSON, it writes Cap'n Proto binary.
2. **What imports what?** `content/catalogs/import-graph.md`, with all four of its silent limits
   stated and measured.
3. **Does this rule exist, is it fixable, was it removed?**
   ```
   rg -P '^F401\t' content/index/rules.tsv
   rg -P '\tRemoved\t' content/index/rules.tsv | cut -f1,2
   ```
   All 970 rules join to a variant of `ruff_linter::codes::Rule`. 141 are `Preview` and do
   nothing without preview mode; 17 are `Removed` and selecting one is an error.
4. **How do I get machine-readable output?** `content/catalogs/structured-outputs.md` — every
   emitter of both tools in one table.
5. **Does this API exist, and where?**
   ```
   rg -i 'visitor|semantic|diagnostic' content/index/symbols.tsv | cut -f1,2,3
   rg -P '^ruff_python_ast::generated::Expr\t' content/index/methods.tsv | cut -f2,4
   ```
6. **Full prose for a resolved item.** `content/api/<module>.md`. The path is a rule, not a
   lookup: the canonical path's module part with `::` replaced by `.`.
   `ruff_python_semantic::model::SemanticModel` → `content/api/ruff_python_semantic.model.md`.
7. **What must I implement to plug in?** `content/traits/<Trait>.md` — 28 extension points with
   their required and provided methods.
8. **Is this typing feature actually supported?** `content/catalogs/conformance.md` — 139 of
   the typing council's 144 conformance tests pass, and the five that do not are named. The
   suite's own sources are in `content/corpus/conformance/third_party/`, so the specification's
   required type at a position is readable rather than guessable.
9. **Which flag, which error kind?** `content/index/cli.tsv` (497 flags, grouped) and
   `content/index/error-kinds.tsv` (144 kinds, in both the enum and CLI spellings).
10. **Is my own code using a rule code that no longer exists, or parsing text output?**
    ```
    ast-grep scan -c queries/sgconfig.yml --filter '^project-' <the repo you are editing>
    ```
    Three rules, all `severity: hint`: a removed ruff code, a `ruff check` whose output must be
    parsed as text, and an `analyze graph` call that trusts its exit status.

## Rules that keep answers correct

**Pick the instrument by the shape of the question.** One position, interactively → LSP. Whole
project, as data → the Glean report. Imports only, cheaply → `ruff analyze graph`. Syntax shapes →
ast-grep. Types → pyrefly. Names and scopes without types → `ruff_python_semantic`.

**The two tools divide at names versus types.** ruff resolves bindings and scopes and has no
types at all; pyrefly has types. That is why they compose rather than compete, and why
"`SemanticModel` will tell me the type" is wrong.

**Parameters are not inferred.** pyrefly infers variables and returns, and treats an unannotated
parameter as `Any` by design. When recovering annotations for an under-typed codebase, returns
come back and parameters do not.

**A version line is not a version.** `symbols.tsv` carries the crate's own version per row
because this index spans `0.16.7`, `0.0.13` and `1.3.1` simultaneously.

**An exit status is not a result.** `ruff check` exits non-zero when it finds diagnostics — a
finding, not a failure. `ruff analyze graph` exits zero on a path that does not exist. Check the
payload, never the status.

**Some capability is only in the private API.** pyrefly's CLI argument structures are indexed
because acquisition documented private items; ruff's are public. Neither is in the published
docs. `content/index/unnameable.tsv` lists what exists but cannot be imported.

**ast-grep establishes syntax, not semantics.** It resolves no imports or types, and it has no
TOML grammar — so `project-removed-rule-code` sees a rule code in Python but not in a
`pyproject.toml`. Every shipped rule is `severity: hint`: a match is a question about your code.

## Reporting

Cite the canonical path and the file you read it in. When a capability exists but you are not
recommending it, say so — the point of this repository is that the caller learns the option
existed. When the index is silent, report silence rather than absence, and say which pin it is
silent at.

## Additional references

Read `reference.md` for the layout, the table schemas, the rule inventory, the known limits and
runnable query recipes. Read `build/README.md` before rebuilding or re-pinning.
