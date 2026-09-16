# `pyrefly`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.json`](../model/pyrefly.json)

## dispatch_anyidx

`macro` · `pyrefly::dispatch_anyidx`

```rust
macro_rules! dispatch_anyidx
```

Dispatches a method call on `self` based on the variant of an `AnyIdx`.

This macro reduces boilerplate by generating a match statement that covers all
`AnyIdx` variants, extracting the typed index and calling the specified method
with the appropriate type parameter.

# Usage

```ignore
// For methods that take only the dereferenced idx:
dispatch_anyidx!(any_idx, self, check_calculation_written)

// For methods that take idx and additional arguments:
dispatch_anyidx!(any_idx, self, commit_typed, result)
```

The extracted index is passed to the method, and the method is called with
the variant's type as the type parameter.

---

## table

`macro` · `pyrefly::table`

```rust
macro_rules! table
```

---

## table_for_each

`macro` · `pyrefly::table_for_each`

```rust
macro_rules! table_for_each
```

---

## table_mut_for_each

`macro` · `pyrefly::table_mut_for_each`

```rust
macro_rules! table_mut_for_each
```

---

## table_try_for_each

`macro` · `pyrefly::table_try_for_each`

```rust
macro_rules! table_try_for_each
```

---
