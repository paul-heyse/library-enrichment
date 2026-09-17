# `grep_cli::human`

Crate `grep-cli` · 2 public items · structured records in [`model/grep_cli.human.json`](../model/grep_cli.human.json)

## parse_human_readable_size

`function` · `grep_cli::human::parse_human_readable_size`

Also reachable as `grep_cli::parse_human_readable_size`

```rust
fn parse_human_readable_size(size: &str) -> Result<u64, ParseSizeError>
```

Parse a human readable size like `2M` into a corresponding number of bytes.

Supported size suffixes are `K` (for kilobyte), `M` (for megabyte) and `G`
(for gigabyte). If a size suffix is missing, then the size is interpreted
as bytes. If the size is too big to fit into a `u64`, then this returns an
error.

Additional suffixes may be added over time.

---

## ParseSizeError

`struct` · `grep_cli::human::ParseSizeError`

Also reachable as `grep_cli::ParseSizeError`

```rust
struct ParseSizeError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An error that occurs when parsing a human readable size description.

This error provides an end user friendly message describing why the
description couldn't be parsed and what the expected format is.

---
