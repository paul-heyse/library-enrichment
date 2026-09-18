# `datafusion_functions::string::btrim::BTrimFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.btrim.BTrimFunc.json).

<a id="op-e0ed77eac92b0cdaacf3878b"></a>
## BTrimFunc

`struct` · `datafusion_functions::string::btrim::BTrimFunc` · datafusion-functions 55.1.0

```rust
struct BTrimFunc
```

Source: `src/string/btrim.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20ab32f2b121d4b413e9ddd5"></a>
## aliases

`function` · `datafusion_functions::string::btrim::BTrimFunc::aliases` · datafusion-functions 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [137, 2], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/btrim.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dd26188e66886cf2ef964ef"></a>
## default

`function` · `datafusion_functions::string::btrim::BTrimFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [88, 2], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/btrim.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07fbae136d7fea82dcd62146"></a>
## documentation

`function` · `datafusion_functions::string::btrim::BTrimFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [137, 2], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/btrim.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d647f71ef653a70d9006551"></a>
## eq

`function` · `datafusion_functions::string::btrim::BTrimFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &BTrimFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 17], "end": [78, 26], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/btrim.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-deef5f3636166ba4e6a0afe2"></a>
## fmt

`function` · `datafusion_functions::string::btrim::BTrimFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 10], "end": [78, 15], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/btrim.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99cad9095de04119b740a333"></a>
## hash

`function` · `datafusion_functions::string::btrim::BTrimFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 32], "end": [78, 36], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/btrim.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44a78bbf610987a839f6133a"></a>
## invoke_with_args

`function` · `datafusion_functions::string::btrim::BTrimFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [137, 2], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/btrim.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd462e19b8f626e6fff3b2d2"></a>
## name

`function` · `datafusion_functions::string::btrim::BTrimFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [137, 2], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/btrim.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7a9a95ae19a317beffefdd5"></a>
## new

`function` · `datafusion_functions::string::btrim::BTrimFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [111, 2], "filename": "src/string/btrim.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/btrim.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ce6d0e457d4b00b370c695d"></a>
## return_type

`function` · `datafusion_functions::string::btrim::BTrimFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [137, 2], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/btrim.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-552ae1e723a8aed6dbf70d7d"></a>
## signature

`function` · `datafusion_functions::string::btrim::BTrimFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::btrim::BTrimFunc", "path": "BTrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [137, 2], "filename": "src/string/btrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/btrim.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
