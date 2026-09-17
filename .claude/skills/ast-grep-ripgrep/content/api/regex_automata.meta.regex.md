# `regex_automata::meta::regex`

Crate `regex-automata` · 8 public items · structured records in [`model/regex_automata.meta.regex.json`](../model/regex_automata.meta.regex.json)

## Builder

`struct` · `regex_automata::meta::regex::Builder`

Also reachable as `regex_automata::meta::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<Regex, BuildError>
fn build_from_hir(&self, hir: &Hir) -> Result<Regex, BuildError>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError>
fn build_many_from_hir<H: Borrow<Hir>>(&self, hirs: &[H]) -> Result<Regex, BuildError>
fn configure(&mut self, config: Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
```

A builder for configuring and constructing a `Regex`.

The builder permits configuring two different aspects of a `Regex`:

* [`Builder::configure`] will set high-level configuration options as
described by a [`Config`].
* [`Builder::syntax`] will set the syntax level configuration options
as described by a [`util::syntax::Config`](crate::util::syntax::Config).
This only applies when building a `Regex` from pattern strings.

Once configured, the builder can then be used to construct a `Regex` from
one of 4 different inputs:

* [`Builder::build`] creates a regex from a single pattern string.
* [`Builder::build_many`] creates a regex from many pattern strings.
* [`Builder::build_from_hir`] creates a regex from a
[`regex-syntax::Hir`](Hir) expression.
* [`Builder::build_many_from_hir`] creates a regex from many
[`regex-syntax::Hir`](Hir) expressions.

The latter two methods in particular provide a way to construct a fully
feature regular expression matcher directly from an `Hir` expression
without having to first convert it to a string. (This is in contrast to the
top-level `regex` crate which intentionally provides no such API in order
to avoid making `regex-syntax` a public dependency.)

As a convenience, this builder may be created via [`Regex::builder`], which
may help avoid an extra import.

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change the
line terminator to the NUL byte:

```
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: disable UTF-8 requirement

By default, regex patterns are required to match UTF-8. This includes
regex patterns that can produce matches of length zero. In the case of an
empty match, by default, matches will not appear between the code units of
a UTF-8 encoded codepoint.

However, it can be useful to disable this requirement, particularly if
you're searching things like `&[u8]` that are not known to be valid UTF-8.

```
use regex_automata::{meta::Regex, util::syntax, Match};

let mut builder = Regex::builder();
// Disables the requirement that non-empty matches match UTF-8.
builder.syntax(syntax::Config::new().utf8(false));
// Disables the requirement that empty matches match UTF-8 boundaries.
builder.configure(Regex::config().utf8_empty(false));

// We can match raw bytes via \xZZ syntax, but we need to disable
// Unicode mode to do that. We could disable it everywhere, or just
// selectively, as shown here.
let re = builder.build(r"(?-u:\xFF)foo(?-u:\xFF)")?;
let hay = b"\xFFfoo\xFF";
assert_eq!(Some(Match::must(0, 0..5)), re.find(hay));

