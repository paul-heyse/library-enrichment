# `pyrefly_util::task_heap`

Crate `pyrefly_util` · 3 public items · structured records in [`model/pyrefly_util.task_heap.json`](../model/pyrefly_util.task_heap.json)

## CancellationHandle

`struct` · `pyrefly_util::task_heap::CancellationHandle`

```rust
struct CancellationHandle
```

**Implements**: `dupe::Dupe`

**Derives**: Clone

**Methods** (2)

```rust
fn cancel(&self)
fn is_cancelled(&self) -> bool
```

---

## Cancelled

`struct` · `pyrefly_util::task_heap::Cancelled`

```rust
struct Cancelled
```

**Derives**: Debug

Used to signal that all the tasks should be cancelled.

---

## TaskHeap

`struct` · `pyrefly_util::task_heap::TaskHeap`

```rust
struct TaskHeap<K, V>
```

**Implements**: `core::iter::traits::collect::FromIterator`

**Derives**: Default

**Methods** (9)

```rust
fn get_cancellation_handle(&self) -> CancellationHandle
fn is_empty(&self) -> bool
fn new() -> Self
fn push(&self, k: K, v: V, is_lifo: bool)
fn push_fifo(&self, k: K, v: V)
fn push_lifo(&self, k: K, v: V)
fn reset_cancellation(&mut self)
fn work(&self, f: impl FnMut(K, V)) -> Result<(), Cancelled>
fn work_without_cancellation(&self, f: impl FnMut(K, V))
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self
```

A heap of tasks, where `K` represents the priority of the task and `V` represents the task.
Add tasks with `push_lifo` and `push_fifo`, and process them with `work`.

---
