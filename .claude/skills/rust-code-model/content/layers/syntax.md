# ra_ap_syntax

**Reach for it when** the question is about text, formatting, position, or code that does not compile.

## What it is for

exactly how source is written: items, tokens, comments, whitespace, offsets, nesting, and incomplete or invalid code.

## What it cannot answer

what any name refers to; what type anything has; what a macro expands to (SY003).

## Getting it

|  |  |
|---|---|
| Obtained by | `ra_ap_syntax::SourceFile::parse, or rust-analyzer parse` |
| Entry point | `ra_ap_syntax::SourceFile` |
| Needs a build | no -- it parses a string |
| Needs a network | no |
| Stability | 0.0.x weekly, but the rowan/AstNode/SyntaxKind design has been stable for years |
| Crates pinned here | ra_ap_span 0.0.352, ra_ap_syntax 0.0.352 |
| Indexed symbols | 484 |

## Questions routed here

- Where exactly is this in the file, including comments and whitespace? — `ra_ap_syntax::SyntaxNode::text_range`
- How do I analyse code that does not compile? — `ra_ap_syntax::SourceFile::parse`
- What node kinds can appear in a Rust syntax tree? — `ra_ap_syntax::SyntaxKind`

## Executed evidence

| Probe | Question | Verdict | Command |
|---|---|---|---|
| SY001 | Does the parser produce a tree for source that does not compile? | confirmed | `rust-analyzer parse` |
| SY002 | Are comments retained in the tree? | confirmed | `rust-analyzer parse` |
| SY003 | Is a macro invocation expanded in the concrete syntax tree? | confirmed | `rust-analyzer parse` |
| SY004 | Does the parser signal failure through its exit code? | recorded | `rust-analyzer parse` |

