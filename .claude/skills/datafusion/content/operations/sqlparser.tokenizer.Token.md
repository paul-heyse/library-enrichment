# `sqlparser::tokenizer::Token`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.Token.json).

<a id="op-8dd5a6484792ec08e2eb8600"></a>
## Token

`enum` · `sqlparser::tokenizer::Token` · sqlparser 0.62.0

```rust
enum Token
```

Source: `src/tokenizer.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL Token enumeration

<a id="op-94af89cbcda727b0105f31c2"></a>
## Ampersand

`variant` · `sqlparser::tokenizer::Token::Ampersand` · sqlparser 0.62.0

```rust
Ampersand
```

Source: `src/tokenizer.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Ampersand `&`

<a id="op-b5bc976972e26f73b816de5a"></a>
## AmpersandLeftAngleBracket

`variant` · `sqlparser::tokenizer::Token::AmpersandLeftAngleBracket` · sqlparser 0.62.0

```rust
AmpersandLeftAngleBracket
```

Source: `src/tokenizer.rs:233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`&<` PostgreSQL/Redshift geometrical binary operator (Overlaps to left?)

<a id="op-d7cac39f7810c6db2479153a"></a>
## AmpersandLeftAngleBracketVerticalBar

`variant` · `sqlparser::tokenizer::Token::AmpersandLeftAngleBracketVerticalBar` · sqlparser 0.62.0

```rust
AmpersandLeftAngleBracketVerticalBar
```

Source: `src/tokenizer.rs:237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`&<|` PostgreSQL/Redshift geometrical binary operator (Does not extend above?)`

<a id="op-95c69441db01d0952b377a8d"></a>
## AmpersandRightAngleBracket

`variant` · `sqlparser::tokenizer::Token::AmpersandRightAngleBracket` · sqlparser 0.62.0

```rust
AmpersandRightAngleBracket
```

Source: `src/tokenizer.rs:235`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`&>` PostgreSQL/Redshift geometrical binary operator (Overlaps to right?)`

<a id="op-e8f5bf28392f6ba3c44fd5a9"></a>
## Arrow

`variant` · `sqlparser::tokenizer::Token::Arrow` · sqlparser 0.62.0

```rust
Arrow
```

Source: `src/tokenizer.rs:223`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`->`, used as a operator to extract json field in PostgreSQL

<a id="op-dd5bb9e06bff6ad494d62f06"></a>
## ArrowAt

`variant` · `sqlparser::tokenizer::Token::ArrowAt` · sqlparser 0.62.0

```rust
ArrowAt
```

Source: `src/tokenizer.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb <@ jsonb -> boolean: Test whether right json contains the left json

<a id="op-3b8c8d4bfc746a9c755342e9"></a>
## Assignment

`variant` · `sqlparser::tokenizer::Token::Assignment` · sqlparser 0.62.0

```rust
Assignment
```

Source: `src/tokenizer.rs:161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Assignment `:=` (used for keyword argument in DuckDB macros and some functions, and for variable declarations in DuckDB and Snowflake)

<a id="op-a4b819559998739c793bd323"></a>
## AtArrow

`variant` · `sqlparser::tokenizer::Token::AtArrow` · sqlparser 0.62.0

```rust
AtArrow
```

Source: `src/tokenizer.rs:263`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb @> jsonb -> boolean: Test whether left json contains the right json

<a id="op-cd4dab2d97cfb93a5c205069"></a>
## AtAt

`variant` · `sqlparser::tokenizer::Token::AtAt` · sqlparser 0.62.0

```rust
AtAt
```

Source: `src/tokenizer.rs:275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb @@ jsonpath → boolean: Returns the result of a JSON path predicate check
for the specified JSON value. Only the first item of the result is taken into
account. If the result is not Boolean, then NULL is returned.

<a id="op-92c83e88b3292f31d599521a"></a>
## AtDashAt

`variant` · `sqlparser::tokenizer::Token::AtDashAt` · sqlparser 0.62.0

```rust
AtDashAt
```

Source: `src/tokenizer.rs:229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`@-@` PostgreSQL/Redshift geometrical unary operator (Length or circumference)

<a id="op-d2533eaa38a2a21220b3d9cb"></a>
## AtQuestion

`variant` · `sqlparser::tokenizer::Token::AtQuestion` · sqlparser 0.62.0

```rust
AtQuestion
```

