# `pyrefly_util::suggest`

Crate `pyrefly_util` · 4 public items · structured records in [`model/pyrefly_util.suggest.json`](../model/pyrefly_util.suggest.json)

## best_suggestion

`function` · `pyrefly_util::suggest::best_suggestion`

```rust
fn best_suggestion<'a, I>(missing: &ruff_python_ast::name::Name, candidates: I) -> Option<ruff_python_ast::name::Name> where I: IntoIterator<Item = Candidate<'a>>
```

Pick the closest candidate to `missing`, preferring smaller `priority` on ties.

Each candidate arrives with its length in characters, because the caller
generating them has already worked it out to filter on.

A convenience for callers that already hold their candidates. One that
generates them should drive a [`Search`] instead, so that a candidate the
bound has already ruled out is never built.

---

## char_mask

`function` · `pyrefly_util::suggest::char_mask`

```rust
fn char_mask(c: char) -> u32
```

Which characters a name contains, folded into 32 buckets.

A lower bound on edit distance: an edit changes at most two buckets, since a
substitution can clear the bucket of the character it removes and set the one
it adds, while an insertion or deletion touches only one. So two names within
distance `d` differ in at most `2 * d` buckets, and a wider difference rules a
candidate out before any table is built. Folding to 32 buckets makes letters
of either case collide, which only weakens the bound, never breaks it.

---

## Candidate

`struct` · `pyrefly_util::suggest::Candidate`

```rust
struct Candidate<'a>
```

**Methods** (2)

```rust
fn measured(name: &'a Name, priority: usize) -> Self
fn new(name: &'a Name, char_len: usize, mask: u32, priority: usize) -> Self
```

A name the search may offer, and what it needs to know about it.

---

## Search

`struct` · `pyrefly_util::suggest::Search`

```rust
struct Search<'a>
```

**Methods** (4)

```rust
fn finish(self) -> Option<Name>
fn new(missing: &'a Name) -> Self
fn offer(&mut self, candidate: Candidate<'_>)
fn offer_if(&mut self, candidate: Candidate<'_>, admit: impl FnOnce() -> bool)
```

The state a search carries between candidates.

Kept between candidates so that a match found early narrows what everything
after it is measured against.

---
