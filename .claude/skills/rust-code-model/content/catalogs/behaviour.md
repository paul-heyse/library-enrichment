# Executed behaviour

Every claim in this repository that could be executed, was. A `confirmed` verdict means
the probe held and its control came out the other way; a probe without a control is
`recorded`, which is weaker and kept distinct on purpose.

Verdicts: **confirmed** 26, **recorded** 7.

| Probe | Layer | Question | Verdict | Evidence |
|---|---|---|---|---|
| DF001 | dataflow | Can the compiler dump the results of its own dataflow analyses? | confirmed | `1` |
| DF002 | dataflow | Are initialisation and borrow analyses dumped alongside liveness? | confirmed | `2` |
| DF003 | mir | Is more than one MIR produced per function during a compilation? | confirmed | `1` |
| DF004 | dataflow | Does NLL region inference output require -Zdump-mir-dataflow? | confirmed | `2` |
| HI001 | hir | Does the semantic layer report inferred bodies for a loaded project? | recorded | `bodies: 3` |
| HI002 | hir | Does structural search-and-replace resolve its replacement path, or treat it as text? | confirmed | `Failed to resolve path` |
| HI003 | hir | Does a failing rust-analyzer subcommand exit non-zero? | confirmed | `exit 1` |
| HI004 | hir | Does the binary emit a machine-readable schema of its own configuration? | recorded | `"title":` |
| HI005 | hir | Does the symbol outline carry inferred signatures? | confirmed | `SymbolKind(Function)` |
| HI006 | hir | Does ssr complete a rewrite whose replacement path does resolve? | recorded | `exit 101` |
| MI001 | mir | Does a conditional produce a switchInt terminator? | confirmed | `switchInt` |
| MI002 | mir | Is a function body presented as a graph of basic blocks? | confirmed | `bb0: {` |
| MI003 | mir | Does an owned value produce an explicit drop terminator? | confirmed | `drop(` |
| MI004 | mir | Does the typed pre-MIR view exist and carry types? | confirmed | `Thir {` |
| MI005 | mir | Is there a semi-stable API surface for MIR? | confirmed | `rustc_public` |
| MI006 | mir | Does the textual MIR view describe itself as an interface? | confirmed | `intended for human consumers only` |
| MI007 | mir | Can the compiler show a macro's expansion? | confirmed | `::std::io::_print` |
| MI008 | mir | Is a graphviz rendering of the control-flow graph available? | confirmed | `digraph` |
| PL001 | cargo-metadata | Does --no-deps omit the resolved dependency graph? | confirmed | `ABSENT` |
| PL002 | cargo-metadata | What schema version does cargo metadata report? | recorded | `1` |
| PL003 | cargo-metadata | Does the metadata report a feature that is declared but not enabled? | confirmed | `DECLARED` |
| RD001 | rustdoc-json | Does rustdoc JSON contain function bodies? | recorded | `exit 0` |
| RD002 | rustdoc-json | Is a function's body present anywhere in its rustdoc JSON? | confirmed | `0` |
| RD003 | rustdoc-json | Does a type in a private module appear without --document-private-items? | confirmed | `1` |
| RD004 | rustdoc-json | Is an item's Id stable when an unrelated item is added before it? | confirmed | `SHIFTED` |
| RD005 | rustdoc-json | Does rustdoc JSON record a cfg(feature) gate for an item it excluded? | confirmed | `0` |
| RD006 | rustdoc-json | Does the locally emitted format version match the pinned one? | recorded | `61` |
| SY001 | syntax | Does the parser produce a tree for source that does not compile? | confirmed | `ERROR@` |
| SY002 | syntax | Are comments retained in the tree? | confirmed | `COMMENT@` |
| SY003 | syntax | Is a macro invocation expanded in the concrete syntax tree? | confirmed | `MACRO_CALL@` |
| SY004 | syntax | Does the parser signal failure through its exit code? | recorded | `exit 0` |
| XL001 | cross-layer | Does the syntax layer know the type of an expression? | confirmed | `-` |
| XL002 | cross-layer | Does MIR retain the name of a local variable? | confirmed | `debug s =` |

`content/index/behaviors.tsv` carries the commands and the controls.
