# `sqlparser::dialect::Dialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.Dialect.json).

<a id="op-f184df0633eac2f7d134147b"></a>
## Dialect

`trait` · `sqlparser::dialect::Dialect` · sqlparser 0.62.0

```rust
trait Dialect: Debug + Any
```

Source: `src/dialect/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Encapsulates the differences between SQL implementations.

# SQL Dialects

SQL implementations deviate from one another, either due to
custom extensions or various historical reasons. This trait
encapsulates the parsing differences between dialects.

[`GenericDialect`](../operations/sqlparser.dialect.generic.GenericDialect.md#op-e9e98e2bf43b55c23c54c302) is the most permissive dialect, and parses the union of
all the other dialects, when there is no ambiguity. However, it does not
currently allow `CREATE TABLE` statements without types specified for all
columns; use [`SQLiteDialect`](../operations/sqlparser.dialect.sqlite.SQLiteDialect.md#op-259947aa56697b5c1b719c5f) if you require that.

# Examples
Most users create a [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) directly, as shown on the [module
level documentation]:

```
# use sqlparser::dialect::AnsiDialect;
let dialect = AnsiDialect {};
```

It is also possible to dynamically create a [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) from its
name. For example:

```
# use sqlparser::dialect::{AnsiDialect, dialect_from_str};
let dialect = dialect_from_str("ansi").unwrap();

// Parsed dialect is an instance of `AnsiDialect`:
assert!(dialect.is::<AnsiDialect>());
```

[module level documentation]: crate

<a id="op-b88e38eafa5aea1237467362"></a>
## allow_extract_custom

`function` · `sqlparser::dialect::Dialect::allow_extract_custom` · sqlparser 0.62.0

```rust
fn allow_extract_custom(&self) -> bool
```

Source: `src/dialect/mod.rs:1000`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect allows the `EXTRACT` function to words other than [`Keyword`](../operations/sqlparser.keywords.Keyword.md#op-fc3be6e31fd1e97bc2df3cdc).

<a id="op-f893338118d41a12373f8a9b"></a>
## allow_extract_single_quotes

`function` · `sqlparser::dialect::Dialect::allow_extract_single_quotes` · sqlparser 0.62.0

```rust
fn allow_extract_single_quotes(&self) -> bool
```

Source: `src/dialect/mod.rs:1005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect allows the `EXTRACT` function to use single quotes in the part being extracted.

<a id="op-b8f822d58c7f3fa79df3ce95"></a>
## convert_type_before_value

`function` · `sqlparser::dialect::Dialect::convert_type_before_value` · sqlparser 0.62.0

```rust
fn convert_type_before_value(&self) -> bool
```

Source: `src/dialect/mod.rs:567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect has a CONVERT function which accepts a type first
and an expression second, e.g. `CONVERT(varchar, 1)`

<a id="op-b6fe51ef411c53750461b252"></a>
## describe_requires_table_keyword

`function` · `sqlparser::dialect::Dialect::describe_requires_table_keyword` · sqlparser 0.62.0

```rust
fn describe_requires_table_keyword(&self) -> bool
```

Source: `src/dialect/mod.rs:995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect requires the `TABLE` keyword after `DESCRIBE`

Defaults to false.

If true, the following statement is valid: `DESCRIBE TABLE my_table`
If false, the following statements are valid: `DESCRIBE my_table` and `DESCRIBE table`

<a id="op-ca74bd2e3c18aec1f67b921e"></a>
## dialect

`function` · `sqlparser::dialect::Dialect::dialect` · sqlparser 0.62.0

```rust
fn dialect(&self) -> TypeId
```

