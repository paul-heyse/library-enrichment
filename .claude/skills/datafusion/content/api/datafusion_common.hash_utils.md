# `datafusion_common::hash_utils`

Crate `datafusion-common` · 11 public items · structured records in [`model/datafusion_common.hash_utils.json`](../model/datafusion_common.hash_utils.json)

## HLL_RANDOM_STATE

`constant` · `datafusion_common::hash_utils::HLL_RANDOM_STATE`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::HLL_RANDOM_STATE`, `datafusion_physical_plan::hash_utils::HLL_RANDOM_STATE`

```rust
const HLL_RANDOM_STATE: QualityRandomState = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.HLL_RANDOM_STATE.md).


Fixed quality hash state used by HyperLogLog sketches.

The seed is part of the HLL wire/storage semantics: serialized sketches only
remain mergeable if every producer uses the same hash state.

---

## combine_hashes

`function` · `datafusion_common::hash_utils::combine_hashes`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::combine_hashes`, `datafusion_physical_plan::hash_utils::combine_hashes`

```rust
fn combine_hashes(l: u64, r: u64) -> u64
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.combine_hashes.md).


---

## create_hashes

`function` · `datafusion_common::hash_utils::create_hashes`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::create_hashes`, `datafusion_physical_plan::hash_utils::create_hashes`

```rust
fn create_hashes<'a, I, T>(arrays: I, random_state: &impl HashState, hashes_buffer: &'a mut [u64]) -> error::Result<&'a mut [u64]> where I: IntoIterator<Item = T>, T: AsDynArray
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.create_hashes.md).


Creates hash values for every row, based on the values in the columns.

The number of rows to hash is determined by `hashes_buffer.len()`.
`hashes_buffer` should be pre-sized appropriately.

---

## create_hashes_with_hasher

`function` · `datafusion_common::hash_utils::create_hashes_with_hasher`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::create_hashes_with_hasher`, `datafusion_physical_plan::hash_utils::create_hashes_with_hasher`

```rust
fn create_hashes_with_hasher<'a, I, T, S>(arrays: I, hash_builder: &S, hashes_buffer: &'a mut [u64]) -> error::Result<&'a mut [u64]> where I: IntoIterator<Item = T>, T: AsDynArray, S: BuildHasher
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.create_hashes_with_hasher.md).


Creates hash values for every row using a caller-provided hash builder.

The number of rows to hash is determined by `hashes_buffer.len()`.
`hashes_buffer` should be pre-sized appropriately.

# Hash compatibility

Hash values are not guaranteed to be bit-for-bit identical to those from
[`create_hashes`], even when `hash_builder` also implements [`HashState`].
The optimized [`HashState`] path seeds the hasher from the previous hash
when rehashing some primitive and byte-view values, whereas this function
combines independently computed hashes. Use one API consistently if hashes
are persisted or exchanged.

---

## with_hashes

`function` · `datafusion_common::hash_utils::with_hashes`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::with_hashes`, `datafusion_physical_plan::hash_utils::with_hashes`

```rust
fn with_hashes<I, T, F, R>(arrays: I, random_state: &impl HashState, callback: F) -> error::Result<R> where I: IntoIterator<Item = T>, T: AsDynArray, F: FnOnce(&[u64]) -> error::Result<R>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.with_hashes.md).


Creates hashes for the given arrays using a thread-local buffer, then calls the provided callback
with an immutable reference to the computed hashes.

This function manages a thread-local buffer to avoid repeated allocations. The buffer is automatically
truncated if it exceeds `MAX_BUFFER_SIZE` after use.

# Arguments
* `arrays` - The arrays to hash (must contain at least one array)
* `random_state` - The random state for hashing
* `callback` - A function that receives an immutable reference to the hash slice and returns a result

# Errors
Returns an error if:
- No arrays are provided
- The function is called reentrantly (i.e., the callback invokes `with_hashes` again on the same thread)
- The function is called during or after thread destruction

