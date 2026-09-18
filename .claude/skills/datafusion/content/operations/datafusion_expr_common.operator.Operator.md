# `datafusion_expr_common::operator::Operator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.operator.Operator.json).

<a id="op-6df4d6ff895a2892210a965a"></a>
## Operator

`enum` · `datafusion_expr_common::operator::Operator` · datafusion-expr-common 55.1.0

```rust
enum Operator
```

Source: `src/operator.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Operators applied to expressions

<a id="op-0ad0d04d4866a5da3fe21f0b"></a>
## And

`variant` · `datafusion_expr_common::operator::Operator::And` · datafusion-expr-common 55.1.0

```rust
And
```

Source: `src/operator.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Logical AND

<a id="op-7f6239fbbe9a8fe88141e592"></a>
## Arrow

`variant` · `datafusion_expr_common::operator::Operator::Arrow` · datafusion-expr-common 55.1.0

```rust
Arrow
```

Source: `src/operator.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Arrow, like `->`.

Not implemented in DataFusion yet.

<a id="op-408d0eed4848e614ad8b42f3"></a>
## ArrowAt

`variant` · `datafusion_expr_common::operator::Operator::ArrowAt` · datafusion-expr-common 55.1.0

```rust
ArrowAt
```

Source: `src/operator.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Arrow at, like `<@`.

Currently only supported to be used with lists:
```sql
select [1,3] <@ [1,2,3]
```

<a id="op-29ff026adb0e4c06a6d062c1"></a>
## AtArrow

`variant` · `datafusion_expr_common::operator::Operator::AtArrow` · datafusion-expr-common 55.1.0

```rust
AtArrow
```

Source: `src/operator.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

At arrow, like `@>`.

Currently only supported to be used with lists:
```sql
select [1,2,3] @> [1,3]
```

<a id="op-8c3ce28b24a13d3dab5a816e"></a>
## AtAt

`variant` · `datafusion_expr_common::operator::Operator::AtAt` · datafusion-expr-common 55.1.0

```rust
AtAt
```

Source: `src/operator.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

At at, like `@@`

Not implemented in DataFusion yet.

<a id="op-2f90f1e02b30f8d827edd17d"></a>
## AtQuestion

`variant` · `datafusion_expr_common::operator::Operator::AtQuestion` · datafusion-expr-common 55.1.0

```rust
AtQuestion
```

Source: `src/operator.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

At question, like `@?`

Not implemented in DataFusion yet.

<a id="op-93a942931b2632863bc961fd"></a>
## BitwiseAnd

`variant` · `datafusion_expr_common::operator::Operator::BitwiseAnd` · datafusion-expr-common 55.1.0

```rust
BitwiseAnd
```

Source: `src/operator.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Bitwise and, like `&`

<a id="op-53296500e43be425611a6818"></a>
## BitwiseOr

`variant` · `datafusion_expr_common::operator::Operator::BitwiseOr` · datafusion-expr-common 55.1.0

```rust
BitwiseOr
```

Source: `src/operator.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Bitwise or, like `|`

<a id="op-4041b3181e79fdaf43a64a06"></a>
## BitwiseShiftLeft

`variant` · `datafusion_expr_common::operator::Operator::BitwiseShiftLeft` · datafusion-expr-common 55.1.0

```rust
BitwiseShiftLeft
```

Source: `src/operator.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Bitwise left, like `<<`

<a id="op-9b1ed005d6529b7cff1c6e71"></a>
## BitwiseShiftRight

`variant` · `datafusion_expr_common::operator::Operator::BitwiseShiftRight` · datafusion-expr-common 55.1.0

```rust
BitwiseShiftRight
```

Source: `src/operator.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Bitwise right, like `>>`

<a id="op-9aab001faa2ebeee364a5869"></a>
## BitwiseXor

`variant` · `datafusion_expr_common::operator::Operator::BitwiseXor` · datafusion-expr-common 55.1.0

```rust
BitwiseXor
```

Source: `src/operator.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Bitwise xor, such as `^` in MySQL or `#` in PostgreSQL

<a id="op-e79835f6ce24b51ddf5491c2"></a>
## Colon

`variant` · `datafusion_expr_common::operator::Operator::Colon` · datafusion-expr-common 55.1.0

```rust
Colon
```

Source: `src/operator.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Colon operator, like `:`

Not implemented in DataFusion yet.

<a id="op-6454f9b44b20182286859e22"></a>
## Divide

`variant` · `datafusion_expr_common::operator::Operator::Divide` · datafusion-expr-common 55.1.0

```rust
Divide
```

Source: `src/operator.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Division

