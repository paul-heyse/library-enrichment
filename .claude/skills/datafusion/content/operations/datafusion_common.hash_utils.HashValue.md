# `datafusion_common::hash_utils::HashValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.hash_utils.HashValue.json).

<a id="op-3388303c72daaba93856175b"></a>
## HashValue

`trait` · `datafusion_common::hash_utils::HashValue` · datafusion-common 55.1.0

```rust
trait HashValue
```

Source: `src/hash_utils.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-628f15343a487ddbf1fd7be8"></a>
## hash_one

`function` · `datafusion_common::hash_utils::HashValue::hash_one` · datafusion-common 55.1.0

```rust
fn hash_one<S: BuildHasher>(&self, state: &S) -> u64
```

Source: `src/hash_utils.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69ff86ce5f19eb7c8987ce49"></a>
## hash_write

`function` · `datafusion_common::hash_utils::HashValue::hash_write` · datafusion-common 55.1.0

```rust
fn hash_write(&self, hasher: &mut impl Hasher)
```

Source: `src/hash_utils.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Write this value into an existing hasher (same data as `hash_one`).
