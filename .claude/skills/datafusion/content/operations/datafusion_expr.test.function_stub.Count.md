# `datafusion_expr::test::function_stub::Count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.test.function_stub.Count.json).

<a id="op-56f5a6c9ca0e2aeaad323abd"></a>
## Count

`struct` · `datafusion_expr::test::function_stub::Count` · datafusion-expr 55.1.0

```rust
struct Count
```

Source: `src/test/function_stub.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Testing stub implementation of COUNT aggregate

<a id="op-232d8da8e41bf14916e1ce1a"></a>
## accumulator

`function` · `datafusion_expr::test::function_stub::Count::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8aa1e597fecdaa4e69cda5d8"></a>
## aliases

`function` · `datafusion_expr::test::function_stub::Count::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d724ba619035f649a8adaa0b"></a>
## create_groups_accumulator

`function` · `datafusion_expr::test::function_stub::Count::create_groups_accumulator` · datafusion-expr 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1914d275457fe146a555bda"></a>
## default

`function` · `datafusion_expr::test::function_stub::Count::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [231, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/function_stub.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ae53552c0cabcb1ababede7"></a>
## eq

`function` · `datafusion_expr::test::function_stub::Count::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Count) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 10], "end": [212, 19], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/test/function_stub.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-869effe85d8f223cb381eed7"></a>
## fmt

`function` · `datafusion_expr::test::function_stub::Count::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 1], "end": [225, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/function_stub.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8007eea5fc032eb04281f549"></a>
## hash

`function` · `datafusion_expr::test::function_stub::Count::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 25], "end": [212, 29], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/test/function_stub.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfa5afc5c5c3b14f6564bc62"></a>
## is_nullable

`function` · `datafusion_expr::test::function_stub::Count::is_nullable` · datafusion-expr 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5c6d9c798502a74a0b9fc22"></a>
## name

`function` · `datafusion_expr::test::function_stub::Count::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d49cca03e1aba5e0f5a5b90"></a>
## new

`function` · `datafusion_expr::test::function_stub::Count::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 1], "end": [240, 2], "filename": "src/test/function_stub.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/function_stub.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59d57f03558b0b33075fa987"></a>
## return_type

`function` · `datafusion_expr::test::function_stub::Count::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7522ddff826140e803586125"></a>
## reverse_expr

`function` · `datafusion_expr::test::function_stub::Count::reverse_expr` · datafusion-expr 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4dc82a307feb007586b234f"></a>
## signature

`function` · `datafusion_expr::test::function_stub::Count::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34cfba998236f7755a0b594d"></a>
## state_fields

`function` · `datafusion_expr::test::function_stub::Count::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 1], "end": [281, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
