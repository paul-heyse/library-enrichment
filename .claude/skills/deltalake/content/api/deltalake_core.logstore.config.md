# `deltalake_core::logstore::config`

Crate `deltalake-core` · 8 public items · structured records in [`model/deltalake_core.logstore.config.json`](../model/deltalake_core.logstore.config.json)

## parse_bool

`function` · `deltalake_core::logstore::config::parse_bool`

Also reachable as `deltalake::logstore::config::parse_bool`

```rust
fn parse_bool(value: &str) -> DeltaResult<bool>
```

Parse a string into a `bool`, accepting common truthy spellings (e.g. "1", "true").

```
use deltalake_core::logstore::config::parse_bool;
assert!(parse_bool("true").unwrap());
assert!(parse_bool("1").unwrap());
assert!(!parse_bool("false").unwrap());
assert!(!parse_bool("0").unwrap());
```

---

## parse_f64

`function` · `deltalake_core::logstore::config::parse_f64`

Also reachable as `deltalake::logstore::config::parse_f64`

```rust
fn parse_f64(value: &str) -> DeltaResult<f64>
```

Parse a string into an `f64`, returning a descriptive error on failure.

```
use deltalake_core::logstore::config::parse_f64;
assert_eq!(parse_f64("3.14").unwrap(), 3.14);
assert!(parse_f64("not_a_number").is_err());
```

---

## parse_string

`function` · `deltalake_core::logstore::config::parse_string`

Also reachable as `deltalake::logstore::config::parse_string`

```rust
fn parse_string(value: &str) -> DeltaResult<String>
```

Parse a configuration value as a plain string (an infallible identity conversion).

---

## parse_usize

`function` · `deltalake_core::logstore::config::parse_usize`

Also reachable as `deltalake::logstore::config::parse_usize`

```rust
fn parse_usize(value: &str) -> DeltaResult<usize>
```

Parse a string into a `usize`, returning a descriptive error on failure.

```
use deltalake_core::logstore::config::parse_usize;
assert_eq!(parse_usize("42").unwrap(), 42);
assert!(parse_usize("not_a_number").is_err());
```

---

## str_is_truthy

`function` · `deltalake_core::logstore::config::str_is_truthy`

Also reachable as `deltalake::logstore::config::str_is_truthy`

```rust
fn str_is_truthy(val: &str) -> bool
```

Return true for all the stringly values typically associated with true

aka YAML booleans

```rust
# use deltalake_core::logstore::config::*;
for value in ["1", "true", "on", "YES", "Y"] {
    assert!(str_is_truthy(value));
}
for value in ["0", "FALSE", "off", "NO", "n", "bork"] {
    assert!(!str_is_truthy(value));
}
```

---

## ParseResult

`struct` · `deltalake_core::logstore::config::ParseResult`

Also reachable as `deltalake::logstore::config::ParseResult`

```rust
struct ParseResult<T: std::fmt::Debug>
```

**Fields**: `config`, `unparsed`, `errors`, `is_default`

**Implements**: `core::iter::traits::collect::FromIterator`

**Derives**: Debug

**Methods** (1)

```rust
fn raise_errors(&self) -> DeltaResult<()>
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

Generic container for parsing configuration

---

## StorageConfig

`struct` · `deltalake_core::logstore::config::StorageConfig`

Also reachable as `deltalake::logstore::StorageConfig`, `deltalake::logstore::config::StorageConfig`, `deltalake_core::logstore::StorageConfig`

```rust
struct StorageConfig
```

**Fields**: `runtime`, `retry`, `limit`, `certificate`, `unknown_properties`, `raw`

**Implements**: `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn decorate_store<T: ObjectStore + Clone>(&self, store: T, table_root: &url::Url) -> DeltaResult<Box<dyn ObjectStore>>
fn parse_options<K, V, I>(options: I) -> DeltaResult<Self> where I: IntoIterator<Item = (K, V)>, K: AsRef<str> + Into<String>, V: AsRef<str> + Into<String>
fn raw(&self) -> impl Iterator<Item = (&String, &String)>
fn with_io_runtime(self, rt: IORuntime) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

Resolved configuration for constructing and decorating an object store backend.

Aggregates the optional dedicated IO runtime, retry/limit/certificate settings and the raw
passthrough options used to build the underlying [`ObjectStore`].

---

## TryUpdateKey

`trait` · `deltalake_core::logstore::config::TryUpdateKey`

Also reachable as `deltalake::logstore::config::TryUpdateKey`

```rust
trait TryUpdateKey: Default
```

**Implementors** (5)

- `deltalake_core::logstore::storage::CertificateConfig`
- `deltalake_core::logstore::storage::LimitConfig`
- `deltalake_core::logstore::storage::runtime::RuntimeConfig`
- `deltalake_core::table::builder::DeltaTableConfig`
- `object_store::client::retry::RetryConfig`

**Methods** (2)

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
fn try_update_key(&mut self, key: &str, value: &str) -> DeltaResult<Option<()>>
```

A configuration type that can be incrementally populated from string key/value pairs.

Implemented by the various storage configuration structs so that options coming from user
input or the environment can be applied generically by key name.

---
