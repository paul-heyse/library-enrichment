# `datafusion_common::utils::hex::HexCase`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.hex.HexCase.json).

<a id="op-e9733982d2c2f0976780952d"></a>
## HexCase

`enum` · `datafusion_common::utils::hex::HexCase` · datafusion-common 55.1.0

```rust
enum HexCase
```

Source: `src/utils/hex.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Case of the emitted hex digits.

<a id="op-0e62c54003efb75084740e03"></a>
## Lower

`variant` · `datafusion_common::utils::hex::HexCase::Lower` · datafusion-common 55.1.0

```rust
Lower
```

Source: `src/utils/hex.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Digits `0123456789abcdef`.

<a id="op-09b6be43b347e3da65cfefb9"></a>
## Upper

`variant` · `datafusion_common::utils::hex::HexCase::Upper` · datafusion-common 55.1.0

```rust
Upper
```

Source: `src/utils/hex.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Digits `0123456789ABCDEF`.

<a id="op-8a2f6f4a7c9997a34f55e088"></a>
## clone

`function` · `datafusion_common::utils::hex::HexCase::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> HexCase
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::hex::HexCase", "path": "HexCase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 22], "filename": "src/utils/hex.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/utils/hex.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01600fd55c98e5e0185d2b4c"></a>
## eq

`function` · `datafusion_common::utils::hex::HexCase::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &HexCase) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::hex::HexCase", "path": "HexCase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 30], "end": [32, 39], "filename": "src/utils/hex.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/utils/hex.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-048116b390fe9318c3ee1127"></a>
## fmt

`function` · `datafusion_common::utils::hex::HexCase::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::utils::hex::HexCase", "path": "HexCase"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/utils/hex.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils/hex.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
