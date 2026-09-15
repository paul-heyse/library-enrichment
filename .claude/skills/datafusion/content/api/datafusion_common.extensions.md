# `datafusion_common::extensions`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.extensions.json`](../model/datafusion_common.extensions.json)

## Extensions

`struct` · `datafusion_common::extensions::Extensions`

```rust
struct Extensions
```

**Derives**: Clone, Debug, Default

**Methods** (10)

```rust
fn contains<T: Any + Send + Sync>(&self) -> bool
fn get<T: Any + Send + Sync>(&self) -> Option<&T>
fn get_arc<T: Any + Send + Sync>(&self) -> Option<Arc<T>>
fn insert<T: Any + Send + Sync>(&mut self, value: T) -> Option<Arc<T>>
fn insert_arc<T: Any + Send + Sync>(&mut self, value: Arc<T>) -> Option<Arc<T>>
fn insert_dyn(&mut self, value: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn Any + Send + Sync>>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn merge(&mut self, other: &Extensions)
fn new() -> Self
```

A type-keyed map of opaque `Arc`'d values. Each Rust type `T` occupies
its own slot, so independent components can each attach their own data
without conflict.

Cloning is cheap: the backing values are reference-counted.

# Example

```
# use std::sync::Arc;
# use datafusion_common::extensions::Extensions;
struct MyData(u32);
struct OtherData(&'static str);

let mut ext = Extensions::new();
ext.insert(MyData(42));
ext.insert_arc(Arc::new(OtherData("hello")));

assert_eq!(ext.get::<MyData>().unwrap().0, 42);
assert_eq!(ext.get::<OtherData>().unwrap().0, "hello");
```

---
