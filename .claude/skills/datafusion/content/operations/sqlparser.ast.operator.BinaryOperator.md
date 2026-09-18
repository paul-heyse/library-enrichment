# `sqlparser::ast::operator::BinaryOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.operator.BinaryOperator.json).

<a id="op-ac3b15536387287ff523ef00"></a>
## BinaryOperator

`enum` · `sqlparser::ast::operator::BinaryOperator` · sqlparser 0.62.0

```rust
enum BinaryOperator
```

Source: `src/ast/operator.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Binary operators

<a id="op-bc9aebfd91fcb79f77e1eba3"></a>
## And

`variant` · `sqlparser::ast::operator::BinaryOperator::And` · sqlparser 0.62.0

```rust
And
```

Source: `src/ast/operator.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

And, e.g. `a AND b`

<a id="op-17b7dc54a3e80d7319289572"></a>
## AndGt

`variant` · `sqlparser::ast::operator::BinaryOperator::AndGt` · sqlparser 0.62.0

```rust
AndGt
```

Source: `src/ast/operator.rs:292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`&>` Overlaps to right? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-b44897ea6f1fb8134de3c8f7"></a>
## AndLt

`variant` · `sqlparser::ast::operator::BinaryOperator::AndLt` · sqlparser 0.62.0

```rust
AndLt
```

Source: `src/ast/operator.rs:289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`&<` Overlaps to left? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-4a9116193e8414aa4310f172"></a>
## AndLtPipe

`variant` · `sqlparser::ast::operator::BinaryOperator::AndLtPipe` · sqlparser 0.62.0

```rust
AndLtPipe
```

Source: `src/ast/operator.rs:301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`&<|` Does not extend above? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-8217389f3f81dad1e457d104"></a>
## Arrow

`variant` · `sqlparser::ast::operator::BinaryOperator::Arrow` · sqlparser 0.62.0

```rust
Arrow
```

Source: `src/ast/operator.rs:184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `->` operator.

On PostgreSQL, this operator extracts a JSON object field or array
element, for example `'{"a":"b"}'::json -> 'a'` or `[1, 2, 3]'::json
-> 2`.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-09ea0e6c744c1d2ac8b23142"></a>
## ArrowAt

`variant` · `sqlparser::ast::operator::BinaryOperator::ArrowAt` · sqlparser 0.62.0

```rust
ArrowAt
```

Source: `src/ast/operator.rs:235`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `<@` operator.

On PostgreSQL, this is used for JSON and text searches.

See <https://www.postgresql.org/docs/current/functions-json.html>.
See <https://www.postgresql.org/docs/current/functions-textsearch.html>.

<a id="op-6f724c36e06a8b630fccb425"></a>
## Assignment

`variant` · `sqlparser::ast::operator::BinaryOperator::Assignment` · sqlparser 0.62.0

```rust
Assignment
```

Source: `src/ast/operator.rs:331`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

':=' Assignment Operator
See <https://dev.mysql.com/doc/refman/8.4/en/assignment-operators.html#operator_assign-value>

<a id="op-0e14226395e5d679eeea1c95"></a>
## At

`variant` · `sqlparser::ast::operator::BinaryOperator::At` · sqlparser 0.62.0

```rust
At
```

Source: `src/ast/operator.rs:325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`@` Contained or on? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-64620148c458e6e83fcec962"></a>
## AtArrow

`variant` · `sqlparser::ast::operator::BinaryOperator::AtArrow` · sqlparser 0.62.0

```rust
AtArrow
```

Source: `src/ast/operator.rs:228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `@>` operator.

On PostgreSQL, this is used for JSON and text searches.

See <https://www.postgresql.org/docs/current/functions-json.html>.
See <https://www.postgresql.org/docs/current/functions-textsearch.html>.

<a id="op-57bb61edaf35b4b9514e8886"></a>
## AtAt

`variant` · `sqlparser::ast::operator::BinaryOperator::AtAt` · sqlparser 0.62.0

```rust
AtAt
```

