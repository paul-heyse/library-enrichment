# `regex_automata::util::prefilter`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.util.prefilter.json`](../model/regex_automata.util.prefilter.json)

## Prefilter

`struct` · `regex_automata::util::prefilter::Prefilter`

```rust
struct Prefilter
```

**Derives**: Clone, Debug

**Methods** (8)

```rust
fn find(&self, haystack: &[u8], span: Span) -> Option<Span>
fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter>
fn from_hirs_prefix<H: Borrow<Hir>>(kind: MatchKind, hirs: &[H]) -> Option<Prefilter>
fn is_fast(&self) -> bool
fn max_needle_len(&self) -> usize
fn memory_usage(&self) -> usize
fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter>
fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>
```

A prefilter for accelerating regex searches.

If you already have your literals that you want to search with,
then the vanilla [`Prefilter::new`] constructor is for you. But
if you have an [`Hir`] value from the `regex-syntax` crate, then
[`Prefilter::from_hir_prefix`] might be more convenient. Namely, it uses
the [`regex-syntax::hir::literal`](regex_syntax::hir::literal) module to
extract literal prefixes for you, optimize them and then select and build a
prefilter matcher.

A prefilter must have **zero false negatives**. However, by its very
nature, it may produce false positives. That is, a prefilter will never
skip over a position in the haystack that corresponds to a match of the
original regex pattern, but it *may* produce a match for a position
in the haystack that does *not* correspond to a match of the original
regex pattern. If you use either the [`Prefilter::from_hir_prefix`] or
[`Prefilter::from_hirs_prefix`] constructors, then this guarantee is
upheld for you automatically. This guarantee is not preserved if you use
[`Prefilter::new`] though, since it is up to the caller to provide correct
literal strings with respect to the original regex pattern.

# Cloning

It is an API guarantee that cloning a prefilter is cheap. That is, cloning
it will not duplicate whatever heap memory is used to represent the
underlying matcher.

# Example

This example shows how to attach a `Prefilter` to the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) in order to accelerate
searches.

```
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::prefilter::Prefilter,
    Match, MatchKind,
};

let pre = Prefilter::new(MatchKind::LeftmostFirst, &["Bruce "])
    .expect("a prefilter");
let re = PikeVM::builder()
    .configure(PikeVM::config().prefilter(Some(pre)))
    .build(r"Bruce \w+")?;
let mut cache = re.create_cache();
assert_eq!(
    Some(Match::must(0, 6..23)),
    re.find(&mut cache, "Hello Bruce Springsteen!"),
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

But note that if you get your prefilter incorrect, it could lead to an
incorrect result!

```
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::prefilter::Prefilter,
    Match, MatchKind,
};

// This prefilter is wrong!
let pre = Prefilter::new(MatchKind::LeftmostFirst, &["Patti "])
    .expect("a prefilter");
let re = PikeVM::builder()
    .configure(PikeVM::config().prefilter(Some(pre)))
    .build(r"Bruce \w+")?;
let mut cache = re.create_cache();
// We find no match even though the regex does match.
assert_eq!(
    None,
    re.find(&mut cache, "Hello Bruce Springsteen!"),
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

---