<a id="op-da895ec9421f7cfcaae0d583"></a>
## Eq

`variant` · `datafusion_expr_common::operator::Operator::Eq` · datafusion-expr-common 55.1.0

```rust
Eq
```

Source: `src/operator.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Expressions are equal

<a id="op-ba09457bd455640ac8a900ad"></a>
## Gt

`variant` · `datafusion_expr_common::operator::Operator::Gt` · datafusion-expr-common 55.1.0

```rust
Gt
```

Source: `src/operator.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Left side is greater than right side

<a id="op-fce00c00fae5e2faefc21a89"></a>
## GtEq

`variant` · `datafusion_expr_common::operator::Operator::GtEq` · datafusion-expr-common 55.1.0

```rust
GtEq
```

Source: `src/operator.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Left side is greater or equal to right side

<a id="op-2b0c1441133dcf05b58a2ed1"></a>
## HashArrow

`variant` · `datafusion_expr_common::operator::Operator::HashArrow` · datafusion-expr-common 55.1.0

```rust
HashArrow
```

Source: `src/operator.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Hash arrow, like `#>`

Not implemented in DataFusion yet.

<a id="op-3604ca675ec37a3eb402548f"></a>
## HashLongArrow

`variant` · `datafusion_expr_common::operator::Operator::HashLongArrow` · datafusion-expr-common 55.1.0

```rust
HashLongArrow
```

Source: `src/operator.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Hash long arrow, like `#>>`

Not implemented in DataFusion yet.

<a id="op-0a5a2e6a37f9f9a4ce404ff2"></a>
## HashMinus

`variant` · `datafusion_expr_common::operator::Operator::HashMinus` · datafusion-expr-common 55.1.0

```rust
HashMinus
```

Source: `src/operator.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Hash Minus, like `#-`

Not implemented in DataFusion yet.

<a id="op-41677814f5f0b63c7e193980"></a>
## ILikeMatch

`variant` · `datafusion_expr_common::operator::Operator::ILikeMatch` · datafusion-expr-common 55.1.0

```rust
ILikeMatch
```

Source: `src/operator.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case insensitive pattern match

<a id="op-8f10b2649c8fb00e94c62d5a"></a>
## IntegerDivide

`variant` · `datafusion_expr_common::operator::Operator::IntegerDivide` · datafusion-expr-common 55.1.0

```rust
IntegerDivide
```

Source: `src/operator.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Integer division operator, like `DIV` from MySQL or `//` from DuckDB

Not implemented in DataFusion yet.

<a id="op-9af446c9692767ffe0a8661d"></a>
## IsDistinctFrom

`variant` · `datafusion_expr_common::operator::Operator::IsDistinctFrom` · datafusion-expr-common 55.1.0

```rust
IsDistinctFrom
```

Source: `src/operator.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

`IS DISTINCT FROM` (see [`distinct`])

[`distinct`]: arrow::compute::kernels::cmp::distinct

<a id="op-157cad8a6c4bca1b14e4984e"></a>
## IsNotDistinctFrom

`variant` · `datafusion_expr_common::operator::Operator::IsNotDistinctFrom` · datafusion-expr-common 55.1.0

```rust
IsNotDistinctFrom
```

Source: `src/operator.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

`IS NOT DISTINCT FROM` (see [`not_distinct`])

[`not_distinct`]: arrow::compute::kernels::cmp::not_distinct

<a id="op-5f339827128476f114b549de"></a>
## LikeMatch

`variant` · `datafusion_expr_common::operator::Operator::LikeMatch` · datafusion-expr-common 55.1.0

```rust
LikeMatch
```

Source: `src/operator.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case sensitive pattern match

<a id="op-c0d25ef0cd67e7c6c280e2bf"></a>
## LongArrow

`variant` · `datafusion_expr_common::operator::Operator::LongArrow` · datafusion-expr-common 55.1.0

```rust
LongArrow
```

Source: `src/operator.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Long arrow, like `->>`

Not implemented in DataFusion yet.

<a id="op-cd6acf014049368f8f713cfc"></a>
## Lt

`variant` · `datafusion_expr_common::operator::Operator::Lt` · datafusion-expr-common 55.1.0

```rust
Lt
```

Source: `src/operator.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Left side is smaller than right side

<a id="op-fbf3b4422ce6720aa8c06a26"></a>
## LtEq

`variant` · `datafusion_expr_common::operator::Operator::LtEq` · datafusion-expr-common 55.1.0

```rust
LtEq
```

Source: `src/operator.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Left side is smaller or equal to right side