Source: `src/ast/operator.rs:221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `@@` operator.

On PostgreSQL, this is used for JSON and text searches.

See <https://www.postgresql.org/docs/current/functions-json.html>.
See <https://www.postgresql.org/docs/current/functions-textsearch.html>.

<a id="op-c3f638a0eb53278090fcf57b"></a>
## AtQuestion

`variant` · `sqlparser::ast::operator::BinaryOperator::AtQuestion` · sqlparser 0.62.0

```rust
AtQuestion
```

Source: `src/ast/operator.rs:249`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `@?` operator.

On PostgreSQL, this operator is used to check the given JSON path
returns an item for the JSON value.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-07a2018570b8b6ca4e5d2eb8"></a>
## BitwiseAnd

`variant` · `sqlparser::ast::operator::BinaryOperator::BitwiseAnd` · sqlparser 0.62.0

```rust
BitwiseAnd
```

Source: `src/ast/operator.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise and, e.g. `a & b`

<a id="op-9eaf3d3eb4eae533215ef768"></a>
## BitwiseOr

`variant` · `sqlparser::ast::operator::BinaryOperator::BitwiseOr` · sqlparser 0.62.0

```rust
BitwiseOr
```

Source: `src/ast/operator.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise or, e.g. `a | b`

<a id="op-a3ba33db9a26984f5aa39ec3"></a>
## BitwiseXor

`variant` · `sqlparser::ast::operator::BinaryOperator::BitwiseXor` · sqlparser 0.62.0

```rust
BitwiseXor
```

Source: `src/ast/operator.rs:137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise XOR, e.g. `a ^ b`

<a id="op-e2e395269786094b3e8217ca"></a>
## Custom

`variant` · `sqlparser::ast::operator::BinaryOperator::Custom` · sqlparser 0.62.0

```rust
Custom
```

Source: `src/ast/operator.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Support for custom operators (such as Postgres custom operators)

<a id="op-73e0df00bc033e0f1b5c6d33"></a>
## Divide

`variant` · `sqlparser::ast::operator::BinaryOperator::Divide` · sqlparser 0.62.0

```rust
Divide
```

Source: `src/ast/operator.rs:107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Divide, e.g. `a / b`

<a id="op-5da3992d1c57f9e1cf4f29dd"></a>
## DoubleHash

`variant` · `sqlparser::ast::operator::BinaryOperator::DoubleHash` · sqlparser 0.62.0

```rust
DoubleHash
```

Source: `src/ast/operator.rs:283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`##` Point of closest proximity (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-9f0a0197d030af6f8bf7b755"></a>
## DuckIntegerDivide

`variant` · `sqlparser::ast::operator::BinaryOperator::DuckIntegerDivide` · sqlparser 0.62.0

```rust
DuckIntegerDivide
```

Source: `src/ast/operator.rs:139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer division operator `//` in DuckDB

<a id="op-41d6443322c779247d65f4b1"></a>
## Eq

`variant` · `sqlparser::ast::operator::BinaryOperator::Eq` · sqlparser 0.62.0

```rust
Eq
```

Source: `src/ast/operator.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Equal, e.g. `a = b`

<a id="op-c11169d3c8bae3456bd8a5a1"></a>
## Gt

`variant` · `sqlparser::ast::operator::BinaryOperator::Gt` · sqlparser 0.62.0

```rust
Gt
```

Source: `src/ast/operator.rs:113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Greater than, e.g. `a > b`

<a id="op-39c6b602d6049bca0b8bf85b"></a>
## GtCaret

`variant` · `sqlparser::ast::operator::BinaryOperator::GtCaret` · sqlparser 0.62.0

```rust
GtCaret
```

Source: `src/ast/operator.rs:310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`>^` Is above? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-ad24d20bd82df7943ef2aa32"></a>
## GtEq

`variant` · `sqlparser::ast::operator::BinaryOperator::GtEq` · sqlparser 0.62.0

```rust
GtEq
```

Source: `src/ast/operator.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Greater equal, e.g. `a >= b`

<a id="op-c6a866ea148a9cb13e7faaba"></a>
## HashArrow

`variant` · `sqlparser::ast::operator::BinaryOperator::HashArrow` · sqlparser 0.62.0

```rust
HashArrow
```

Source: `src/ast/operator.rs:203`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

 The `#>` operator.

 On PostgreSQL, this operator extracts a JSON sub-object at the specified
 path, for example:

 ```notrust
'{"a": {"b": ["foo","bar"]}}'::json #> '{a,b,1}'
 ```

 See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-dbed2c4da79b88bf38c5dda3"></a>
## HashLongArrow

`variant` · `sqlparser::ast::operator::BinaryOperator::HashLongArrow` · sqlparser 0.62.0

```rust
HashLongArrow
```

