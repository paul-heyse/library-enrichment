# `datafusion_expr::var_provider::VarType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.var_provider.VarType.json).

<a id="op-0a70a9bb25e00f52741ed0b3"></a>
## VarType

`enum` · `datafusion_expr::var_provider::VarType` · datafusion-expr 55.1.0

```rust
enum VarType
```

Source: `src/var_provider.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Variable type, system/user defined

<a id="op-1aae0192811ef3548ecdb18a"></a>
## System

`variant` · `datafusion_expr::var_provider::VarType::System` · datafusion-expr 55.1.0

```rust
System
```

Source: `src/var_provider.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

System variable, like @@version

<a id="op-9613a3e510ed465f98e4a5bc"></a>
## UserDefined

`variant` · `datafusion_expr::var_provider::VarType::UserDefined` · datafusion-expr 55.1.0

```rust
UserDefined
```

Source: `src/var_provider.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

User defined variable, like @name

<a id="op-6a0ce8847095b5bb420aa1dc"></a>
## clone

`function` · `datafusion_expr::var_provider::VarType::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> VarType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::var_provider::VarType", "path": "VarType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 17], "end": [24, 22], "filename": "src/var_provider.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/var_provider.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02a8a9d8b9cfcbbd915d3d93"></a>
## eq

`function` · `datafusion_expr::var_provider::VarType::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &VarType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::var_provider::VarType", "path": "VarType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 24], "end": [24, 33], "filename": "src/var_provider.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/var_provider.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b9f70cf3b96a4635ab2a35b"></a>
## fmt

`function` · `datafusion_expr::var_provider::VarType::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::var_provider::VarType", "path": "VarType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 10], "end": [24, 15], "filename": "src/var_provider.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/var_provider.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72b81ba421b8e43b95822121"></a>
## hash

`function` · `datafusion_expr::var_provider::VarType::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::var_provider::VarType", "path": "VarType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 39], "end": [24, 43], "filename": "src/var_provider.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/var_provider.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
