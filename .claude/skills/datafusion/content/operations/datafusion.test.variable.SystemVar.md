# `datafusion::test::variable::SystemVar`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.variable.SystemVar.json).

<a id="op-a61f1995f62e37c810915457"></a>
## SystemVar

`struct` · `datafusion::test::variable::SystemVar` · datafusion 55.1.0

```rust
struct SystemVar
```

Source: `src/test/variable.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

System variable

<a id="op-5b1d82eced3a07bd8f97fcf4"></a>
## default

`function` · `datafusion::test::variable::SystemVar::default` · datafusion 55.1.0

```rust
fn default() -> SystemVar
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::SystemVar", "path": "SystemVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 17], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/variable.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1401a22b918c39aa11eb380"></a>
## fmt

`function` · `datafusion::test::variable::SystemVar::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::SystemVar", "path": "SystemVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 19], "end": [26, 24], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/variable.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea70eda5f8f17b32447168fb"></a>
## get_type

`function` · `datafusion::test::variable::SystemVar::get_type` · datafusion 55.1.0

```rust
fn get_type(&self, _: &[String]) -> Option<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::SystemVar", "path": "SystemVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [46, 2], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "datafusion_expr::var_provider::VarProvider", "path": "VarProvider"}, "trait_path": "datafusion_expr::var_provider::VarProvider"}`

Source: `src/test/variable.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e98912d4d6f93f159251155e"></a>
## get_value

`function` · `datafusion::test::variable::SystemVar::get_value` · datafusion 55.1.0

```rust
fn get_value(&self, var_names: Vec<String>) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::SystemVar", "path": "SystemVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [46, 2], "filename": "src/test/variable.rs"}, "trait": {"args": null, "id": "datafusion_expr::var_provider::VarProvider", "path": "VarProvider"}, "trait_path": "datafusion_expr::var_provider::VarProvider"}`

Source: `src/test/variable.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

get system variable value

<a id="op-e41af0b7c5ffb32af85a6073"></a>
## new

`function` · `datafusion::test::variable::SystemVar::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test::variable::SystemVar", "path": "SystemVar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [34, 2], "filename": "src/test/variable.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/variable.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

new system variable
