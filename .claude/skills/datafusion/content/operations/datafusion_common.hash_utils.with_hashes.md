# `datafusion_common::hash_utils::with_hashes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.with_hashes.json).

<a id="op-31c70923996226d4d1940a34"></a>
## with_hashes

`function` · `datafusion_common::hash_utils::with_hashes` · datafusion-common 55.1.0

```rust
fn with_hashes<I, T, F, R>(arrays: I, random_state: &impl HashState, callback: F) -> error::Result<R> where I: IntoIterator<Item = T>, T: AsDynArray, F: FnOnce(&[u64]) -> error::Result<R>
```

Source: `src/hash_utils.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

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