<a id="op-c2b2bfb269428777061d27c2"></a>
## Minus

`variant` · `datafusion_expr_common::operator::Operator::Minus` · datafusion-expr-common 55.1.0

```rust
Minus
```

Source: `src/operator.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Subtraction

<a id="op-4a1ca252aebaa8e3dcb8640b"></a>
## Modulo

`variant` · `datafusion_expr_common::operator::Operator::Modulo` · datafusion-expr-common 55.1.0

```rust
Modulo
```

Source: `src/operator.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Remainder

<a id="op-604d75e6a7f16bec22d573aa"></a>
## Multiply

`variant` · `datafusion_expr_common::operator::Operator::Multiply` · datafusion-expr-common 55.1.0

```rust
Multiply
```

Source: `src/operator.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Multiplication

<a id="op-28ed5922e3b6f5c50800e747"></a>
## NotEq

`variant` · `datafusion_expr_common::operator::Operator::NotEq` · datafusion-expr-common 55.1.0

```rust
NotEq
```

Source: `src/operator.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Expressions are not equal

<a id="op-4846bee8a4d05cb996a56e1c"></a>
## NotILikeMatch

`variant` · `datafusion_expr_common::operator::Operator::NotILikeMatch` · datafusion-expr-common 55.1.0

```rust
NotILikeMatch
```

Source: `src/operator.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case insensitive pattern not match

<a id="op-9583af5d5694f0229b2fb0ba"></a>
## NotLikeMatch

`variant` · `datafusion_expr_common::operator::Operator::NotLikeMatch` · datafusion-expr-common 55.1.0

```rust
NotLikeMatch
```

Source: `src/operator.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case sensitive pattern not match

<a id="op-81f2d2ce4e5956994336bc7e"></a>
## Or

`variant` · `datafusion_expr_common::operator::Operator::Or` · datafusion-expr-common 55.1.0

```rust
Or
```

Source: `src/operator.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Logical OR

<a id="op-0a702bcd3e25f766bb449762"></a>
## Plus

`variant` · `datafusion_expr_common::operator::Operator::Plus` · datafusion-expr-common 55.1.0

```rust
Plus
```

Source: `src/operator.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Addition

<a id="op-38a05ab8f45e812c50ab7117"></a>
## Question

`variant` · `datafusion_expr_common::operator::Operator::Question` · datafusion-expr-common 55.1.0

```rust
Question
```

Source: `src/operator.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Question, like `?`

Not implemented in DataFusion yet.

<a id="op-020c95cd4612aafa7b459332"></a>
## QuestionAnd

`variant` · `datafusion_expr_common::operator::Operator::QuestionAnd` · datafusion-expr-common 55.1.0

```rust
QuestionAnd
```

Source: `src/operator.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Question and, like `?&`

Not implemented in DataFusion yet.

<a id="op-f3d5f87ded95329918830104"></a>
## QuestionPipe

`variant` · `datafusion_expr_common::operator::Operator::QuestionPipe` · datafusion-expr-common 55.1.0

```rust
QuestionPipe
```

Source: `src/operator.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Question pipe, like `?|`

Not implemented in DataFusion yet.

<a id="op-1cb1e2da049e5495e7e7f598"></a>
## RegexIMatch

`variant` · `datafusion_expr_common::operator::Operator::RegexIMatch` · datafusion-expr-common 55.1.0

```rust
RegexIMatch
```

Source: `src/operator.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case insensitive regex match

<a id="op-9fe74578fe322e2fe19b2232"></a>
## RegexMatch

`variant` · `datafusion_expr_common::operator::Operator::RegexMatch` · datafusion-expr-common 55.1.0

```rust
RegexMatch
```

Source: `src/operator.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case sensitive regex match

<a id="op-9ec5a55ba7e8832efe5a54cd"></a>
## RegexNotIMatch

`variant` · `datafusion_expr_common::operator::Operator::RegexNotIMatch` · datafusion-expr-common 55.1.0

```rust
RegexNotIMatch
```

Source: `src/operator.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case insensitive regex not match

<a id="op-0afdebf66eda02b374383542"></a>
## RegexNotMatch

`variant` · `datafusion_expr_common::operator::Operator::RegexNotMatch` · datafusion-expr-common 55.1.0

```rust
RegexNotMatch
```

Source: `src/operator.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Case sensitive regex not match

<a id="op-7191f399584982f1729c9981"></a>
## StringConcat

`variant` · `datafusion_expr_common::operator::Operator::StringConcat` · datafusion-expr-common 55.1.0

```rust
StringConcat
```

Source: `src/operator.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