Source: `src/tokenizer.rs:271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb @? jsonpath -> boolean: Does JSON path return any item for the specified
JSON value?

<a id="op-3b85e326081e44b5abffbcfc"></a>
## AtSign

`variant` · `sqlparser::tokenizer::Token::AtSign` · sqlparser 0.62.0

```rust
AtSign
```

Source: `src/tokenizer.rs:213`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

AtSign `@` used for PostgreSQL abs operator, also PostgreSQL/Redshift geometrical unary/binary operator (Center, Contained or on)

<a id="op-2cb9aebcccf7b9da7bb422e7"></a>
## Backslash

`variant` · `sqlparser::tokenizer::Token::Backslash` · sqlparser 0.62.0

```rust
Backslash
```

Source: `src/tokenizer.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Backslash `\` used in terminating the COPY payload with `\.`

<a id="op-fd6f23c4c920f3781a681321"></a>
## Caret

`variant` · `sqlparser::tokenizer::Token::Caret` · sqlparser 0.62.0

```rust
Caret
```

Source: `src/tokenizer.rs:175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Caret `^`

<a id="op-48f19c9325ca9bac05bc7ed0"></a>
## CaretAt

`variant` · `sqlparser::tokenizer::Token::CaretAt` · sqlparser 0.62.0

```rust
CaretAt
```

Source: `src/tokenizer.rs:215`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`^@`, a "starts with" string operator in PostgreSQL

<a id="op-ef28a0eea7a55cca1c66774f"></a>
## Char

`variant` · `sqlparser::tokenizer::Token::Char` · sqlparser 0.62.0

```rust
Char
```

Source: `src/tokenizer.rs:66`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A character that could not be tokenized

<a id="op-a21f9f7dfc7953a857309715"></a>
## Colon

`variant` · `sqlparser::tokenizer::Token::Colon` · sqlparser 0.62.0

```rust
Colon
```

Source: `src/tokenizer.rs:157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Colon `:`

<a id="op-0fcba0b114e2ab6d64990099"></a>
## Comma

`variant` · `sqlparser::tokenizer::Token::Comma` · sqlparser 0.62.0

```rust
Comma
```

Source: `src/tokenizer.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Comma

<a id="op-bd3a9401894dca45e59de5a6"></a>
## CustomBinaryOperator

`variant` · `sqlparser::tokenizer::Token::CustomBinaryOperator` · sqlparser 0.62.0

```rust
CustomBinaryOperator
```

Source: `src/tokenizer.rs:288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Custom binary operator
This is used to represent any custom binary operator that is not part of the SQL standard.
PostgreSQL allows defining custom binary operators using CREATE OPERATOR.

<a id="op-b7f7b3bb15dc85fa91b5f3ba"></a>
## Div

`variant` · `sqlparser::tokenizer::Token::Div` · sqlparser 0.62.0

```rust
Div
```

Source: `src/tokenizer.rs:143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Division operator `/`

<a id="op-ed20dd2d14b78b3968485041"></a>
## DollarQuotedString

`variant` · `sqlparser::tokenizer::Token::DollarQuotedString` · sqlparser 0.62.0

```rust
DollarQuotedString
```

Source: `src/tokenizer.rs:78`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dollar quoted string: i.e: $$string$$ or $tag_name$string$tag_name$

<a id="op-0a52b3aeebe4c645a6670240"></a>
## DoubleColon

`variant` · `sqlparser::tokenizer::Token::DoubleColon` · sqlparser 0.62.0

```rust
DoubleColon
```

Source: `src/tokenizer.rs:159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DoubleColon `::` (used for casting in PostgreSQL)

<a id="op-771e2754de7464e2b44510ab"></a>
## DoubleEq

`variant` · `sqlparser::tokenizer::Token::DoubleEq` · sqlparser 0.62.0

```rust
DoubleEq
```

Source: `src/tokenizer.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double equals sign `==`

<a id="op-394f0736f9c5d0be6006c86b"></a>
## DoubleExclamationMark

`variant` · `sqlparser::tokenizer::Token::DoubleExclamationMark` · sqlparser 0.62.0

```rust
DoubleExclamationMark
```

Source: `src/tokenizer.rs:211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double Exclamation Mark `!!` used for PostgreSQL prefix factorial operator

<a id="op-c883798242b5be7b08ff3556"></a>
## DoubleQuotedByteStringLiteral

`variant` · `sqlparser::tokenizer::Token::DoubleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
DoubleQuotedByteStringLiteral
```

Source: `src/tokenizer.rs:83`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Byte string literal: i.e: b"string" or B"string"

<a id="op-fb3798f407d949c898ad8214"></a>
## DoubleQuotedRawStringLiteral

`variant` · `sqlparser::tokenizer::Token::DoubleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
DoubleQuotedRawStringLiteral
```

Source: `src/tokenizer.rs:95`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double quoted literal with raw string prefix. Example `R"abc"`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-546dec3e7195cfbd3e103b5c"></a>
## DoubleQuotedString

`variant` · `sqlparser::tokenizer::Token::DoubleQuotedString` · sqlparser 0.62.0

```rust
DoubleQuotedString
```

Source: `src/tokenizer.rs:70`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double quoted string: i.e: "string"

<a id="op-83f79d9de44699706e48695e"></a>
## DoubleSharp

`variant` · `sqlparser::tokenizer::Token::DoubleSharp` · sqlparser 0.62.0

```rust
DoubleSharp
```

Source: `src/tokenizer.rs:185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`##` PostgreSQL/Redshift geometrical binary operator (Point of closest proximity)

