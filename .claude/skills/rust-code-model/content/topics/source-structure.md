# How code is written

Items, tokens, comments, whitespace, offsets, nesting, and code that does not compile.

## Mental model

A lossless concrete syntax tree: the source text can be reconstructed from it exactly. Two properties follow that no other layer has. Trivia survives, so this is the only place comments and formatting exist. And parsing never fails -- broken input yields a tree containing ERROR nodes rather than an error -- which makes it the only layer usable on code mid-edit. It resolves nothing and infers nothing.

## Questions this covers

| Question | Start at | Instead of |
|---|---|---|
| Where exactly is this in the file, including comments and whitespace? | `ra_ap_syntax::SyntaxNode::text_range` | hir, mir |
| How do I analyse code that does not compile? | `ra_ap_syntax::SourceFile::parse` | hir, mir, rustdoc-json |
| What node kinds can appear in a Rust syntax tree? | `ra_ap_syntax::SyntaxKind` | rustdoc-json |

## Why not the neighbouring layer

- **Where exactly is this in the file, including comments and whitespace?** -- not `hir`: discards trivia
- **Where exactly is this in the file, including comments and whitespace?** -- not `mir`: keeps spans but not the text between them
- **How do I analyse code that does not compile?** -- not `hir`: needs a resolvable program
- **How do I analyse code that does not compile?** -- not `mir`: needs a successful compilation
- **How do I analyse code that does not compile?** -- not `rustdoc-json`: needs rustdoc to succeed
- **What node kinds can appear in a Rust syntax tree?** -- not `rustdoc-json`: documents the types, not the grammar they encode

## What was executed

Each row ran against the pinned toolchain. A `confirmed` verdict means the probe
held *and* its control came out the other way; `recorded` means there was nothing
for a control to falsify.

| Probe | Question | Verdict |
|---|---|---|
| SY001 | Does the parser produce a tree for source that does not compile? | confirmed |
| SY002 | Are comments retained in the tree? | confirmed |
| SY003 | Is a macro invocation expanded in the concrete syntax tree? | confirmed |
| SY004 | Does the parser signal failure through its exit code? | recorded |

**SY001** — The defining property of this layer, and the reason it is the only one that can be used on code mid-edit. Valid source yields no ERROR node, so the marker is a real signal rather than something the parser always emits.

**SY002** — Trivia survives here and nowhere above. Any question about comments, formatting or exact source offsets has exactly one layer that can answer it.

**SY003** — The tree holds the invocation, not its expansion. The control looks for machinery that only appears once println! is expanded, and does not find it. To see through a macro you need the semantic layer or the compiler's own `-Zunpretty=expanded`.

**SY004** — It exits 0 on input it could not parse cleanly. Scripting this means reading the tree for ERROR nodes; the exit code discriminates nothing.

