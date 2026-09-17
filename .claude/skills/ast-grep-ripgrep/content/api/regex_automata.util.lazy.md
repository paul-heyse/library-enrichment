# `regex_automata::util::lazy`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.util.lazy.json`](../model/regex_automata.util.lazy.json)

## Lazy

`struct` · `regex_automata::util::lazy::Lazy`

```rust
struct Lazy<T, F = fn() -> T>
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Debug

**Methods** (2)

```rust
fn get(this: &Lazy<T, F>) -> &T
const fn new(create: F) -> Lazy<T, F>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &T
```

A lazily initialized value that implements `Deref` for `T`.

A `Lazy` takes an initialization function and permits callers from any
thread to access the result of that initialization function in a safe
manner. In effect, this permits one-time initialization of global resources
in a (possibly) multi-threaded program.

This type and its functionality are available even when neither the `alloc`
nor the `std` features are enabled. In exchange, a `Lazy` does **not**
guarantee that the given `create` function is called at most once. It
might be called multiple times. Moreover, a call to `Lazy::get` (either
explicitly or implicitly via `Lazy`'s `Deref` impl) may block until a `T`
is available.

This is very similar to `lazy_static` or `once_cell`, except it doesn't
guarantee that the initialization function will be run once and it works
in no-alloc no-std environments. With that said, if you need stronger
guarantees or a more flexible API, then it is recommended to use either
`lazy_static` or `once_cell`.

# Warning: may use a spin lock

When this crate is compiled _without_ the `alloc` feature, then this type
may used a spin lock internally. This can have subtle effects that may
be undesirable. See [Spinlocks Considered Harmful][spinharm] for a more
thorough treatment of this topic.

[spinharm]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html

# Example

This type is useful for creating regexes once, and then using them from
multiple threads simultaneously without worrying about synchronization.

```
use regex_automata::{dfa::regex::Regex, util::lazy::Lazy, Match};

static RE: Lazy<Regex> = Lazy::new(|| Regex::new("foo[0-9]+bar").unwrap());

let expected = Some(Match::must(0, 3..14));
assert_eq!(expected, RE.find(b"zzzfoo12345barzzz"));
```

---
