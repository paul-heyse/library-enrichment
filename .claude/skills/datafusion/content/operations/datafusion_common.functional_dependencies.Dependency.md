# `datafusion_common::functional_dependencies::Dependency`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.Dependency.json).

<a id="op-2b98b96c4aecda138f44bc6f"></a>
## Dependency

`enum` · `datafusion_common::functional_dependencies::Dependency` · datafusion-common 55.1.0

```rust
enum Dependency
```

Source: `src/functional_dependencies.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Describes functional dependency mode.

<a id="op-c928244e98cc46f5af648701"></a>
## Multi

`variant` · `datafusion_common::functional_dependencies::Dependency::Multi` · datafusion-common 55.1.0

```rust
Multi
```

Source: `src/functional_dependencies.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A determinant key may occur multiple times (in multiple rows).

<a id="op-804d04e652430f98334b2bf6"></a>
## Single

`variant` · `datafusion_common::functional_dependencies::Dependency::Single` · datafusion-common 55.1.0

```rust
Single
```

Source: `src/functional_dependencies.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A determinant key may occur only once.

<a id="op-254c92a07ebeb34b5759f809"></a>
## clone

`function` · `datafusion_common::functional_dependencies::Dependency::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Dependency
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Dependency", "path": "Dependency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 17], "end": [152, 22], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/functional_dependencies.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-171aa4ad0fdfeb485a7c5af8"></a>
## eq

`function` · `datafusion_common::functional_dependencies::Dependency::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Dependency) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Dependency", "path": "Dependency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 30], "end": [152, 39], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/functional_dependencies.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0733cf8a368c125c917a6612"></a>
## fmt

`function` · `datafusion_common::functional_dependencies::Dependency::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::functional_dependencies::Dependency", "path": "Dependency"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 10], "end": [152, 15], "filename": "src/functional_dependencies.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/functional_dependencies.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