// We can also match between code units.
let re = builder.build(r"")?;
let hay = "☃";
assert_eq!(re.find_iter(hay).collect::<Vec<Match>>(), vec![
    Match::must(0, 0..0),
    Match::must(0, 1..1),
    Match::must(0, 2..2),
    Match::must(0, 3..3),
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Cache

`struct` · `regex_automata::meta::regex::Cache`

Also reachable as `regex_automata::meta::Cache`

```rust
struct Cache
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn memory_usage(&self) -> usize
fn new(re: &Regex) -> Cache
fn reset(&mut self, re: &Regex)
```

Represents mutable scratch space used by regex engines during a search.

Most of the regex engines in this crate require some kind of
mutable state in order to execute a search. This mutable state is
explicitly separated from the core regex object (such as a
[`thompson::NFA`](crate::nfa::thompson::NFA)) so that the read-only regex
object can be shared across multiple threads simultaneously without any
synchronization. Conversely, a `Cache` must either be duplicated if using
the same `Regex` from multiple threads, or else there must be some kind of
synchronization that guarantees exclusive access while it's in use by one
thread.

A `Regex` attempts to do this synchronization for you by using a thread
pool internally. Its size scales roughly with the number of simultaneous
regex searches.

For cases where one does not want to rely on a `Regex`'s internal thread
pool, lower level routines such as [`Regex::search_with`] are provided
that permit callers to pass a `Cache` into the search routine explicitly.

General advice is that the thread pool is often more than good enough.
However, it may be possible to observe the effects of its latency,
especially when searching many small haystacks from many threads
simultaneously.

Caches can be created from their corresponding `Regex` via
[`Regex::create_cache`]. A cache can only be used with either the `Regex`
that created it, or the `Regex` that was most recently used to reset it
with [`Cache::reset`]. Using a cache with any other `Regex` may result in
panics or incorrect results.

# Example

```
use regex_automata::{meta::Regex, Input, Match};

let re = Regex::new(r"(?-u)m\w+\s+m\w+")?;
let mut cache = re.create_cache();
let input = Input::new("crazy janey and her mission man");
assert_eq!(
    Some(Match::must(0, 20..31)),
    re.search_with(&mut cache, &input),
);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## CapturesMatches

`struct` · `regex_automata::meta::regex::CapturesMatches`

Also reachable as `regex_automata::meta::CapturesMatches`

```rust
struct CapturesMatches<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**Methods** (2)

```rust
fn input<'s>(&'s self) -> &'s Input<'h>
fn regex(&self) -> &'r Regex
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Captures>
```

An iterator over all non-overlapping leftmost matches with their capturing
groups.

The iterator yields a [`Captures`] value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`Regex::captures_iter`] method.

---

## Config

`struct` · `regex_automata::meta::regex::Config`

Also reachable as `regex_automata::meta::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (33)

```rust
fn auto_prefilter(self, yes: bool) -> Config
fn backtrack(self, yes: bool) -> Config
fn byte_classes(self, yes: bool) -> Config
fn dfa(self, yes: bool) -> Config
fn dfa_size_limit(self, limit: Option<usize>) -> Config
fn dfa_state_limit(self, limit: Option<usize>) -> Config
fn get_auto_prefilter(&self) -> bool
fn get_backtrack(&self) -> bool
fn get_byte_classes(&self) -> bool
fn get_dfa(&self) -> bool
fn get_dfa_size_limit(&self) -> Option<usize>
fn get_dfa_state_limit(&self) -> Option<usize>
fn get_hybrid(&self) -> bool
fn get_hybrid_cache_capacity(&self) -> usize
fn get_line_terminator(&self) -> u8
fn get_match_kind(&self) -> MatchKind
fn get_nfa_size_limit(&self) -> Option<usize>
fn get_onepass(&self) -> bool
fn get_onepass_size_limit(&self) -> Option<usize>
fn get_prefilter(&self) -> Option<&Prefilter>
fn get_utf8_empty(&self) -> bool
fn get_which_captures(&self) -> WhichCaptures
fn hybrid(self, yes: bool) -> Config
fn hybrid_cache_capacity(self, limit: usize) -> Config
fn line_terminator(self, byte: u8) -> Config
fn match_kind(self, kind: MatchKind) -> Config
fn new() -> Config
fn nfa_size_limit(self, limit: Option<usize>) -> Config
fn onepass(self, yes: bool) -> Config
fn onepass_size_limit(self, limit: Option<usize>) -> Config
fn prefilter(self, pre: Option<Prefilter>) -> Config
fn utf8_empty(self, yes: bool) -> Config
fn which_captures(self, which_captures: WhichCaptures) -> Config
```

An object describing the configuration of a `Regex`.

This configuration only includes options for the
non-syntax behavior of a `Regex`, and can be applied via the
[`Builder::configure`] method. For configuring the syntax options, see
[`util::syntax::Config`](crate::util::syntax::Config).

# Example: lower the NFA size limit

In some cases, the default size limit might be too big. The size limit can
be lowered, which will prevent large regex patterns from compiling.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::meta::Regex;

let result = Regex::builder()
    .configure(Regex::config().nfa_size_limit(Some(20 * (1<<10))))
    // Not even 20KB is enough to build a single large Unicode class!
    .build(r"\pL");
assert!(result.is_err());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## FindMatches

`struct` · `regex_automata::meta::regex::FindMatches`

Also reachable as `regex_automata::meta::FindMatches`

```rust
struct FindMatches<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**Methods** (2)

```rust
fn input<'s>(&'s self) -> &'s Input<'h>
fn regex(&self) -> &'r Regex
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<Match>
```

An iterator over all non-overlapping matches.

The iterator yields a [`Match`] value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`Regex::find_iter`] method.

---

## Regex

`struct` · `regex_automata::meta::regex::Regex`

Also reachable as `regex_automata::meta::Regex`

```rust
struct Regex
```

**Derives**: Clone, Debug

