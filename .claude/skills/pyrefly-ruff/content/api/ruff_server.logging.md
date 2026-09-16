# `ruff_server::logging`

Crate `ruff_server` · 2 public items · structured records in [`model/ruff_server.logging.json`](../model/ruff_server.logging.json)

## LogLevel

`enum` · `ruff_server::logging::LogLevel`

Also reachable as `ruff_server::LogLevel`

```rust
enum LogLevel
```

**Variants**: `Error`, `Warn`, `Info`, `Debug`, `Trace`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

The log level for the server as provided by the client during initialization.

The default log level is `info`.

---

## init_logging

`function` · `ruff_server::logging::init_logging`

Also reachable as `ruff_server::init_logging`

```rust
fn init_logging(log_level: LogLevel, log_file: Option<&std::path::Path>)
```

---
