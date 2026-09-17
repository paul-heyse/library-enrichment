# `regex_automata::nfa::thompson::compiler`

Crate `regex-automata` · 3 public items · structured records in [`model/regex_automata.nfa.thompson.compiler.json`](../model/regex_automata.nfa.thompson.compiler.json)

## WhichCaptures

`enum` · `regex_automata::nfa::thompson::compiler::WhichCaptures`

Also reachable as `regex_automata::nfa::thompson::WhichCaptures`

```rust
enum WhichCaptures
```

**Variants**: `All`, `Implicit`, `None`

**Derives**: Clone, Copy, Debug, Default

**Methods** (2)

```rust
fn is_any(&self) -> bool
fn is_none(&self) -> bool
```

A configuration indicating which kinds of
[`State::Capture`](crate::nfa::thompson::State::Capture) states to include.

This configuration can be used with [`Config::which_captures`] to control
which capture states are compiled into a Thompson NFA.

The default configuration is [`WhichCaptures::All`].

---

## Compiler

`struct` · `regex_automata::nfa::thompson::compiler::Compiler`

Also reachable as `regex_automata::nfa::thompson::Compiler`

```rust
struct Compiler
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<NFA, BuildError>
fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<NFA, BuildError>
fn build_many_from_hir<H: Borrow<Hir>>(&self, exprs: &[H]) -> Result<NFA, BuildError>
fn configure(&mut self, config: Config) -> &mut Compiler
fn new() -> Compiler
fn syntax(&mut self, config: util::syntax::Config) -> &mut Compiler
```

A builder for compiling an NFA from a regex's high-level intermediate
representation (HIR).

This compiler provides a way to translate a parsed regex pattern into an
NFA state graph. The NFA state graph can either be used directly to execute
a search (e.g., with a Pike VM), or it can be further used to build a DFA.

This compiler provides APIs both for compiling regex patterns directly from
their concrete syntax, or via a [`regex_syntax::hir::Hir`].

This compiler has various options that may be configured via
[`thompson::Config`](Config).

Note that a compiler is not the same as a [`thompson::Builder`](Builder).
A `Builder` provides a lower level API that is uncoupled from a regex
pattern's concrete syntax or even its HIR. Instead, it permits stitching
together an NFA by hand. See its docs for examples.

# Example: compilation from concrete syntax

This shows how to compile an NFA from a pattern string while setting a size
limit on how big the NFA is allowed to be (in terms of bytes of heap used).

```
use regex_automata::{
    nfa::thompson::{NFA, pikevm::PikeVM},
    Match,
};

let config = NFA::config().nfa_size_limit(Some(1_000));
let nfa = NFA::compiler().configure(config).build(r"(?-u)\w")?;

let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 3..4));
re.captures(&mut cache, "!@#A#@!", &mut caps);
assert_eq!(expected, caps.get_match());

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: compilation from HIR

This shows how to hand assemble a regular expression via its HIR, and then
compile an NFA directly from it.

```
use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};
use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
    ClassBytesRange::new(b'0', b'9'),
    ClassBytesRange::new(b'A', b'Z'),
    ClassBytesRange::new(b'_', b'_'),
    ClassBytesRange::new(b'a', b'z'),
])));

let config = NFA::config().nfa_size_limit(Some(1_000));
let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 3..4));
re.captures(&mut cache, "!@#A#@!", &mut caps);
assert_eq!(expected, caps.get_match());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Config

`struct` · `regex_automata::nfa::thompson::compiler::Config`

Also reachable as `regex_automata::nfa::thompson::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (15)

```rust
fn captures(self, yes: bool) -> Config
fn get_captures(&self) -> bool
fn get_look_matcher(&self) -> LookMatcher
fn get_nfa_size_limit(&self) -> Option<usize>
fn get_reverse(&self) -> bool
fn get_shrink(&self) -> bool
fn get_utf8(&self) -> bool
fn get_which_captures(&self) -> WhichCaptures
fn look_matcher(self, m: LookMatcher) -> Config
fn new() -> Config
fn nfa_size_limit(self, bytes: Option<usize>) -> Config
fn reverse(self, yes: bool) -> Config
fn shrink(self, yes: bool) -> Config
fn utf8(self, yes: bool) -> Config
fn which_captures(self, which_captures: WhichCaptures) -> Config
```

The configuration used for a Thompson NFA compiler.

---