Source: `src/ast/operator.rs:214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

 The `#>>` operator.

 A PostgreSQL-specific operator that extracts JSON sub-object at the
 specified path, for example

 ```notrust
'{"a": {"b": ["foo","bar"]}}'::json #>> '{a,b,1}'
 ```

 See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-9eba857dbc539a31ff19a65a"></a>
## HashMinus

`variant` · `sqlparser::ast::operator::BinaryOperator::HashMinus` · sqlparser 0.62.0

```rust
HashMinus
```

Source: `src/ast/operator.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `#-` operator.

On PostgreSQL, this operator is used to delete a field or array element
at a specified path.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-fd6081b6430b5adfd929f0ba"></a>
## LongArrow

`variant` · `sqlparser::ast::operator::BinaryOperator::LongArrow` · sqlparser 0.62.0

```rust
LongArrow
```

Source: `src/ast/operator.rs:192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `->>` operator.

On PostgreSQL, this operator extracts a JSON object field or JSON
array element and converts it to text, for example `'{"a":"b"}'::json
->> 'a'` or `[1, 2, 3]'::json ->> 2`.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-185ed960590caf1929f36f2e"></a>
## Lt

`variant` · `sqlparser::ast::operator::BinaryOperator::Lt` · sqlparser 0.62.0

```rust
Lt
```

Source: `src/ast/operator.rs:115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Less than, e.g. `a < b`

<a id="op-2a363e94c730b52828069b24"></a>
## LtCaret

`variant` · `sqlparser::ast::operator::BinaryOperator::LtCaret` · sqlparser 0.62.0

```rust
LtCaret
```

Source: `src/ast/operator.rs:307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<^` Is below? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-924941b3e6bf0e24c206cd7e"></a>
## LtDashGt

`variant` · `sqlparser::ast::operator::BinaryOperator::LtDashGt` · sqlparser 0.62.0

```rust
LtDashGt
```

Source: `src/ast/operator.rs:286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<->` Distance between (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-41685c4097040d93426071d7"></a>
## LtEq

`variant` · `sqlparser::ast::operator::BinaryOperator::LtEq` · sqlparser 0.62.0

```rust
LtEq
```

Source: `src/ast/operator.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Less equal, e.g. `a <= b`

<a id="op-bab26a9ada6bb57e4e876c3c"></a>
## LtLtPipe

`variant` · `sqlparser::ast::operator::BinaryOperator::LtLtPipe` · sqlparser 0.62.0

```rust
LtLtPipe
```

Source: `src/ast/operator.rs:295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<<|` Is strictly below? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-dd0c591fefa9128eab77e0ab"></a>
## Match

`variant` · `sqlparser::ast::operator::BinaryOperator::Match` · sqlparser 0.62.0

```rust
Match
```

Source: `src/ast/operator.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MATCH operator, e.g. `a MATCH b` (SQLite-specific)
See <https://www.sqlite.org/lang_expr.html#the_like_glob_regexp_match_and_extract_operators>

<a id="op-c8ab13f343250afc7e2863e8"></a>
## Minus

`variant` · `sqlparser::ast::operator::BinaryOperator::Minus` · sqlparser 0.62.0

```rust
Minus
```

Source: `src/ast/operator.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Minus, e.g. `a - b`

<a id="op-f5092ed217823aabed8c75fd"></a>
## Modulo

`variant` · `sqlparser::ast::operator::BinaryOperator::Modulo` · sqlparser 0.62.0

```rust
Modulo
```

Source: `src/ast/operator.rs:109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modulo, e.g. `a % b`

<a id="op-563a8422efc348e4b882ba53"></a>
## Multiply

`variant` · `sqlparser::ast::operator::BinaryOperator::Multiply` · sqlparser 0.62.0

```rust
Multiply
```

Source: `src/ast/operator.rs:105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multiply, e.g. `a * b`

<a id="op-92e9b0fa945356b861b7555d"></a>
## MyIntegerDivide

`variant` · `sqlparser::ast::operator::BinaryOperator::MyIntegerDivide` · sqlparser 0.62.0

```rust
MyIntegerDivide
```

Source: `src/ast/operator.rs:141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL [`DIV`](https://dev.mysql.com/doc/refman/8.0/en/arithmetic-functions.html) integer division

<a id="op-30e2f068874c82efc2065182"></a>
## NotEq

`variant` · `sqlparser::ast::operator::BinaryOperator::NotEq` · sqlparser 0.62.0

```rust
NotEq
```

Source: `src/ast/operator.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Not equal, e.g. `a <> b`

