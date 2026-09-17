# `cargo_metadata::errors`

Crate `cargo_metadata` · 2 public items · structured records in [`model/cargo_metadata.errors.json`](../model/cargo_metadata.errors.json)

## Error

`enum` · `cargo_metadata::errors::Error`

Also reachable as `cargo_metadata::Error`

```rust
enum Error
```

**Variants**: `CargoMetadata`, `Io`, `Utf8`, `ErrUtf8`, `Json`, `NoJson`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: Utf8Error) -> Self
fn from(source: io::Error) -> Self
fn from(source: ::serde_json::Error) -> Self
fn from(source: FromUtf8Error) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private17::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Error returned when executing/parsing `cargo metadata` fails.

# Note about Backtraces

This error type does not contain backtraces, but each error variant
comes from _one_ specific place, so it's not really needed for the
inside of this crate. If you need a backtrace down to, but not inside
of, a failed call of `cargo_metadata` you can do one of multiple thinks:

1. Convert it to a `failure::Error` (possible using the `?` operator),
   which is similar to a `Box<::std::error::Error + 'static + Send  + Sync>`.
2. Have appropriate variants in your own error type. E.g. you could wrap
   a `failure::Context<Error>` or add a `failure::Backtrace` field (which
   is empty if `RUST_BACKTRACE` is not set, so it's simple to use).
3. You still can place a failure based error into a `error_chain` if you
   really want to. (Either through foreign_links or by making it a field
   value of a `ErrorKind` variant).

---

## Result

`type_alias` · `cargo_metadata::errors::Result`

Also reachable as `cargo_metadata::Result`

```rust
type Result<T, E = Error> = ::std::result::Result<T, E>
```

Custom result type for `cargo_metadata::Error`

---
