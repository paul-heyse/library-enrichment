# `regex_automata::dfa::onepass`

Crate `regex-automata` · 5 public items · structured records in [`model/regex_automata.dfa.onepass.json`](../model/regex_automata.dfa.onepass.json)

## BuildError

`struct` · `regex_automata::dfa::onepass::BuildError`

```rust
struct BuildError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurred during the construction of a one-pass DFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`thompson::BuildError`] type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building a one-pass DFA directly from a pattern
string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

---

## Builder

`struct` · `regex_automata::dfa::onepass::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<DFA, BuildError>
fn build_from_nfa(&self, nfa: NFA) -> Result<DFA, BuildError>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError>
fn configure(&mut self, config: Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
fn thompson(&mut self, config: thompson::Config) -> &mut Builder
```

A builder for a [one-pass DFA](DFA).

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the DFA construction. This builder is different from a
general purpose regex builder in that it permits fine grain configuration
of the construction process. The trade off for this is complexity, and
the possibility of setting a configuration that might not make sense. For
example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`] controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the NFA.
This is generally what you want for matching on arbitrary bytes.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    nfa::thompson,
    util::syntax,
    Match,
};

let re = DFA::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

let haystack = b"foo\xFFarzz\xE2\x98\xFF\n";
re.captures(&mut cache, haystack, &mut caps);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a one-pass DFA Config,
//  since that only impacts regexes that can
// produce matches of length 0.
assert_eq!(Some(Match::must(0, 0..8)), caps.get_match());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Cache

`struct` · `regex_automata::dfa::onepass::Cache`

```rust
struct Cache
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn memory_usage(&self) -> usize
fn new(re: &DFA) -> Cache
fn reset(&mut self, re: &DFA)
```

A cache represents mutable state that a one-pass [`DFA`] requires during a
search.

For a given one-pass DFA, its corresponding cache may be created either via
[`DFA::create_cache`], or via [`Cache::new`]. They are equivalent in every
way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the one-pass DFA from which it was
created. It may only be used with that one-pass DFA. A cache and its
allocations may be re-purposed via [`Cache::reset`], in which case, it can
only be used with the new one-pass DFA (and not the old one).

---

## Config

`struct` · `regex_automata::dfa::onepass::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn byte_classes(self, yes: bool) -> Config
fn get_byte_classes(&self) -> bool
fn get_match_kind(&self) -> MatchKind
fn get_size_limit(&self) -> Option<usize>
fn get_starts_for_each_pattern(&self) -> bool
fn match_kind(self, kind: MatchKind) -> Config
fn new() -> Config
fn size_limit(self, limit: Option<usize>) -> Config
fn starts_for_each_pattern(self, yes: bool) -> Config
```

The configuration used for building a [one-pass DFA](DFA).

A one-pass DFA configuration is a simple data object that is typically used
with [`Builder::configure`]. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with [`DFA::config`].

---

## DFA

`struct` · `regex_automata::dfa::onepass::DFA`

```rust
struct DFA
```

**Derives**: Clone, Debug

**Methods** (23)

```rust
fn alphabet_len(&self) -> usize
fn always_match() -> Result<DFA, BuildError>
fn builder() -> Builder
fn captures<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I, caps: &mut Captures)
fn config() -> Config
fn create_cache(&self) -> Cache
fn create_captures(&self) -> Captures
fn find<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> Option<Match>
fn get_config(&self) -> &Config
fn get_nfa(&self) -> &NFA
fn is_match<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> bool
fn memory_usage(&self) -> usize
fn never_match() -> Result<DFA, BuildError>
fn new(pattern: &str) -> Result<DFA, BuildError>
fn new_from_nfa(nfa: NFA) -> Result<DFA, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError>
fn pattern_len(&self) -> usize
fn reset_cache(&self, cache: &mut Cache)
fn state_len(&self) -> usize
fn stride(&self) -> usize
fn stride2(&self) -> usize
fn try_search(&self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures) -> Result<(), MatchError>
fn try_search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Result<Option<PatternID>, MatchError>
```

A one-pass DFA for executing a subset of anchored regex searches while
resolving capturing groups.

A one-pass DFA can be built from an NFA that is one-pass. An NFA is
one-pass when there is never any ambiguity about how to continue a search.
For example, `a*a` is not one-pass because during a search, it's not
possible to know whether to continue matching the `a*` or to move on to
the single `a`. However, `a*b` is one-pass, because for every byte in the
input, it's always clear when to move on from `a*` to `b`.

# Only anchored searches are supported

In this crate, especially for DFAs, unanchored searches are implemented by
treating the pattern as if it had a `(?s-u:.)*?` prefix. While the prefix
is one-pass on its own, adding anything after it, e.g., `(?s-u:.)*?a` will
make the overall pattern not one-pass. Why? Because the `(?s-u:.)` matches
any byte, and there is therefore ambiguity as to when the prefix should
stop matching and something else should start matching.

