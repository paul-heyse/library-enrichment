# Mapping code without reading it

**What is in this file or directory, without opening everything?**

`ast-grep outline` produces a syntax-aware table of contents: top-level items, and their direct members, with line numbers. It exists so an agent can learn a file's shape for a few hundred tokens instead of reading two thousand lines.

Defaults are chosen for that use: a directory yields exported names, a file yields a richer digest. `--view` trades detail against tokens -- `names`, `signatures`, `digest`, `expanded`, in that order of cost.

Two behaviours will surprise you. `--match` filters **top-level items only**, so filtering for a method name finds nothing even though the method is right there in the output (A009); select its parent instead. And `outline` exits 0 no matter what -- for an empty result, which is defensible, and also for a file that does not exist, which is not (A007). The exit status carries no information; branch on stdout and watch stderr.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| A007 | confirmed | Does outline exit nonzero when it finds nothing? | `ast-grep outline --match ZZZ_no_such_symbol src/lib.rs` |
| A008 | confirmed | Does --view expanded list members of one item? | `ast-grep outline src/lib.rs --match Parser --view expanded` |
| A009 | confirmed | Does --match filter members, or only top-level items? | `ast-grep outline src/lib.rs --match recover` |
| A012 | confirmed | Does --items imports isolate dependency direction? | `ast-grep outline src/app.py --items imports` |

## Decision rules

- Unknown directory: `outline <dir>` and read the exported names.
- Known file, unknown shape: the default digest.
- Known type, want its members: `--match <Type> --view expanded`.
- Dependency direction: `--items imports`. It is syntax only -- it does not follow a re-export to its source.
- Feeding another program: `--json=stream`, one file object per line, coordinates zero-based.

## Anti-patterns

- `--match` on a member name and concluding the member is absent (A009).
- Branching on outline's exit status (A007).
- Treating an export list as a resolved public API. It is syntax-only classification.
- Reading a whole file to find its class names.

## Checklist

- Is the smallest sufficient `--view` in use?
- Does `--match` target an item rather than a member?
- Is empty output being distinguished from a read error via stderr?
