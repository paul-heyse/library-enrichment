# `datafusion_execution::cache::lru_queue::LruQueue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.lru_queue.LruQueue.json).

<a id="op-fcc52d1bc812f8a2cb2cea92"></a>
## LruQueue

`struct` · `datafusion_execution::cache::lru_queue::LruQueue` · datafusion-execution 55.1.0

```rust
struct LruQueue<K: Eq + Hash + Clone, V>
```

Source: `src/cache/lru_queue.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Provides a Least Recently Used queue with unbounded capacity.

# Examples

```
use datafusion_execution::cache::lru_queue::LruQueue;

let mut lru_queue: LruQueue<i32, i32> = LruQueue::new();
lru_queue.put(1, 10);
lru_queue.put(2, 20);
lru_queue.put(3, 30);
assert_eq!(lru_queue.get(&2), Some(&20));
assert_eq!(lru_queue.pop(), Some((1, 10)));
assert_eq!(lru_queue.pop(), Some((3, 30)));
assert_eq!(lru_queue.pop(), Some((2, 20)));
assert_eq!(lru_queue.pop(), None);
```

<a id="op-1e34d388e3a19f4eec4681aa"></a>
## clear

`function` · `datafusion_execution::cache::lru_queue::LruQueue::clear` · datafusion-execution 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Removes all entries from the queue.

<a id="op-e9f33a07a4ca27ece4f079d2"></a>
## contains_key

`function` · `datafusion_execution::cache::lru_queue::LruQueue::contains_key` · datafusion-execution 55.1.0

```rust
fn contains_key(&self, key: &K) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Checks whether there is an entry with key `key` in the queue.
Does not affect the queue order.

<a id="op-d4d9dd2a856fa8ee011157a4"></a>
## default

`function` · `datafusion_execution::cache::lru_queue::LruQueue::default` · datafusion-execution 55.1.0

```rust
fn default() -> LruQueue<K, V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "$crate::default::Default"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "$crate::default::Default"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 17], "filename": "src/cache/lru_queue.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/cache/lru_queue.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f175c7437c0d01fc2a94725"></a>
## get

`function` · `datafusion_execution::cache::lru_queue::LruQueue::get` · datafusion-execution 55.1.0

```rust
fn get(&mut self, key: &K) -> Option<&V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a reference to value mapped by `key`, if it exists.
If the entry exists, it becomes the most recently used.

<a id="op-0a24549bcb39a31c8ed255fd"></a>
## is_empty

`function` · `datafusion_execution::cache::lru_queue::LruQueue::is_empty` · datafusion-execution 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Checks whether the queue has no items.

<a id="op-9f3dee84bd63bbcb92f55ca9"></a>
## keys

`function` · `datafusion_execution::cache::lru_queue::LruQueue::keys` · datafusion-execution 55.1.0

```rust
fn keys(&self) -> impl Iterator<Item = &K>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns an iterator over references to the keys currently in the queue.
The order is unspecified and does not reflect the LRU order.

<a id="op-6e7cbfc59af12dfbeecacbae"></a>
## len

`function` · `datafusion_execution::cache::lru_queue::LruQueue::len` · datafusion-execution 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns the number of entries in the queue.

<a id="op-ddc8cb5e425e54e963678fb6"></a>
## list_entries

`function` · `datafusion_execution::cache::lru_queue::LruQueue::list_entries` · datafusion-execution 55.1.0

```rust
fn list_entries(&self) -> HashMap<&K, &V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a reference to the entries currently in the queue.

<a id="op-4ec52fb5bb812900bcd8eede"></a>
## new

`function` · `datafusion_execution::cache::lru_queue::LruQueue::new` · datafusion-execution 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f13e4ceb7077c202c430a53a"></a>
## peek

`function` · `datafusion_execution::cache::lru_queue::LruQueue::peek` · datafusion-execution 55.1.0

```rust
fn peek(&self, key: &K) -> Option<&V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a reference to value mapped by `key`, if it exists.
Does not affect the queue order.

<a id="op-e85e748022ff35b595810141"></a>
## pop

`function` · `datafusion_execution::cache::lru_queue::LruQueue::pop` · datafusion-execution 55.1.0

```rust
fn pop(&mut self) -> Option<(K, V)>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Removes and returns the least recently used value.
Returns `None` if the queue is empty.

<a id="op-98ab69acbeabcd9fe09f523d"></a>
## put

`function` · `datafusion_execution::cache::lru_queue::LruQueue::put` · datafusion-execution 55.1.0

```rust
fn put(&mut self, key: K, value: V) -> Option<V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Inserts an entry in the queue, becoming the most recently used.
If the entry already exists, returns the previous value.

<a id="op-d68d1013df5180a383ce5115"></a>
## remove

`function` · `datafusion_execution::cache::lru_queue::LruQueue::remove` · datafusion-execution 55.1.0

```rust
fn remove(&mut self, key: &K) -> Option<V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::lru_queue::LruQueue", "path": "LruQueue"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [221, 2], "filename": "src/cache/lru_queue.rs"}, "trait": null, "trait_path": null}`

Source: `src/cache/lru_queue.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Removes a specific entry from the queue, if it exists.
