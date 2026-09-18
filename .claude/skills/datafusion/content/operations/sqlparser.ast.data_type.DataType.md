# `sqlparser::ast::data_type::DataType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.DataType.json).

<a id="op-699c49167b5eef37ce7bd69a"></a>
## DataType

`enum` · `sqlparser::ast::data_type::DataType` · sqlparser 0.62.0

```rust
enum DataType
```

Source: `src/ast/data_type.rs:49`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL data types

<a id="op-0f556c05bfddf92ca691d23d"></a>
## AnyType

`variant` · `sqlparser::ast::data_type::DataType::AnyType` · sqlparser 0.62.0

```rust
AnyType
```

Source: `src/ast/data_type.rs:486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Any data type, used in BigQuery UDF definitions for templated parameters, see [BigQuery].

[BigQuery]: https://cloud.google.com/bigquery/docs/user-defined-functions#templated-sql-udf-parameters

<a id="op-e245083dbac524a33d8d5a4a"></a>
## Array

`variant` · `sqlparser::ast::data_type::DataType::Array` · sqlparser 0.62.0

```rust
Array
```

Source: `src/ast/data_type.rs:441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arrays.

<a id="op-9718a36aa29e977d1d6a9d52"></a>
## BigDecimal

`variant` · `sqlparser::ast::data_type::DataType::BigDecimal` · sqlparser 0.62.0

```rust
BigDecimal
```

Source: `src/ast/data_type.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This is alias for `BigNumeric` type used in BigQuery.

[BigDecimal]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#decimal_types

<a id="op-8399bb5cf44cd307ba160342"></a>
## BigInt

`variant` · `sqlparser::ast::data_type::DataType::BigInt` · sqlparser 0.62.0

```rust
BigInt
```

Source: `src/ast/data_type.rs:281`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Big integer with optional display width, e.g. BIGINT or BIGINT(20).

<a id="op-ba54fdccc74715613687acf9"></a>
## BigIntUnsigned

`variant` · `sqlparser::ast::data_type::DataType::BigIntUnsigned` · sqlparser 0.62.0

```rust
BigIntUnsigned
```

Source: `src/ast/data_type.rs:283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned big integer with optional display width, e.g. BIGINT UNSIGNED or BIGINT(20) UNSIGNED.

<a id="op-1958a82e806610d847eef16f"></a>
## BigNumeric

`variant` · `sqlparser::ast::data_type::DataType::BigNumeric` · sqlparser 0.62.0

```rust
BigNumeric
```

Source: `src/ast/data_type.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[BigNumeric] type used in BigQuery.

[BigNumeric]: https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#bignumeric_literals

<a id="op-2fcc02f374cfd71e92a737fc"></a>
## Binary

`variant` · `sqlparser::ast::data_type::DataType::Binary` · sqlparser 0.62.0

```rust
Binary
```

Source: `src/ast/data_type.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fixed-length binary type with optional length,
see [SQL Standard], [MS SQL Server].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#binary-string-type
[MS SQL Server]: https://learn.microsoft.com/pt-br/sql/t-sql/data-types/binary-and-varbinary-transact-sql?view=sql-server-ver16

<a id="op-bcc765d9bb5bb777f86f4550"></a>
## Bit

`variant` · `sqlparser::ast::data_type::DataType::Bit` · sqlparser 0.62.0

```rust
Bit
```

