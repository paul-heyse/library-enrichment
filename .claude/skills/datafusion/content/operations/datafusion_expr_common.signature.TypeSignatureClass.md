# `datafusion_expr_common::signature::TypeSignatureClass`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.TypeSignatureClass.json).

<a id="op-dffc1e98d93d952fdb6f02ca"></a>
## TypeSignatureClass

`enum` · `datafusion_expr_common::signature::TypeSignatureClass` · datafusion-expr-common 55.1.0

```rust
enum TypeSignatureClass
```

Source: `src/signature.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Represents the class of types that can be used in a function signature.

This is used to specify what types are valid for function arguments in a more flexible way than
just listing specific DataTypes. For example, TypeSignatureClass::Timestamp matches any timestamp
type regardless of timezone or precision.

Used primarily with [`TypeSignature::Coercible`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-3487ca51800f476df4c4ab66) to define function signatures that can accept
arguments that can be coerced to a particular class of types.

<a id="op-3d7e45d3b17acbcd244cd8ea"></a>
## Any

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Any` · datafusion-expr-common 55.1.0

```rust
Any
```

Source: `src/signature.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Allows an arbitrary type argument without coercing the argument.

<a id="op-cdeb3b08061fef8358a82ed2"></a>
## Binary

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Binary` · datafusion-expr-common 55.1.0

```rust
Binary
```

Source: `src/signature.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Encompasses both the native Binary/LargeBinary types as well as arbitrarily sized FixedSizeBinary types

<a id="op-9046c87573eacd1d29f38b20"></a>
## Decimal

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Decimal` · datafusion-expr-common 55.1.0

```rust
Decimal
```

Source: `src/signature.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

All decimal types, allowing arbitrary precision & scale

<a id="op-203a401ad0c26900b7170ced"></a>
## Duration

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Duration` · datafusion-expr-common 55.1.0

```rust
Duration
```

Source: `src/signature.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

All duration types

<a id="op-fb99c89abf16b50da20363f5"></a>
## Float

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Float` · datafusion-expr-common 55.1.0

```rust
Float
```

Source: `src/signature.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

All float types

<a id="op-0a93409845247d59e9bbfca7"></a>
## Integer

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Integer` · datafusion-expr-common 55.1.0

```rust
Integer
```

Source: `src/signature.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Signed and unsigned integers

<a id="op-61847ae04308c1504d0cc1a8"></a>
## Interval

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Interval` · datafusion-expr-common 55.1.0

```rust
Interval
```

Source: `src/signature.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

All interval types

<a id="op-6ad1c630e0749e8492378f1e"></a>
## Native

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Native` · datafusion-expr-common 55.1.0

```rust
Native
```

Source: `src/signature.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A specific native type

<a id="op-e4d40c994fdb156de4767d05"></a>
## Numeric

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Numeric` · datafusion-expr-common 55.1.0

```rust
Numeric
```

Source: `src/signature.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Integers, floats and decimals

<a id="op-14bb44ff3898a9cab5947594"></a>
## Time

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Time` · datafusion-expr-common 55.1.0

```rust
Time
```

Source: `src/signature.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

All time types

<a id="op-da4350292427df5bea27ed36"></a>
## Timestamp

`variant` · `datafusion_expr_common::signature::TypeSignatureClass::Timestamp` · datafusion-expr-common 55.1.0

```rust
Timestamp
```

Source: `src/signature.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Timestamps, allowing arbitrary (or no) timezones

<a id="op-32076f184a9a6cf8be3954cc"></a>
## clone

`function` · `datafusion_expr_common::signature::TypeSignatureClass::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> TypeSignatureClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 17], "end": [370, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-661febdeeb25fa94ca9c75f6"></a>
## default_casted_type

`function` · `datafusion_expr_common::signature::TypeSignatureClass::default_casted_type` · datafusion-expr-common 55.1.0

```rust
fn default_casted_type(&self, native_type: &NativeType, origin_type: &DataType) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [524, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

What type would `origin_type` be casted to when casting to the specified native type?

<a id="op-70f246a5797e11750083190d"></a>
## eq

`function` · `datafusion_expr_common::signature::TypeSignatureClass::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &TypeSignatureClass) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 28], "end": [370, 37], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca7262d5bf9411b380481821"></a>
## fmt

`function` · `datafusion_expr_common::signature::TypeSignatureClass::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 10], "end": [370, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e927c5f03c90dd38679dffea"></a>
## fmt

`function` · `datafusion_expr_common::signature::TypeSignatureClass::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [396, 1], "end": [412, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/signature.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e36e7c72abede259ad5419f2"></a>
## hash

`function` · `datafusion_expr_common::signature::TypeSignatureClass::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 51], "end": [370, 55], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0344faa9e19a0bc876eeee6f"></a>
## matches_native_type

`function` · `datafusion_expr_common::signature::TypeSignatureClass::matches_native_type` · datafusion-expr-common 55.1.0

```rust
fn matches_native_type(&self, logical_type: &NativeType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [524, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Does the specified `NativeType` match this type signature class?

<a id="op-d8b826ca8434149574f5bfb3"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::TypeSignatureClass::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &TypeSignatureClass) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignatureClass", "path": "TypeSignatureClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [370, 39], "end": [370, 49], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
