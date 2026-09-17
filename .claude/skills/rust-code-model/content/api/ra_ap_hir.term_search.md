# `ra_ap_hir::term_search`

Crate `ra_ap_hir` · 3 public items · structured records in [`model/ra_ap_hir.term_search.json`](../model/ra_ap_hir.term_search.json)

## term_search

`function` · `ra_ap_hir::term_search::term_search`

```rust
fn term_search<'db, DB: HirDatabase>(ctx: &TermSearchCtx<'_, 'db, DB>) -> Vec<Expr<'db>>
```

# Term search

Search for terms (expressions) that unify with the `goal` type.

# Arguments
* `ctx` - Context for term search

Internally this function uses Breadth First Search to find path to `goal` type.
The general idea is following:
1. Populate lookup (frontier for BFS) from values (local variables, statics, constants, etc)
   as well as from well knows values (such as `true/false` and `()`)
2. Iteratively expand the frontier (or contents of the lookup) by trying different type
   transformation tactics. For example functions take as from set of types (arguments) to some
   type (return type). Other transformations include methods on type, type constructors and
   projections to struct fields (field access).
3. If we run out of fuel (term search takes too long) we stop iterating.
4. Return all the paths (type trees) that take us to the `goal` type.

Note that there are usually more ways we can get to the `goal` type but some are discarded to
reduce the memory consumption. It is also unlikely anyone is willing ti browse through
thousands of possible responses so we currently take first 10 from every tactic.

---

## TermSearchConfig

`struct` · `ra_ap_hir::term_search::TermSearchConfig`

```rust
struct TermSearchConfig
```

**Fields**: `many_alternatives_threshold`, `fuel`

**Derives**: Clone, Copy, Debug, Default

Configuration options for the term search

---

## TermSearchCtx

`struct` · `ra_ap_hir::term_search::TermSearchCtx`

```rust
struct TermSearchCtx<'a, 'db, DB: HirDatabase>
```

**Fields**: `sema`, `scope`, `goal`, `config`

**Derives**: Debug

Context for the `term_search` function

---
