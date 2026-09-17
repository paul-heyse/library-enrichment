# Compiler views

One flag walks every layer the compiler builds. The list is read from the compiler
itself at build time, so it cannot drift from the toolchain that produced the
observations here.

| View | Shows | Loses | Probe |
|---|---|---|---|
| `normal` | the source, pretty-printed | nothing; it is the input | MI002 |
| `expanded` | macros expanded, prelude injected | the original macro invocations | MI007 |
| `expanded,identified` | the expansion with node ids | as expanded | - |
| `expanded,hygiene` | the expansion with hygiene markers | as expanded | - |
| `ast-tree` | the compiler's AST before expansion | trivia | - |
| `ast-tree,expanded` | the AST after expansion | trivia and the invocations | - |
| `hir` | HIR pretty-printed back as Rust | trivia, some sugar | - |
| `hir,identified` | HIR with node ids | as hir | - |
| `hir,typed` | HIR with inferred types written in | as hir | - |
| `hir-tree` | the HIR data structure | as hir | MI004 |
| `thir-tree` | typed HIR as a tree | untyped sugar | - |
| `thir-flat` | typed HIR, flattened and fully typed | untyped sugar | MI004 |
| `mir` | the control-flow graph: blocks, terminators, locals | names, nesting, source shape | MI001 |
| `mir-cfg` | the same graph as graphviz | as mir | MI008 |
| `stable-mir` | MIR through the rustc_public projection | some compiler-internal detail | MI005 |

`-Zdump-mir=all` writes every pass separately, and the filenames are the pass pipeline.
Adding `-Zdump-mir-dataflow=yes` adds the dataflow results as graphviz; region inference
comes from `-Zdump-mir=all` alone and does not need the second flag (DF004).
