# `object_store::util`

Crate `object_store` · 5 public items · structured records in [`model/object_store.util.json`](../model/object_store.util.json)

## OBJECT_STORE_COALESCE_DEFAULT

`constant` · `object_store::util::OBJECT_STORE_COALESCE_DEFAULT`

Also reachable as `datafusion::object_store::OBJECT_STORE_COALESCE_DEFAULT`, `object_store::OBJECT_STORE_COALESCE_DEFAULT`

```rust
const OBJECT_STORE_COALESCE_DEFAULT: u64 = _
```

Range requests with a gap less than or equal to this,
will be coalesced into a single request by [`coalesce_ranges`]

---

## GetRange

`enum` · `object_store::util::GetRange`

Also reachable as `datafusion::object_store::GetRange`, `object_store::GetRange`

```rust
enum GetRange
```

**Variants**: `Bounded`, `Offset`, `Suffix`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn as_range(&self, len: u64) -> Result<Range<u64>, InvalidGetRange>
fn is_valid(&self) -> Result<(), InvalidGetRange>
```

**via `core::convert::From`**

```rust
fn from(value: T) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Request only a portion of an object's bytes

These can be created from [usize] ranges, like

```rust
# use object_store::GetRange;
let range1: GetRange = (50..150).into();
let range2: GetRange = (50..=150).into();
let range3: GetRange = (50..).into();
let range4: GetRange = (..150).into();
```

Implementations may wish to inspect [`GetResult`] for the exact byte
range returned.

[`GetResult`]: crate::GetResult

---

## InvalidGetRange

`enum` · `object_store::util::InvalidGetRange`

```rust
enum InvalidGetRange
```

**Variants**: `StartTooLarge`, `Inconsistent`, `TooLarge`

---

## coalesce_ranges

`function` · `object_store::util::coalesce_ranges`

Also reachable as `datafusion::object_store::coalesce_ranges`, `object_store::coalesce_ranges`

```rust
async fn coalesce_ranges<F, E, Fut>(ranges: &[std::ops::Range<u64>], fetch: F, coalesce: u64) -> super::Result<Vec<bytes::Bytes>, E> where F: Send + FnMut(std::ops::Range<u64>) -> Fut, E: Send, Fut: std::future::Future<Output = super::Result<bytes::Bytes, E>> + Send
```

Takes a function `fetch` that can fetch a range of bytes and uses this to
fetch the provided byte `ranges`

To improve performance it will:

* Combine ranges less than `coalesce` bytes apart into a single call to `fetch`
* Make multiple `fetch` requests in parallel (up to maximum of 10)

---

## collect_bytes

`function` · `object_store::util::collect_bytes`

Also reachable as `datafusion::object_store::collect_bytes`, `object_store::collect_bytes`

```rust
async fn collect_bytes<S, E>(stream: S, size_hint: Option<u64>) -> super::Result<bytes::Bytes, E> where E: Send, S: Stream<Item = super::Result<bytes::Bytes, E>> + Send + Unpin
```

Collect a stream into [`Bytes`] avoiding copying in the event of a single chunk

---
