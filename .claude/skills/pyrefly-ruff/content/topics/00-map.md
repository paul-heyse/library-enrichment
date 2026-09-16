# Capability map

Each page maps one capability to the types that implement it, the upstream guide that explains it, and the decisions worth making deliberately.

| Topic | Covers |
|---|---|
| [Getting machine-readable output](machine-readable-output.md) | JSON from both tools, the formats, and the two traps |
| [Cross-references and call graphs](cross-references-and-calls.md) | who calls this, where is this referenced, and the Glean report that answers both |
| [The import and module graph](import-graph.md) | ruff analyze graph, its four silent limits, and the crate behind it |
| [Type inference and annotation recovery](type-inference.md) | the type representation, the solver, and how to recover types the source omits |
| [Parsing Python](parsing-python.md) | the parser, its result type, and error recovery |
| [The AST and traversal](the-ast.md) | node types, the reference enums, and the three visitor flavours |
| [The semantic model: bindings and scopes](semantic-model.md) | what ruff knows beyond syntax, and what it deliberately does not |
| [Diagnostics, edits and fixes](diagnostics-and-fixes.md) | the diagnostic model, applicability, and why some fixes are withheld |
| [The formatter](the-formatter.md) | the IR, the printer, and range formatting |
| [Writing and selecting lint rules](writing-a-lint-rule.md) | the rule enum, selectors, and the three statuses that decide whether selection works |
| [Language servers: LSP and TSP](language-servers.md) | what each server advertises, and when to use a server rather than a report |
| [Configuration surfaces](configuration.md) | ruff's 173 keys, the checker's config and its CLI overrides, and error kinds |
| [Embedding the crates in Rust](embedding-the-crates.md) | the two version lines, what is publishable, and what to depend on |