<a id="op-c10a7884ce9dafe15915ee72"></a>
## DoubleTilde

`variant` · `sqlparser::tokenizer::Token::DoubleTilde` · sqlparser 0.62.0

```rust
DoubleTilde
```

Source: `src/tokenizer.rs:195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`~~`, a case sensitive match pattern operator in PostgreSQL

<a id="op-775a1633c5d35ba55068d91a"></a>
## DoubleTildeAsterisk

`variant` · `sqlparser::tokenizer::Token::DoubleTildeAsterisk` · sqlparser 0.62.0

```rust
DoubleTildeAsterisk
```

Source: `src/tokenizer.rs:197`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`~~*`, a case insensitive match pattern operator in PostgreSQL

<a id="op-d4b21f287fe6a650f20c8271"></a>
## DuckIntDiv

`variant` · `sqlparser::tokenizer::Token::DuckIntDiv` · sqlparser 0.62.0

```rust
DuckIntDiv
```

Source: `src/tokenizer.rs:145`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer division operator `//` in DuckDB

<a id="op-e3b3bdb4e63cf95a06aad31e"></a>
## EOF

`variant` · `sqlparser::tokenizer::Token::EOF` · sqlparser 0.62.0

```rust
EOF
```

Source: `src/tokenizer.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An end-of-file marker, not a real token

<a id="op-b6e96a0ed9e591c40a5d7ea9"></a>
## Eq

`variant` · `sqlparser::tokenizer::Token::Eq` · sqlparser 0.62.0

```rust
Eq
```

Source: `src/tokenizer.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Equality operator `=`

<a id="op-e2795a9509a74b2939c7e643"></a>
## EscapedStringLiteral

`variant` · `sqlparser::tokenizer::Token::EscapedStringLiteral` · sqlparser 0.62.0

```rust
EscapedStringLiteral
```

Source: `src/tokenizer.rs:111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

"escaped" string literal, which are an extension to the SQL standard: i.e: e'first \n second' or E 'first \n second'

<a id="op-b4506290ef61ea2a05fbc575"></a>
## ExclamationMark

`variant` · `sqlparser::tokenizer::Token::ExclamationMark` · sqlparser 0.62.0

```rust
ExclamationMark
```

Source: `src/tokenizer.rs:209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exclamation Mark `!` used for PostgreSQL factorial operator

<a id="op-754d32dbcc6cd163a64ee167"></a>
## ExclamationMarkDoubleTilde

`variant` · `sqlparser::tokenizer::Token::ExclamationMarkDoubleTilde` · sqlparser 0.62.0

```rust
ExclamationMarkDoubleTilde
```

Source: `src/tokenizer.rs:199`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`!~~`, a case sensitive not match pattern operator in PostgreSQL

<a id="op-6a6c4d36b60ac1b9c99e0916"></a>
## ExclamationMarkDoubleTildeAsterisk

`variant` · `sqlparser::tokenizer::Token::ExclamationMarkDoubleTildeAsterisk` · sqlparser 0.62.0

```rust
ExclamationMarkDoubleTildeAsterisk
```

Source: `src/tokenizer.rs:201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`!~~*`, a case insensitive not match pattern operator in PostgreSQL

<a id="op-659f33ce4e48ecf6057b68f8"></a>
## ExclamationMarkTilde

`variant` · `sqlparser::tokenizer::Token::ExclamationMarkTilde` · sqlparser 0.62.0

```rust
ExclamationMarkTilde
```

Source: `src/tokenizer.rs:191`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`!~` , a case sensitive not match regular expression operator in PostgreSQL

<a id="op-834c3d7e2bced4feba983db8"></a>
## ExclamationMarkTildeAsterisk

`variant` · `sqlparser::tokenizer::Token::ExclamationMarkTildeAsterisk` · sqlparser 0.62.0

```rust
ExclamationMarkTildeAsterisk
```

Source: `src/tokenizer.rs:193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`!~*` , a case insensitive not match regular expression operator in PostgreSQL

<a id="op-ea458eb824bd886e3394ec25"></a>
## Gt

`variant` · `sqlparser::tokenizer::Token::Gt` · sqlparser 0.62.0

```rust
Gt
```

Source: `src/tokenizer.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Greater Than operator `>`

<a id="op-17db9a03153dd260d70c04e2"></a>
## GtEq

`variant` · `sqlparser::tokenizer::Token::GtEq` · sqlparser 0.62.0

```rust
GtEq
```

Source: `src/tokenizer.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Greater Than Or Equals operator `>=`

<a id="op-dbfda6261b22a32fa379cd78"></a>
## HashArrow