<a id="op-3d225f3811110cbe22c40f15"></a>
## Or

`variant` · `sqlparser::ast::operator::BinaryOperator::Or` · sqlparser 0.62.0

```rust
Or
```

Source: `src/ast/operator.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Or, e.g. `a OR b`

<a id="op-33f515c175ffb15fa399248e"></a>
## Overlaps

`variant` · `sqlparser::ast::operator::BinaryOperator::Overlaps` · sqlparser 0.62.0

```rust
Overlaps
```

Source: `src/ast/operator.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `OVERLAPS` operator

Specifies a test for an overlap between two datetime periods:
<https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#overlaps-predicate>

<a id="op-53f19500ce544bdddca46281"></a>
## PGBitwiseShiftLeft

`variant` · `sqlparser::ast::operator::BinaryOperator::PGBitwiseShiftLeft` · sqlparser 0.62.0

```rust
PGBitwiseShiftLeft
```

Source: `src/ast/operator.rs:152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise shift left, e.g. `a << b` (PostgreSQL-specific)

<a id="op-154335c624b5f694d82ff3b6"></a>
## PGBitwiseShiftRight

`variant` · `sqlparser::ast::operator::BinaryOperator::PGBitwiseShiftRight` · sqlparser 0.62.0

```rust
PGBitwiseShiftRight
```

Source: `src/ast/operator.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise shift right, e.g. `a >> b` (PostgreSQL-specific)

<a id="op-8c1bc07325c51c1d2d510e19"></a>
## PGBitwiseXor

`variant` · `sqlparser::ast::operator::BinaryOperator::PGBitwiseXor` · sqlparser 0.62.0

```rust
PGBitwiseXor
```

Source: `src/ast/operator.rs:150`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bitwise XOR, e.g. `a # b` (PostgreSQL-specific)

<a id="op-acc978a2700904814f366126"></a>
## PGCustomBinaryOperator

`variant` · `sqlparser::ast::operator::BinaryOperator::PGCustomBinaryOperator` · sqlparser 0.62.0

```rust
PGCustomBinaryOperator
```

Source: `src/ast/operator.rs:275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL-specific custom operator.

See [CREATE OPERATOR](https://www.postgresql.org/docs/current/sql-createoperator.html)
for more information.

<a id="op-39b98642424186ac5eceb016"></a>
## PGExp

`variant` · `sqlparser::ast::operator::BinaryOperator::PGExp` · sqlparser 0.62.0

```rust
PGExp
```

Source: `src/ast/operator.rs:156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exponent, e.g. `a ^ b` (PostgreSQL-specific)

<a id="op-6dc1cad4c47dd0d7aa5540e3"></a>
## PGILikeMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGILikeMatch` · sqlparser 0.62.0

```rust
PGILikeMatch
```

Source: `src/ast/operator.rs:170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String matches pattern (case insensitively), e.g. `a ~~* b` (PostgreSQL-specific)

<a id="op-e762827cc471272884275643"></a>
## PGLikeMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGLikeMatch` · sqlparser 0.62.0

```rust
PGLikeMatch
```

Source: `src/ast/operator.rs:168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String matches pattern (case sensitively), e.g. `a ~~ b` (PostgreSQL-specific)

<a id="op-37f8f02e7406abdfee412dc8"></a>
## PGNotILikeMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGNotILikeMatch` · sqlparser 0.62.0

```rust
PGNotILikeMatch
```

Source: `src/ast/operator.rs:174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String does not match pattern (case insensitively), e.g. `a !~~* b` (PostgreSQL-specific)

<a id="op-6188d393e2dab2f12ed1ac68"></a>
## PGNotLikeMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGNotLikeMatch` · sqlparser 0.62.0

```rust
PGNotLikeMatch
```

Source: `src/ast/operator.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String does not match pattern (case sensitively), e.g. `a !~~ b` (PostgreSQL-specific)

<a id="op-cfd4093717673c70bde83ea7"></a>
## PGOverlap

`variant` · `sqlparser::ast::operator::BinaryOperator::PGOverlap` · sqlparser 0.62.0

```rust
PGOverlap
```

Source: `src/ast/operator.rs:158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Overlap operator, e.g. `a && b` (PostgreSQL-specific)

