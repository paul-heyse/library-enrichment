# `datafusion_common::join_type::JoinSide`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.join_type.JoinSide.json).

<a id="op-f4682cb898c87d37b9d490a3"></a>
## JoinSide

`enum` · `datafusion_common::join_type::JoinSide` · datafusion-common 55.1.0

```rust
enum JoinSide
```

Source: `src/join_type.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Join side.
Stores the referred table side during calculations

<a id="op-5f7f4c68abf9e75f1138f7b4"></a>
## Left

`variant` · `datafusion_common::join_type::JoinSide::Left` · datafusion-common 55.1.0

```rust
Left
```

Source: `src/join_type.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Left side of the join

<a id="op-5e0a7412df9ff9b25212aa0e"></a>
## None

`variant` · `datafusion_common::join_type::JoinSide::None` · datafusion-common 55.1.0

```rust
None
```

Source: `src/join_type.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Neither side of the join, used for Mark joins where the mark column does not belong to
either side of the join

<a id="op-fb7b485de584896a40f58901"></a>
## Right

`variant` · `datafusion_common::join_type::JoinSide::Right` · datafusion-common 55.1.0

```rust
Right
```

Source: `src/join_type.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Right side of the join

<a id="op-1ef3e2ff760780d2881117cb"></a>
## clone

`function` · `datafusion_common::join_type::JoinSide::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> JoinSide
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinSide", "path": "JoinSide"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 17], "end": [239, 22], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/join_type.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6395e833eb6bc9992763dd54"></a>
## eq

`function` · `datafusion_common::join_type::JoinSide::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &JoinSide) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinSide", "path": "JoinSide"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 30], "end": [239, 39], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/join_type.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96627ef1f02d4881369e5193"></a>
## fmt

`function` · `datafusion_common::join_type::JoinSide::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinSide", "path": "JoinSide"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [235, 2], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/join_type.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe015b3159e9f15c73dbb45b"></a>
## fmt

`function` · `datafusion_common::join_type::JoinSide::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinSide", "path": "JoinSide"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 10], "end": [239, 15], "filename": "src/join_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/join_type.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07f9666093b70026f1ab4c67"></a>
## negate

`function` · `datafusion_common::join_type::JoinSide::negate` · datafusion-common 55.1.0

```rust
fn negate(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::join_type::JoinSide", "path": "JoinSide"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [259, 2], "filename": "src/join_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/join_type.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Inverse the join side
