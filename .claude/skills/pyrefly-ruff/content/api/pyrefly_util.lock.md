# `pyrefly_util::lock`

Crate `pyrefly_util` · 4 public items · structured records in [`model/pyrefly_util.lock.json`](../model/pyrefly_util.lock.json)

## Condvar

`struct` · `pyrefly_util::lock::Condvar`

```rust
struct Condvar
```

**Derives**: Debug, Default

**Methods** (7)

```rust
fn new() -> Self
fn notify_all(&self)
fn notify_one(&self)
fn wait<'a, T>(&self, guard: sync::MutexGuard<'a, T>) -> sync::MutexGuard<'a, T>
fn wait_timeout<'a, T>(&self, guard: sync::MutexGuard<'a, T>, duration: std::time::Duration) -> (sync::MutexGuard<'a, T>, sync::WaitTimeoutResult)
fn wait_timeout_while<'a, T>(&self, guard: sync::MutexGuard<'a, T>, duration: std::time::Duration, condition: impl FnMut(&mut T) -> bool) -> (sync::MutexGuard<'a, T>, sync::WaitTimeoutResult)
fn wait_while<'a, T, F>(&self, guard: sync::MutexGuard<'a, T>, condition: F) -> sync::MutexGuard<'a, T> where F: FnMut(&mut T) -> bool
```

---

## FinishHandle

`struct` · `pyrefly_util::lock::FinishHandle`

```rust
struct FinishHandle
```

**Methods** (3)

```rust
fn new() -> Self
fn notify_finished(&self)
fn wait_for_finish(&self, timeout: Duration) -> bool
```

---

## Mutex

`struct` · `pyrefly_util::lock::Mutex`

```rust
struct Mutex<T>
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn get_mut(&mut self) -> &mut T
fn into_inner(self) -> T
fn lock(&self) -> sync::MutexGuard<'_, T>
fn new(t: T) -> Self
fn try_lock(&self) -> Option<sync::MutexGuard<'_, T>>
```

---

## RwLock

`struct` · `pyrefly_util::lock::RwLock`

```rust
struct RwLock<T>
```

**Derives**: Debug, Default

**Methods** (4)

```rust
fn into_inner(self) -> T
fn new(t: T) -> Self
fn read(&self) -> sync::RwLockReadGuard<'_, T>
fn write(&self) -> sync::RwLockWriteGuard<'_, T>
```

---
