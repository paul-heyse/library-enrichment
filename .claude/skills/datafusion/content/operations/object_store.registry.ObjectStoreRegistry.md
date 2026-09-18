# `object_store::registry::ObjectStoreRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.registry.ObjectStoreRegistry.json).

<a id="op-c074e1961f5f8e41ba850998"></a>
## ObjectStoreRegistry

`trait` · `object_store::registry::ObjectStoreRegistry` · object_store 0.13.2

```rust
trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + 'static
```

Source: `src/registry.rs:28`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

[`ObjectStoreRegistry`](../operations/object_store.registry.ObjectStoreRegistry.md#op-c074e1961f5f8e41ba850998) maps a URL to an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) instance

<a id="op-e45c49eed6b0a38f6fd4672c"></a>
## register

`function` · `object_store::registry::ObjectStoreRegistry::register` · object_store 0.13.2

```rust
fn register(&self, url: Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

Source: `src/registry.rs:32`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Register a new store for the provided store URL

If a store with the same URL existed before, it is replaced and returned

<a id="op-056067a28520d110732df3b9"></a>
## resolve

`function` · `object_store::registry::ObjectStoreRegistry::resolve` · object_store 0.13.2

```rust
fn resolve(&self, url: &Url) -> Result<(Arc<dyn ObjectStore>, Path)>
```

Source: `src/registry.rs:84`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Resolve an object URL

If [`ObjectStoreRegistry::register`](../operations/object_store.registry.ObjectStoreRegistry.md#op-e45c49eed6b0a38f6fd4672c) has been called with a URL with the same
scheme, and authority as the object URL, and a path that is a prefix of the object
URL's, it should be returned along with the trailing path. Paths should be matched
on a path segment basis, and in the event of multiple possibilities the longest
path match should be returned.

If a store hasn't been registered, an [`ObjectStoreRegistry`](../operations/object_store.registry.ObjectStoreRegistry.md#op-c074e1961f5f8e41ba850998) may lazily create
one if the URL is understood

For example

```
# use std::sync::Arc;
# use url::Url;
# use object_store::memory::InMemory;
# use object_store::ObjectStore;
# use object_store::prefix::PrefixStore;
# use object_store::registry::{DefaultObjectStoreRegistry, ObjectStoreRegistry};
#
let registry = DefaultObjectStoreRegistry::new();

let bucket1 = Arc::new(InMemory::new()) as Arc<dyn ObjectStore>;
let base = Url::parse("s3://bucket1/").unwrap();
registry.register(base, bucket1.clone());

let url = Url::parse("s3://bucket1/path/to/object").unwrap();
let (ret, path) = registry.resolve(&url).unwrap();
assert_eq!(path.as_ref(), "path/to/object");
assert!(Arc::ptr_eq(&ret, &bucket1));

let bucket2 = Arc::new(InMemory::new()) as Arc<dyn ObjectStore>;
let base = Url::parse("https://s3.region.amazonaws.com/bucket").unwrap();
registry.register(base, bucket2.clone());

let url = Url::parse("https://s3.region.amazonaws.com/bucket/path/to/object").unwrap();
let (ret, path) = registry.resolve(&url).unwrap();
assert_eq!(path.as_ref(), "path/to/object");
assert!(Arc::ptr_eq(&ret, &bucket2));

let bucket3 = Arc::new(PrefixStore::new(InMemory::new(), "path")) as Arc<dyn ObjectStore>;
let base = Url::parse("https://s3.region.amazonaws.com/bucket/path").unwrap();
registry.register(base, bucket3.clone());

let url = Url::parse("https://s3.region.amazonaws.com/bucket/path/to/object").unwrap();
let (ret, path) = registry.resolve(&url).unwrap();
assert_eq!(path.as_ref(), "to/object");
assert!(Arc::ptr_eq(&ret, &bucket3));
```
