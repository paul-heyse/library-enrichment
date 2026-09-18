# `arrow_avro::codec`

Crate `arrow-avro` · 1 public items · structured records in [`model/arrow_avro.codec.json`](../model/arrow_avro.codec.json)

## Tz

`enum` · `arrow_avro::codec::Tz`

```rust
enum Tz
```

**Variants**: `OffsetZero`, `Utc`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/arrow_avro.codec.Tz.md).


Timezone representation for timestamps.

Avro only distinguishes between UTC and local time (no timezone), but Arrow supports
any of the two identifiers of the UTC timezone: "+00:00" and "UTC".
The data types using these time zone IDs behave identically, but are not logically equal.

---
