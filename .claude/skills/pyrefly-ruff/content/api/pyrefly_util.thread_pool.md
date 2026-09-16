# `pyrefly_util::thread_pool`

Crate `pyrefly_util` · 3 public items · structured records in [`model/pyrefly_util.thread_pool.json`](../model/pyrefly_util.thread_pool.json)

## TEST_THREAD_COUNT

`constant` · `pyrefly_util::thread_pool::TEST_THREAD_COUNT`

```rust
const TEST_THREAD_COUNT: ThreadCount = _
```

Thread count used by tests. Enough threads to see parallelism bugs, but not too many to debug through.

---

## ThreadCount

`enum` · `pyrefly_util::thread_pool::ThreadCount`

```rust
enum ThreadCount
```

**Variants**: `AllThreads`, `NumThreads`, `Inline`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

---

## ThreadPool

`struct` · `pyrefly_util::thread_pool::ThreadPool`

```rust
struct ThreadPool
```

**Methods** (5)

```rust
fn async_spawn(&self, f: impl FnOnce() + Send + 'static)
fn install<OP, R>(&self, op: OP) -> R where OP: FnOnce() -> R + Send, R: Send
fn new(count: ThreadCount) -> Self
fn spawn_many(&self, f: impl Fn() + Sync)
fn stack_size() -> usize
```

A WASM compatible thread-pool.

---
