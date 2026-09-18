# `object_store::multipart`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.multipart.json).

<a id="op-6e2e1c1150c841b5ef52c274"></a>
## multipart

`module` · `object_store::multipart` · object_store 0.13.2

```rust
mod multipart
```

Source: `src/multipart.rs:18`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Cloud Multipart Upload

This crate provides an asynchronous interface for multipart file uploads to
cloud storage services. It's designed to offer efficient, non-blocking operations,
especially useful when dealing with large files or high-throughput systems.
