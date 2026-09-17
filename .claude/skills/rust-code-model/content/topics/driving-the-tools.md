# Running these from a script

Invocation shapes, exit codes and the failures that do not look like failures.

## Mental model

Every rust-analyzer subcommand is documented as carrying no stability guarantee, and they are not consistent with each other: `ssr` exits 1 on a refused rule while `parse` exits 0 on source it could not parse. `ssr` and `search` take no path and work on the current directory, so passing one makes the path be parsed as a rule and the error names the wrong cause.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| Can I script these tools and trust the exit code? | `-` | hir |
| Which layer should I use at all? | `content/index/layers.tsv` | - |

## Why not the neighbouring layer

- **Can I script these tools and trust the exit code?** -- not `hir`: sometimes: ssr exits 1 on a refused rule, but parse exits 0 on source it could not parse

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| XL001 | Does the syntax layer know the type of an expression? | confirmed |
| XL002 | Does MIR retain the name of a local variable? | confirmed |

**XL001** — The control finds the return type as written -- RET_TYPE is a syntax node. What is absent is any inferred type. The tree records what the source says, never what it means, which is the single most common layer confusion.

**XL002** — Partly. Locals are numbered, and the original names survive only as `debug` annotations for the debugger. Code that matches on variable names will not find them where it expects.

