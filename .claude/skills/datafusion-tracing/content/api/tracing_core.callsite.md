# `tracing_core::callsite`

Crate `tracing-core` · 5 public items · structured records in [`model/tracing_core.callsite.json`](../model/tracing_core.callsite.json)

## rebuild_interest_cache

`function` · `tracing_core::callsite::rebuild_interest_cache`

```rust
fn rebuild_interest_cache()
```

Clear and reregister interest on every [`Callsite`]

This function is intended for runtime reconfiguration of filters on traces
when the filter recalculation is much less frequent than trace events are.
The alternative is to have the [`Subscriber`] that supports runtime
reconfiguration of filters always return [`Interest::sometimes()`] so that
[`enabled`] is evaluated for every event.

This function will also re-compute the global maximum level as determined by
the [`max_level_hint`] method. If a [`Subscriber`]
implementation changes the value returned by its `max_level_hint`
implementation at runtime, then it **must** call this function after that
value changes, in order for the change to be reflected.

See the [documentation on callsite interest caching][cache-docs] for
additional information on this function's usage.

[`max_level_hint`]: super::subscriber::Subscriber::max_level_hint
[`Callsite`]: super::callsite::Callsite
[`enabled`]: super::subscriber::Subscriber#tymethod.enabled
[`Interest::sometimes()`]: super::subscriber::Interest::sometimes
[`Subscriber`]: super::subscriber::Subscriber
[cache-docs]: crate::callsite#rebuilding-cached-interest

---

## register

`function` · `tracing_core::callsite::register`

```rust
fn register(callsite: &'static dyn Callsite)
```

Register a new [`Callsite`] with the global registry.

This should be called once per callsite after the callsite has been
constructed.

See the [documentation on callsite registration][reg-docs] for details
on the global callsite registry.

[`Callsite`]: crate::callsite::Callsite
[reg-docs]: crate::callsite#registering-callsites

---

## DefaultCallsite

`struct` · `tracing_core::callsite::DefaultCallsite`

```rust
struct DefaultCallsite
```

**Implements**: `tracing_core::callsite::Callsite`

**Derives**: Debug

**Methods** (3)

```rust
fn interest(&'static self) -> Interest
const fn new(meta: &'static Metadata<'static>) -> Self
fn register(&'static self) -> Interest
```

**via `tracing_core::callsite::Callsite`**

```rust
fn metadata(&self) -> &Metadata<'static>
fn set_interest(&self, interest: Interest)
```

A default [`Callsite`] implementation.

---

## Identifier

`struct` · `tracing_core::callsite::Identifier`

```rust
struct Identifier
```

**Derives**: Clone, Debug, Eq, Hash, PartialEq

Uniquely identifies a [`Callsite`]

Two `Identifier`s are equal if they both refer to the same callsite.

[`Callsite`]: super::callsite::Callsite

---

## Callsite

`trait` · `tracing_core::callsite::Callsite`

Also reachable as `tracing_core::Callsite`

```rust
trait Callsite: Sync
```

**Implementors** (1)

- `tracing_core::callsite::DefaultCallsite`

**Methods** (2)

```rust
fn metadata(&self) -> &Metadata<'_>
fn set_interest(&self, interest: Interest)
```

Trait implemented by callsites.

These functions are only intended to be called by the callsite registry, which
correctly handles determining the common interest between all subscribers.

See the [module-level documentation](crate::callsite) for details on
callsites.

---
