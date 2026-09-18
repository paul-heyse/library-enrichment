# `datafusion_execution::cache::lru_queue`

Crate `datafusion-execution` · 1 public items · structured records in [`model/datafusion_execution.cache.lru_queue.json`](../model/datafusion_execution.cache.lru_queue.json)

## LruQueue

`struct` · `datafusion_execution::cache::lru_queue::LruQueue`

```rust
struct LruQueue<K: Eq + Hash + Clone, V>
```

**Derives**: Default

**Methods** (12)

```rust
fn clear(&mut self)
fn contains_key(&self, key: &K) -> bool
fn get(&mut self, key: &K) -> Option<&V>
fn is_empty(&self) -> bool
fn keys(&self) -> impl Iterator<Item = &K>
fn len(&self) -> usize
fn list_entries(&self) -> HashMap<&K, &V>
fn new() -> Self
fn peek(&self, key: &K) -> Option<&V>
fn pop(&mut self) -> Option<(K, V)>
fn put(&mut self, key: K, value: V) -> Option<V>
fn remove(&mut self, key: &K) -> Option<V>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.cache.lru_queue.LruQueue.md).


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

---
