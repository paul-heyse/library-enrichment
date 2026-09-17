# ra_ap_hir

**Reach for it when** the question is what a symbol means or where it resolves to.

## What it is for

what names mean: resolution, definitions and references, inferred types, method resolution, trait and impl relationships, macro-expanded semantics, and the mapping back to syntax.

## What it cannot answer

how a body executes as a graph; anything about a project it has not loaded; trivia, which is discarded above the syntax layer.

## Getting it

|  |  |
|---|---|
| Obtained by | `ra_ap_hir::Semantics over a loaded database` |
| Entry point | `ra_ap_hir::Semantics` |
| Needs a build | a loaded database, and proc macros need building to expand |
| Needs a network | no, once dependencies are vendored |
| Stability | 0.0.x weekly, every release semver-breaking, no changelog; only 12.84% documented |
| Crates pinned here | ra_ap_hir 0.0.352, ra_ap_ide 0.0.352 |
| Indexed symbols | 280 |

## Questions routed here

- What type does this expression have? — `ra_ap_hir::Semantics::type_of_expr`
- Where is this symbol defined, and where is it used? — `ra_ap_ide::Analysis`
- Which impl does this method call resolve to? — `ra_ap_hir::Semantics::resolve_method_call`
- How do I rewrite every occurrence of a pattern? — `rust-analyzer ssr`
- Which rust-analyzer version am I actually running? — `rust-analyzer --version`

## Executed evidence

| Probe | Question | Verdict | Command |
|---|---|---|---|
| HI001 | Does the semantic layer report inferred bodies for a loaded project? | recorded | `rust-analyzer analysis-stats . --no-sysroot --disable-build-scripts --disable-proc-macros` |
| HI002 | Does structural search-and-replace resolve its replacement path, or treat it as text? | confirmed | `rust-analyzer ssr old($a) ==>> no_such_function($a)` |
| HI003 | Does a failing rust-analyzer subcommand exit non-zero? | confirmed | `rust-analyzer ssr old($a) ==>> no_such_function($a)` |
| HI004 | Does the binary emit a machine-readable schema of its own configuration? | recorded | `rust-analyzer --print-config-schema` |
| HI005 | Does the symbol outline carry inferred signatures? | confirmed | `rust-analyzer symbols` |
| HI006 | Does ssr complete a rewrite whose replacement path does resolve? | recorded | `rust-analyzer ssr old($a) ==>> renamed($a)` |

