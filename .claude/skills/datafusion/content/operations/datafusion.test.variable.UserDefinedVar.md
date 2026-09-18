# `datafusion::test::variable::UserDefinedVar`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.variable.UserDefinedVar.json).

<a id="op-c1e02644f826e9c9ba7aa592"></a>
## UserDefinedVar

`struct` · `datafusion::test::variable::UserDefinedVar` · datafusion 55.1.0

```rust
struct UserDefinedVar
```

Source: `src/test/variable.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

user defined variable

<a id="op-a9e7f4c669f805e949039b2c"></a>
## default

`function` · `datafusion::test::variable::UserDefinedVar::default` · datafusion 55.1.0

```rust
fn default() -> UserDefinedVar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::UserDefinedVar", "path": "UserDefinedVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 17], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/variable.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c7d59c973b267d1410bfb45"></a>
## fmt

`function` · `datafusion::test::variable::UserDefinedVar::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::UserDefinedVar", "path": "UserDefinedVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 19], "end": [49, 24], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/variable.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46590a6f301f799c2fdf5db7"></a>
## get_type

`function` · `datafusion::test::variable::UserDefinedVar::get_type` · datafusion 55.1.0

```rust
fn get_type(&self, var_names: &[String]) -> Option<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::UserDefinedVar", "path": "UserDefinedVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [77, 2], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "datafusion_expr::var_provider::VarProvider", "path": "VarProvider"}, "trait_path": "datafusion_expr::var_provider::VarProvider"}`

Source: `src/test/variable.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f04f2f3d97eed5c2b6720b5b"></a>
## get_value

`function` · `datafusion::test::variable::UserDefinedVar::get_value` · datafusion 55.1.0

```rust
fn get_value(&self, var_names: Vec<String>) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::UserDefinedVar", "path": "UserDefinedVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [77, 2], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "datafusion_expr::var_provider::VarProvider", "path": "VarProvider"}, "trait_path": "datafusion_expr::var_provider::VarProvider"}`

Source: `src/test/variable.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get user defined variable value

<a id="op-f76db7da622b14847ea66476"></a>
## new

`function` · `datafusion::test::variable::UserDefinedVar::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::UserDefinedVar", "path": "UserDefinedVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [57, 2], "filename": "src/test/variable.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/variable.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

new user defined variable