Therefore, one-pass DFAs do not support unanchored searches. In addition
to many regexes simply not being one-pass, it implies that one-pass DFAs
have limited utility. With that said, when a one-pass DFA can be used, it
can potentially provide a dramatic speed up over alternatives like the
[`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)
and the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). In particular,
a one-pass DFA is the only DFA capable of reporting the spans of matching
capturing groups.

To clarify, when we say that unanchored searches are not supported, what
that actually means is:

* The high level routines, [`DFA::is_match`] and [`DFA::captures`], always
do anchored searches.
* Since iterators are most useful in the context of unanchored searches,
there is no `DFA::captures_iter` method.
* For lower level routines like [`DFA::try_search`], an error will be
returned if the given [`Input`] is configured to do an unanchored search or
search for an invalid pattern ID. (Note that an [`Input`] is configured to
do an unanchored search by default, so just giving a `Input::new` is
guaranteed to return an error.)

# Other limitations

In addition to the [configurable heap limit](Config::size_limit) and
the requirement that a regex pattern be one-pass, there are some other
limitations:

* There is an internal limit on the total number of explicit capturing
groups that appear across all patterns. It is somewhat small and there is
no way to configure it. If your pattern(s) exceed this limit, then building
a one-pass DFA will fail.
* If the number of patterns exceeds an internal unconfigurable limit, then
building a one-pass DFA will fail. This limit is quite large and you're
unlikely to hit it.
* If the total number of states exceeds an internal unconfigurable limit,
then building a one-pass DFA will fail. This limit is quite large and
you're unlikely to hit it.

# Other examples of regexes that aren't one-pass

One particularly unfortunate example is that enabling Unicode can cause
regexes that were one-pass to no longer be one-pass. Consider the regex
`(?-u)\w*\s` for example. It is one-pass because there is exactly no
overlap between the ASCII definitions of `\w` and `\s`. But `\w*\s`
(i.e., with Unicode enabled) is *not* one-pass because `\w` and `\s` get
translated to UTF-8 automatons. And while the *codepoints* in `\w` and `\s`
do not overlap, the underlying UTF-8 encodings do. Indeed, because of the
overlap between UTF-8 automata, the use of Unicode character classes will
tend to vastly increase the likelihood of a regex not being one-pass.

# How does one know if a regex is one-pass or not?

At the time of writing, the only way to know is to try and build a one-pass
DFA. The one-pass property is checked while constructing the DFA.

This does mean that you might potentially waste some CPU cycles and memory
by optimistically trying to build a one-pass DFA. But this is currently the
only way. In the future, building a one-pass DFA might be able to use some
heuristics to detect common violations of the one-pass property and bail
more quickly.

# Resource usage

Unlike a general DFA, a one-pass DFA has stricter bounds on its resource
usage. Namely, construction of a one-pass DFA has a time and space
complexity of `O(n)`, where `n ~ nfa.states().len()`. (A general DFA's time
and space complexity is `O(2^n)`.) This smaller time bound is achieved
because there is at most one DFA state created for each NFA state. If
additional DFA states would be required, then the pattern is not one-pass
and construction will fail.

Note though that currently, this DFA uses a fully dense representation.
This means that while its space complexity is no worse than an NFA, it may
in practice use more memory because of higher constant factors. The reason
for this trade off is two-fold. Firstly, a dense representation makes the
search faster. Secondly, the bigger an NFA, the more unlikely it is to be
one-pass. Therefore, most one-pass DFAs are usually pretty small.

# Example

This example shows that the one-pass DFA implements Unicode word boundaries
correctly while simultaneously reporting spans for capturing groups that
participate in a match. (This is the only DFA that implements full support
for Unicode word boundaries.)

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{dfa::onepass::DFA, Match, Span};

let re = DFA::new(r"\b(?P<first>\w+)[[:space:]]+(?P<last>\w+)\b")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "Шерлок Холмс", &mut caps);
assert_eq!(Some(Match::must(0, 0..23)), caps.get_match());
assert_eq!(Some(Span::from(0..12)), caps.get_group_by_name("first"));
assert_eq!(Some(Span::from(13..23)), caps.get_group_by_name("last"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: iteration

Unlike other regex engines in this crate, this one does not provide
iterator search functions. This is because a one-pass DFA only supports
anchored searches, and so iterator functions are generally not applicable.

However, if you know that all of your matches are
directly adjacent, then an iterator can be used. The
[`util::iter::Searcher`](crate::util::iter::Searcher) type can be used for
this purpose:

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    util::iter::Searcher,
    Anchored, Input, Span,
};

let re = DFA::new(r"\w(\d)\w")?;
let (mut cache, caps) = (re.create_cache(), re.create_captures());
let input = Input::new("a1zb2yc3x").anchored(Anchored::Yes);

let mut it = Searcher::new(input).into_captures_iter(caps, |input, caps| {
    Ok(re.try_search(&mut cache, input, caps)?)
}).infallible();
let caps0 = it.next().unwrap();
assert_eq!(Some(Span::from(1..2)), caps0.get_group(1));

# Ok::<(), Box<dyn std::error::Error>>(())
```

---
