# `datafusion_common::dfschema::ToDFSchema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.dfschema.ToDFSchema.json).

<a id="op-eb7890b7db42853056d1ffdb"></a>
## ToDFSchema

`trait` · `datafusion_common::dfschema::ToDFSchema` · datafusion-common 55.1.0

```rust
trait ToDFSchema where Self: Sized
```

Source: `src/dfschema.rs:1152`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience trait to convert Schema like things to DFSchema and DFSchemaRef with fewer keystrokes

<a id="op-344a06adbf3c78192e81942c"></a>
## to_dfschema

`function` · `datafusion_common::dfschema::ToDFSchema::to_dfschema` · datafusion-common 55.1.0

```rust
fn to_dfschema(self) -> Result<DFSchema>
```

Source: `src/dfschema.rs:1157`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Attempt to create a DSSchema

<a id="op-2165f44fe698e4558f6c7414"></a>
## to_dfschema_ref

`function` · `datafusion_common::dfschema::ToDFSchema::to_dfschema_ref` · datafusion-common 55.1.0

```rust
fn to_dfschema_ref(self) -> Result<DFSchemaRef>
```

Source: `src/dfschema.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Attempt to create a DSSchemaRef