<a id="op-fcd554f259eb540daea2fdd6"></a>
## PGRegexIMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGRegexIMatch` · sqlparser 0.62.0

```rust
PGRegexIMatch
```

Source: `src/ast/operator.rs:162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String matches regular expression (case insensitively), e.g. `a ~* b` (PostgreSQL-specific)

<a id="op-19a37dd8ee227548edc32a10"></a>
## PGRegexMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGRegexMatch` · sqlparser 0.62.0

```rust
PGRegexMatch
```

Source: `src/ast/operator.rs:160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String matches regular expression (case sensitively), e.g. `a ~ b` (PostgreSQL-specific)

<a id="op-26f1aa1bfd9321f5a6e0df7f"></a>
## PGRegexNotIMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGRegexNotIMatch` · sqlparser 0.62.0

```rust
PGRegexNotIMatch
```

Source: `src/ast/operator.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String does not match regular expression (case insensitively), e.g. `a !~* b` (PostgreSQL-specific)

<a id="op-d626d54a95711f27d58b72a5"></a>
## PGRegexNotMatch

`variant` · `sqlparser::ast::operator::BinaryOperator::PGRegexNotMatch` · sqlparser 0.62.0

```rust
PGRegexNotMatch
```

Source: `src/ast/operator.rs:164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String does not match regular expression (case sensitively), e.g. `a !~ b` (PostgreSQL-specific)

<a id="op-c450fcb78d49e8c422032cdc"></a>
## PGStartsWith

`variant` · `sqlparser::ast::operator::BinaryOperator::PGStartsWith` · sqlparser 0.62.0

```rust
PGStartsWith
```

Source: `src/ast/operator.rs:176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String "starts with", eg: `a ^@ b` (PostgreSQL-specific)

<a id="op-fb0dda03855b920954a222e0"></a>
## PipeAndGt

`variant` · `sqlparser::ast::operator::BinaryOperator::PipeAndGt` · sqlparser 0.62.0

```rust
PipeAndGt
```

Source: `src/ast/operator.rs:304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`|&>` Does not extend below? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-e9a3ff5068d7f32eca11280c"></a>
## PipeGtGt

`variant` · `sqlparser::ast::operator::BinaryOperator::PipeGtGt` · sqlparser 0.62.0

```rust
PipeGtGt
```

Source: `src/ast/operator.rs:298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`|>>` Is strictly above? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-092c51f75ba8fcbf29cb1906"></a>
## Plus

`variant` · `sqlparser::ast::operator::BinaryOperator::Plus` · sqlparser 0.62.0

```rust
Plus
```

Source: `src/ast/operator.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Plus, e.g. `a + b`

<a id="op-790689fdfd37fb3452bcb873"></a>
## Question

`variant` · `sqlparser::ast::operator::BinaryOperator::Question` · sqlparser 0.62.0

```rust
Question
```

Source: `src/ast/operator.rs:256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `?` operator.

On PostgreSQL, this operator is used to check whether a string exists as a top-level key
within the JSON value

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-30a334ffed613de16fce72a8"></a>
## QuestionAnd

`variant` · `sqlparser::ast::operator::BinaryOperator::QuestionAnd` · sqlparser 0.62.0

```rust
QuestionAnd
```

Source: `src/ast/operator.rs:263`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `?&` operator.

On PostgreSQL, this operator is used to check whether all of the the indicated array
members exist as top-level keys.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-afc86e49b5a13ebbbe6cf553"></a>
## QuestionDash

`variant` · `sqlparser::ast::operator::BinaryOperator::QuestionDash` · sqlparser 0.62.0

```rust
QuestionDash
```

Source: `src/ast/operator.rs:316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?-` Is horizontal? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-25c3a41b32d053da90cbbbc6"></a>
## QuestionDashPipe

`variant` · `sqlparser::ast::operator::BinaryOperator::QuestionDashPipe` · sqlparser 0.62.0

```rust
QuestionDashPipe
```

Source: `src/ast/operator.rs:319`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?-|` Is perpendicular? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-752a3c0d074b0c4489748a46"></a>
## QuestionDoublePipe

`variant` · `sqlparser::ast::operator::BinaryOperator::QuestionDoublePipe` · sqlparser 0.62.0

```rust
QuestionDoublePipe
```

Source: `src/ast/operator.rs:322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?||` Are Parallel? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-51f2e80f19434d48340c841f"></a>
## QuestionHash