# Example
```ignore
use datafusion_common::hash_utils::{with_hashes, RandomState};
use arrow::array::{Int32Array, ArrayRef};
use std::sync::Arc;

let array: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3]));
let random_state = RandomState::default();

let result = with_hashes([&array], &random_state, |hashes| {
    // Use the hashes here
    Ok(hashes.len())
})?;
```

---

## with_hashes_with_hasher

`function` · `datafusion_common::hash_utils::with_hashes_with_hasher`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::with_hashes_with_hasher`, `datafusion_physical_plan::hash_utils::with_hashes_with_hasher`

```rust
fn with_hashes_with_hasher<I, T, F, R, S>(arrays: I, hash_builder: &S, callback: F) -> error::Result<R> where I: IntoIterator<Item = T>, T: AsDynArray, F: FnOnce(&[u64]) -> error::Result<R>, S: BuildHasher
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.with_hashes_with_hasher.md).


Creates hashes for the given arrays using a thread-local buffer and a custom
hash builder, then calls the provided callback with the computed hashes.

Hash compatibility with [`with_hashes`] follows the rules documented on
[`create_hashes_with_hasher`].

---

## AsDynArray

`trait` · `datafusion_common::hash_utils::AsDynArray`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::AsDynArray`, `datafusion_physical_plan::hash_utils::AsDynArray`

```rust
trait AsDynArray
```

**Implementors** (1)

- `arrow_array::array::ArrayRef`

**Methods** (1)

```rust
fn as_dyn_array(&self) -> &dyn Array
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.AsDynArray.md).


Something that can be returned as a `&dyn Array`.

We want `create_hashes` to accept either `&dyn Array` or `ArrayRef`,
and this seems the best way to do so.

We tried having it accept `AsRef<dyn Array>`
but that is not implemented for and cannot be implemented for
`&dyn Array` so callers that have the latter would not be able
to call `create_hashes` directly. This shim trait makes it possible.

---

## HashState

`trait` · `datafusion_common::hash_utils::HashState`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::HashState`, `datafusion_physical_plan::hash_utils::HashState`

```rust
trait HashState: BuildHasher
```

**Implementors** (2)

- `foldhash::fast::FixedState`
- `foldhash::quality::FixedState`

**Methods** (1)

```rust
fn seeded_state(&self, seed: u64) -> Self::SeededState
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.HashState.md).


Hash state used by [`create_hashes`].

Multi-column hashing folds the previous column hash into a fresh hasher
before hashing the next column. This trait keeps that seeded hasher in the
same foldhash tier as the top-level hash state.

---

## HashValue

`trait` · `datafusion_common::hash_utils::HashValue`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::HashValue`, `datafusion_physical_plan::hash_utils::HashValue`

```rust
trait HashValue
```

**Implementors** (4)

- `arrow_buffer::bigint::i256`
- `arrow_buffer::interval::IntervalDayTime`
- `arrow_buffer::interval::IntervalMonthDayNano`
- `half::binary16::f16`

**Methods** (2)

```rust
fn hash_one<S: BuildHasher>(&self, state: &S) -> u64
fn hash_write(&self, hasher: &mut impl Hasher)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.HashValue.md).


---

## QualityRandomState

`type_alias` · `datafusion_common::hash_utils::QualityRandomState`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::QualityRandomState`, `datafusion_physical_plan::hash_utils::QualityRandomState`

```rust
type QualityRandomState = foldhash::quality::FixedState
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.QualityRandomState.md).


---

## RandomState

`type_alias` · `datafusion_common::hash_utils::RandomState`

Also reachable as `datafusion_physical_plan::execution_plan::hash_utils::RandomState`, `datafusion_physical_plan::hash_utils::RandomState`

```rust
type RandomState = foldhash::fast::FixedState
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.hash_utils.RandomState.md).


[`RandomState`] is optimized for speed and suitable for hash tables and
bloom filters. [`QualityRandomState`] is optimized for statistical quality
and suitable for algorithms such as HyperLogLog. The tradeoff is that the
fast variant gives up some statistical quality, while the quality variant
is slightly slower.

See: <https://docs.rs/foldhash/0.2.0/src/foldhash/lib.rs.html#17-21>

---
