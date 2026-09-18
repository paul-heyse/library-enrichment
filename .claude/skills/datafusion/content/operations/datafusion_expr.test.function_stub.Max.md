# `datafusion_expr::test::function_stub::Max`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.test.function_stub.Max.json).

<a id="op-2ef0593dc70189b64e0ff389"></a>
## Max

`struct` · `datafusion_expr::test::function_stub::Max` · datafusion-expr 55.1.0

```rust
struct Max
```

Source: `src/test/function_stub.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Testing stub implementation of MAX aggregate

<a id="op-47048c4550d8eddb811c2825"></a>
## accumulator

`function` · `datafusion_expr::test::function_stub::Max::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c50ae628d1db1e85765dd7"></a>
## create_groups_accumulator

`function` · `datafusion_expr::test::function_stub::Max::create_groups_accumulator` · datafusion-expr 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac4556b20f6839da72be6dcc"></a>
## default

`function` · `datafusion_expr::test::function_stub::Max::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [393, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/function_stub.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddee723a71addcf04136a481"></a>
## eq

`function` · `datafusion_expr::test::function_stub::Max::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Max) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 10], "end": [375, 19], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/test/function_stub.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6966ea7e75322bb357a40ab9"></a>
## fmt

`function` · `datafusion_expr::test::function_stub::Max::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [387, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/function_stub.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7d654f336e86fe410739dae"></a>
## hash

`function` · `datafusion_expr::test::function_stub::Max::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 25], "end": [375, 29], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/test/function_stub.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed10b2698153669de67a0b18"></a>
## is_descending

`function` · `datafusion_expr::test::function_stub::Max::is_descending` · datafusion-expr 55.1.0

```rust
fn is_descending(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3300fd48fc1b120698dd915e"></a>
## name

`function` · `datafusion_expr::test::function_stub::Max::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-147380902cf55bc68e7bb50f"></a>
## new

`function` · `datafusion_expr::test::function_stub::Max::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [395, 1], "end": [401, 2], "filename": "src/test/function_stub.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/function_stub.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3da4f88ca91f239d85e5c84"></a>
## return_type

`function` · `datafusion_expr::test::function_stub::Max::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34f426e3dc708f4b6986e7d8"></a>
## reverse_expr

`function` · `datafusion_expr::test::function_stub::Max::reverse_expr` · datafusion-expr 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9193f5c32120297cba0fa50"></a>
## signature

`function` · `datafusion_expr::test::function_stub::Max::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:408`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4f4934464910eb4887b73a6"></a>
## state_fields

`function` · `datafusion_expr::test::function_stub::Max::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [437, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
