# `object_store::throttle::ThrottleConfig`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.throttle.ThrottleConfig.json).

<a id="op-f654d5ccefcee949c33cc538"></a>
## ThrottleConfig

`struct` · `object_store::throttle::ThrottleConfig` · object_store 0.13.2

```rust
struct ThrottleConfig
```

Source: `src/throttle.rs:36`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configuration settings for throttled store

<a id="op-38cf4272a0c2c36a423402c1"></a>
## clone

`function` · `object_store::throttle::ThrottleConfig::clone` · object_store 0.13.2

```rust
fn clone(&self) -> ThrottleConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::throttle::ThrottleConfig", "path": "ThrottleConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 26], "end": [35, 31], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/throttle.rs:35`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d78d36882a72fd1be716a4b"></a>
## default

`function` · `object_store::throttle::ThrottleConfig::default` · object_store 0.13.2

```rust
fn default() -> ThrottleConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::throttle::ThrottleConfig", "path": "ThrottleConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 24], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/throttle.rs:35`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38b29a1aa06a74b6fa7e2fe8"></a>
## fmt

`function` · `object_store::throttle::ThrottleConfig::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::throttle::ThrottleConfig", "path": "ThrottleConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/throttle.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/throttle.rs:35`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c096a09834ccf02ba72c8aad"></a>
## wait_delete_per_call

`struct_field` · `object_store::throttle::ThrottleConfig::wait_delete_per_call` · object_store 0.13.2

```rust
wait_delete_per_call: std::time::Duration
```

Source: `src/throttle.rs:44`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every call to [`delete`], or every element in [`delete_stream`].

Sleeping is done before the underlying store is called and independently of the success of
the operation.

[`delete`]: crate::ObjectStoreExt::delete
[`delete_stream`]: ThrottledStore::delete_stream

<a id="op-b5ff4a1645b4a8f1c72ee5b1"></a>
## wait_get_per_byte

`struct_field` · `object_store::throttle::ThrottleConfig::wait_get_per_byte` · object_store 0.13.2

```rust
wait_get_per_byte: std::time::Duration
```

Source: `src/throttle.rs:54`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every byte received during [`get_opts`](ThrottledStore::get_opts).

Sleeping is performed after the underlying store returned and only for successful gets. The
sleep duration is additive to [`wait_get_per_call`](Self::wait_get_per_call).

Note that the per-byte sleep only happens as the user consumes the output bytes. Should
there be an intermediate failure (i.e. after partly consuming the output bytes), the
resulting sleep time will be partial as well.

<a id="op-c7b4be40435c9320fb0bda28"></a>
## wait_get_per_call

`struct_field` · `object_store::throttle::ThrottleConfig::wait_get_per_call` · object_store 0.13.2

```rust
wait_get_per_call: std::time::Duration
```

Source: `src/throttle.rs:61`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every call to [`get_opts`](ThrottledStore::get_opts).

Sleeping is done before the underlying store is called and independently of the success of
the operation. The sleep duration is additive to
[`wait_get_per_byte`](Self::wait_get_per_byte).

<a id="op-fdff4c5d9377711993040b5f"></a>
## wait_list_per_call

`struct_field` · `object_store::throttle::ThrottleConfig::wait_list_per_call` · object_store 0.13.2

```rust
wait_list_per_call: std::time::Duration
```

Source: `src/throttle.rs:68`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every call to [`list`](ThrottledStore::list).

Sleeping is done before the underlying store is called and independently of the success of
the operation. The sleep duration is additive to
[`wait_list_per_entry`](Self::wait_list_per_entry).

<a id="op-e1f1a0a670df24c2b09d3a20"></a>
## wait_list_per_entry

`struct_field` · `object_store::throttle::ThrottleConfig::wait_list_per_entry` · object_store 0.13.2

```rust
wait_list_per_entry: std::time::Duration
```

Source: `src/throttle.rs:78`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every entry received during [`list`](ThrottledStore::list).

Sleeping is performed after the underlying store returned and only for successful lists.
The sleep duration is additive to [`wait_list_per_call`](Self::wait_list_per_call).

Note that the per-entry sleep only happens as the user consumes the output entries. Should
there be an intermediate failure (i.e. after partly consuming the output entries), the
resulting sleep time will be partial as well.

<a id="op-27e4c4bbe03646c59f8382c2"></a>
## wait_list_with_delimiter_per_call

`struct_field` · `object_store::throttle::ThrottleConfig::wait_list_with_delimiter_per_call` · object_store 0.13.2

```rust
wait_list_with_delimiter_per_call: std::time::Duration
```

Source: `src/throttle.rs:86`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every call to
[`list_with_delimiter`](ThrottledStore::list_with_delimiter).

Sleeping is done before the underlying store is called and independently of the success of
the operation. The sleep duration is additive to
[`wait_list_with_delimiter_per_entry`](Self::wait_list_with_delimiter_per_entry).

<a id="op-37a82e044144f3428c616ba1"></a>
## wait_list_with_delimiter_per_entry

`struct_field` · `object_store::throttle::ThrottleConfig::wait_list_with_delimiter_per_entry` · object_store 0.13.2

```rust
wait_list_with_delimiter_per_entry: std::time::Duration
```

Source: `src/throttle.rs:94`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every entry received during
[`list_with_delimiter`](ThrottledStore::list_with_delimiter).

Sleeping is performed after the underlying store returned and only for successful gets. The
sleep duration is additive to
[`wait_list_with_delimiter_per_call`](Self::wait_list_with_delimiter_per_call).

<a id="op-5e1cdc7a17175c1054f1f451"></a>
## wait_put_per_call

`struct_field` · `object_store::throttle::ThrottleConfig::wait_put_per_call` · object_store 0.13.2

```rust
wait_put_per_call: std::time::Duration
```

Source: `src/throttle.rs:100`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sleep duration for every call to [`put_opts`](ThrottledStore::put_opts).

Sleeping is done before the underlying store is called and independently of the success of
the operation.
