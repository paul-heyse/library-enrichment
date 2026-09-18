# `object_store::integration::multipart_race_condition`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.integration.multipart_race_condition.json).

<a id="op-0fa3771c4d39cbbbe726b74b"></a>
## multipart_race_condition

`function` · `object_store::integration::multipart_race_condition` · object_store 0.13.2

```rust
async fn multipart_race_condition(storage: &dyn ObjectStore, last_writer_wins: bool)
```

Source: `src/integration.rs:1065`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Tests a race condition where 2 threads are performing multipart writes to the same path