`variant` · `sqlparser::tokenizer::Token::HashArrow` · sqlparser 0.62.0

```rust
HashArrow
```

Source: `src/tokenizer.rs:227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`#>`, extracts JSON sub-object at the specified path

<a id="op-dd7a0e24a0c56c8eeeb9fd2c"></a>
## HashLongArrow

`variant` · `sqlparser::tokenizer::Token::HashLongArrow` · sqlparser 0.62.0

```rust
HashLongArrow
```

Source: `src/tokenizer.rs:261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`#>>`, extracts JSON sub-object at the specified path as text

<a id="op-d147b4f5db5af44fbe48adb9"></a>
## HashMinus

`variant` · `sqlparser::tokenizer::Token::HashMinus` · sqlparser 0.62.0

```rust
HashMinus
```

Source: `src/tokenizer.rs:268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb #- text[] -> jsonb: Deletes the field or array element at the specified
path, where path elements can be either field keys or array indexes.

<a id="op-11e3fa119736d2e7301d169c"></a>
## HexStringLiteral

`variant` · `sqlparser::tokenizer::Token::HexStringLiteral` · sqlparser 0.62.0

```rust
HexStringLiteral
```

Source: `src/tokenizer.rs:115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hexadecimal string literal: i.e.: X'deadbeef'

<a id="op-12be436fe81e1664f0a2b3e1"></a>
## LBrace

`variant` · `sqlparser::tokenizer::Token::LBrace` · sqlparser 0.62.0

```rust
LBrace
```