String concatenation, like `||`

<a id="op-9dbca385176d9a8377b6160a"></a>
## clone

`function` · `datafusion_expr_common::operator::Operator::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> Operator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 23], "end": [21, 28], "filename": "src/operator.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/operator.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a22a5b2b6c6b89a41e902ed"></a>
## eq

`function` · `datafusion_expr_common::operator::Operator::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Operator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 30], "end": [21, 39], "filename": "src/operator.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/operator.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96f136a01a16c459f2b60748"></a>
## fmt

`function` · `datafusion_expr_common::operator::Operator::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [438, 1], "end": [487, 2], "filename": "src/operator.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/operator.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7abd7def80a852a4a607e8d"></a>
## fmt

`function` · `datafusion_expr_common::operator::Operator::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "src/operator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b54db790d0e9c9a1b0bb7608"></a>
## from_proto_name

`function` · `datafusion_expr_common::operator::Operator::from_proto_name` · datafusion-expr-common 55.1.0

```rust
fn from_proto_name(name: &str) -> Option<Operator>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Parse an `Operator` from the string name `datafusion-proto` uses on the
wire (the `Debug` name of the variant, e.g. `"Eq"`).

Returns `None` for names with no binary-operator counterpart. This is
the canonical proto-string mapping, shared by `datafusion-proto`
(logical plans) and `PhysicalExpr` decoders such as `BinaryExpr`, so the
mapping is not duplicated across crates.

<a id="op-100d2ce200e6ecb6e872b76f"></a>
## hash

`function` · `datafusion_expr_common::operator::Operator::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 57], "end": [21, 61], "filename": "src/operator.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/operator.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-700faa23bb7e2ddcb4ea3f4b"></a>
## is_logic_operator

`function` · `datafusion_expr_common::operator::Operator::is_logic_operator` · datafusion-expr-common 55.1.0

```rust
fn is_logic_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return true if the operator is a logic operator.

For example, 'Binary(Binary(a, >, b), AND, Binary(a, <, b + 3))' would
be a logical expression.

<a id="op-ff925604fdc87fcc3d3bea2f"></a>
## is_numerical_operators

`function` · `datafusion_expr_common::operator::Operator::is_numerical_operators` · datafusion-expr-common 55.1.0

```rust
fn is_numerical_operators(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return true if the operator is a numerical operator.

For example, 'Binary(a, +, b)' would be a numerical expression.
PostgresSQL concept: <https://www.postgresql.org/docs/7.0/operators2198.htm>

<a id="op-7b98a849b4352b26a1ffdfff"></a>
## negate

`function` · `datafusion_expr_common::operator::Operator::negate` · datafusion-expr-common 55.1.0

```rust
fn negate(&self) -> Option<Operator>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

If the operator can be negated, return the negated operator
otherwise return None

<a id="op-c4ad89599c290c182cca1ffa"></a>
## partial_cmp

`function` · `datafusion_expr_common::operator::Operator::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &Operator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 45], "end": [21, 55], "filename": "src/operator.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/operator.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cf2260edd8f0f64ca36f182"></a>
## precedence

`function` · `datafusion_expr_common::operator::Operator::precedence` · datafusion-expr-common 55.1.0

```rust
fn precedence(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Get the operator precedence
use <https://www.postgresql.org/docs/7.2/sql-precedence.html> as a reference

<a id="op-cee37765f1ef7a7621d5702f"></a>
## returns_null_on_null

`function` · `datafusion_expr_common::operator::Operator::returns_null_on_null` · datafusion-expr-common 55.1.0

```rust
fn returns_null_on_null(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns true if the `Expr::BinaryOperator` with this operator
is guaranteed to return null if either side is null.

<a id="op-97fd1b2df34d0e5cfcf5cf4d"></a>
## supports_propagation

`function` · `datafusion_expr_common::operator::Operator::supports_propagation` · datafusion-expr-common 55.1.0

```rust
fn supports_propagation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return true if the comparison operator can be used in interval arithmetic and constraint
propagation

For example, 'Binary(a, >, b)' expression supports propagation.

<a id="op-e133ca01564946d487805345"></a>
## swap

`function` · `datafusion_expr_common::operator::Operator::swap` · datafusion-expr-common 55.1.0

```rust
fn swap(&self) -> Option<Operator>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::operator::Operator", "path": "Operator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [436, 2], "filename": "src/operator.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return the operator where swapping lhs and rhs wouldn't change the result.

For example `Binary(50, >=, a)` could also be represented as `Binary(a, <=, 50)`.