**Methods** (30)

```rust
fn builder() -> Builder
fn captures<'h, I: Into<Input<'h>>>(&self, input: I, caps: &mut Captures)
fn captures_iter<'r, 'h, I: Into<Input<'h>>>(&'r self, input: I) -> CapturesMatches<'r, 'h>
fn captures_len(&self) -> usize
fn config() -> Config
fn create_cache(&self) -> Cache
fn create_captures(&self) -> Captures
fn find<'h, I: Into<Input<'h>>>(&self, input: I) -> Option<Match>
fn find_iter<'r, 'h, I: Into<Input<'h>>>(&'r self, input: I) -> FindMatches<'r, 'h>
fn get_config(&self) -> &Config
fn group_info(&self) -> &GroupInfo
fn is_accelerated(&self) -> bool
fn is_match<'h, I: Into<Input<'h>>>(&self, input: I) -> bool
fn memory_usage(&self) -> usize
fn new(pattern: &str) -> Result<Regex, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>
fn pattern_len(&self) -> usize
fn search(&self, input: &Input<'_>) -> Option<Match>
fn search_captures(&self, input: &Input<'_>, caps: &mut Captures)
fn search_captures_with(&self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures)
fn search_half(&self, input: &Input<'_>) -> Option<HalfMatch>
fn search_half_with(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>
fn search_slots(&self, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>
fn search_slots_with(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>
fn search_with(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>
fn split<'r, 'h, I: Into<Input<'h>>>(&'r self, input: I) -> Split<'r, 'h>
fn splitn<'r, 'h, I: Into<Input<'h>>>(&'r self, input: I, limit: usize) -> SplitN<'r, 'h>
fn static_captures_len(&self) -> Option<usize>
fn which_overlapping_matches(&self, input: &Input<'_>, patset: &mut PatternSet)
fn which_overlapping_matches_with(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)
```

A regex matcher that works by composing several other regex matchers
automatically.

In effect, a meta regex papers over a lot of the quirks or performance
problems in each of the regex engines in this crate. Its goal is to provide
an infallible and simple API that "just does the right thing" in the common
case.

A meta regex is the implementation of a `Regex` in the `regex` crate.
Indeed, the `regex` crate API is essentially just a light wrapper over
this type. This includes the `regex` crate's `RegexSet` API!

# Composition

This is called a "meta" matcher precisely because it uses other regex
matchers to provide a convenient high level regex API. Here are some
examples of how other regex matchers are composed:

* When calling [`Regex::captures`], instead of immediately
running a slower but more capable regex engine like the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM), the meta regex engine
will usually first look for the bounds of a match with a higher throughput
regex engine like a [lazy DFA](crate::hybrid). Only when a match is found
is a slower engine like `PikeVM` used to find the matching span for each
capture group.
* While higher throughout engines like the lazy DFA cannot handle
Unicode word boundaries in general, they can still be used on pure ASCII
haystacks by pretending that Unicode word boundaries are just plain ASCII
word boundaries. However, if a haystack is not ASCII, the meta regex engine
will automatically switch to a (possibly slower) regex engine that supports
Unicode word boundaries in general.
* In some cases where a regex pattern is just a simple literal or a small
set of literals, an actual regex engine won't be used at all. Instead,
substring or multi-substring search algorithms will be employed.

There are many other forms of composition happening too, but the above
should give a general idea. In particular, it may perhaps be surprising
that *multiple* regex engines might get executed for a single search. That
is, the decision of what regex engine to use is not _just_ based on the
pattern, but also based on the dynamic execution of the search itself.

The primary reason for this composition is performance. The fundamental
tension is that the faster engines tend to be less capable, and the more
capable engines tend to be slower.

Note that the forms of composition that are allowed are determined by
compile time crate features and configuration. For example, if the `hybrid`
feature isn't enabled, or if [`Config::hybrid`] has been disabled, then the
meta regex engine will never use a lazy DFA.

# Synchronization and cloning

Most of the regex engines in this crate require some kind of mutable
"scratch" space to read and write from while performing a search. Since
a meta regex composes these regex engines, a meta regex also requires
mutable scratch space. This scratch space is called a [`Cache`].

Most regex engines _also_ usually have a read-only component, typically
a [Thompson `NFA`](crate::nfa::thompson::NFA).