Source: `src/dialect/mod.rs:209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine the [`TypeId`] of this dialect.

By default, return the same [`TypeId`] as [`Any::type_id`]. Can be overridden by
dialects that behave like other dialects (for example, when wrapping a dialect).

Unresolved upstream links (retained, not inferred): ``TypeId``, ``Any::type_id``.

<a id="op-7e015b24b81c616f9fedcdca"></a>
## get_next_precedence

`function` · `sqlparser::dialect::Dialect::get_next_precedence` · sqlparser 0.62.0

```rust
fn get_next_precedence(&self, _parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
```

Source: `src/dialect/mod.rs:781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dialect-specific precedence override

This method is called to get the precedence of the next token.

If `None` is returned, falls back to the default behavior.

<a id="op-e3ff372a4f83da33d19d36f4"></a>
## get_next_precedence_default

`function` · `sqlparser::dialect::Dialect::get_next_precedence_default` · sqlparser 0.62.0

```rust
fn get_next_precedence_default(&self, parser: &Parser<'_>) -> Result<u8, ParserError>
```

Source: `src/dialect/mod.rs:795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Get the precedence of the next token, looking at the full token stream.

A higher number => higher precedence

See [`Self::get_next_precedence`](../operations/sqlparser.dialect.Dialect.md#op-7e015b24b81c616f9fedcdca) to override the behavior for just the
next token.

The default implementation is used for many dialects, but can be
overridden to provide dialect-specific behavior.

<a id="op-a9a94b50795a709343cfa913"></a>
## get_reserved_grantees_types

`function` · `sqlparser::dialect::Dialect::get_reserved_grantees_types` · sqlparser 0.62.0

```rust
fn get_reserved_grantees_types(&self) -> &[GranteesType]
```

Source: `src/dialect/mod.rs:1275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns grantee types that should be treated as identifiers

<a id="op-cf21cbfac9a194e463eff707"></a>
## get_reserved_keywords_for_select_item_operator

`function` · `sqlparser::dialect::Dialect::get_reserved_keywords_for_select_item_operator` · sqlparser 0.62.0

```rust
fn get_reserved_keywords_for_select_item_operator(&self) -> &[Keyword]
```

Source: `src/dialect/mod.rs:1270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns reserved keywords that may prefix a select item expression
e.g. `SELECT CONNECT_BY_ROOT name FROM Tbl2` (Snowflake)

<a id="op-12a02992ae2b3528d96081a5"></a>
## identifier_quote_style

`function` · `sqlparser::dialect::Dialect::identifier_quote_style` · sqlparser 0.62.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Source: `src/dialect/mod.rs:253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the character used to quote identifiers.

<a id="op-2bbd00c2809c744fca741a7d"></a>
## ignores_wildcard_escapes

`function` · `sqlparser::dialect::Dialect::ignores_wildcard_escapes` · sqlparser 0.62.0

```rust
fn ignores_wildcard_escapes(&self) -> bool
```

Source: `src/dialect/mod.rs:309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine whether the dialect strips the backslash when escaping LIKE wildcards (%, _).

[MySQL] has a special case when escaping single quoted strings which leaves these unescaped
so they can be used in LIKE patterns without double-escaping (as is necessary in other
escaping dialects, such as [Snowflake]). Generally, special characters have escaping rules
causing them to be replaced with a different byte sequences (e.g. `'\0'` becoming the zero
byte), and the default if an escaped character does not have a specific escaping rule is to
strip the backslash (e.g. there is no rule for `h`, so `'\h' = 'h'`). MySQL's special case
for ignoring LIKE wildcard escapes is to *not* strip the backslash, so that `'\%' = '\\%'`.
This applies to all string literals though, not just those used in LIKE patterns.

```text
mysql> select '\_', hex('\\'), hex('_'), hex('\_');
+----+-----------+----------+-----------+
| \_ | hex('\\') | hex('_') | hex('\_') |
+----+-----------+----------+-----------+
| \_ | 5C        | 5F       | 5C5F      |
+----+-----------+----------+-----------+
1 row in set (0.00 sec)
```

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/string-literals.html
[Snowflake]: https://docs.snowflake.com/en/sql-reference/functions/like#usage-notes

<a id="op-526c414b348cabbba74fa3b2"></a>
## is_column_alias

`function` · `sqlparser::dialect::Dialect::is_column_alias` · sqlparser 0.62.0

```rust
fn is_column_alias(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
```

Source: `src/dialect/mod.rs:1327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the specified keyword should be parsed as a column identifier.
See [keywords::RESERVED_FOR_COLUMN_ALIAS](../operations/sqlparser.keywords.RESERVED_FOR_COLUMN_ALIAS.md#op-f11a6f0e7e4d4424a601974b)

<a id="op-ecf2305573416bae3ecb6a62"></a>
## is_custom_operator_part

`function` · `sqlparser::dialect::Dialect::is_custom_operator_part` · sqlparser 0.62.0

```rust
fn is_custom_operator_part(&self, _ch: char) -> bool
```

Source: `src/dialect/mod.rs:264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Most dialects do not have custom operators. Override this method to provide custom operators.

<a id="op-c8e5377443895a0722ce9eaf"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::Dialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Source: `src/dialect/mod.rs:218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine if a character starts a quoted identifier. The default
implementation, accepting "double quoted" ids is both ANSI-compliant
and appropriate for most dialects (with the notable exception of
MySQL, MS SQL, and sqlite). You can accept one of characters listed
in `Word::matching_end_quote` here

<a id="op-cbc6d5df161352d31fa462c3"></a>
## is_identifier_generating_function_name

`function` · `sqlparser::dialect::Dialect::is_identifier_generating_function_name` · sqlparser 0.62.0

```rust
fn is_identifier_generating_function_name(&self, _ident: &Ident, _name_parts: &[ObjectNamePart]) -> bool
```

Source: `src/dialect/mod.rs:1447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect considers the specified ident as a function
that returns an identifier. Typically used to generate identifiers
programmatically.

- [Snowflake](https://docs.snowflake.com/en/sql-reference/identifier-literal)

<a id="op-5f9e589d59524957a6b4fdff"></a>
## is_identifier_part

`function` · `sqlparser::dialect::Dialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Source: `src/dialect/mod.rs:261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine if a character is a valid unquoted identifier character

<a id="op-3cbb7b189b31d908b84f71c3"></a>
## is_identifier_start

`function` · `sqlparser::dialect::Dialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Source: `src/dialect/mod.rs:258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine if a character is a valid start character for an unquoted identifier

<a id="op-3083264fca56ec47b5704b9c"></a>
## is_nested_delimited_identifier_start

`function` · `sqlparser::dialect::Dialect::is_nested_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_nested_delimited_identifier_start(&self, _ch: char) -> bool
```

Source: `src/dialect/mod.rs:230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine if a character starts a potential nested quoted identifier.
Example: RedShift supports the following quote styles to all mean the same thing:
```sql
SELECT 1 AS foo;
SELECT 1 AS "foo";
SELECT 1 AS [foo];
SELECT 1 AS ["foo"];
```

<a id="op-67b55074c445fc3208b97d4f"></a>
## is_reserved_for_identifier

`function` · `sqlparser::dialect::Dialect::is_reserved_for_identifier` · sqlparser 0.62.0

```rust
fn is_reserved_for_identifier(&self, kw: Keyword) -> bool
```

Source: `src/dialect/mod.rs:1264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the specified keyword is reserved and cannot be
used as an identifier without special handling like quoting.

<a id="op-49b620b94945287ce47ab13b"></a>
## is_select_item_alias

`function` · `sqlparser::dialect::Dialect::is_select_item_alias` · sqlparser 0.62.0

```rust
fn is_select_item_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
```

Source: `src/dialect/mod.rs:1334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the specified keyword should be parsed as a select item alias.
When explicit is true, the keyword is preceded by an `AS` word. Parser is provided
to enable looking ahead if needed.

<a id="op-b8618ffb73ae5069f48d77d6"></a>
## is_table_alias

`function` · `sqlparser::dialect::Dialect::is_table_alias` · sqlparser 0.62.0

```rust
fn is_table_alias(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
```

Source: `src/dialect/mod.rs:1346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the specified keyword should be parsed as a table factor alias.
See [keywords::RESERVED_FOR_TABLE_ALIAS](../operations/sqlparser.keywords.RESERVED_FOR_TABLE_ALIAS.md#op-c0faeaf69f1f5235a1d47011)

<a id="op-682822d62e21088f68fe0f2d"></a>
## is_table_factor

`function` · `sqlparser::dialect::Dialect::is_table_factor` · sqlparser 0.62.0

```rust
fn is_table_factor(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
```

Source: `src/dialect/mod.rs:1340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the specified keyword should be parsed as a table factor identifier.
See [keywords::RESERVED_FOR_TABLE_FACTOR](../operations/sqlparser.keywords.RESERVED_FOR_TABLE_FACTOR.md#op-cfd79c4cf4740c1f75843cb0)

<a id="op-0b9dbfcaad691daa06c8984e"></a>
## is_table_factor_alias

`function` · `sqlparser::dialect::Dialect::is_table_factor_alias` · sqlparser 0.62.0

```rust
fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
```

Source: `src/dialect/mod.rs:1353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the specified keyword should be parsed as a table factor alias.
When explicit is true, the keyword is preceded by an `AS` word. Parser is provided
to enable looking ahead if needed.

<a id="op-78fc4a562546f1821ae0b66e"></a>
## parse_column_option

`function` · `sqlparser::dialect::Dialect::parse_column_option` · sqlparser 0.62.0

```rust
fn parse_column_option(&self, _parser: &mut Parser<'_>) -> Result<Option<Result<Option<ColumnOption>, ParserError>>, ParserError>
```

Source: `src/dialect/mod.rs:950`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dialect-specific column option parser override

This method is called to parse the next column option.

If `None` is returned, falls back to the default behavior.

<a id="op-1aba7512c52f7dd3d27b76b5"></a>
## parse_infix

`function` · `sqlparser::dialect::Dialect::parse_infix` · sqlparser 0.62.0

```rust
fn parse_infix(&self, _parser: &mut Parser<'_>, _expr: &Expr, _precedence: u8) -> Option<Result<Expr, ParserError>>
```

Source: `src/dialect/mod.rs:766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dialect-specific infix parser override

This method is called to parse the next infix expression.

If `None` is returned, falls back to the default behavior.

<a id="op-dfc6d2e09687071be1936ccd"></a>
## parse_prefix

`function` · `sqlparser::dialect::Dialect::parse_prefix` · sqlparser 0.62.0

```rust
fn parse_prefix(&self, _parser: &mut Parser<'_>) -> Option<Result<Expr, ParserError>>
```

Source: `src/dialect/mod.rs:578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dialect-specific prefix parser override

<a id="op-4450b4d3a81d6405c7222e4b"></a>
## parse_statement

`function` · `sqlparser::dialect::Dialect::parse_statement` · sqlparser 0.62.0

```rust
fn parse_statement(&self, _parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
```

Source: `src/dialect/mod.rs:940`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dialect-specific statement parser override

This method is called to parse the next statement.

If `None` is returned, falls back to the default behavior.

<a id="op-b462dc1cc4473eda70f0c69e"></a>
## peek_nested_delimited_identifier_quotes

`function` · `sqlparser::dialect::Dialect::peek_nested_delimited_identifier_quotes` · sqlparser 0.62.0

```rust
fn peek_nested_delimited_identifier_quotes(&self, _chars: Peekable<Chars<'_>>) -> Option<(char, Option<char>)>
```

Source: `src/dialect/mod.rs:245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only applicable whenever [`Self::is_nested_delimited_identifier_start`](../operations/sqlparser.dialect.Dialect.md#op-3083264fca56ec47b5704b9c) returns true
If the next sequence of tokens potentially represent a nested identifier, then this method
returns a tuple containing the outer quote style, and if present, the inner (nested) quote style.

Example (Redshift):
```text
`["foo"]` => Some(`[`, Some(`"`))
`[foo]` => Some(`[`, None)
`[0]` => None
`"foo"` => None
```

<a id="op-e5621e483947db032120748c"></a>
## prec_unknown

`function` · `sqlparser::dialect::Dialect::prec_unknown` · sqlparser 0.62.0

```rust
fn prec_unknown(&self) -> u8
```

Source: `src/dialect/mod.rs:985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns the precedence when the precedence is otherwise unknown

<a id="op-91566cd66515f64bc307a6c8"></a>
## prec_value

`function` · `sqlparser::dialect::Dialect::prec_value` · sqlparser 0.62.0

```rust
fn prec_value(&self, prec: Precedence) -> u8
```

Source: `src/dialect/mod.rs:961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Decide the lexical Precedence of operators.

Uses (APPROXIMATELY) <https://www.postgresql.org/docs/7.0/operators.htm#AEN2026> as a reference

<a id="op-1fff448970e531a2b801ccf1"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::Dialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Source: `src/dialect/mod.rs:1125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `INTERVAL` expressions require units (called "qualifiers" in the ANSI SQL spec) to be specified,
e.g. `INTERVAL 1 DAY` vs `INTERVAL 1`.

Expressions within intervals (e.g. `INTERVAL '1' + '1' DAY`) are only allowed when units are required.

See <https://github.com/sqlparser-rs/sqlparser-rs/pull/1398> for more information.

When `true`:
* `INTERVAL '1' DAY` is VALID
* `INTERVAL 1 + 1 DAY` is VALID
* `INTERVAL '1' + '1' DAY` is VALID
* `INTERVAL '1'` is INVALID

When `false`:
* `INTERVAL '1'` is VALID
* `INTERVAL '1' DAY` is VALID — unit is not required, but still allowed
* `INTERVAL 1 + 1 DAY` is INVALID

<a id="op-ad2924000454a2c6ac5dd141"></a>
## requires_single_line_comment_whitespace

`function` · `sqlparser::dialect::Dialect::requires_single_line_comment_whitespace` · sqlparser 0.62.0

```rust
fn requires_single_line_comment_whitespace(&self) -> bool
```

Source: `src/dialect/mod.rs:1380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect requires a whitespace character after `--` to start a single line comment.

MySQL: <https://dev.mysql.com/doc/refman/8.4/en/ansi-diff-comments.html>
e.g. UPDATE account SET balance=balance--1

<a id="op-6a901d2dc8ffca02fc075ab4"></a>
## support_map_literal_syntax

`function` · `sqlparser::dialect::Dialect::support_map_literal_syntax` · sqlparser 0.62.0

```rust
fn support_map_literal_syntax(&self) -> bool
```

Source: `src/dialect/mod.rs:511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports defining object using the
syntax like `Map {1: 10, 2: 20}`.

<a id="op-f0e18546b1a4159e3da560d9"></a>
## supports_alter_column_type_using

`function` · `sqlparser::dialect::Dialect::supports_alter_column_type_using` · sqlparser 0.62.0

```rust
fn supports_alter_column_type_using(&self) -> bool
```

Source: `src/dialect/mod.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `USING` clause in an `ALTER COLUMN` statement.
Example:
 ```sql
 ALTER TABLE tbl ALTER COLUMN col SET DATA TYPE <type> USING <exp>`
```

<a id="op-d381808f8380e702974360c9"></a>
## supports_array_join_syntax

`function` · `sqlparser::dialect::Dialect::supports_array_join_syntax` · sqlparser 0.62.0

```rust
fn supports_array_join_syntax(&self) -> bool
```

Source: `src/dialect/mod.rs:365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports ClickHouse-style `ARRAY JOIN` / `LEFT ARRAY JOIN` /
`INNER ARRAY JOIN` syntax for unnesting arrays inline.

<https://clickhouse.com/docs/en/sql-reference/statements/select/array-join>

<a id="op-0a7db40f153a88cba370efc7"></a>
## supports_array_typedef_with_brackets

`function` · `sqlparser::dialect::Dialect::supports_array_typedef_with_brackets` · sqlparser 0.62.0

```rust
fn supports_array_typedef_with_brackets(&self) -> bool
```

Source: `src/dialect/mod.rs:1388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports array type definition with brackets with
an optional size. For example:
```CREATE TABLE my_table (arr1 INT[], arr2 INT[3])```
```SELECT x::INT[]```

<a id="op-711c962c7dd425eeec999db3"></a>
## supports_array_typedef_without_element_type

`function` · `sqlparser::dialect::Dialect::supports_array_typedef_without_element_type` · sqlparser 0.62.0

```rust
fn supports_array_typedef_without_element_type(&self) -> bool
```

Source: `src/dialect/mod.rs:1057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `ARRAY` type without
specifying an element type.

Example:
```sql
CREATE TABLE t (a ARRAY);
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/data-types-semistructured#array)

<a id="op-66541b10b73f947db7375097"></a>
## supports_asc_desc_in_column_definition

`function` · `sqlparser::dialect::Dialect::supports_asc_desc_in_column_definition` · sqlparser 0.62.0

```rust
fn supports_asc_desc_in_column_definition(&self) -> bool
```

Source: `src/dialect/mod.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `ASC` and `DESC` in column definitions
e.g. `CREATE TABLE t (a INT ASC, b INT DESC);`

<a id="op-5a0fa539a4aa2a79e6ff0bf2"></a>
## supports_bang_not_operator

`function` · `sqlparser::dialect::Dialect::supports_bang_not_operator` · sqlparser 0.62.0

```rust
fn supports_bang_not_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:1180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `!a` syntax for boolean `NOT` expressions.

<a id="op-3d04e0bf9e7b3b2f3614f104"></a>
## supports_binary_kw_as_cast

`function` · `sqlparser::dialect::Dialect::supports_binary_kw_as_cast` · sqlparser 0.62.0

```rust
fn supports_binary_kw_as_cast(&self) -> bool
```

Source: `src/dialect/mod.rs:1545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports casting an expression to a binary type
using the `BINARY <expr>` syntax.

<a id="op-9fc5f97dd446a61d9e260e68"></a>
## supports_bitwise_shift_operators

`function` · `sqlparser::dialect::Dialect::supports_bitwise_shift_operators` · sqlparser 0.62.0

```rust
fn supports_bitwise_shift_operators(&self) -> bool
```

Source: `src/dialect/mod.rs:1147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `<<` and `>>` shift operators.

<a id="op-be7ac56ec2d863d18bbcf6a8"></a>
## supports_boolean_literals

`function` · `sqlparser::dialect::Dialect::supports_boolean_literals` · sqlparser 0.62.0

```rust
fn supports_boolean_literals(&self) -> bool
```

Source: `src/dialect/mod.rs:1207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports boolean literals (`true` and `false`).
For example, in MSSQL these are treated as identifiers rather than boolean literals.

<a id="op-44a23c6b70850465b5b68e8f"></a>
## supports_column_definition_trailing_commas

`function` · `sqlparser::dialect::Dialect::supports_column_definition_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_column_definition_trailing_commas(&self) -> bool
```

Source: `src/dialect/mod.rs:626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports trailing commas in the
column definitions list of a `CREATE` statement.
Example: `CREATE TABLE T (x INT, y TEXT,)`

<a id="op-cce80d3e8ba0ccd1f685b037"></a>
## supports_comma_separated_drop_column_list

`function` · `sqlparser::dialect::Dialect::supports_comma_separated_drop_column_list` · sqlparser 0.62.0

```rust
fn supports_comma_separated_drop_column_list(&self) -> bool
```

Source: `src/dialect/mod.rs:1438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `ALTER TABLE tbl DROP COLUMN c1, ..., cn`

<a id="op-8b6f1374f2705ba988e9e7bd"></a>
## supports_comma_separated_set_assignments

`function` · `sqlparser::dialect::Dialect::supports_comma_separated_set_assignments` · sqlparser 0.62.0

```rust
fn supports_comma_separated_set_assignments(&self) -> bool
```

Source: `src/dialect/mod.rs:540`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports multiple `SET` statements
in a single statement.

```sql
SET variable = expression [, variable = expression];
```

<a id="op-054b602698043045774ad0a1"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::Dialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Source: `src/dialect/mod.rs:1715`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the two-argument comma-separated
form of the `TRIM` function: `TRIM(expr, characters)`.

<a id="op-09ee000064af5d1b3450805f"></a>
## supports_comment_on

`function` · `sqlparser::dialect::Dialect::supports_comment_on` · sqlparser 0.62.0

```rust
fn supports_comment_on(&self) -> bool
```

Source: `src/dialect/mod.rs:1218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `COMMENT` statement

<a id="op-7403a3098c007b6c585adf4d"></a>
## supports_comment_optimizer_hint

`function` · `sqlparser::dialect::Dialect::supports_comment_optimizer_hint` · sqlparser 0.62.0

```rust
fn supports_comment_optimizer_hint(&self) -> bool
```

Source: `src/dialect/mod.rs:1534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns `true` if the dialect supports query optimizer hints in the
format of single and multi line comments immediately following a
`SELECT`, `INSERT`, `REPLACE`, `DELETE`, or `MERGE` keyword.

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/optimizer-hints.html)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/19/sqlrf/Comments.html#SQLRF-GUID-D316D545-89E2-4D54-977F-FC97815CD62E)

<a id="op-987617c81c6c8434bc906f9a"></a>
## supports_connect_by

`function` · `sqlparser::dialect::Dialect::supports_connect_by` · sqlparser 0.62.0

```rust
fn supports_connect_by(&self) -> bool
```

Source: `src/dialect/mod.rs:419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports CONNECT BY.

<a id="op-197910cbffd2a293645fb706"></a>
## supports_constraint_keyword_without_name

`function` · `sqlparser::dialect::Dialect::supports_constraint_keyword_without_name` · sqlparser 0.62.0

```rust
fn supports_constraint_keyword_without_name(&self) -> bool
```

Source: `src/dialect/mod.rs:1246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `CONSTRAINT` keyword without a name
in table constraint definitions.

Example:
```sql
CREATE TABLE t (a INT, CONSTRAINT CHECK (a > 0))
```

This is a MySQL extension; the SQL standard requires a name after `CONSTRAINT`.
When the name is omitted, the output normalizes to just the constraint type
without the `CONSTRAINT` keyword (e.g., `CHECK (a > 0)`).

<https://dev.mysql.com/doc/refman/8.4/en/create-table.html>

<a id="op-1b9c16fb77689c92b605075e"></a>
## supports_create_index_with_clause

`function` · `sqlparser::dialect::Dialect::supports_create_index_with_clause` · sqlparser 0.62.0

```rust
fn supports_create_index_with_clause(&self) -> bool
```

Source: `src/dialect/mod.rs:1104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support with clause in create index statement?
e.g. `CREATE INDEX idx ON t WITH (key = value, key2)`

<a id="op-b04c90de9825eadadd7ef991"></a>
## supports_create_table_like_parenthesized

`function` · `sqlparser::dialect::Dialect::supports_create_table_like_parenthesized` · sqlparser 0.62.0

```rust
fn supports_create_table_like_parenthesized(&self) -> bool
```

Source: `src/dialect/mod.rs:1503`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports specifying which table to copy
the schema from inside parenthesis.

Not parenthesized:
'''sql
CREATE TABLE new LIKE old ...
'''
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-table#label-create-table-like)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_table_like)

Parenthesized:
'''sql
CREATE TABLE new (LIKE old ...)
'''
[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_TABLE_NEW.html)

<a id="op-269f7c74f2974a92bec3739b"></a>
## supports_create_table_multi_schema_info_sources

`function` · `sqlparser::dialect::Dialect::supports_create_table_multi_schema_info_sources` · sqlparser 0.62.0

```rust
fn supports_create_table_multi_schema_info_sources(&self) -> bool
```

Source: `src/dialect/mod.rs:744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports specifying multiple options
in a `CREATE TABLE` statement for the structure of the new table. For example:
`CREATE TABLE t (a INT, b INT) AS SELECT 1 AS b, 2 AS a`

<a id="op-f972b4584269ec63881b7080"></a>
## supports_create_table_select

`function` · `sqlparser::dialect::Dialect::supports_create_table_select` · sqlparser 0.62.0

```rust
fn supports_create_table_select(&self) -> bool
```

Source: `src/dialect/mod.rs:1223`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `CREATE TABLE SELECT` statement

<a id="op-293f613093ebbc352431b7fd"></a>
## supports_create_table_using

`function` · `sqlparser::dialect::Dialect::supports_create_table_using` · sqlparser 0.62.0

```rust
fn supports_create_table_using(&self) -> bool
```

Source: `src/dialect/mod.rs:1760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `USING <format>` in `CREATE TABLE`.

Example:
```sql
CREATE TABLE t (i INT) USING PARQUET
```

[Spark SQL](https://spark.apache.org/docs/latest/sql-ref-syntax-ddl-create-table-datasource.html)

<a id="op-aee4d1f0f321a85590e7ae2f"></a>
## supports_create_view_comment_syntax

`function` · `sqlparser::dialect::Dialect::supports_create_view_comment_syntax` · sqlparser 0.62.0

```rust
fn supports_create_view_comment_syntax(&self) -> bool
```

Source: `src/dialect/mod.rs:1044`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `COMMENT` clause in
`CREATE VIEW` statements using the `COMMENT = 'comment'` syntax.

Example:
```sql
CREATE VIEW v COMMENT = 'my comment' AS SELECT 1;
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-view#optional-parameters)

<a id="op-3775eda8b1dc944cbf649dce"></a>
## supports_cross_join_constraint

`function` · `sqlparser::dialect::Dialect::supports_cross_join_constraint` · sqlparser 0.62.0

```rust
fn supports_cross_join_constraint(&self) -> bool
```

Source: `src/dialect/mod.rs:414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports a join specification on CROSS JOIN.

<a id="op-3b85815d995dba8e99e2a06c"></a>
## supports_cte_without_as

`function` · `sqlparser::dialect::Dialect::supports_cte_without_as` · sqlparser 0.62.0

```rust
fn supports_cte_without_as(&self) -> bool
```

Source: `src/dialect/mod.rs:1726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `AS` keyword being
optional in a CTE definition. For example:
```sql
WITH cte_name (SELECT ...)
```

[Databricks](https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-qry-select-cte)

<a id="op-c60d127d3be3cc9d5ac0d5a5"></a>
## supports_data_type_signed_suffix

`function` · `sqlparser::dialect::Dialect::supports_data_type_signed_suffix` · sqlparser 0.62.0

```rust
fn supports_data_type_signed_suffix(&self) -> bool
```

Source: `src/dialect/mod.rs:1469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect allows an optional `SIGNED` suffix after integer data types.

Example:
```sql
CREATE TABLE t (i INT(20) SIGNED);
```

Note that this is canonicalized to `INT(20)`.

<a id="op-a8320226866b58acd53a4b6e"></a>
## supports_detach

`function` · `sqlparser::dialect::Dialect::supports_detach` · sqlparser 0.62.0

```rust
fn supports_detach(&self) -> bool
```

Source: `src/dialect/mod.rs:1634`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `DETACH` statement.

Example:
```sql
DETACH DATABASE db_name;
```

[DuckDB](https://duckdb.org/docs/sql/statements/attach#detach-syntax)

<a id="op-bb63c68fe9f2e6f21ab02370"></a>
## supports_dictionary_syntax

`function` · `sqlparser::dialect::Dialect::supports_dictionary_syntax` · sqlparser 0.62.0

```rust
fn supports_dictionary_syntax(&self) -> bool
```

Source: `src/dialect/mod.rs:505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports defining structs or objects using a
syntax like `{'x': 1, 'y': 2, 'z': 3}`.

<a id="op-d434544d13156cd451a0c670"></a>
## supports_dollar_as_money_prefix

`function` · `sqlparser::dialect::Dialect::supports_dollar_as_money_prefix` · sqlparser 0.62.0

```rust
fn supports_dollar_as_money_prefix(&self) -> bool
```

Source: `src/dialect/mod.rs:1098`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports `$` as a prefix for money literals
e.g. `SELECT $123.45` (SQL Server)

<a id="op-38ce57bb3d15f9fc7931d258"></a>
## supports_dollar_placeholder

`function` · `sqlparser::dialect::Dialect::supports_dollar_placeholder` · sqlparser 0.62.0

```rust
fn supports_dollar_placeholder(&self) -> bool
```

Source: `src/dialect/mod.rs:1092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect allows dollar placeholders
e.g. `SELECT $var` (SQLite)

<a id="op-fd03394be78160e34fae320e"></a>
## supports_double_ampersand_operator

`function` · `sqlparser::dialect::Dialect::supports_double_ampersand_operator` · sqlparser 0.62.0

```rust
fn supports_double_ampersand_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:1539`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect considers the `&&` operator as a boolean AND operator.

<a id="op-ef48eaa60281b3132a043a28"></a>
## supports_empty_projections

`function` · `sqlparser::dialect::Dialect::supports_empty_projections` · sqlparser 0.62.0

```rust
fn supports_empty_projections(&self) -> bool
```

Source: `src/dialect/mod.rs:656`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return true if the dialect supports empty projections in SELECT statements

Example
```sql
SELECT from table_name
```

<a id="op-a44b22812f60ccc43c145cfd"></a>
## supports_end_transaction_modifier

`function` · `sqlparser::dialect::Dialect::supports_end_transaction_modifier` · sqlparser 0.62.0

```rust
fn supports_end_transaction_modifier(&self) -> bool
```

Source: `src/dialect/mod.rs:444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `END {TRY | CATCH}` statements

<a id="op-ef7a3ff36ebea66d77002157"></a>
## supports_eq_alias_assignment

`function` · `sqlparser::dialect::Dialect::supports_eq_alias_assignment` · sqlparser 0.62.0

```rust
fn supports_eq_alias_assignment(&self) -> bool
```

Source: `src/dialect/mod.rs:1170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports treating the equals operator `=` within a `SelectItem`
as an alias assignment operator, rather than a boolean expression.
For example: the following statements are equivalent for such a dialect:
```sql
 SELECT col_alias = col FROM tbl;
 SELECT col_alias AS col FROM tbl;
```

<a id="op-be07d149318019fb0ece74cc"></a>
## supports_execute_immediate

`function` · `sqlparser::dialect::Dialect::supports_execute_immediate` · sqlparser 0.62.0

```rust
fn supports_execute_immediate(&self) -> bool
```

Source: `src/dialect/mod.rs:424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `EXECUTE IMMEDIATE` statements.

<a id="op-805589ff8e3b6b01fe9b2df2"></a>
## supports_explain_with_utility_options

`function` · `sqlparser::dialect::Dialect::supports_explain_with_utility_options` · sqlparser 0.62.0

```rust
fn supports_explain_with_utility_options(&self) -> bool
```

Source: `src/dialect/mod.rs:1131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `EXPLAIN` statements with utility options
e.g. `EXPLAIN (ANALYZE TRUE, BUFFERS TRUE) SELECT * FROM tbl;`

<a id="op-23eb1510da1ff0ca6dca5ef8"></a>
## supports_extract_comma_syntax

`function` · `sqlparser::dialect::Dialect::supports_extract_comma_syntax` · sqlparser 0.62.0

```rust
fn supports_extract_comma_syntax(&self) -> bool
```

Source: `src/dialect/mod.rs:1018`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `EXTRACT` function
with a comma separator instead of `FROM`.

Example:
```sql
SELECT EXTRACT(YEAR, date_column) FROM table;
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/functions/extract)

<a id="op-2e867c58f468e1b286edfcdf"></a>
## supports_factorial_operator

`function` · `sqlparser::dialect::Dialect::supports_factorial_operator` · sqlparser 0.62.0

```rust
fn supports_factorial_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:1142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `a!` expressions

<a id="op-000a44932ed59c4ae812b8da"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::Dialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Source: `src/dialect/mod.rs:328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support `FILTER (WHERE expr)` for aggregate queries?

<a id="op-65fb3c03bfafe55f16f53978"></a>
## supports_from_first_insert

`function` · `sqlparser::dialect::Dialect::supports_from_first_insert` · sqlparser 0.62.0

```rust
fn supports_from_first_insert(&self) -> bool
```

Source: `src/dialect/mod.rs:693`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return true if the dialect supports "FROM-first" inserts.

Example:
```sql
WITH cte AS (SELECT key FROM src)
FROM cte
INSERT OVERWRITE table my_table
SELECT *

See <https://hive.apache.org/docs/latest/language/common-table-expression/>
```

<a id="op-5a8be2754961fed70a261512"></a>
## supports_from_first_select

`function` · `sqlparser::dialect::Dialect::supports_from_first_select` · sqlparser 0.62.0

```rust
fn supports_from_first_select(&self) -> bool
```

Source: `src/dialect/mod.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return true if the dialect supports "FROM-first" selects.

Example:
```sql
FROM table
SELECT *
```

<a id="op-3f5bc840ccd5e12e2e3ad47b"></a>
## supports_from_trailing_commas

`function` · `sqlparser::dialect::Dialect::supports_from_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_from_trailing_commas(&self) -> bool
```

Source: `src/dialect/mod.rs:619`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports trailing commas in the `FROM` clause of a `SELECT` statement.
Example: `SELECT 1 FROM T, U, LIMIT 1`

<a id="op-e8b3bf0a2ab88d0adf1f739e"></a>
## supports_geometric_types

`function` · `sqlparser::dialect::Dialect::supports_geometric_types` · sqlparser 0.62.0

```rust
fn supports_geometric_types(&self) -> bool
```

Source: `src/dialect/mod.rs:1395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports geometric types.

Postgres: <https://www.postgresql.org/docs/9.5/functions-geometry.html>
e.g. @@ circle '((0,0),10)'

<a id="op-47836755f36ade97474f7521"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::Dialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Source: `src/dialect/mod.rs:370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialects supports `group sets, roll up, or cube` expressions.

<a id="op-b7b461fbce77c634ddfcbffd"></a>
## supports_group_by_with_modifier

`function` · `sqlparser::dialect::Dialect::supports_group_by_with_modifier` · sqlparser 0.62.0

```rust
fn supports_group_by_with_modifier(&self) -> bool
```

Source: `src/dialect/mod.rs:376`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialects supports `GROUP BY` modifiers prefixed by a `WITH` keyword.
Example: `GROUP BY value WITH ROLLUP`.

<a id="op-892617d5700d4232a9318160"></a>
## supports_in_empty_list

`function` · `sqlparser::dialect::Dialect::supports_in_empty_list` · sqlparser 0.62.0

```rust
fn supports_in_empty_list(&self) -> bool
```

Source: `src/dialect/mod.rs:434`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `(NOT) IN ()` expressions

<a id="op-e43d0a340920f4e7b8083d20"></a>
## supports_insert_format

`function` · `sqlparser::dialect::Dialect::supports_insert_format` · sqlparser 0.62.0

```rust
fn supports_insert_format(&self) -> bool
```

Source: `src/dialect/mod.rs:1310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support insert formats, e.g. `INSERT INTO ... FORMAT <format>`

<a id="op-f529183d16cdc58b1991b1a2"></a>
## supports_insert_set

`function` · `sqlparser::dialect::Dialect::supports_insert_set` · sqlparser 0.62.0

```rust
fn supports_insert_set(&self) -> bool
```

Source: `src/dialect/mod.rs:1293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `INSERT INTO ... SET col1 = 1, ...` syntax.

MySQL: <https://dev.mysql.com/doc/refman/8.4/en/insert.html>

<a id="op-5398d8c7d56c5eabf00fc95f"></a>
## supports_insert_table_alias

`function` · `sqlparser::dialect::Dialect::supports_insert_table_alias` · sqlparser 0.62.0

```rust
fn supports_insert_table_alias(&self) -> bool
```

Source: `src/dialect/mod.rs:1315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports `INSERT INTO t [[AS] alias] ...`.

<a id="op-a14624031fac9311ab0006b1"></a>
## supports_insert_table_function

`function` · `sqlparser::dialect::Dialect::supports_insert_table_function` · sqlparser 0.62.0

```rust
fn supports_insert_table_function(&self) -> bool
```

Source: `src/dialect/mod.rs:1298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support table function in insertion?

<a id="op-811408112ca9ed922d4a1bca"></a>
## supports_insert_table_query

`function` · `sqlparser::dialect::Dialect::supports_insert_table_query` · sqlparser 0.62.0

```rust
fn supports_insert_table_query(&self) -> bool
```

Source: `src/dialect/mod.rs:1305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support table queries in insertion?

e.g. `SELECT INTO (<query>) ...`

<a id="op-27ac493156082e236f643950"></a>
## supports_install

`function` · `sqlparser::dialect::Dialect::supports_install` · sqlparser 0.62.0

```rust
fn supports_install(&self) -> bool
```

Source: `src/dialect/mod.rs:1622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `INSTALL` statement.

Example:
```sql
INSTALL extension_name;
```

[DuckDB](https://duckdb.org/docs/extensions/overview)

<a id="op-72e9d40adcdcffbd945a11f4"></a>
## supports_interpolate

`function` · `sqlparser::dialect::Dialect::supports_interpolate` · sqlparser 0.62.0

```rust
fn supports_interpolate(&self) -> bool
```

Source: `src/dialect/mod.rs:1685`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `INTERPOLATE` clause
in `ORDER BY` expressions.

Example:
```sql
SELECT * FROM table ORDER BY col WITH FILL INTERPOLATE (col2 AS col2 + 1);
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/order-by#order-by-expr-with-fill-modifier)

<a id="op-84cb254b3a01ffcaed263a0c"></a>
## supports_interval_options

`function` · `sqlparser::dialect::Dialect::supports_interval_options` · sqlparser 0.62.0

```rust
fn supports_interval_options(&self) -> bool
```

Source: `src/dialect/mod.rs:1484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `INTERVAL` data type with [Postgres]-style options.

Examples:
```sql
CREATE TABLE t (i INTERVAL YEAR TO MONTH);
SELECT '1 second'::INTERVAL HOUR TO SECOND(3);
```

See [`crate::ast::DataType::Interval`](../operations/sqlparser.ast.data_type.DataType.md#op-0e30775bca9a6b671256d870) and [`crate::ast::IntervalFields`](../operations/sqlparser.ast.data_type.IntervalFields.md#op-b6dd859f176b9781491b678c).

[Postgres]: https://www.postgresql.org/docs/17/datatype-datetime.html

<a id="op-50600e56c1c04d730bebed66"></a>
## supports_key_column_option

`function` · `sqlparser::dialect::Dialect::supports_key_column_option` · sqlparser 0.62.0

```rust
fn supports_key_column_option(&self) -> bool
```

Source: `src/dialect/mod.rs:1258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `KEY` keyword as part of
column-level constraints in a `CREATE TABLE` statement.

When enabled, the parser accepts these MySQL-specific column options:
- `UNIQUE [KEY]` — optional `KEY` after `UNIQUE`
- `[PRIMARY] KEY` — standalone `KEY` as shorthand for `PRIMARY KEY`

<https://dev.mysql.com/doc/refman/8.4/en/create-table.html>

<a id="op-f16c51ae4999c2185ca0a7fd"></a>
## supports_lambda_functions

`function` · `sqlparser::dialect::Dialect::supports_lambda_functions` · sqlparser 0.62.0

```rust
fn supports_lambda_functions(&self) -> bool
```

Source: `src/dialect/mod.rs:520`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports lambda functions, for example:

```sql
SELECT transform(array(1, 2, 3), x -> x + 1); -- returns [2,3,4]
```

<a id="op-724b15ce4b156943d37975d1"></a>
## supports_left_associative_joins_without_parens

`function` · `sqlparser::dialect::Dialect::supports_left_associative_joins_without_parens` · sqlparser 0.62.0

```rust
fn supports_left_associative_joins_without_parens(&self) -> bool
```

Source: `src/dialect/mod.rs:404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates whether the dialect supports left-associative join parsing
by default when parentheses are omitted in nested joins.

Most dialects (like MySQL or Postgres) assume **left-associative** precedence,
so a query like:

```sql
SELECT * FROM t1 NATURAL JOIN t5 INNER JOIN t0 ON ...
```
is interpreted as:
```sql
((t1 NATURAL JOIN t5) INNER JOIN t0 ON ...)
```
and internally represented as a **flat list** of joins.

In contrast, some dialects (e.g. **Snowflake**) assume **right-associative**
precedence and interpret the same query as:
```sql
(t1 NATURAL JOIN (t5 INNER JOIN t0 ON ...))
```
which results in a **nested join** structure in the AST.

If this method returns `false`, the parser must build nested join trees
even in the absence of parentheses to reflect the correct associativity

<a id="op-3386c16f9481b1aff3891d8c"></a>
## supports_limit_by

`function` · `sqlparser::dialect::Dialect::supports_limit_by` · sqlparser 0.62.0

```rust
fn supports_limit_by(&self) -> bool
```

Source: `src/dialect/mod.rs:1672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `LIMIT BY` clause.

Example:
```sql
SELECT * FROM table LIMIT 10 BY col;
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/limit-by)

<a id="op-fc8148ac0aa9abc5eafc4266"></a>
## supports_limit_comma

`function` · `sqlparser::dialect::Dialect::supports_limit_comma` · sqlparser 0.62.0

```rust
fn supports_limit_comma(&self) -> bool
```

Source: `src/dialect/mod.rs:589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support parsing `LIMIT 1, 2` as `LIMIT 2 OFFSET 1`?

<a id="op-99eee2aab2904a8f2dd1f295"></a>
## supports_listen_notify

`function` · `sqlparser::dialect::Dialect::supports_listen_notify` · sqlparser 0.62.0

```rust
fn supports_listen_notify(&self) -> bool
```

Source: `src/dialect/mod.rs:1185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `LISTEN`, `UNLISTEN` and `NOTIFY` statements

<a id="op-8e3209a8a0f5b13e596ea7cd"></a>
## supports_load_data

`function` · `sqlparser::dialect::Dialect::supports_load_data` · sqlparser 0.62.0

```rust
fn supports_load_data(&self) -> bool
```

Source: `src/dialect/mod.rs:1190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `LOAD DATA` statement

<a id="op-cdd9ca95ecc1688a4fc77f5b"></a>
## supports_load_extension

`function` · `sqlparser::dialect::Dialect::supports_load_extension` · sqlparser 0.62.0

```rust
fn supports_load_extension(&self) -> bool
```

Source: `src/dialect/mod.rs:1195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `LOAD extension` statement

<a id="op-6861e92c39252f1b0f5678a2"></a>
## supports_long_type_as_bigint

`function` · `sqlparser::dialect::Dialect::supports_long_type_as_bigint` · sqlparser 0.62.0

```rust
fn supports_long_type_as_bigint(&self) -> bool
```

Source: `src/dialect/mod.rs:1772`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect treats `LONG` as an alias for `BIGINT`.

Example:
```sql
CREATE TABLE t (id LONG)
```

[Spark SQL](https://spark.apache.org/docs/latest/sql-ref-datatypes.html)

<a id="op-a1f97f691754fce8e63dfb3a"></a>
## supports_map_literal_with_angle_brackets

`function` · `sqlparser::dialect::Dialect::supports_map_literal_with_angle_brackets` · sqlparser 0.62.0

```rust
fn supports_map_literal_with_angle_brackets(&self) -> bool
```

Source: `src/dialect/mod.rs:1784`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `MAP<K, V>` angle-bracket syntax for the MAP data type.

Example:
```sql
CREATE TABLE t (m MAP<STRING, INT>)
```

[Spark SQL](https://spark.apache.org/docs/latest/sql-ref-datatypes.html)

<a id="op-f068bb41552e47dae243267c"></a>
## supports_match_against

`function` · `sqlparser::dialect::Dialect::supports_match_against` · sqlparser 0.62.0

```rust
fn supports_match_against(&self) -> bool
```

Source: `src/dialect/mod.rs:717`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support the `MATCH() AGAINST()` syntax?

<a id="op-151256ec1c071171622117ab"></a>
## supports_match_recognize

`function` · `sqlparser::dialect::Dialect::supports_match_recognize` · sqlparser 0.62.0

```rust
fn supports_match_recognize(&self) -> bool
```

Source: `src/dialect/mod.rs:429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the MATCH_RECOGNIZE operation.

<a id="op-ad49c717a8649f80828dc59a"></a>
## supports_multiline_comment_hints

`function` · `sqlparser::dialect::Dialect::supports_multiline_comment_hints` · sqlparser 0.62.0

```rust
fn supports_multiline_comment_hints(&self) -> bool
```

Source: `src/dialect/mod.rs:1159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports optimizer hints in multiline comments
e.g. `/*!50110 KEY_BLOCK_SIZE = 1024*/`

<a id="op-3e9943cbdaf5c36a3a3f1c6c"></a>
## supports_named_fn_args_with_assignment_operator

`function` · `sqlparser::dialect::Dialect::supports_named_fn_args_with_assignment_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_assignment_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports named arguments of the form `FUN(a := '1', b := '2')`.

<a id="op-a7b7a12300cb3129efe47d56"></a>
## supports_named_fn_args_with_colon_operator

`function` · `sqlparser::dialect::Dialect::supports_named_fn_args_with_colon_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_colon_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports named arguments of the form `FUN(a : '1', b : '2')`.

<a id="op-1c5bdeaa57adfcce6e040b78"></a>
## supports_named_fn_args_with_eq_operator

`function` · `sqlparser::dialect::Dialect::supports_named_fn_args_with_eq_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_eq_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports named arguments of the form `FUN(a = '1', b = '2')`.

<a id="op-47c17ad203d3c84822d5d8d5"></a>
## supports_named_fn_args_with_expr_name

`function` · `sqlparser::dialect::Dialect::supports_named_fn_args_with_expr_name` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_expr_name(&self) -> bool
```

Source: `src/dialect/mod.rs:472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if dialect supports argument name as arbitrary expression.
e.g. `FUN(LOWER('a'):'1',  b:'2')`
Such function arguments are represented in the AST by the `FunctionArg::ExprNamed` variant,
otherwise use the `FunctionArg::Named` variant (compatible reason).

<a id="op-5cfd82691cb102c205472e55"></a>
## supports_named_fn_args_with_rarrow_operator

`function` · `sqlparser::dialect::Dialect::supports_named_fn_args_with_rarrow_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_rarrow_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:464`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports named arguments of the form `FUN(a => '1', b => '2')`.

<a id="op-2ea82bc2d6782808fd3b9523"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::Dialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Source: `src/dialect/mod.rs:1153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports nested comments
e.g. `/* /* nested */ */`

<a id="op-978fed88ec129aa09d88620d"></a>
## supports_notnull_operator

`function` · `sqlparser::dialect::Dialect::supports_notnull_operator` · sqlparser 0.62.0

```rust
fn supports_notnull_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:1457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `x NOTNULL`
operator expression.

<a id="op-3ab87503ef2c23e30496b294"></a>
## supports_numeric_literal_underscores

`function` · `sqlparser::dialect::Dialect::supports_numeric_literal_underscores` · sqlparser 0.62.0

```rust
fn supports_numeric_literal_underscores(&self) -> bool
```

Source: `src/dialect/mod.rs:483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports numbers containing underscores, e.g. `10_000_000`

<a id="op-c872e967d5f7f518757ae946"></a>
## supports_numeric_prefix

`function` · `sqlparser::dialect::Dialect::supports_numeric_prefix` · sqlparser 0.62.0

```rust
fn supports_numeric_prefix(&self) -> bool
```

Source: `src/dialect/mod.rs:478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports identifiers starting with a numeric
prefix such as tables named `59901_user_login`

<a id="op-a754404d5327fbaa6a18db11"></a>
## supports_object_name_double_dot_notation

`function` · `sqlparser::dialect::Dialect::supports_object_name_double_dot_notation` · sqlparser 0.62.0

```rust
fn supports_object_name_double_dot_notation(&self) -> bool
```

Source: `src/dialect/mod.rs:636`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports double dot notation for object names

Example
```sql
SELECT * FROM db_name..table_name
```

<a id="op-99fe1a66e37f8505d98657eb"></a>
## supports_optimize_table

`function` · `sqlparser::dialect::Dialect::supports_optimize_table` · sqlparser 0.62.0

```rust
fn supports_optimize_table(&self) -> bool
```

Source: `src/dialect/mod.rs:1610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `OPTIMIZE TABLE` statement.

Example:
```sql
OPTIMIZE TABLE table_name;
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

<a id="op-7d24f7b46ddd2d126597e1ab"></a>
## supports_order_by_all

`function` · `sqlparser::dialect::Dialect::supports_order_by_all` · sqlparser 0.62.0

```rust
fn supports_order_by_all(&self) -> bool
```

Source: `src/dialect/mod.rs:1403`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `ORDER BY ALL`.
`ALL` which means all columns of the SELECT clause.

For example: ```SELECT * FROM addresses ORDER BY ALL;```.

<a id="op-27e1ff09349d0cb0683786aa"></a>
## supports_outer_join_operator

`function` · `sqlparser::dialect::Dialect::supports_outer_join_operator` · sqlparser 0.62.0

```rust
fn supports_outer_join_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the `(+)` syntax for OUTER JOIN.

<a id="op-bc758721109261d95ff38ddb"></a>
## supports_parens_around_table_factor

`function` · `sqlparser::dialect::Dialect::supports_parens_around_table_factor` · sqlparser 0.62.0

```rust
fn supports_parens_around_table_factor(&self) -> bool
```

Source: `src/dialect/mod.rs:1072`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports extra parentheses around
lone table names or derived tables in the `FROM` clause.

Example:
```sql
SELECT * FROM (mytable);
SELECT * FROM ((SELECT 1));
SELECT * FROM (mytable) AS alias;
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/constructs/from)

<a id="op-7b95ac2dc0401999c99624eb"></a>
## supports_parenthesized_set_variables

`function` · `sqlparser::dialect::Dialect::supports_parenthesized_set_variables` · sqlparser 0.62.0

```rust
fn supports_parenthesized_set_variables(&self) -> bool
```

Source: `src/dialect/mod.rs:530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports multiple variable assignment
using parentheses in a `SET` variable declaration.

```sql
SET (variable[, ...]) = (expression[, ...]);
```

<a id="op-c092e29689ed953a43d61e97"></a>
## supports_partiql

`function` · `sqlparser::dialect::Dialect::supports_partiql` · sqlparser 0.62.0

```rust
fn supports_partiql(&self) -> bool
```

Source: `src/dialect/mod.rs:1229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports PartiQL for querying semi-structured data
<https://partiql.org/index.html>

<a id="op-37370463652fa14de99cc2d0"></a>
## supports_partition_by_after_order_by

`function` · `sqlparser::dialect::Dialect::supports_partition_by_after_order_by` · sqlparser 0.62.0

```rust
fn supports_partition_by_after_order_by(&self) -> bool
```

Source: `src/dialect/mod.rs:357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `PARTITION BY` appearing after `ORDER BY`
in a `CREATE TABLE` statement (in addition to the standard placement before `ORDER BY`).

ClickHouse DDL uses this ordering:
<https://clickhouse.com/docs/en/sql-reference/statements/create/table#partition-by>

<a id="op-1cf6969a26cb524c7717a7f4"></a>
## supports_pipe_operator

`function` · `sqlparser::dialect::Dialect::supports_pipe_operator` · sqlparser 0.62.0

```rust
fn supports_pipe_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:707`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return true if the dialect supports pipe operator.

Example:
```sql
SELECT *
FROM table
|> limit 1
```

See <https://cloud.google.com/bigquery/docs/pipe-syntax-guide#basic_syntax>

<a id="op-8523f3a4205d4cc68ce2add3"></a>
## supports_prewhere

`function` · `sqlparser::dialect::Dialect::supports_prewhere` · sqlparser 0.62.0

```rust
fn supports_prewhere(&self) -> bool
```

Source: `src/dialect/mod.rs:1647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `PREWHERE` clause
in `SELECT` statements.

Example:
```sql
SELECT * FROM table PREWHERE col > 0 WHERE col < 100;
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/prewhere)

<a id="op-1cd0622eceb898315b0d7f06"></a>
## supports_projection_trailing_commas

`function` · `sqlparser::dialect::Dialect::supports_projection_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_projection_trailing_commas(&self) -> bool
```

Source: `src/dialect/mod.rs:613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support trailing commas in the projection list?

<a id="op-b25fa714dfdbb1d9f21c4ec6"></a>
## supports_quote_delimited_string

`function` · `sqlparser::dialect::Dialect::supports_quote_delimited_string` · sqlparser 0.62.0

```rust
fn supports_quote_delimited_string(&self) -> bool
```

Source: `src/dialect/mod.rs:1524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Support quote delimited string literals, e.g. `Q'{...}'`

[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/19/sqlrf/Literals.html#GUID-1824CBAA-6E16-4921-B2A6-112FB02248DA)

<a id="op-a79ac93beafe41bced6a834c"></a>
## supports_select_exclude

`function` · `sqlparser::dialect::Dialect::supports_select_exclude` · sqlparser 0.62.0

```rust
fn supports_select_exclude(&self) -> bool
```

Source: `src/dialect/mod.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports an exclude option
as the last item in the projection section, not necessarily
after a wildcard. For example:
`SELECT *, c1, c2 EXCLUDE c3 FROM tbl`

[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_EXCLUDE_list.html)

<a id="op-332770deb61582b417f193ad"></a>
## supports_select_expr_star

`function` · `sqlparser::dialect::Dialect::supports_select_expr_star` · sqlparser 0.62.0

```rust
fn supports_select_expr_star(&self) -> bool
```

Source: `src/dialect/mod.rs:667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return true if the dialect supports wildcard expansion on
arbitrary expressions in projections.

Example:
```sql
SELECT STRUCT<STRING>('foo').* FROM T
```

<a id="op-f1beac311433bbd3be699d11"></a>
## supports_select_format

`function` · `sqlparser::dialect::Dialect::supports_select_format` · sqlparser 0.62.0

```rust
fn supports_select_format(&self) -> bool
```

Source: `src/dialect/mod.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `FORMAT` clause in `SELECT` statements.

Example:
```sql
SELECT * FROM table FORMAT JSON;
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/format)

<a id="op-d403e0cd59bfadf867c0e3dd"></a>
## supports_select_item_multi_column_alias

`function` · `sqlparser::dialect::Dialect::supports_select_item_multi_column_alias` · sqlparser 0.62.0

```rust
fn supports_select_item_multi_column_alias(&self) -> bool
```

Source: `src/dialect/mod.rs:1737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports parenthesized multi-column
aliases in SELECT items. For example:
```sql
SELECT stack(2, 'a', 'b') AS (col1, col2)
```

[Spark SQL](https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select.html)

<a id="op-d3512a09ea0cca68139b8f64"></a>
## supports_select_modifiers

`function` · `sqlparser::dialect::Dialect::supports_select_modifiers` · sqlparser 0.62.0

```rust
fn supports_select_modifiers(&self) -> bool
```

Source: `src/dialect/mod.rs:757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports MySQL-specific SELECT modifiers
like `HIGH_PRIORITY`, `STRAIGHT_JOIN`, `SQL_SMALL_RESULT`, etc.

For example:
```sql
SELECT HIGH_PRIORITY STRAIGHT_JOIN SQL_SMALL_RESULT * FROM t1 JOIN t2 ON ...
```

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/select.html)

<a id="op-2873b9b6aae9908efdb669dd"></a>
## supports_select_wildcard_except

`function` · `sqlparser::dialect::Dialect::supports_select_wildcard_except` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_except(&self) -> bool
```

Source: `src/dialect/mod.rs:561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports an `EXCEPT` clause following a
wildcard in a select list.

For example
```sql
SELECT * EXCEPT order_id FROM orders;
```

<a id="op-001e75e065e1e405173623fd"></a>
## supports_select_wildcard_exclude

`function` · `sqlparser::dialect::Dialect::supports_select_wildcard_exclude` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_exclude(&self) -> bool
```

Source: `src/dialect/mod.rs:727`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports an exclude option
following a wildcard in the projection section. For example:
`SELECT * EXCLUDE col1 FROM tbl`.

[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_EXCLUDE_list.html)
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/select)

<a id="op-0ec869dc413268fb6070264e"></a>
## supports_select_wildcard_ilike

`function` · `sqlparser::dialect::Dialect::supports_select_wildcard_ilike` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_ilike(&self) -> bool
```

Source: `src/dialect/mod.rs:1574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `ILIKE` option in a
`SELECT *` wildcard expression.

Example:
```sql
SELECT * ILIKE '%pattern%' FROM table;
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/select#parameters)

<a id="op-cea02e94faa1a2636dbc7e40"></a>
## supports_select_wildcard_rename

`function` · `sqlparser::dialect::Dialect::supports_select_wildcard_rename` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_rename(&self) -> bool
```

Source: `src/dialect/mod.rs:1587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `RENAME` option in a
`SELECT *` wildcard expression.

Example:
```sql
SELECT * RENAME col1 AS col1_alias FROM table;
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/select#parameters)

<a id="op-64ff84b77d2a1d44dad36d76"></a>
## supports_select_wildcard_replace

`function` · `sqlparser::dialect::Dialect::supports_select_wildcard_replace` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_replace(&self) -> bool
```

Source: `src/dialect/mod.rs:1561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `REPLACE` option in a
`SELECT *` wildcard expression.

Example:
```sql
SELECT * REPLACE (col1 AS col1_alias) FROM table;
```

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_replace)
[ClickHouse](https://clickhouse.com/docs/sql-reference/statements/select#replace)
[DuckDB](https://duckdb.org/docs/sql/query_syntax/select#replace-clause)
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/select#parameters)

<a id="op-84f0734e72cec3de72214854"></a>
## supports_select_wildcard_with_alias

`function` · `sqlparser::dialect::Dialect::supports_select_wildcard_with_alias` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_with_alias(&self) -> bool
```

Source: `src/dialect/mod.rs:1598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports aliasing a wildcard select item.

Example:
```sql
SELECT t.* alias FROM t
SELECT t.* AS alias FROM t
```

<a id="op-9d886787a5df4ed042a9cce3"></a>
## supports_semantic_view_table_factor

`function` · `sqlparser::dialect::Dialect::supports_semantic_view_table_factor` · sqlparser 0.62.0

```rust
fn supports_semantic_view_table_factor(&self) -> bool
```

Source: `src/dialect/mod.rs:1517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `SEMANTIC_VIEW()` table functions.

```sql
SELECT * FROM SEMANTIC_VIEW(
    model_name
    DIMENSIONS customer.name, customer.region
    METRICS orders.revenue, orders.count
    WHERE customer.active = true
)
```

<a id="op-79e714bf22a7232a142fb42f"></a>
## supports_set_names

`function` · `sqlparser::dialect::Dialect::supports_set_names` · sqlparser 0.62.0

```rust
fn supports_set_names(&self) -> bool
```

Source: `src/dialect/mod.rs:1413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `SET NAMES <charset_name> [COLLATE <collation_name>]`.

- [MySQL](https://dev.mysql.com/doc/refman/8.4/en/set-names.html)
- [PostgreSQL](https://www.postgresql.org/docs/17/sql-set.html)

Note: Postgres doesn't support the `COLLATE` clause, but we permissively parse it anyway.

<a id="op-182fb8e54cdbc0c4166079e7"></a>
## supports_set_stmt_without_operator

`function` · `sqlparser::dialect::Dialect::supports_set_stmt_without_operator` · sqlparser 0.62.0

```rust
fn supports_set_stmt_without_operator(&self) -> bool
```

Source: `src/dialect/mod.rs:1321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports `SET` statements without an explicit
assignment operator such as `=`. For example: `SET SHOWPLAN_XML ON`.

<a id="op-eb58440cddacb7d011e6bfdb"></a>
## supports_settings

`function` · `sqlparser::dialect::Dialect::supports_settings` · sqlparser 0.62.0

```rust
fn supports_settings(&self) -> bool
```

Source: `src/dialect/mod.rs:1697`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `SETTINGS` clause.

Example:
```sql
SELECT * FROM table SETTINGS max_threads = 4;
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select#settings-in-select-query)

<a id="op-292daaf51b57e68dc33e0d3c"></a>
## supports_show_like_before_in

`function` · `sqlparser::dialect::Dialect::supports_show_like_before_in` · sqlparser 0.62.0

```rust
fn supports_show_like_before_in(&self) -> bool
```

Source: `src/dialect/mod.rs:1213`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `LIKE 'pattern'` option in
a `SHOW` statement before the `IN` option

<a id="op-05ed512844d4af9ddbd91094"></a>
## supports_space_separated_column_options

`function` · `sqlparser::dialect::Dialect::supports_space_separated_column_options` · sqlparser 0.62.0

```rust
fn supports_space_separated_column_options(&self) -> bool
```

Source: `src/dialect/mod.rs:1424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports space-separated column options
in a `CREATE TABLE` statement. For example:
```sql
CREATE TABLE tbl (
    col INT NOT NULL DEFAULT 0
);
```

<a id="op-5cd37b553f7f63cc2060b8fa"></a>
## supports_start_transaction_modifier

`function` · `sqlparser::dialect::Dialect::supports_start_transaction_modifier` · sqlparser 0.62.0

```rust
fn supports_start_transaction_modifier(&self) -> bool
```

Source: `src/dialect/mod.rs:439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `BEGIN {DEFERRED | IMMEDIATE | EXCLUSIVE | TRY | CATCH} [TRANSACTION]` statements

<a id="op-54b882fec87957dda7485548"></a>
## supports_string_escape_constant

`function` · `sqlparser::dialect::Dialect::supports_string_escape_constant` · sqlparser 0.62.0

```rust
fn supports_string_escape_constant(&self) -> bool
```

Source: `src/dialect/mod.rs:1366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the E'...' syntax for string literals

Postgres: <https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-ESCAPE>

<a id="op-51f2a32c315f7fb643833af4"></a>
## supports_string_literal_backslash_escape

`function` · `sqlparser::dialect::Dialect::supports_string_literal_backslash_escape` · sqlparser 0.62.0

```rust
fn supports_string_literal_backslash_escape(&self) -> bool
```

Source: `src/dialect/mod.rs:282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine if the dialect supports escaping characters via '\' in string literals.

Some dialects like BigQuery and Snowflake support this while others like
Postgres do not. Such that the following is accepted by the former but
rejected by the latter.
```sql
SELECT 'ab\'cd';
```

Conversely, such dialects reject the following statement which
otherwise would be valid in the other dialects.
```sql
SELECT '\';
```

<a id="op-bac98a44dd4f4dc478880f98"></a>
## supports_string_literal_concatenation

`function` · `sqlparser::dialect::Dialect::supports_string_literal_concatenation` · sqlparser 0.62.0

```rust
fn supports_string_literal_concatenation(&self) -> bool
```

Source: `src/dialect/mod.rs:595`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports concatenating of string literal
Example: `SELECT 'Hello ' "world" => SELECT 'Hello world'`

<a id="op-c2b7ddd74addcdb11a3abc44"></a>
## supports_string_literal_concatenation_with_newline

`function` · `sqlparser::dialect::Dialect::supports_string_literal_concatenation_with_newline` · sqlparser 0.62.0

```rust
fn supports_string_literal_concatenation_with_newline(&self) -> bool
```

Source: `src/dialect/mod.rs:608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports concatenating string literals with a newline.
For example, the following statement would return `true`:
```sql
SELECT 'abc' in (
  'a'
  'b'
  'c'
);
```

<a id="op-065b1027b15df9843dee77d2"></a>
## supports_struct_literal

`function` · `sqlparser::dialect::Dialect::supports_struct_literal` · sqlparser 0.62.0

```rust
fn supports_struct_literal(&self) -> bool
```

Source: `src/dialect/mod.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return true if the dialect supports the STRUCT literal

Example
```sql
SELECT STRUCT(1 as one, 'foo' as foo, false)
```

<a id="op-07fc9d984c6b7ffcd4da99d9"></a>
## supports_subquery_as_function_arg

`function` · `sqlparser::dialect::Dialect::supports_subquery_as_function_arg` · sqlparser 0.62.0

```rust
fn supports_subquery_as_function_arg(&self) -> bool
```

Source: `src/dialect/mod.rs:1031`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports a subquery passed to a function
as the only argument without enclosing parentheses.

Example:
```sql
SELECT FLATTEN(SELECT * FROM tbl);
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/functions/flatten)

<a id="op-b30b944f5cb31ffb96d54634"></a>
## supports_table_hints

`function` · `sqlparser::dialect::Dialect::supports_table_hints` · sqlparser 0.62.0

```rust
fn supports_table_hints(&self) -> bool
```

Source: `src/dialect/mod.rs:1371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports the table hints in the `FROM` clause.

<a id="op-019c7877a6ad11d74e70c53d"></a>
## supports_table_sample_before_alias

`function` · `sqlparser::dialect::Dialect::supports_table_sample_before_alias` · sqlparser 0.62.0

```rust
fn supports_table_sample_before_alias(&self) -> bool
```

Source: `src/dialect/mod.rs:1286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `TABLESAMPLE` option
before the table alias option. For example:

Table sample before alias: `SELECT * FROM tbl AS t TABLESAMPLE (10)`
Table sample after alias: `SELECT * FROM tbl TABLESAMPLE (10) AS t`

<https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_7_6_table_reference>

<a id="op-67568f4b3d26af2f89ca8fb0"></a>
## supports_table_versioning

`function` · `sqlparser::dialect::Dialect::supports_table_versioning` · sqlparser 0.62.0

```rust
fn supports_table_versioning(&self) -> bool
```

Source: `src/dialect/mod.rs:1359`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports querying historical table data
by specifying which version of the data to query.

<a id="op-f40cb45bb9e81963fab1010e"></a>
## supports_top_before_distinct

`function` · `sqlparser::dialect::Dialect::supports_top_before_distinct` · sqlparser 0.62.0

```rust
fn supports_top_before_distinct(&self) -> bool
```

Source: `src/dialect/mod.rs:1201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect expects the `TOP` option
before the `ALL`/`DISTINCT` options in a `SELECT` statement.

<a id="op-9269f17d48a6f9d5e36059f0"></a>
## supports_trailing_commas

`function` · `sqlparser::dialect::Dialect::supports_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_trailing_commas(&self) -> bool
```

Source: `src/dialect/mod.rs:584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support trailing commas around the query?

<a id="op-a54d7d9c48350e975719ee23"></a>
## supports_triple_quoted_string

`function` · `sqlparser::dialect::Dialect::supports_triple_quoted_string` · sqlparser 0.62.0

```rust
fn supports_triple_quoted_string(&self) -> bool
```

Source: `src/dialect/mod.rs:573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports triple quoted string
e.g. `"""abc"""`

<a id="op-56079c882f6468388d2a6259"></a>
## supports_try_convert

`function` · `sqlparser::dialect::Dialect::supports_try_convert` · sqlparser 0.62.0

```rust
fn supports_try_convert(&self) -> bool
```

Source: `src/dialect/mod.rs:1175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `TRY_CONVERT` function

<a id="op-c51a1db26055bd3a6556621b"></a>
## supports_unicode_string_literal

`function` · `sqlparser::dialect::Dialect::supports_unicode_string_literal` · sqlparser 0.62.0

```rust
fn supports_unicode_string_literal(&self) -> bool
```

Source: `src/dialect/mod.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determine if the dialect supports string literals with `U&` prefix.
This is used to specify Unicode code points in string literals.
For example, in PostgreSQL, the following is a valid string literal:
```sql
SELECT U&'\0061\0062\0063';
```
This is equivalent to the string literal `'abc'`.
See
 - [Postgres docs](https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-UESCAPE)
 - [H2 docs](http://www.h2database.com/html/grammar.html#string)

<a id="op-e101dd5655d5d491c7f990b5"></a>
## supports_update_order_by

`function` · `sqlparser::dialect::Dialect::supports_update_order_by` · sqlparser 0.62.0

```rust
fn supports_update_order_by(&self) -> bool
```

Source: `src/dialect/mod.rs:550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `ORDER BY` in `UPDATE` statements.

```sql
UPDATE foo SET bar = false WHERE foo = true ORDER BY foo ASC;
```
See <https://dev.mysql.com/doc/refman/8.4/en/update.html>

<a id="op-476c0b9b78413fdf9ff30353"></a>
## supports_user_host_grantee

`function` · `sqlparser::dialect::Dialect::supports_user_host_grantee` · sqlparser 0.62.0

```rust
fn supports_user_host_grantee(&self) -> bool
```

Source: `src/dialect/mod.rs:712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Does the dialect support MySQL-style `'user'@'host'` grantee syntax?

<a id="op-9824b9f8e76bfa059aa1e7cc"></a>
## supports_values_as_table_factor

`function` · `sqlparser::dialect::Dialect::supports_values_as_table_factor` · sqlparser 0.62.0

```rust
fn supports_values_as_table_factor(&self) -> bool
```

Source: `src/dialect/mod.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports `VALUES` as a table factor
without requiring parentheses around the entire clause.

Example:
```sql
SELECT * FROM VALUES (1, 'a'), (2, 'b') AS t (col1, col2);
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/constructs/values)
[Databricks](https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-qry-select-values.html)

<a id="op-5ea20143add2361d93045491"></a>
## supports_window_clause_named_window_reference

`function` · `sqlparser::dialect::Dialect::supports_window_clause_named_window_reference` · sqlparser 0.62.0

```rust
fn supports_window_clause_named_window_reference(&self) -> bool
```

Source: `src/dialect/mod.rs:340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports referencing another named window
within a window clause declaration.

Example
```sql
SELECT * FROM mytable
WINDOW mynamed_window AS another_named_window
```

<a id="op-bb7887685712553768653152"></a>
## supports_window_function_null_treatment_arg

`function` · `sqlparser::dialect::Dialect::supports_window_function_null_treatment_arg` · sqlparser 0.62.0

```rust
fn supports_window_function_null_treatment_arg(&self) -> bool
```

Source: `src/dialect/mod.rs:499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialects supports specifying null treatment
as part of a window function's parameter list as opposed
to after the parameter list.

i.e The following syntax returns true
```sql
FIRST_VALUE(a IGNORE NULLS) OVER ()
```
while the following syntax returns false
```sql
FIRST_VALUE(a) IGNORE NULLS OVER ()
```

<a id="op-5f25db775d123581f98f072b"></a>
## supports_with_fill

`function` · `sqlparser::dialect::Dialect::supports_with_fill` · sqlparser 0.62.0

```rust
fn supports_with_fill(&self) -> bool
```

Source: `src/dialect/mod.rs:1660`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect supports the `WITH FILL` clause
in `ORDER BY` expressions.

Example:
```sql
SELECT * FROM table ORDER BY col WITH FILL FROM 1 TO 10 STEP 1;
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/order-by#order-by-expr-with-fill-modifier)

<a id="op-c9a82164bb63037c2a687db9"></a>
## supports_within_after_array_aggregation

`function` · `sqlparser::dialect::Dialect::supports_within_after_array_aggregation` · sqlparser 0.62.0

```rust
fn supports_within_after_array_aggregation(&self) -> bool
```

Source: `src/dialect/mod.rs:348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports `ARRAY_AGG() [WITHIN GROUP (ORDER BY)]` expressions.
Otherwise, the dialect should expect an `ORDER BY` without the `WITHIN GROUP` clause, e.g. [`ANSI`]

[`ANSI`]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#array-aggregate-function

<a id="op-4eb48549d2f4dbefdc0224d4"></a>
## supports_xml_expressions

`function` · `sqlparser::dialect::Dialect::supports_xml_expressions` · sqlparser 0.62.0

```rust
fn supports_xml_expressions(&self) -> bool
```

Source: `src/dialect/mod.rs:1748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if the dialect supports XML-related expressions
such as `xml '<foo/>'` typed strings, XML functions like
`XMLCONCAT`, `XMLELEMENT`, etc.

When this returns false, `xml` is treated as a regular identifier.

[PostgreSQL](https://www.postgresql.org/docs/current/functions-xml.html)