Source: `src/tokenizer.rs:177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left brace `{`

<a id="op-8bb89e2753b19cf0e267d2b3"></a>
## LBracket

`variant` · `sqlparser::tokenizer::Token::LBracket` · sqlparser 0.62.0

```rust
LBracket
```

Source: `src/tokenizer.rs:167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left bracket `[`

<a id="op-7d5bcc2bbdd5f3e088ee9134"></a>
## LParen

`variant` · `sqlparser::tokenizer::Token::LParen` · sqlparser 0.62.0

```rust
LParen
```

Source: `src/tokenizer.rs:151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left parenthesis `(`

<a id="op-3a4a382ae9597494cdccaba8"></a>
## LeftAngleBracketCaret

`variant` · `sqlparser::tokenizer::Token::LeftAngleBracketCaret` · sqlparser 0.62.0

```rust
LeftAngleBracketCaret
```

Source: `src/tokenizer.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<^` PostgreSQL/Redshift geometrical binary operator (Is below?)

<a id="op-2fc5c11590a237d2d908792f"></a>
## LongArrow

`variant` · `sqlparser::tokenizer::Token::LongArrow` · sqlparser 0.62.0

```rust
LongArrow
```

Source: `src/tokenizer.rs:225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`->>`, used as a operator to extract json field as text in PostgreSQL

<a id="op-708de8547d8860fd7e7516f1"></a>
## Lt

`variant` · `sqlparser::tokenizer::Token::Lt` · sqlparser 0.62.0

```rust
Lt
```

Source: `src/tokenizer.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Less Than operator `<`

<a id="op-67009255044f534da45d937c"></a>
## LtEq

`variant` · `sqlparser::tokenizer::Token::LtEq` · sqlparser 0.62.0

```rust
LtEq
```

Source: `src/tokenizer.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Less Than Or Equals operator `<=`

<a id="op-9595111afb65d067ce900f7c"></a>
## Minus

`variant` · `sqlparser::tokenizer::Token::Minus` · sqlparser 0.62.0

```rust
Minus
```

Source: `src/tokenizer.rs:139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Minus operator `-`

<a id="op-616419878b14525a05cdbbed"></a>
## Mod

`variant` · `sqlparser::tokenizer::Token::Mod` · sqlparser 0.62.0

```rust
Mod
```

Source: `src/tokenizer.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modulo Operator `%`

<a id="op-c579a56166911ac9c9b2afdf"></a>
## Mul

`variant` · `sqlparser::tokenizer::Token::Mul` · sqlparser 0.62.0

```rust
Mul
```

Source: `src/tokenizer.rs:141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multiplication operator `*`

<a id="op-b7593d1a9dc6a59bc7e9a6ea"></a>
## NationalQuoteDelimitedStringLiteral

`variant` · `sqlparser::tokenizer::Token::NationalQuoteDelimitedStringLiteral` · sqlparser 0.62.0

```rust
NationalQuoteDelimitedStringLiteral
```

Source: `src/tokenizer.rs:109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

"Nationa" quote delimited literal. Examples `NQ'{ab'c}'`, `NQ'|ab'c|'`, `NQ'|ab|c|'`
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Literals.html#GUID-1824CBAA-6E16-4921-B2A6-112FB02248DA)

<a id="op-a50e2dbb16ebefcfa010acb9"></a>
## NationalStringLiteral

`variant` · `sqlparser::tokenizer::Token::NationalStringLiteral` · sqlparser 0.62.0

```rust
NationalStringLiteral
```

Source: `src/tokenizer.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

"National" string literal: i.e: N'string'

<a id="op-44c2310cfa4483364b8c2810"></a>
## Neq

`variant` · `sqlparser::tokenizer::Token::Neq` · sqlparser 0.62.0

```rust
Neq
```

Source: `src/tokenizer.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Not Equals operator `<>` (or `!=` in some dialects)

<a id="op-79c49c3b067ba25be87684f5"></a>
## Number

`variant` · `sqlparser::tokenizer::Token::Number` · sqlparser 0.62.0

```rust
Number
```

Source: `src/tokenizer.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unsigned numeric literal

<a id="op-25d4ea5bc19e35ef624cfd8d"></a>
## Overlap

`variant` · `sqlparser::tokenizer::Token::Overlap` · sqlparser 0.62.0

```rust
Overlap
```

Source: `src/tokenizer.rs:207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`&&`, an overlap operator in PostgreSQL

<a id="op-13a330df4a34e31cb2f2bd66"></a>
## PGCubeRoot

`variant` · `sqlparser::tokenizer::Token::PGCubeRoot` · sqlparser 0.62.0

```rust
PGCubeRoot
```

Source: `src/tokenizer.rs:219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`||/`, a cube root math operator in PostgreSQL

<a id="op-557f4aa00c14681fc4c9000c"></a>
## PGSquareRoot

`variant` · `sqlparser::tokenizer::Token::PGSquareRoot` · sqlparser 0.62.0

```rust
PGSquareRoot
```

Source: `src/tokenizer.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`|/`, a square root math operator in PostgreSQL

<a id="op-61d763705828c380608da114"></a>
## Period

`variant` · `sqlparser::tokenizer::Token::Period` · sqlparser 0.62.0

```rust
Period
```

Source: `src/tokenizer.rs:155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Period (used for compound identifiers or projections into nested types)

<a id="op-fedeaba385d2b86b1ccfe733"></a>
## Pipe

`variant` · `sqlparser::tokenizer::Token::Pipe` · sqlparser 0.62.0

```rust
Pipe
```

Source: `src/tokenizer.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pipe `|`

<a id="op-c970a534b4e39e931cde05fe"></a>
## Placeholder

`variant` · `sqlparser::tokenizer::Token::Placeholder` · sqlparser 0.62.0

```rust
Placeholder
```

Source: `src/tokenizer.rs:221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?` or `$` , a prepared statement arg placeholder

<a id="op-8288abba346d9e007967cc86"></a>
## Plus

`variant` · `sqlparser::tokenizer::Token::Plus` · sqlparser 0.62.0

```rust
Plus
```

Source: `src/tokenizer.rs:137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Plus operator `+`

<a id="op-291f77760d93bad274b8a520"></a>
## Question

`variant` · `sqlparser::tokenizer::Token::Question` · sqlparser 0.62.0

```rust
Question
```

Source: `src/tokenizer.rs:278`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb ? text -> boolean: Checks whether the string exists as a top-level key within the
jsonb object

<a id="op-4613bbc2646f2821c24ebf1d"></a>
## QuestionAnd

`variant` · `sqlparser::tokenizer::Token::QuestionAnd` · sqlparser 0.62.0

```rust
QuestionAnd
```

Source: `src/tokenizer.rs:281`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb ?& text[] -> boolean: Check whether all members of the text array exist as top-level
keys within the jsonb object

<a id="op-89115153225095ca85b2e1a6"></a>
## QuestionMarkDash

`variant` · `sqlparser::tokenizer::Token::QuestionMarkDash` · sqlparser 0.62.0

```rust
QuestionMarkDash
```

Source: `src/tokenizer.rs:231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?-` PostgreSQL/Redshift geometrical unary/binary operator (Is horizontal?/Are horizontally aligned?)

<a id="op-21fc7af413738e17753f4d17"></a>
## QuestionMarkDashVerticalBar

`variant` · `sqlparser::tokenizer::Token::QuestionMarkDashVerticalBar` · sqlparser 0.62.0

```rust
QuestionMarkDashVerticalBar
```

Source: `src/tokenizer.rs:249`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?-|` PostgreSQL/Redshift geometrical binary operator (Is perpendicular?)

<a id="op-b7d3fa7c60a04562adf4f6df"></a>
## QuestionMarkDoubleVerticalBar

`variant` · `sqlparser::tokenizer::Token::QuestionMarkDoubleVerticalBar` · sqlparser 0.62.0

```rust
QuestionMarkDoubleVerticalBar
```

Source: `src/tokenizer.rs:251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?||` PostgreSQL/Redshift geometrical binary operator (Are parallel?)

<a id="op-dd883d4ea2ab38c3369cb746"></a>
## QuestionMarkSharp

`variant` · `sqlparser::tokenizer::Token::QuestionMarkSharp` · sqlparser 0.62.0

```rust
QuestionMarkSharp
```

Source: `src/tokenizer.rs:247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?#` PostgreSQL/Redshift geometrical binary operator (Intersects or overlaps)

<a id="op-a324eac9b01d71a8fc816717"></a>
## QuestionPipe

`variant` · `sqlparser::tokenizer::Token::QuestionPipe` · sqlparser 0.62.0

```rust
QuestionPipe
```

Source: `src/tokenizer.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

jsonb ?| text[] -> boolean: Check whether any member of the text array exists as top-level
keys within the jsonb object

<a id="op-8d728065cd9648a29a3e7708"></a>
## QuoteDelimitedStringLiteral

`variant` · `sqlparser::tokenizer::Token::QuoteDelimitedStringLiteral` · sqlparser 0.62.0

```rust
QuoteDelimitedStringLiteral
```

Source: `src/tokenizer.rs:106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Quote delimited literal. Examples `Q'{ab'c}'`, `Q'|ab'c|'`, `Q'|ab|c|'`
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Literals.html#GUID-1824CBAA-6E16-4921-B2A6-112FB02248DA)

<a id="op-728fde006bfda455a2c30a70"></a>
## RArrow

`variant` · `sqlparser::tokenizer::Token::RArrow` · sqlparser 0.62.0

```rust
RArrow
```

Source: `src/tokenizer.rs:181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right Arrow `=>`

<a id="op-a00c4648579ebb835a859713"></a>
## RBrace

`variant` · `sqlparser::tokenizer::Token::RBrace` · sqlparser 0.62.0

```rust
RBrace
```

Source: `src/tokenizer.rs:179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right brace `}`

<a id="op-ea1f76ee07df605b176d5a81"></a>
## RBracket

`variant` · `sqlparser::tokenizer::Token::RBracket` · sqlparser 0.62.0

```rust
RBracket
```

Source: `src/tokenizer.rs:169`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right bracket `]`

<a id="op-deb5d817b2ba79b6c29931ff"></a>
## RParen

`variant` · `sqlparser::tokenizer::Token::RParen` · sqlparser 0.62.0

```rust
RParen
```

Source: `src/tokenizer.rs:153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right parenthesis `)`

<a id="op-c0c0671a793a441cc221cfcf"></a>
## RightAngleBracketCaret

`variant` · `sqlparser::tokenizer::Token::RightAngleBracketCaret` · sqlparser 0.62.0

```rust
RightAngleBracketCaret
```

Source: `src/tokenizer.rs:245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`>^` PostgreSQL/Redshift geometrical binary operator (Is above?)

<a id="op-8ee90b40f7288ea934bcee7f"></a>
## SemiColon

`variant` · `sqlparser::tokenizer::Token::SemiColon` · sqlparser 0.62.0

```rust
SemiColon
```

Source: `src/tokenizer.rs:163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SemiColon `;` used as separator for COPY and payload

<a id="op-e332f163dca81bfa66f5c792"></a>
## Sharp

`variant` · `sqlparser::tokenizer::Token::Sharp` · sqlparser 0.62.0

```rust
Sharp
```

Source: `src/tokenizer.rs:183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sharp `#` used for PostgreSQL Bitwise XOR operator, also PostgreSQL/Redshift geometrical unary/binary operator (Number of points in path or polygon/Intersection)

<a id="op-cd61973870b7a2486822af5e"></a>
## ShiftLeft

`variant` · `sqlparser::tokenizer::Token::ShiftLeft` · sqlparser 0.62.0

```rust
ShiftLeft
```

Source: `src/tokenizer.rs:203`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<<`, a bitwise shift left operator in PostgreSQL

<a id="op-8d9b94147a8407546273a968"></a>
## ShiftLeftVerticalBar

`variant` · `sqlparser::tokenizer::Token::ShiftLeftVerticalBar` · sqlparser 0.62.0

```rust
ShiftLeftVerticalBar
```

Source: `src/tokenizer.rs:255`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<<| PostgreSQL/Redshift geometrical binary operator (Is strictly below?)

<a id="op-d9f9b9d86199b298e072cbc7"></a>
## ShiftRight

`variant` · `sqlparser::tokenizer::Token::ShiftRight` · sqlparser 0.62.0

```rust
ShiftRight
```

Source: `src/tokenizer.rs:205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`>>`, a bitwise shift right operator in PostgreSQL

<a id="op-7a7088e1d6b399f33a8ec9ed"></a>
## SingleQuotedByteStringLiteral

`variant` · `sqlparser::tokenizer::Token::SingleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
SingleQuotedByteStringLiteral
```

Source: `src/tokenizer.rs:81`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Byte string literal: i.e: b'string' or B'string' (note that some backends, such as
PostgreSQL, may treat this syntax as a bit string literal instead, i.e: b'10010101')

<a id="op-47c3e171e4681b5e8c528a77"></a>
## SingleQuotedRawStringLiteral

`variant` · `sqlparser::tokenizer::Token::SingleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
SingleQuotedRawStringLiteral
```

Source: `src/tokenizer.rs:92`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single quoted literal with raw string prefix. Example `R'abc'`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-6a324b51484cb4e1dfa4288e"></a>
## SingleQuotedString

`variant` · `sqlparser::tokenizer::Token::SingleQuotedString` · sqlparser 0.62.0

```rust
SingleQuotedString
```

Source: `src/tokenizer.rs:68`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single quoted string: i.e: 'string'

<a id="op-c3097d46f6d19540accdf83a"></a>
## Spaceship

`variant` · `sqlparser::tokenizer::Token::Spaceship` · sqlparser 0.62.0

```rust
Spaceship
```

Source: `src/tokenizer.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Spaceship operator <=>

<a id="op-a3f01a5610ade755eb2729b8"></a>
## StringConcat

`variant` · `sqlparser::tokenizer::Token::StringConcat` · sqlparser 0.62.0

```rust
StringConcat
```

Source: `src/tokenizer.rs:149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String concatenation `||`

<a id="op-c6de29a87bf3808ad36a20d2"></a>
## Tilde

`variant` · `sqlparser::tokenizer::Token::Tilde` · sqlparser 0.62.0

```rust
Tilde
```

Source: `src/tokenizer.rs:187`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tilde `~` used for PostgreSQL Bitwise NOT operator or case sensitive match regular expression operator

<a id="op-e0c764806ee529d603d4b017"></a>
## TildeAsterisk

`variant` · `sqlparser::tokenizer::Token::TildeAsterisk` · sqlparser 0.62.0

```rust
TildeAsterisk
```

Source: `src/tokenizer.rs:189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`~*` , a case insensitive match regular expression operator in PostgreSQL

<a id="op-882efc1dbc1443f310b5fff7"></a>
## TildeEqual

`variant` · `sqlparser::tokenizer::Token::TildeEqual` · sqlparser 0.62.0

```rust
TildeEqual
```

Source: `src/tokenizer.rs:253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`~=` PostgreSQL/Redshift geometrical binary operator (Same as)

<a id="op-09040cd23b9a2eddb54c1bb9"></a>
## TripleDoubleQuotedByteStringLiteral

`variant` · `sqlparser::tokenizer::Token::TripleDoubleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
TripleDoubleQuotedByteStringLiteral
```

Source: `src/tokenizer.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple double quoted literal with byte string prefix. Example `B"""abc"""`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-306dad3e1c36bfa0e817756b"></a>
## TripleDoubleQuotedRawStringLiteral

`variant` · `sqlparser::tokenizer::Token::TripleDoubleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
TripleDoubleQuotedRawStringLiteral
```

Source: `src/tokenizer.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple double quoted literal with raw string prefix. Example `R"""abc"""`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-65204fe5453577141f8407d1"></a>
## TripleDoubleQuotedString

`variant` · `sqlparser::tokenizer::Token::TripleDoubleQuotedString` · sqlparser 0.62.0

```rust
TripleDoubleQuotedString
```

Source: `src/tokenizer.rs:76`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple double quoted strings: Example """abc"""
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-38b3aba94e4fcc43336b300d"></a>
## TripleSingleQuotedByteStringLiteral

`variant` · `sqlparser::tokenizer::Token::TripleSingleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
TripleSingleQuotedByteStringLiteral
```

Source: `src/tokenizer.rs:86`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple single quoted literal with byte string prefix. Example `B'''abc'''`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-2e833f18fd9eb09f635be060"></a>
## TripleSingleQuotedRawStringLiteral

`variant` · `sqlparser::tokenizer::Token::TripleSingleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
TripleSingleQuotedRawStringLiteral
```

Source: `src/tokenizer.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple single quoted literal with raw string prefix. Example `R'''abc'''`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-aa40b37354b4b0a967c665b1"></a>
## TripleSingleQuotedString

`variant` · `sqlparser::tokenizer::Token::TripleSingleQuotedString` · sqlparser 0.62.0

```rust
TripleSingleQuotedString
```

Source: `src/tokenizer.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple single quoted strings: Example '''abc'''
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-c123e531f7d9e72d1152d2f9"></a>
## TwoWayArrow

`variant` · `sqlparser::tokenizer::Token::TwoWayArrow` · sqlparser 0.62.0

```rust
TwoWayArrow
```

Source: `src/tokenizer.rs:241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<->` PostgreSQL/Redshift geometrical binary operator (Distance between)

<a id="op-7b10cea49cf3abbbff00dba5"></a>
## UnicodeStringLiteral

`variant` · `sqlparser::tokenizer::Token::UnicodeStringLiteral` · sqlparser 0.62.0

```rust
UnicodeStringLiteral
```

Source: `src/tokenizer.rs:113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unicode string literal: i.e: U&'first \000A second'

<a id="op-1c395f5da01bfd4ee0d112b3"></a>
## VerticalBarAmpersandRightAngleBracket

`variant` · `sqlparser::tokenizer::Token::VerticalBarAmpersandRightAngleBracket` · sqlparser 0.62.0

```rust
VerticalBarAmpersandRightAngleBracket
```

Source: `src/tokenizer.rs:239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`|&>` PostgreSQL/Redshift geometrical binary operator (Does not extend below?)`

<a id="op-06b69dc693a5c0b9d68ad1ba"></a>
## VerticalBarRightAngleBracket

`variant` · `sqlparser::tokenizer::Token::VerticalBarRightAngleBracket` · sqlparser 0.62.0

```rust
VerticalBarRightAngleBracket
```

Source: `src/tokenizer.rs:259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`|> BigQuery pipe operator

<a id="op-b2f07bef90b53c9f5c97d6aa"></a>
## VerticalBarShiftRight

`variant` · `sqlparser::tokenizer::Token::VerticalBarShiftRight` · sqlparser 0.62.0

```rust
VerticalBarShiftRight
```

Source: `src/tokenizer.rs:257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`|>> PostgreSQL/Redshift geometrical binary operator (Is strictly above?)

<a id="op-ec1746b26500d364b59274c3"></a>
## Whitespace

`variant` · `sqlparser::tokenizer::Token::Whitespace` · sqlparser 0.62.0

```rust
Whitespace
```

Source: `src/tokenizer.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whitespace (space, tab, etc)

<a id="op-b133b1c894ab379208940337"></a>
## Word

`variant` · `sqlparser::tokenizer::Token::Word` · sqlparser 0.62.0

```rust
Word
```

Source: `src/tokenizer.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A keyword (like SELECT) or an optionally quoted SQL identifier

<a id="op-dcb27ab8f65c2278d0e704e8"></a>
## clone

`function` · `sqlparser::tokenizer::Token::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Token
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 17], "end": [55, 22], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tokenizer.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faa5e8575b0958ac9710c6ed"></a>
## cmp

