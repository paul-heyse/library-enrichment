# `buoyant_kernel::schema::compare`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.compare.json).

<a id="op-57c1de8513d520c87dc68106"></a>
## compare

`module` · `buoyant_kernel::schema::compare` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod compare
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/compare.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/compare.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provides utilities to perform comparisons between a [`Schema`]s. The api used to check schema
compatibility is [`can_read_as`] that is exposed through the [`SchemaComparison`] trait.

# Examples
 ```rust, ignore
 # use delta_kernel::schema::StructType;
 # use delta_kernel::schema::StructField;
 # use delta_kernel::schema::DataType;
 let schema = StructType::try_new([
    StructField::new("id", DataType::LONG, false),
    StructField::new("value", DataType::STRING, true),
 ])?;
 let read_schema = StructType::try_new([
    StructField::new("id", DataType::LONG, true),
    StructField::new("value", DataType::STRING, true),
    StructField::new("year", DataType::INTEGER, true),
 ])?;
 // Schemas are compatible since the `read_schema` adds a nullable column `year`
 assert!(schema.can_read_as(&read_schema).is_ok());
 ````

[`Schema`]: crate::schema::Schema

Unresolved upstream links (retained, not inferred): ``SchemaComparison``.
