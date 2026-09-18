# `datafusion_functions_window::nth_value::NthValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.nth_value.NthValue.json).

<a id="op-e6e43042817e1fa464fa6815"></a>
## NthValue

`struct` · `datafusion_functions_window::nth_value::NthValue` · datafusion-functions-window 55.1.0

```rust
struct NthValue
```

Source: `src/nth_value.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10281699347b3af7f9cd396c"></a>
## documentation

`function` · `datafusion_functions_window::nth_value::NthValue::documentation` · datafusion-functions-window 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [357, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/nth_value.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b12b5d01c0bf9c17f58c00ca"></a>
## eq

`function` · `datafusion_functions_window::nth_value::NthValue::eq` · datafusion-functions-window 55.1.0

```rust
fn eq(&self, other: &NthValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 17], "end": [90, 26], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/nth_value.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-332648c10f537c9a4313b9ed"></a>
## field

`function` · `datafusion_functions_window::nth_value::NthValue::field` · datafusion-functions-window 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [357, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/nth_value.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b459121cc3aa9d5edb6a54b4"></a>
## first

`function` · `datafusion_functions_window::nth_value::NthValue::first` · datafusion-functions-window 55.1.0

```rust
fn first() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [126, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8f616c022c232551c75f63"></a>
## fmt

`function` · `datafusion_functions_window::nth_value::NthValue::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 10], "end": [90, 15], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/nth_value.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad2dd181a8def50c64902da7"></a>
## hash

`function` · `datafusion_functions_window::nth_value::NthValue::hash` · datafusion-functions-window 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 32], "end": [90, 36], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/nth_value.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85620e7f66077a630f70a508"></a>
## kind

`function` · `datafusion_functions_window::nth_value::NthValue::kind` · datafusion-functions-window 55.1.0

```rust
fn kind(&self) -> &NthValueKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [126, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eae2159b6549773fc7647d85"></a>
## last

`function` · `datafusion_functions_window::nth_value::NthValue::last` · datafusion-functions-window 55.1.0

```rust
fn last() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [126, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1674b2351ea729f303e0d11"></a>
## limit_effect

`function` · `datafusion_functions_window::nth_value::NthValue::limit_effect` · datafusion-functions-window 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [357, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/nth_value.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cc653d4181b2bb30fadfec3"></a>
## name

`function` · `datafusion_functions_window::nth_value::NthValue::name` · datafusion-functions-window 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [357, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/nth_value.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d30f1f341a1303e73ac82c64"></a>
## new

`function` · `datafusion_functions_window::nth_value::NthValue::new` · datafusion-functions-window 55.1.0

```rust
fn new(kind: NthValueKind) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [126, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a new `nth_value` function

<a id="op-e7b757eb35fdb6095c3a85d0"></a>
## nth

`function` · `datafusion_functions_window::nth_value::NthValue::nth` · datafusion-functions-window 55.1.0

```rust
fn nth() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [126, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a315f18f6d9dff8d4b5ca995"></a>
## partition_evaluator

`function` · `datafusion_functions_window::nth_value::NthValue::partition_evaluator` · datafusion-functions-window 55.1.0

```rust
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [357, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/nth_value.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cab8b43c01c157e58a61f252"></a>
## reverse_expr

`function` · `datafusion_functions_window::nth_value::NthValue::reverse_expr` · datafusion-functions-window 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDWF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [357, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/nth_value.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d92ad128fa3e4bbb65ad78e"></a>
## signature

`function` · `datafusion_functions_window::nth_value::NthValue::signature` · datafusion-functions-window 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValue", "path": "NthValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [357, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/nth_value.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