`function` · `sqlparser::tokenizer::Token::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Token) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 51], "end": [55, 54], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/tokenizer.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-162f21f266f27e89e45bd0f0"></a>
## deserialize

`function` · `sqlparser::tokenizer::Token::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 49], "end": [56, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/tokenizer.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63c265d6707a75c3e1659b5c"></a>
## eq

`function` · `sqlparser::tokenizer::Token::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Token) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 24], "end": [55, 33], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-789f57403039b1ea7070a3f6"></a>
## eq

`function` · `sqlparser::tokenizer::Token::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TokenWithSpan) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [776, 1], "end": [780, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}}], "constraints": []}}, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b217111d3f1a8caf9d24f35d"></a>
## fmt

`function` · `sqlparser::tokenizer::Token::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 10], "end": [55, 15], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tokenizer.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1f7bea459f0141264c55306"></a>
## fmt

`function` · `sqlparser::tokenizer::Token::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [401, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tokenizer.rs:292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c0a8adbf2566e7412aa071d"></a>
## hash

`function` · `sqlparser::tokenizer::Token::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 56], "end": [55, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/tokenizer.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80c54f75a43c2dd9aa9055c5"></a>
## make_keyword

`function` · `sqlparser::tokenizer::Token::make_keyword` · sqlparser 0.62.0

```rust
fn make_keyword(keyword: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [432, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:407`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a `Token::Word` from an unquoted `keyword`.

The lookup is case-insensitive; unknown values become `Keyword::NoKeyword`.

<a id="op-849adc6c32cfd35ab576f5eb"></a>
## make_word

`function` · `sqlparser::tokenizer::Token::make_word` · sqlparser 0.62.0

```rust
fn make_word(word: &str, quote_style: Option<char>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [432, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a `Token::Word` from `word` with an optional `quote_style`.

When `quote_style` is `None`, the parser attempts a case-insensitive keyword
lookup and sets the `Word::keyword` accordingly.

<a id="op-2f47e5cb755d01baa0ec95d8"></a>
## partial_cmp

`function` · `sqlparser::tokenizer::Token::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Token) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 35], "end": [55, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/tokenizer.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f09176f0a9d9cbfc92a6a54a"></a>
## serialize

`function` · `sqlparser::tokenizer::Token::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 38], "end": [56, 47], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/tokenizer.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-567eb775425cbfd4bca82607"></a>
## visit

`function` · `sqlparser::tokenizer::Token::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 47], "end": [57, 55], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/tokenizer.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-691ba1820b8509f2d5e4cfbc"></a>
## visit

`function` · `sqlparser::tokenizer::Token::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 40], "end": [57, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/tokenizer.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
