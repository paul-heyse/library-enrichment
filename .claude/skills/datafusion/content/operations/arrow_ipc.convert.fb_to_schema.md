# `arrow_ipc::convert::fb_to_schema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.convert.fb_to_schema.json).

<a id="op-6ffcfc831af9be3afc89cc82"></a>
## fb_to_schema

`function` · `arrow_ipc::convert::fb_to_schema` · arrow-ipc 59.3.0

```rust
fn fb_to_schema(fb: Schema<'_>) -> Schema
```

Source: `src/convert.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Deserialize an ipc [crate::Schema`](../operations/arrow_ipc.gen.Schema.Schema.md#op-e23a38b8fc7c2dfef482c15e) from flat buffers to an arrow [Schema](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050).