Source: `src/ast/data_type.rs:429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bit string, see [PostgreSQL], [MySQL], or [MSSQL].

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype-bit.html
[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/bit-type.html
[MSSQL]: https://learn.microsoft.com/en-us/sql/t-sql/data-types/bit-transact-sql?view=sql-server-ver16

<a id="op-48bed1b14452671184883f41"></a>
## BitVarying

`variant` · `sqlparser::ast::data_type::DataType::BitVarying` · sqlparser 0.62.0

```rust
BitVarying
```

Source: `src/ast/data_type.rs:433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`BIT VARYING(n)`: Variable-length bit string, see [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype-bit.html

<a id="op-644c2653d3c621abc2fabe96"></a>
## Blob

`variant` · `sqlparser::ast::data_type::DataType::Blob` · sqlparser 0.62.0

```rust
Blob
```

Source: `src/ast/data_type.rs:111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Large binary object with optional length,
see [SQL Standard], [Oracle].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#binary-large-object-string-type
[Oracle]: https://docs.oracle.com/javadb/10.8.3.0/ref/rrefblob.html

<a id="op-efd19b8e097eb90d4dcc98ba"></a>
## Bool

`variant` · `sqlparser::ast::data_type::DataType::Bool` · sqlparser 0.62.0

```rust
Bool
```

Source: `src/ast/data_type.rs:352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bool is an alias for Boolean, see [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html

<a id="op-6eb0ffe86d1d9405883e6768"></a>
## Boolean

`variant` · `sqlparser::ast::data_type::DataType::Boolean` · sqlparser 0.62.0

```rust
Boolean
```

Source: `src/ast/data_type.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Boolean type.

<a id="op-017bf3bedb75a1a53f84a93e"></a>
## Bytea

`variant` · `sqlparser::ast::data_type::DataType::Bytea` · sqlparser 0.62.0

```rust
Bytea
```

Source: `src/ast/data_type.rs:423`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bytea type, see [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype-bit.html

<a id="op-7832910a5f583f3f2a5129fe"></a>
## Bytes

`variant` · `sqlparser::ast::data_type::DataType::Bytes` · sqlparser 0.62.0

```rust
Bytes
```

Source: `src/ast/data_type.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variable-length binary data with optional length.

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#bytes_type

<a id="op-22cf876cf0c8b8e6a50d7e34"></a>
## Char

`variant` · `sqlparser::ast::data_type::DataType::Char` · sqlparser 0.62.0

```rust
Char
```

Source: `src/ast/data_type.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fixed-length char type, e.g. CHAR(10).

<a id="op-25019b29e35c91b065be415d"></a>
## CharLargeObject

`variant` · `sqlparser::ast::data_type::DataType::CharLargeObject` · sqlparser 0.62.0

```rust
CharLargeObject
```

Source: `src/ast/data_type.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Large character object with optional length,
e.g. CHAR LARGE OBJECT, CHAR LARGE OBJECT(1000), [SQL Standard].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-large-object-type

<a id="op-932edfdcda73c5b6f7a64ab1"></a>
## CharVarying

`variant` · `sqlparser::ast::data_type::DataType::CharVarying` · sqlparser 0.62.0

```rust
CharVarying
```

Source: `src/ast/data_type.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Char varying type, e.g. CHAR VARYING(10).

<a id="op-3214315b947ed314503d2cd5"></a>
## Character

`variant` · `sqlparser::ast::data_type::DataType::Character` · sqlparser 0.62.0

```rust
Character
```

Source: `src/ast/data_type.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fixed-length character type, e.g. CHARACTER(10).

<a id="op-fb70c12cfe275a9d6f84cb1d"></a>
## CharacterLargeObject

`variant` · `sqlparser::ast::data_type::DataType::CharacterLargeObject` · sqlparser 0.62.0

```rust
CharacterLargeObject
```

Source: `src/ast/data_type.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Large character object with optional length,
e.g. CHARACTER LARGE OBJECT, CHARACTER LARGE OBJECT(1000), [SQL Standard].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-large-object-type

<a id="op-769003d5e77234ce54a392cf"></a>
## CharacterVarying

`variant` · `sqlparser::ast::data_type::DataType::CharacterVarying` · sqlparser 0.62.0

```rust
CharacterVarying
```

Source: `src/ast/data_type.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Character varying type, e.g. CHARACTER VARYING(10).

<a id="op-6ff9cd3c5588bcb474aa8bca"></a>
## Clob

`variant` · `sqlparser::ast::data_type::DataType::Clob` · sqlparser 0.62.0

```rust
Clob
```

Source: `src/ast/data_type.rs:93`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Large character object with optional length,
e.g. CLOB, CLOB(1000), [SQL Standard].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-large-object-type
[Oracle]: https://docs.oracle.com/javadb/10.10.1.2/ref/rrefclob.html

<a id="op-c8ee5a3ed8a6cdc6dae49e40"></a>
## Custom

`variant` · `sqlparser::ast::data_type::DataType::Custom` · sqlparser 0.62.0

```rust
Custom
```

Source: `src/ast/data_type.rs:439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Custom types.

<a id="op-06ce8fafd455d6f4d218e801"></a>
## Date

`variant` · `sqlparser::ast::data_type::DataType::Date` · sqlparser 0.62.0

```rust
Date
```

Source: `src/ast/data_type.rs:356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Date type.

<a id="op-d2daa5fcb0dafdfa2d64f059"></a>
## Date32

`variant` · `sqlparser::ast::data_type::DataType::Date32` · sqlparser 0.62.0

```rust
Date32
```

Source: `src/ast/data_type.rs:360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Date32 with the same range as Datetime64.

[1]: https://clickhouse.com/docs/en/sql-reference/data-types/date32

<a id="op-53ef2615f96c1ea7b29bdec1"></a>
## Datetime

`variant` · `sqlparser::ast::data_type::DataType::Datetime` · sqlparser 0.62.0

```rust
Datetime
```

Source: `src/ast/data_type.rs:368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Datetime with optional time precision, see [MySQL][1].

[1]: https://dev.mysql.com/doc/refman/8.0/en/datetime.html

<a id="op-0778f27eee06738e616aab66"></a>
## Datetime64

`variant` · `sqlparser::ast::data_type::DataType::Datetime64` · sqlparser 0.62.0

```rust
Datetime64
```

Source: `src/ast/data_type.rs:372`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Datetime with time precision and optional timezone, see [ClickHouse][1].

[1]: https://clickhouse.com/docs/en/sql-reference/data-types/datetime64

<a id="op-029fbe49b3d2dc70f70d51c3"></a>
## Dec

`variant` · `sqlparser::ast::data_type::DataType::Dec` · sqlparser 0.62.0

```rust
Dec
```

Source: `src/ast/data_type.rs:152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dec type with optional precision and scale, e.g. DEC(10,2), [SQL Standard][1].

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#exact-numeric-type

<a id="op-0a9855521ff3d62f38d5ad9d"></a>
## DecUnsigned

`variant` · `sqlparser::ast::data_type::DataType::DecUnsigned` · sqlparser 0.62.0

```rust
DecUnsigned
```

Source: `src/ast/data_type.rs:157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] unsigned decimal (DEC alias) with optional precision and scale, e.g. DEC UNSIGNED or DEC(10,2) UNSIGNED.
Note: Using UNSIGNED with DEC is deprecated in recent versions of MySQL.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/numeric-type-syntax.html

<a id="op-26f257985540fe2a03224ae7"></a>
## Decimal

`variant` · `sqlparser::ast::data_type::DataType::Decimal` · sqlparser 0.62.0

```rust
Decimal
```

Source: `src/ast/data_type.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Decimal type with optional precision and scale, e.g. DECIMAL(10,2), [SQL Standard][1].

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#exact-numeric-type

<a id="op-e89d48710bbc27163ba9a9e7"></a>
## DecimalUnsigned

`variant` · `sqlparser::ast::data_type::DataType::DecimalUnsigned` · sqlparser 0.62.0

```rust
DecimalUnsigned
```

Source: `src/ast/data_type.rs:140`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] unsigned decimal with optional precision and scale, e.g. DECIMAL UNSIGNED or DECIMAL(10,2) UNSIGNED.
Note: Using UNSIGNED with DECIMAL is deprecated in recent versions of MySQL.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/numeric-type-syntax.html

<a id="op-1f0da2197862dd0da41e9979"></a>
## Double

`variant` · `sqlparser::ast::data_type::DataType::Double` · sqlparser 0.62.0

```rust
Double
```

Source: `src/ast/data_type.rs:333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double

<a id="op-9364ed61d58300d7ab034ed8"></a>
## DoublePrecision

`variant` · `sqlparser::ast::data_type::DataType::DoublePrecision` · sqlparser 0.62.0

```rust
DoublePrecision
```

Source: `src/ast/data_type.rs:343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double Precision, see [SQL Standard], [PostgreSQL].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#approximate-numeric-type
[PostgreSQL]: https://www.postgresql.org/docs/current/datatype-numeric.html

<a id="op-bc263976155eb63c5bb40e0b"></a>
## DoublePrecisionUnsigned

`variant` · `sqlparser::ast::data_type::DataType::DoublePrecisionUnsigned` · sqlparser 0.62.0

```rust
DoublePrecisionUnsigned
```

Source: `src/ast/data_type.rs:348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] unsigned double precision, e.g. DOUBLE PRECISION UNSIGNED.
Note: Using UNSIGNED with DOUBLE PRECISION is deprecated in recent versions of MySQL.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/numeric-type-syntax.html

