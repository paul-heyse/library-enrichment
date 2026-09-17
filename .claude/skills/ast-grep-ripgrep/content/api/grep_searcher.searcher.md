# `grep_searcher::searcher`

Crate `grep-searcher` · 5 public items · structured records in [`model/grep_searcher.searcher.json`](../model/grep_searcher.searcher.json)

## ConfigError

`enum` · `grep_searcher::searcher::ConfigError`

Also reachable as `grep_searcher::ConfigError`

```rust
enum ConfigError
```

**Variants**: `SearchUnavailable`, `MismatchedLineTerminators`, `UnknownEncoding`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An error that can occur when building a searcher.

This error occurs when a non-sensical configuration is present when trying
to construct a `Searcher` from a `SearcherBuilder`.

---

## BinaryDetection

`struct` · `grep_searcher::searcher::BinaryDetection`

Also reachable as `grep_searcher::BinaryDetection`

```rust
struct BinaryDetection
```

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn convert(binary_byte: u8) -> BinaryDetection
fn convert_byte(&self) -> Option<u8>
fn none() -> BinaryDetection
fn quit(binary_byte: u8) -> BinaryDetection
fn quit_byte(&self) -> Option<u8>
```

The behavior of binary detection while searching.

Binary detection is the process of _heuristically_ identifying whether a
given chunk of data is binary or not, and then taking an action based on
the result of that heuristic. The motivation behind detecting binary data
is that binary data often indicates data that is undesirable to search
using textual patterns. Of course, there are many cases in which this isn't
true, which is why binary detection is disabled by default.

Unfortunately, binary detection works differently depending on the type of
search being executed:

1. When performing a search using a fixed size buffer, binary detection is
   applied to the buffer's contents as it is filled. Binary detection must
   be applied to the buffer directly because binary files may not contain
   line terminators, which could result in exorbitant memory usage.
2. When performing a search using memory maps or by reading data off the
   heap, then binary detection is only guaranteed to be applied to the
   parts corresponding to a match. When `Quit` is enabled, then the first
   few KB of the data are searched for binary data.

---

## Encoding

`struct` · `grep_searcher::searcher::Encoding`

Also reachable as `grep_searcher::Encoding`

```rust
struct Encoding
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(label: &str) -> Result<Encoding, ConfigError>
```

An encoding to use when searching.

An encoding can be used to configure a [`SearcherBuilder`] to transcode
source data from an encoding to UTF-8 before searching.

An `Encoding` will always be cheap to clone.

---

## Searcher

`struct` · `grep_searcher::searcher::Searcher`

Also reachable as `grep_searcher::Searcher`

```rust
struct Searcher
```

**Derives**: Clone, Debug

**Methods** (17)

```rust
fn after_context(&self) -> usize
fn before_context(&self) -> usize
fn binary_detection(&self) -> &BinaryDetection
fn invert_match(&self) -> bool
fn line_number(&self) -> bool
fn line_terminator(&self) -> LineTerminator
fn max_matches(&self) -> Option<u64>
fn multi_line(&self) -> bool
fn multi_line_with_matcher<M: Matcher>(&self, matcher: M) -> bool
fn new() -> Searcher
fn passthru(&self) -> bool
fn search_file<M, S>(&mut self, matcher: M, file: &File, write_to: S) -> Result<(), S::Error> where M: Matcher, S: Sink
fn search_path<P, M, S>(&mut self, matcher: M, path: P, write_to: S) -> Result<(), S::Error> where P: AsRef<Path>, M: Matcher, S: Sink
fn search_reader<M, R, S>(&mut self, matcher: M, read_from: R, write_to: S) -> Result<(), S::Error> where M: Matcher, R: io::Read, S: Sink
fn search_slice<M, S>(&mut self, matcher: M, slice: &[u8], write_to: S) -> Result<(), S::Error> where M: Matcher, S: Sink
fn set_binary_detection(&mut self, detection: BinaryDetection)
fn stop_on_nonmatch(&self) -> bool
```

A searcher executes searches over a haystack and writes results to a caller
provided sink.

Matches are detected via implementations of the `Matcher` trait, which must
be provided by the caller when executing a search.

When possible, a searcher should be reused.

---

## SearcherBuilder

`struct` · `grep_searcher::searcher::SearcherBuilder`

Also reachable as `grep_searcher::SearcherBuilder`

```rust
struct SearcherBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (16)

```rust
fn after_context(&mut self, line_count: usize) -> &mut SearcherBuilder
fn before_context(&mut self, line_count: usize) -> &mut SearcherBuilder
fn binary_detection(&mut self, detection: BinaryDetection) -> &mut SearcherBuilder
fn bom_sniffing(&mut self, yes: bool) -> &mut SearcherBuilder
fn build(&self) -> Searcher
fn encoding(&mut self, encoding: Option<Encoding>) -> &mut SearcherBuilder
fn heap_limit(&mut self, bytes: Option<usize>) -> &mut SearcherBuilder
fn invert_match(&mut self, yes: bool) -> &mut SearcherBuilder
fn line_number(&mut self, yes: bool) -> &mut SearcherBuilder
fn line_terminator(&mut self, line_term: LineTerminator) -> &mut SearcherBuilder
fn max_matches(&mut self, limit: Option<u64>) -> &mut SearcherBuilder
fn memory_map(&mut self, strategy: MmapChoice) -> &mut SearcherBuilder
fn multi_line(&mut self, yes: bool) -> &mut SearcherBuilder
fn new() -> SearcherBuilder
fn passthru(&mut self, yes: bool) -> &mut SearcherBuilder
fn stop_on_nonmatch(&mut self, stop_on_nonmatch: bool) -> &mut SearcherBuilder
```

A builder for configuring a searcher.

A search builder permits specifying the configuration of a searcher,
including options like whether to invert the search or to enable multi
line search.

Once a searcher has been built, it is beneficial to reuse that searcher
for multiple searches, if possible.

---
