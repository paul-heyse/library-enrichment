# Making a slow search fast

**This is taking too long on a large tree.**

Three independent costs: how many files are opened, how much of each is read, and how expensive the pattern is per byte. They are attacked separately, and the first is almost always the cheapest win.

Narrowing by type and path removes files before anything is parsed or matched, and unlike pattern tuning it cannot change what a match means. Do it first, every time.

Pattern cost is dominated by whether a literal can be extracted. A pattern with a fixed substring lets the engine skip most of the haystack with a fast scan; one that begins with `.*` or an alternation of classes cannot. Under `-P`, JIT compilation matters too, and it is attempted but not guaranteed. For ast-grep the equivalent lever is anchoring on a node kind so that most files are rejected before any relational traversal runs.

## Decision rules

- Narrow the file set first. It is free and safe.
- Keep a literal in the pattern where you can; it is what makes the fast path possible.
- Anchor ast-grep rules on a positive `kind` before adding `inside` or `has`.
- Bound `stopBy`. `stopBy: end` walks to the root on every candidate.
- More threads is not always faster, especially on a network filesystem.

## Anti-patterns

- `-uuu` on a large tree, which adds `.git` and every build artifact.
- Repository-wide multiline dotall.
- Unbounded nested quantifiers under `-P`.
- Optimising the pattern when the file set was the problem.

## Checklist

- How many files are actually being opened?
- Does the pattern contain a usable literal?
- Is the ast-grep rule anchored on a kind?
