# `datafusion_common::dfschema::SchemaExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.dfschema.SchemaExt.json).

<a id="op-2f4b9f7691ef51a039ebd15d"></a>
## SchemaExt

`trait` · `datafusion_common::dfschema::SchemaExt` · datafusion-common 55.1.0

```rust
trait SchemaExt
```

Source: `src/dfschema.rs:1271`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

DataFusion-specific extensions to [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050).

<a id="op-2baf4c3717e2761a7e1228a7"></a>
## equivalent_names_and_types

`function` · `datafusion_common::dfschema::SchemaExt::equivalent_names_and_types` · datafusion-common 55.1.0

```rust
fn equivalent_names_and_types(&self, other: &Self) -> bool
```

Source: `src/dfschema.rs:1276`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This is a specialized version of Eq that ignores differences
in nullability and metadata.

It works the same as [`DFSchema::has_equivalent_names_and_types`](../operations/datafusion_common.dfschema.DFSchema.md#op-92d655bc799e1ee3296c2e3c).

<a id="op-1e4ab0d32aed6bf206ebee35"></a>
## logically_equivalent_names_and_types

`function` · `datafusion_common::dfschema::SchemaExt::logically_equivalent_names_and_types` · datafusion-common 55.1.0

```rust
fn logically_equivalent_names_and_types(&self, other: &Self) -> Result<()>
```

Source: `src/dfschema.rs:1285`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns nothing if the two schemas have the same qualified named
fields with logically equivalent data types. Returns internal error otherwise.

Use [DFSchema](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98)::has_equivalent_names_and_types for stricter semantic type
equivalence checking.

It is only used by insert into cases.