<a id="op-c79bf34e608e0e74177a865a"></a>
## DoubleUnsigned

`variant` · `sqlparser::ast::data_type::DataType::DoubleUnsigned` · sqlparser 0.62.0

```rust
DoubleUnsigned
```

Source: `src/ast/data_type.rs:338`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] unsigned double precision with optional precision, e.g. DOUBLE UNSIGNED or DOUBLE(10,2) UNSIGNED.
Note: Using UNSIGNED with DOUBLE is deprecated in recent versions of MySQL.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/numeric-type-syntax.html

<a id="op-a9e057ba477323d201241562"></a>
## Enum

`variant` · `sqlparser::ast::data_type::DataType::Enum` · sqlparser 0.62.0

```rust
Enum
```

Source: `src/ast/data_type.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enum type.

<a id="op-f8c6ea059898466b7bc47c4d"></a>
## FixedString

`variant` · `sqlparser::ast::data_type::DataType::FixedString` · sqlparser 0.62.0

```rust
FixedString
```

Source: `src/ast/data_type.rs:419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A fixed-length string e.g [ClickHouse][1].

[1]: https://clickhouse.com/docs/en/sql-reference/data-types/fixedstring

<a id="op-7b426e3f14bc535a5899393b"></a>
## Float

`variant` · `sqlparser::ast::data_type::DataType::Float` · sqlparser 0.62.0

```rust
Float
```

Source: `src/ast/data_type.rs:159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Floating point with optional precision and scale, e.g. FLOAT, FLOAT(8), or FLOAT(8,2).

<a id="op-daece432fdcfad61be0d62ec"></a>
## Float32

`variant` · `sqlparser::ast::data_type::DataType::Float32` · sqlparser 0.62.0

```rust
Float32
```

Source: `src/ast/data_type.rs:315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Floating point in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/float

<a id="op-4fc5aed35989dfb9f7c849da"></a>
## Float4

`variant` · `sqlparser::ast::data_type::DataType::Float4` · sqlparser 0.62.0

```rust
Float4
```

