# `datafusion_common::dfschema::qualified_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.dfschema.qualified_name.json).

<a id="op-2a4e45e8f9fb42221c5ff79e"></a>
## qualified_name

`function` · `datafusion_common::dfschema::qualified_name` · datafusion-common 55.1.0

```rust
fn qualified_name(qualifier: Option<&TableReference>, name: &str) -> String
```

Source: `src/dfschema.rs:1342`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Build a fully-qualified field name string. This is equivalent to
`format!("{q}.{name}")` when `qualifier` is `Some`, or just `name` when
`None`. We avoid going through the `fmt` machinery for performance reasons.
