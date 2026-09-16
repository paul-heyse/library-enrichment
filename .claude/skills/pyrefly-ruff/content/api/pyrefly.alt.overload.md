# `pyrefly::alt::overload`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.alt.overload.json`](../model/pyrefly.alt.overload.json)

## ArgsExpander

`struct` · `pyrefly::alt::overload::ArgsExpander`

```rust
struct ArgsExpander<'a, Ans: LookupAnswer>
```

**Methods** (2)

```rust
fn expand(&mut self, errors: &ErrorCollector, owner: &'a Owner<Type>) -> Option<Vec<(Vec<CallArg<'a>>, Vec<CallKeyword<'a>>)>>
fn new(posargs: Vec<CallArg<'a>>, keywords: Vec<CallKeyword<'a>>, solver: &'a AnswersSolver<'a, 'a, Ans>) -> Self
```

Performs argument type expansion for arguments to an overloaded function.

---