Source: `src/ast/data_type.rs:311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Float4 is an alias for Real in [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html

<a id="op-bae4fe0db873ba8ddefb7bdb"></a>
## Float64

`variant` · `sqlparser::ast::data_type::DataType::Float64` · sqlparser 0.62.0

```rust
Float64
```

Source: `src/ast/data_type.rs:320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Floating point in [BigQuery].

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#floating_point_types
[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/float

<a id="op-34d2530b020f7183e2413af1"></a>
## Float8

`variant` · `sqlparser::ast::data_type::DataType::Float8` · sqlparser 0.62.0

```rust
Float8
```

Source: `src/ast/data_type.rs:331`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Float8 is an alias for Double in [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html

<a id="op-46214d3bfa43ffc0443c9bde"></a>
## FloatUnsigned

`variant` · `sqlparser::ast::data_type::DataType::FloatUnsigned` · sqlparser 0.62.0

```rust
FloatUnsigned
```

Source: `src/ast/data_type.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] unsigned floating point with optional precision and scale, e.g.
FLOAT UNSIGNED, FLOAT(10) UNSIGNED or FLOAT(10,2) UNSIGNED.
Note: Using UNSIGNED with FLOAT is deprecated in recent versions of MySQL.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/numeric-type-syntax.html

<a id="op-95abdb8c0b1e06887399e179"></a>
## GeometricType

`variant` · `sqlparser::ast::data_type::DataType::GeometricType` · sqlparser 0.62.0

```rust
GeometricType
```

Source: `src/ast/data_type.rs:490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Geometric type, see [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/9.5/functions-geometry.html

<a id="op-0129d9d895c314dc1192bbec"></a>
## HugeInt

`variant` · `sqlparser::ast::data_type::DataType::HugeInt` · sqlparser 0.62.0

```rust
HugeInt
```

Source: `src/ast/data_type.rs:247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

128-bit integer type, e.g. HUGEINT.

<a id="op-0f9607215407f803d870ec72"></a>
## Int

`variant` · `sqlparser::ast::data_type::DataType::Int` · sqlparser 0.62.0

```rust
Int
```

Source: `src/ast/data_type.rs:199`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Int with optional display width, e.g. INT or INT(11).

<a id="op-695ac4d7fad76b23e623864b"></a>
## Int128

`variant` · `sqlparser::ast::data_type::DataType::Int128` · sqlparser 0.62.0

```rust
Int128
```

Source: `src/ast/data_type.rs:232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer type in [ClickHouse].
Note: Int128 means 128 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-a529af20c530a3804be68d14"></a>
## Int16

`variant` · `sqlparser::ast::data_type::DataType::Int16` · sqlparser 0.62.0

```rust
Int16
```

Source: `src/ast/data_type.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer type in [ClickHouse].
Note: Int16 means 16 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-91a09304ffc4210d52e129ed"></a>
## Int2

`variant` · `sqlparser::ast::data_type::DataType::Int2` · sqlparser 0.62.0

```rust
Int2
```

Source: `src/ast/data_type.rs:178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Int2 is an alias for SmallInt in [PostgreSQL].
Note: Int2 means 2 bytes in PostgreSQL (not 2 bits).
Int2 with optional display width, e.g. INT2 or INT2(5).

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html

<a id="op-7ce6f1f66f7930bb8891cbd1"></a>
## Int256

`variant` · `sqlparser::ast::data_type::DataType::Int256` · sqlparser 0.62.0

```rust
Int256
```

Source: `src/ast/data_type.rs:237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer type in [ClickHouse].
Note: Int256 means 256 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-f279085bc8af1fbf377544e4"></a>
## Int2Unsigned

`variant` · `sqlparser::ast::data_type::DataType::Int2Unsigned` · sqlparser 0.62.0

```rust
Int2Unsigned
```

Source: `src/ast/data_type.rs:180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned Int2 with optional display width, e.g. INT2 UNSIGNED or INT2(5) UNSIGNED.

<a id="op-ed6801e83b5d5becf7eba272"></a>
## Int32

`variant` · `sqlparser::ast::data_type::DataType::Int32` · sqlparser 0.62.0

```rust
Int32
```

Source: `src/ast/data_type.rs:222`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer type in [ClickHouse].
Note: Int32 means 32 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-6efb3795f9d75662e08fb362"></a>
## Int4

`variant` · `sqlparser::ast::data_type::DataType::Int4` · sqlparser 0.62.0

```rust
Int4
```

Source: `src/ast/data_type.rs:205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Int4 is an alias for Integer in [PostgreSQL].
Note: Int4 means 4 bytes in PostgreSQL (not 4 bits).
Int4 with optional display width, e.g. Int4 or Int4(11).

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html

<a id="op-77a074cb2cc3954766c38bfa"></a>
## Int4Unsigned

`variant` · `sqlparser::ast::data_type::DataType::Int4Unsigned` · sqlparser 0.62.0

```rust
Int4Unsigned
```

Source: `src/ast/data_type.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned int4 with optional display width, e.g. INT4 UNSIGNED or INT4(11) UNSIGNED.

<a id="op-d8cb321ae35b69f876b5d3af"></a>
## Int64

`variant` · `sqlparser::ast::data_type::DataType::Int64` · sqlparser 0.62.0

```rust
Int64
```

Source: `src/ast/data_type.rs:227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer type in [BigQuery], [ClickHouse].

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#integer_types
[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-086f7e1353f6cf23e50c343e"></a>
## Int8

`variant` · `sqlparser::ast::data_type::DataType::Int8` · sqlparser 0.62.0

```rust
Int8
```

Source: `src/ast/data_type.rs:212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Int8 is an alias for BigInt in [PostgreSQL] and Integer type in [ClickHouse].
Int8 with optional display width, e.g. INT8 or INT8(11).
Note: Int8 means 8 bytes in [PostgreSQL], but 8 bits in [ClickHouse].

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html
[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-7358c6fb661c7ec3e61040d4"></a>
## Int8Unsigned

`variant` · `sqlparser::ast::data_type::DataType::Int8Unsigned` · sqlparser 0.62.0

```rust
Int8Unsigned
```

Source: `src/ast/data_type.rs:287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned Int8 with optional display width, e.g. INT8 UNSIGNED or INT8(11) UNSIGNED.

<a id="op-4bb1b7d0859ef1c68b509d0c"></a>
## IntUnsigned

`variant` · `sqlparser::ast::data_type::DataType::IntUnsigned` · sqlparser 0.62.0

```rust
IntUnsigned
```

Source: `src/ast/data_type.rs:241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned int with optional display width, e.g. INT UNSIGNED or INT(11) UNSIGNED.

<a id="op-e1b8d8cce4345716f590ed17"></a>
## Integer

`variant` · `sqlparser::ast::data_type::DataType::Integer` · sqlparser 0.62.0

```rust
Integer
```

Source: `src/ast/data_type.rs:239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer with optional display width, e.g. INTEGER or INTEGER(11).

<a id="op-eab7b873e79e409769b3f322"></a>
## IntegerUnsigned

`variant` · `sqlparser::ast::data_type::DataType::IntegerUnsigned` · sqlparser 0.62.0

```rust
IntegerUnsigned
```

Source: `src/ast/data_type.rs:245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer with optional display width, e.g. INTEGER UNSIGNED or INTEGER(11) UNSIGNED.

<a id="op-0e30775bca9a6b671256d870"></a>
## Interval

`variant` · `sqlparser::ast::data_type::DataType::Interval` · sqlparser 0.62.0

```rust
Interval
```

Source: `src/ast/data_type.rs:382`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Interval type.

<a id="op-792a07f7cbf7a05c357415af"></a>
## JSON

`variant` · `sqlparser::ast::data_type::DataType::JSON` · sqlparser 0.62.0

```rust
JSON
```

Source: `src/ast/data_type.rs:393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

JSON type.

<a id="op-1db287b3af7c44edfd3a0776"></a>
## JSONB

`variant` · `sqlparser::ast::data_type::DataType::JSONB` · sqlparser 0.62.0

```rust
JSONB
```

Source: `src/ast/data_type.rs:395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Binary JSON type.

<a id="op-f71dfc8dddd6cc2e32b610e5"></a>
## LongBlob

`variant` · `sqlparser::ast::data_type::DataType::LongBlob` · sqlparser 0.62.0

```rust
LongBlob
```

Source: `src/ast/data_type.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] blob with up to 2**32 bytes.

[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/blob.html

<a id="op-71d75a80d141f1ecb3f3b067"></a>
## LongText

`variant` · `sqlparser::ast::data_type::DataType::LongText` · sqlparser 0.62.0

```rust
LongText
```

Source: `src/ast/data_type.rs:413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] text with up to 2**32 bytes.

[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/blob.html

<a id="op-4b5a98023b858744723470ac"></a>
## LowCardinality

`variant` · `sqlparser::ast::data_type::DataType::LowCardinality` · sqlparser 0.62.0

```rust
LowCardinality
```

Source: `src/ast/data_type.rs:474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LowCardinality - changes the internal representation of other data types to be dictionary-encoded.

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/lowcardinality

<a id="op-4a2de0d4e6a56a1a8602723e"></a>
## Map

`variant` · `sqlparser::ast::data_type::DataType::Map` · sqlparser 0.62.0

```rust
Map
```

Source: `src/ast/data_type.rs:445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Map, see [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/map

<a id="op-8a356177726c727e984b96e1"></a>
## MediumBlob

`variant` · `sqlparser::ast::data_type::DataType::MediumBlob` · sqlparser 0.62.0

```rust
MediumBlob
```

Source: `src/ast/data_type.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] blob with up to 2**24 bytes.

[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/blob.html

<a id="op-38ab1eb8ca89aa741bb5b91f"></a>
## MediumInt

`variant` · `sqlparser::ast::data_type::DataType::MediumInt` · sqlparser 0.62.0

```rust
MediumInt
```

Source: `src/ast/data_type.rs:192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL medium integer ([1]) with optional display width,
e.g. MEDIUMINT or MEDIUMINT(5).

[1]: https://dev.mysql.com/doc/refman/8.0/en/integer-types.html

<a id="op-23c1fd32d1b173602fff1b80"></a>
## MediumIntUnsigned

`variant` · `sqlparser::ast::data_type::DataType::MediumIntUnsigned` · sqlparser 0.62.0

```rust
MediumIntUnsigned
```

Source: `src/ast/data_type.rs:197`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned medium integer ([1]) with optional display width,
e.g. MEDIUMINT UNSIGNED or MEDIUMINT(5) UNSIGNED.

[1]: https://dev.mysql.com/doc/refman/8.0/en/integer-types.html

<a id="op-7dfdb965fae90af8a144cd24"></a>
## MediumText

`variant` · `sqlparser::ast::data_type::DataType::MediumText` · sqlparser 0.62.0

```rust
MediumText
```

Source: `src/ast/data_type.rs:409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] text with up to 2**24 bytes.

[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/blob.html

<a id="op-9b1d522d74c8b728489bfc9c"></a>
## NamedTable

`variant` · `sqlparser::ast::data_type::DataType::NamedTable` · sqlparser 0.62.0

```rust
NamedTable
```

Source: `src/ast/data_type.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table type with a name, e.g. CREATE FUNCTION RETURNS @result TABLE(...).

[MsSQl]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql?view=sql-server-ver16#table

<a id="op-561888984cce7408678b70b1"></a>
## Nested

`variant` · `sqlparser::ast::data_type::DataType::Nested` · sqlparser 0.62.0

```rust
Nested
```

Source: `src/ast/data_type.rs:453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Nested type, see [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/nested-data-structures/nested

<a id="op-058a6e4406c38af37a838609"></a>
## Nullable

`variant` · `sqlparser::ast::data_type::DataType::Nullable` · sqlparser 0.62.0

```rust
Nullable
```

Source: `src/ast/data_type.rs:470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Nullable - special marker NULL represents in ClickHouse as a data type.

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/nullable

<a id="op-86dad2f109f61fa578bacdfd"></a>
## Numeric

`variant` · `sqlparser::ast::data_type::DataType::Numeric` · sqlparser 0.62.0

```rust
Numeric
```

Source: `src/ast/data_type.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Numeric type with optional precision and scale, e.g. NUMERIC(10,2), [SQL Standard][1].

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#exact-numeric-type

<a id="op-da7e64aa9a5e7af8c8cc7c39"></a>
## Nvarchar

`variant` · `sqlparser::ast::data_type::DataType::Nvarchar` · sqlparser 0.62.0

```rust
Nvarchar
```

Source: `src/ast/data_type.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variable-length character type, e.g. NVARCHAR(10).

<a id="op-63ed8138a6a1e77c02a89239"></a>
## Real

`variant` · `sqlparser::ast::data_type::DataType::Real` · sqlparser 0.62.0

```rust
Real
```

Source: `src/ast/data_type.rs:322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Floating point, e.g. REAL.

<a id="op-865555516a4b5da221c295ec"></a>
## RealUnsigned

`variant` · `sqlparser::ast::data_type::DataType::RealUnsigned` · sqlparser 0.62.0

```rust
RealUnsigned
```

Source: `src/ast/data_type.rs:327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] unsigned real, e.g. REAL UNSIGNED.
Note: Using UNSIGNED with REAL is deprecated in recent versions of MySQL.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/numeric-type-syntax.html

<a id="op-ee2ad80303b19db3ae621f11"></a>
## Regclass

`variant` · `sqlparser::ast::data_type::DataType::Regclass` · sqlparser 0.62.0

```rust
Regclass
```

Source: `src/ast/data_type.rs:399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Regclass used in [PostgreSQL] serial.

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html

<a id="op-955dce7cbb6fe609e1df95d9"></a>
## Set

`variant` · `sqlparser::ast::data_type::DataType::Set` · sqlparser 0.62.0

```rust
Set
```

Source: `src/ast/data_type.rs:457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set type.

<a id="op-cd3d149ae5338de31cddb2c2"></a>
## Signed

`variant` · `sqlparser::ast::data_type::DataType::Signed` · sqlparser 0.62.0

```rust
Signed
```

Source: `src/ast/data_type.rs:292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Signed integer as used in [MySQL CAST] target types, without optional `INTEGER` suffix,
e.g. `SIGNED`

[MySQL CAST]: https://dev.mysql.com/doc/refman/8.4/en/cast-functions.html

<a id="op-221e32c466b25ea7ac594f9d"></a>
## SignedInteger

`variant` · `sqlparser::ast::data_type::DataType::SignedInteger` · sqlparser 0.62.0

```rust
SignedInteger
```

Source: `src/ast/data_type.rs:297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Signed integer as used in [MySQL CAST] target types, with optional `INTEGER` suffix,
e.g. `SIGNED INTEGER`

[MySQL CAST]: https://dev.mysql.com/doc/refman/8.4/en/cast-functions.html

<a id="op-17dabf9dd4b819b30b6cbfd0"></a>
## SmallInt

`variant` · `sqlparser::ast::data_type::DataType::SmallInt` · sqlparser 0.62.0

```rust
SmallInt
```

Source: `src/ast/data_type.rs:182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Small integer with optional display width, e.g. SMALLINT or SMALLINT(5).

<a id="op-d1fb3359a66e5a399515385e"></a>
## SmallIntUnsigned

`variant` · `sqlparser::ast::data_type::DataType::SmallIntUnsigned` · sqlparser 0.62.0

```rust
SmallIntUnsigned
```

Source: `src/ast/data_type.rs:185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned small integer with optional display width,
e.g. SMALLINT UNSIGNED or SMALLINT(5) UNSIGNED.

<a id="op-7cbc7ba6eaed8ba985604f01"></a>
## String

`variant` · `sqlparser::ast::data_type::DataType::String` · sqlparser 0.62.0

```rust
String
```

Source: `src/ast/data_type.rs:415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

String with optional length.

<a id="op-ab29bb05d4bc8673642e9570"></a>
## Struct

`variant` · `sqlparser::ast::data_type::DataType::Struct` · sqlparser 0.62.0

```rust
Struct
```

Source: `src/ast/data_type.rs:462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Struct type, see [Hive], [BigQuery].

[Hive]: https://docs.cloudera.com/cdw-runtime/cloud/impala-sql-reference/topics/impala-struct.html
[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#struct_type

<a id="op-a4c29e07cf33657412d72961"></a>
## Table

`variant` · `sqlparser::ast::data_type::DataType::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/data_type.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table type in [PostgreSQL], e.g. CREATE FUNCTION RETURNS TABLE(...).

[PostgreSQL]: https://www.postgresql.org/docs/15/sql-createfunction.html
[MsSQL]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql?view=sql-server-ver16#c-create-a-multi-statement-table-valued-function

<a id="op-515765b3d3404f76d113af33"></a>
## Text

`variant` · `sqlparser::ast::data_type::DataType::Text` · sqlparser 0.62.0

```rust
Text
```

Source: `src/ast/data_type.rs:401`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Text type.

<a id="op-9b83e4788114268a5b9abc55"></a>
## Time

`variant` · `sqlparser::ast::data_type::DataType::Time` · sqlparser 0.62.0

```rust
Time
```

Source: `src/ast/data_type.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Time with optional time precision and time zone information, see [SQL Standard][1].

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#datetime-type

<a id="op-13fd82264cc2e0583972e558"></a>
## Timestamp

`variant` · `sqlparser::ast::data_type::DataType::Timestamp` · sqlparser 0.62.0

```rust
Timestamp
```

Source: `src/ast/data_type.rs:376`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Timestamp with optional time precision and time zone information, see [SQL Standard][1].

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#datetime-type

<a id="op-c44f8fb54a8edac6a27f2bf2"></a>
## TimestampNtz

`variant` · `sqlparser::ast::data_type::DataType::TimestampNtz` · sqlparser 0.62.0

```rust
TimestampNtz
```

Source: `src/ast/data_type.rs:380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Databricks timestamp without time zone. See [1].

[1]: https://docs.databricks.com/aws/en/sql/language-manual/data-types/timestamp-ntz-type

<a id="op-8f6c0ef458e64faed9328e34"></a>
## TinyBlob

`variant` · `sqlparser::ast::data_type::DataType::TinyBlob` · sqlparser 0.62.0

```rust
TinyBlob
```

Source: `src/ast/data_type.rs:115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] blob with up to 2**8 bytes.

[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/blob.html

<a id="op-b386148e0ab31b372c19dae5"></a>
## TinyInt

`variant` · `sqlparser::ast::data_type::DataType::TinyInt` · sqlparser 0.62.0

```rust
TinyInt
```

Source: `src/ast/data_type.rs:167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tiny integer with optional display width, e.g. TINYINT or TINYINT(3).

<a id="op-ce4a0110e61a87cec18ba9d4"></a>
## TinyIntUnsigned

`variant` · `sqlparser::ast::data_type::DataType::TinyIntUnsigned` · sqlparser 0.62.0

```rust
TinyIntUnsigned
```

Source: `src/ast/data_type.rs:170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned tiny integer with optional display width,
e.g. TINYINT UNSIGNED or TINYINT(3) UNSIGNED.

<a id="op-f7573bf3ad6e3aa9d2b2e8de"></a>
## TinyText

`variant` · `sqlparser::ast::data_type::DataType::TinyText` · sqlparser 0.62.0

```rust
TinyText
```

Source: `src/ast/data_type.rs:405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] text with up to 2**8 bytes.

[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/blob.html

<a id="op-1cf8db3c28355f7669cafc9e"></a>
## Trigger

`variant` · `sqlparser::ast::data_type::DataType::Trigger` · sqlparser 0.62.0

```rust
Trigger
```

Source: `src/ast/data_type.rs:482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trigger data type, returned by functions associated with triggers, see [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/current/plpgsql-trigger.html

<a id="op-7cdfa804637c62b5ca60b46d"></a>
## TsQuery

`variant` · `sqlparser::ast::data_type::DataType::TsQuery` · sqlparser 0.62.0

```rust
TsQuery
```

Source: `src/ast/data_type.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL text search query, see [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/17/datatype-textsearch.html

<a id="op-2efbd1761ab3b24c386c1d63"></a>
## TsVector

`variant` · `sqlparser::ast::data_type::DataType::TsVector` · sqlparser 0.62.0

```rust
TsVector
```

Source: `src/ast/data_type.rs:494`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL text search vectors, see [PostgreSQL].

[PostgreSQL]: https://www.postgresql.org/docs/17/datatype-textsearch.html

<a id="op-e6dab77b6b3d0966df2f27ab"></a>
## Tuple

`variant` · `sqlparser::ast::data_type::DataType::Tuple` · sqlparser 0.62.0

```rust
Tuple
```

Source: `src/ast/data_type.rs:449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tuple, see [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/tuple

<a id="op-b5735c2f4f0fa3aba5e9dd1a"></a>
## UBigInt

`variant` · `sqlparser::ast::data_type::DataType::UBigInt` · sqlparser 0.62.0

```rust
UBigInt
```

Source: `src/ast/data_type.rs:285`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned big integer, e.g. UBIGINT.

<a id="op-774cc4868d291bccac68f2db"></a>
## UHugeInt

`variant` · `sqlparser::ast::data_type::DataType::UHugeInt` · sqlparser 0.62.0

```rust
UHugeInt
```

Source: `src/ast/data_type.rs:249`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned 128-bit integer type, e.g. UHUGEINT.

<a id="op-81aba90fa71efb0b918a98eb"></a>
## UInt128

`variant` · `sqlparser::ast::data_type::DataType::UInt128` · sqlparser 0.62.0

```rust
UInt128
```

Source: `src/ast/data_type.rs:274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer type in [ClickHouse].
Note: UInt128 means 128 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-b40c8ab4264f30136d8c53c5"></a>
## UInt16

`variant` · `sqlparser::ast::data_type::DataType::UInt16` · sqlparser 0.62.0

```rust
UInt16
```

Source: `src/ast/data_type.rs:259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer type in [ClickHouse].
Note: UInt16 means 16 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-f179e52665ce0af751763f8c"></a>
## UInt256

`variant` · `sqlparser::ast::data_type::DataType::UInt256` · sqlparser 0.62.0

```rust
UInt256
```

Source: `src/ast/data_type.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer type in [ClickHouse].
Note: UInt256 means 256 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-edac1df81c86906591782b21"></a>
## UInt32

`variant` · `sqlparser::ast::data_type::DataType::UInt32` · sqlparser 0.62.0

```rust
UInt32
```

Source: `src/ast/data_type.rs:264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer type in [ClickHouse].
Note: UInt32 means 32 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-c9e5ecddecf4e37ba23dc5d8"></a>
## UInt64

`variant` · `sqlparser::ast::data_type::DataType::UInt64` · sqlparser 0.62.0

```rust
UInt64
```

Source: `src/ast/data_type.rs:269`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer type in [ClickHouse].
Note: UInt64 means 64 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-522820ada18cbd85ef418849"></a>
## UInt8

`variant` · `sqlparser::ast::data_type::DataType::UInt8` · sqlparser 0.62.0

```rust
UInt8
```

Source: `src/ast/data_type.rs:254`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer type in [ClickHouse].
Note: UInt8 means 8 bits in [ClickHouse].

[ClickHouse]: https://clickhouse.com/docs/en/sql-reference/data-types/int-uint

<a id="op-47a50300b6ac5b45cae995d5"></a>
## USmallInt

`variant` · `sqlparser::ast::data_type::DataType::USmallInt` · sqlparser 0.62.0

```rust
USmallInt
```

Source: `src/ast/data_type.rs:187`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned small integer, e.g. USMALLINT.

<a id="op-6d6c810f70c62f9e9f8dfc10"></a>
## UTinyInt

`variant` · `sqlparser::ast::data_type::DataType::UTinyInt` · sqlparser 0.62.0

```rust
UTinyInt
```

Source: `src/ast/data_type.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned tiny integer, e.g. UTINYINT

<a id="op-26393076988e9b23a128ef5b"></a>
## Union

`variant` · `sqlparser::ast::data_type::DataType::Union` · sqlparser 0.62.0

```rust
Union
```

Source: `src/ast/data_type.rs:466`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Union type, see [DuckDB].

[DuckDB]: https://duckdb.org/docs/sql/data_types/union.html

<a id="op-f2f9f78e83a3f1d8801de6fc"></a>
## Unsigned

`variant` · `sqlparser::ast::data_type::DataType::Unsigned` · sqlparser 0.62.0

```rust
Unsigned
```

Source: `src/ast/data_type.rs:302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Signed integer as used in [MySQL CAST] target types, without optional `INTEGER` suffix,
e.g. `SIGNED`

[MySQL CAST]: https://dev.mysql.com/doc/refman/8.4/en/cast-functions.html

<a id="op-f1a04307bef86a4568a725c7"></a>
## UnsignedInteger

`variant` · `sqlparser::ast::data_type::DataType::UnsignedInteger` · sqlparser 0.62.0

```rust
UnsignedInteger
```

Source: `src/ast/data_type.rs:307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unsigned integer as used in [MySQL CAST] target types, with optional `INTEGER` suffix,
e.g. `UNSIGNED INTEGER`.

[MySQL CAST]: https://dev.mysql.com/doc/refman/8.4/en/cast-functions.html

<a id="op-1637dfb67226b5b48ce055d7"></a>
## Unspecified

`variant` · `sqlparser::ast::data_type::DataType::Unspecified` · sqlparser 0.62.0

```rust
Unspecified
```

Source: `src/ast/data_type.rs:478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No type specified - only used with
[`SQLiteDialect`](crate::dialect::SQLiteDialect), from statements such
as `CREATE TABLE t1 (a)`.

<a id="op-7ad7a5f6445bead82e969fa3"></a>
## Uuid

`variant` · `sqlparser::ast::data_type::DataType::Uuid` · sqlparser 0.62.0

```rust
Uuid
```

Source: `src/ast/data_type.rs:77`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Uuid type.

<a id="op-d3b68bb854d1c2bc1e07e09c"></a>
## VarBit

`variant` · `sqlparser::ast::data_type::DataType::VarBit` · sqlparser 0.62.0

```rust
VarBit
```

Source: `src/ast/data_type.rs:437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`VARBIT(n)`: Variable-length bit string. [PostgreSQL] alias for `BIT VARYING`.

[PostgreSQL]: https://www.postgresql.org/docs/current/datatype.html

<a id="op-21c0ea40530a810ab1993e8a"></a>
## Varbinary

`variant` · `sqlparser::ast::data_type::DataType::Varbinary` · sqlparser 0.62.0

```rust
Varbinary
```

Source: `src/ast/data_type.rs:105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variable-length binary with optional length type,
see [SQL Standard], [MS SQL Server].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#binary-string-type
[MS SQL Server]: https://learn.microsoft.com/pt-br/sql/t-sql/data-types/binary-and-varbinary-transact-sql?view=sql-server-ver16

<a id="op-38f244e46785d36a96d2f74e"></a>
## Varchar

`variant` · `sqlparser::ast::data_type::DataType::Varchar` · sqlparser 0.62.0

```rust
Varchar
```

Source: `src/ast/data_type.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variable-length character type, e.g. VARCHAR(10).

<a id="op-532643581e4382426f9c3152"></a>
## clone

`function` · `sqlparser::ast::data_type::DataType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 17], "end": [46, 22], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38dafc0710464db4c4afd61d"></a>
## cmp

`function` · `sqlparser::ast::data_type::DataType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DataType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 51], "end": [46, 54], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1809fff5111bda067cfd0781"></a>
## deserialize

`function` · `sqlparser::ast::data_type::DataType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 49], "end": [47, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa66a6857f6c02e4326f0d93"></a>
## eq

`function` · `sqlparser::ast::data_type::DataType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 24], "end": [46, 33], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3468d96ad166e7552848b6a3"></a>
## fmt

`function` · `sqlparser::ast::data_type::DataType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [501, 1], "end": [816, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5d3f2782fb72b9d0900c1d4"></a>
## fmt

`function` · `sqlparser::ast::data_type::DataType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd5b14acee0a7eef4f57331b"></a>
## hash

`function` · `sqlparser::ast::data_type::DataType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 56], "end": [46, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b52ab450e66f507b57f3721e"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::DataType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DataType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 35], "end": [46, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a60ad4eefbfb2f2a7e4aba7c"></a>
## serialize

`function` · `sqlparser::ast::data_type::DataType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 38], "end": [47, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e0f094ae8e39b548d415449"></a>
## visit

`function` · `sqlparser::ast::data_type::DataType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 47], "end": [48, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ff532499cc28faf35f54880"></a>
## visit

`function` · `sqlparser::ast::data_type::DataType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::DataType", "path": "DataType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 40], "end": [48, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
