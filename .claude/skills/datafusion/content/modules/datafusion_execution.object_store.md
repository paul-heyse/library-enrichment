# `datafusion_execution::object_store`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.object_store.json).

<a id="op-0ab0f683bdec0b1ca4083274"></a>
## object_store

`module` · `datafusion_execution::object_store` · datafusion-execution 55.1.0

```rust
mod object_store
```

Source: `src/object_store.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

ObjectStoreRegistry holds all the object stores at Runtime with a scheme for each store.
This allows the user to extend DataFusion with different storage systems such as S3 or HDFS
and query data inside these systems.