`variant` · `sqlparser::ast::operator::BinaryOperator::QuestionHash` · sqlparser 0.62.0

```rust
QuestionHash
```

Source: `src/ast/operator.rs:313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?#` Intersects? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-068f087e272081e82f2c9c79"></a>
## QuestionPipe

`variant` · `sqlparser::ast::operator::BinaryOperator::QuestionPipe` · sqlparser 0.62.0

```rust
QuestionPipe
```

Source: `src/ast/operator.rs:270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `?|` operator.

On PostgreSQL, this operator is used to check whether any of the the indicated array
members exist as top-level keys.

See <https://www.postgresql.org/docs/current/functions-json.html>.

<a id="op-0139cd02e535750847c2f52c"></a>
## Regexp

`variant` · `sqlparser::ast::operator::BinaryOperator::Regexp` · sqlparser 0.62.0

```rust
Regexp
```

Source: `src/ast/operator.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

REGEXP operator, e.g. `a REGEXP b` (SQLite-specific)

<a id="op-55014e381d0f33f8449dc102"></a>
## Spaceship

`variant` · `sqlparser::ast::operator::BinaryOperator::Spaceship` · sqlparser 0.62.0

```rust
Spaceship
```

Source: `src/ast/operator.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Spaceship, e.g. `a <=> b`

<a id="op-b039d3faddb318454f1cb1c8"></a>
## StringConcat

`variant` · `sqlparser::ast::operator::BinaryOperator::StringConcat` · sqlparser 0.62.0

```rust
StringConcat
```

Source: `src/ast/operator.rs:111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String/Array Concat operator, e.g. `a || b`

<a id="op-3547f0ae95ca99f21e3c3898"></a>
## TildeEq

`variant` · `sqlparser::ast::operator::BinaryOperator::TildeEq` · sqlparser 0.62.0

```rust
TildeEq
```

Source: `src/ast/operator.rs:328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`~=` Same as? (PostgreSQL/Redshift geometric operator)
See <https://www.postgresql.org/docs/9.5/functions-geometry.html>

<a id="op-5264bf7f3d0cc94e02e46eea"></a>
## Xor

`variant` · `sqlparser::ast::operator::BinaryOperator::Xor` · sqlparser 0.62.0

```rust
Xor
```

Source: `src/ast/operator.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

XOR, e.g. `a XOR b`

<a id="op-413f3cd832b932f8b9d4c5c9"></a>
## clone

`function` · `sqlparser::ast::operator::BinaryOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> BinaryOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 17], "end": [96, 22], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/operator.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efd5ce4a6a283196632fcf41"></a>
## cmp

`function` · `sqlparser::ast::operator::BinaryOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &BinaryOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 51], "end": [96, 54], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/operator.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ccbfea5fdb1bace79466931"></a>
## deserialize

`function` · `sqlparser::ast::operator::BinaryOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 49], "end": [97, 60], "filename": "src/ast/operator.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/operator.rs:97`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88ddaa46901e4ec3ee515576"></a>
## eq

`function` · `sqlparser::ast::operator::BinaryOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &BinaryOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 24], "end": [96, 33], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/operator.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46e5b2fde38fbab8aacbef7a"></a>
## fmt

`function` · `sqlparser::ast::operator::BinaryOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 10], "end": [96, 15], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/operator.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8d8f4a68fc52f20eb18f9f9"></a>
## fmt

`function` · `sqlparser::ast::operator::BinaryOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [334, 1], "end": [410, 2], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/operator.rs:335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fedb72de950bd6813c98e5cd"></a>
## hash

`function` · `sqlparser::ast::operator::BinaryOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 56], "end": [96, 60], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/operator.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0169d192f966e06a9b29a34"></a>
## partial_cmp

`function` · `sqlparser::ast::operator::BinaryOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &BinaryOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 35], "end": [96, 45], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/operator.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3074f02708d9f6289b49946"></a>
## serialize

`function` · `sqlparser::ast::operator::BinaryOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 38], "end": [97, 47], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/operator.rs:97`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e2d0eee163be41449ac4abe"></a>
## visit

`function` · `sqlparser::ast::operator::BinaryOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 40], "end": [98, 45], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/operator.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8c6719281af266a2d61cbef"></a>
## visit

`function` · `sqlparser::ast::operator::BinaryOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::operator::BinaryOperator", "path": "BinaryOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 47], "end": [98, 55], "filename": "src/ast/operator.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/operator.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
