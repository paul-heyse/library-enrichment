# `ruff_macros`

Crate `ruff_macros` · 5 public items · structured records in [`model/ruff_macros.json`](../model/ruff_macros.json)

## attribute_env_vars_metadata

`proc_macro` · `ruff_macros::attribute_env_vars_metadata`

```rust
macro attribute_env_vars_metadata
```

Generates metadata for environment variables declared in the impl block.

This attribute macro should be applied to an `impl EnvVars` block.
It will generate a `metadata()` method that returns all non-hidden
environment variables with their documentation.

---

## derive_message_formats

`proc_macro` · `ruff_macros::derive_message_formats`

```rust
macro derive_message_formats
```

---

## kebab_case

`proc_macro` · `ruff_macros::kebab_case`

```rust
macro kebab_case
```

Converts an identifier to a kebab case string.

---

## map_codes

`proc_macro` · `ruff_macros::map_codes`

```rust
macro map_codes
```

---

## newtype_index

`proc_macro` · `ruff_macros::newtype_index`

Also reachable as `ruff_index::newtype_index`

```rust
macro newtype_index
```

Derives a newtype wrapper that can be used as an index.
The wrapper can represent indices up to `u32::MAX - 1`.

The `u32::MAX - 1` is an optimization so that `Option<Index>` has the same size as `Index`.

Can store at most `u32::MAX - 1` values

## Warning

Additional `derive` attributes must come AFTER this attribute:

Good:

```ignore
use ruff_macros::newtype_index;

#[newtype_index]
#[derive(Ord, PartialOrd)]
struct MyIndex;
```

---
