# `tracing_core`

Crate `tracing-core` · 2 public items · structured records in [`model/tracing_core.json`](../model/tracing_core.json)

## identify_callsite

`macro` · `tracing_core::identify_callsite`

```rust
macro_rules! identify_callsite
```

Statically constructs an [`Identifier`] for the provided [`Callsite`].

This may be used in contexts such as static initializers.

For example:
```rust
use tracing_core::{callsite, identify_callsite};
# use tracing_core::{Metadata, subscriber::Interest};
# fn main() {
pub struct MyCallsite {
   // ...
}
impl callsite::Callsite for MyCallsite {
# fn set_interest(&self, _: Interest) { unimplemented!() }
# fn metadata(&self) -> &Metadata { unimplemented!() }
    // ...
}

static CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static CALLSITE_ID: callsite::Identifier = identify_callsite!(&CALLSITE);
# }
```

[`Identifier`]: callsite::Identifier
[`Callsite`]: callsite::Callsite

---

## metadata

`macro` · `tracing_core::metadata`

```rust
macro_rules! metadata
```

Statically constructs new span [metadata].

/// For example:
```rust
# use tracing_core::{callsite::Callsite, subscriber::Interest};
use tracing_core::metadata;
use tracing_core::metadata::{Kind, Level, Metadata};
# fn main() {
# pub struct MyCallsite { }
# impl Callsite for MyCallsite {
# fn set_interest(&self, _: Interest) { unimplemented!() }
# fn metadata(&self) -> &Metadata { unimplemented!() }
# }
#
static FOO_CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static FOO_METADATA: Metadata = metadata!{
    name: "foo",
    target: module_path!(),
    level: Level::DEBUG,
    fields: &["bar", "baz"],
    callsite: &FOO_CALLSITE,
    kind: Kind::SPAN,
};
# }
```

[metadata]: metadata::Metadata
[`Metadata::new`]: metadata::Metadata::new

---
