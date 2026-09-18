# `datafusion_expr::test::function_stub::Avg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.test.function_stub.Avg.json).

<a id="op-0b9143f36b742198a5b87e05"></a>
## Avg

`struct` · `datafusion_expr::test::function_stub::Avg` · datafusion-expr 55.1.0

```rust
struct Avg
```

Source: `src/test/function_stub.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Testing stub implementation of avg aggregate

<a id="op-b63c079d2e10a4d23bb5a711"></a>
## accumulator

`function` · `datafusion_expr::test::function_stub::Avg::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [551, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:540`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f9f79d5e1e580a3264730d3"></a>
## aliases

`function` · `datafusion_expr::test::function_stub::Avg::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [551, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:548`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-697a629e74e67e0d57e456d9"></a>
## coerce_types

`function` · `datafusion_expr::test::function_stub::Avg::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [551, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d69a43e7ee5ea572e33d54a7"></a>
## default

`function` · `datafusion_expr::test::function_stub::Avg::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 1], "end": [472, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/function_stub.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0ebaea923113ec8aa615136"></a>
## eq

`function` · `datafusion_expr::test::function_stub::Avg::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Avg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 17], "end": [440, 26], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/test/function_stub.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-245a11430f9d47c117b57be6"></a>
## fmt

`function` · `datafusion_expr::test::function_stub::Avg::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 10], "end": [440, 15], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/function_stub.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3de80719a67d96331fbab61"></a>
## hash

`function` · `datafusion_expr::test::function_stub::Avg::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [440, 32], "end": [440, 36], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/test/function_stub.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24d1a210a4eba37fef296ab3"></a>
## name

`function` · `datafusion_expr::test::function_stub::Avg::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [551, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-559b424da7a63ce5009afa6a"></a>
## new

`function` · `datafusion_expr::test::function_stub::Avg::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [446, 1], "end": [466, 2], "filename": "src/test/function_stub.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/function_stub.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfda8861a9234d5c425aff69"></a>
## return_type

`function` · `datafusion_expr::test::function_stub::Avg::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [551, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-249f46eb4a1b8b5163e4461d"></a>
## signature

`function` · `datafusion_expr::test::function_stub::Avg::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [551, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:479`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0c617fc134c6d196e4574c0"></a>
## state_fields

`function` · `datafusion_expr::test::function_stub::Avg::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [474, 1], "end": [551, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