In order to make the `Regex` API convenient, most of the routines hide
the fact that a `Cache` is needed at all. To achieve this, a [memory
pool](crate::util::pool::Pool) is used internally to retrieve `Cache`
values in a thread safe way that also permits reuse. This in turn implies
that every such search call requires some form of synchronization. Usually
this synchronization is fast enough to not notice, but in some cases, it
can be a bottleneck. This typically occurs when all of the following are
true:

* The same `Regex` is shared across multiple threads simultaneously,
usually via a [`util::lazy::Lazy`](crate::util::lazy::Lazy) or something
similar from the `once_cell` or `lazy_static` crates.
* The primary unit of work in each thread is a regex search.
* Searches are run on very short haystacks.

This particular case can lead to high contention on the pool used by a
`Regex` internally, which can in turn increase latency to a noticeable
effect. This cost can be mitigated in one of the following ways:

* Use a distinct copy of a `Regex` in each thread, usually by cloning it.
Cloning a `Regex` _does not_ do a deep copy of its read-only component.
But it does lead to each `Regex` having its own memory pool, which in
turn eliminates the problem of contention. In general, this technique should
not result in any additional memory usage when compared to sharing the same
`Regex` across multiple threads simultaneously.
* Use lower level APIs, like [`Regex::search_with`], which permit passing
a `Cache` explicitly. In this case, it is up to you to determine how best
to provide a `Cache`. For example, you might put a `Cache` in thread-local
storage if your use case allows for it.

Overall, this is an issue that happens rarely in practice, but it can
happen.

# Warning: spin-locks may be used in alloc-only mode

When this crate is built without the `std` feature and the high level APIs
on a `Regex` are used, then a spin-lock will be used to synchronize access
to an internal pool of `Cache` values. This may be undesirable because
a spin-lock is [effectively impossible to implement correctly in user
space][spinlocks-are-bad]. That is, more concretely, the spin-lock could
result in a deadlock.

[spinlocks-are-bad]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html

If one wants to avoid the use of spin-locks when the `std` feature is
disabled, then you must use APIs that accept a `Cache` value explicitly.
For example, [`Regex::search_with`].

# Example

```
use regex_automata::meta::Regex;

let re = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$")?;
assert!(re.is_match("2010-03-14"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: anchored search

This example shows how to use [`Input::anchored`] to run an anchored
search, even when the regex pattern itself isn't anchored. An anchored
search guarantees that if a match is found, then the start offset of the
match corresponds to the offset at which the search was started.

```
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"\bfoo\b")?;
let input = Input::new("xx foo xx").range(3..).anchored(Anchored::Yes);
// The offsets are in terms of the original haystack.
assert_eq!(Some(Match::must(0, 3..6)), re.find(input));

// Notice that no match occurs here, because \b still takes the
// surrounding context into account, even if it means looking back
// before the start of your search.
let hay = "xxfoo xx";
let input = Input::new(hay).range(2..).anchored(Anchored::Yes);
assert_eq!(None, re.find(input));
// Indeed, you cannot achieve the above by simply slicing the
// haystack itself, since the regex engine can't see the
// surrounding context. This is why 'Input' permits setting
// the bounds of a search!
let input = Input::new(&hay[2..]).anchored(Anchored::Yes);
// WRONG!
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: earliest search

This example shows how to use [`Input::earliest`] to run a search that
might stop before finding the typical leftmost match.

```
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"[a-z]{3}|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 1..2)), re.find(input));

// Note that "earliest" isn't really a match semantic unto itself.
// Instead, it is merely an instruction to whatever regex engine
// gets used internally to quit as soon as it can. For example,
// this regex uses a different search technique, and winds up
// producing a different (but valid) match!
let re = Regex::new(r"abc|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change
the line terminator to the NUL byte:

```
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Split

`struct` · `regex_automata::meta::regex::Split`

Also reachable as `regex_automata::meta::Split`

```rust
struct Split<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**Methods** (1)

```rust
fn input<'s>(&'s self) -> &'s Input<'h>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Span>
```

Yields all substrings delimited by a regular expression match.

The spans correspond to the offsets between matches.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`Regex::split`] method.

---

## SplitN

`struct` · `regex_automata::meta::regex::SplitN`

Also reachable as `regex_automata::meta::SplitN`

```rust
struct SplitN<'r, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**Methods** (1)

```rust
fn input<'s>(&'s self) -> &'s Input<'h>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Span>
fn size_hint(&self) -> (usize, Option<usize>)
```

Yields at most `N` spans delimited by a regular expression match.

The spans correspond to the offsets between matches. The last span will be
whatever remains after splitting.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`Regex::splitn`] method.

---
