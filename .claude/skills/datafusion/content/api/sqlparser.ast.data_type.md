# `sqlparser::ast::data_type`

Crate `sqlparser` · 11 public items · structured records in [`model/sqlparser.ast.data_type.json`](../model/sqlparser.ast.data_type.json)

## ArrayElemTypeDef

`enum` · `sqlparser::ast::data_type::ArrayElemTypeDef`

Also reachable as `sqlparser::ast::ArrayElemTypeDef`

```rust
enum ArrayElemTypeDef
```

**Variants**: `None`, `AngleBracket`, `SquareBracket`, `Parenthesis`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.ArrayElemTypeDef.md).


Represents the data type of the elements in an array (if any) as well as
the syntax used to declare the array.

For example: Bigquery/Hive use `ARRAY<INT>` whereas snowflake uses ARRAY.

---

## BinaryLength

`enum` · `sqlparser::ast::data_type::BinaryLength`

Also reachable as `sqlparser::ast::BinaryLength`

```rust
enum BinaryLength
```

**Variants**: `IntegerLength`, `Max`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.BinaryLength.md).


Information about [binary length][1], including length and possibly unit.

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#binary-length

---

## CharLengthUnits

`enum` · `sqlparser::ast::data_type::CharLengthUnits`

Also reachable as `sqlparser::ast::CharLengthUnits`

```rust
enum CharLengthUnits
```

**Variants**: `Characters`, `Octets`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.CharLengthUnits.md).


Possible units for characters, initially based on 2016 ANSI [SQL Standard][1].

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#char-length-units

---

## CharacterLength

`enum` · `sqlparser::ast::data_type::CharacterLength`

Also reachable as `sqlparser::ast::CharacterLength`

```rust
enum CharacterLength
```

**Variants**: `IntegerLength`, `Max`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.CharacterLength.md).


Information about [character length][1], including length and possibly unit.

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-length

---

## DataType

`enum` · `sqlparser::ast::data_type::DataType`

Also reachable as `sqlparser::ast::DataType`

```rust
enum DataType
```

**Variants**: `Table`, `NamedTable`, `Character`, `Char`, `CharacterVarying`, `CharVarying`, `Varchar`, `Nvarchar`, `Uuid`, `CharacterLargeObject`, `CharLargeObject`, `Clob`, `Binary`, `Varbinary`, `Blob`, `TinyBlob`, `MediumBlob`, `LongBlob`, `Bytes`, `Numeric`, `Decimal`, `DecimalUnsigned`, `BigNumeric`, `BigDecimal`, `Dec`, `DecUnsigned`, `Float`, `FloatUnsigned`, `TinyInt`, `TinyIntUnsigned`, `UTinyInt`, `Int2`, `Int2Unsigned`, `SmallInt`, `SmallIntUnsigned`, `USmallInt`, `MediumInt`, `MediumIntUnsigned`, `Int`, `Int4`, `Int8`, `Int16`, `Int32`, `Int64`, `Int128`, `Int256`, `Integer`, `IntUnsigned`, `Int4Unsigned`, `IntegerUnsigned`, `HugeInt`, `UHugeInt`, `UInt8`, `UInt16`, `UInt32`, `UInt64`, `UInt128`, `UInt256`, `BigInt`, `BigIntUnsigned`, `UBigInt`, `Int8Unsigned`, `Signed`, `SignedInteger`, `Unsigned`, `UnsignedInteger`, `Float4`, `Float32`, `Float64`, `Real`, `RealUnsigned`, `Float8`, `Double`, `DoubleUnsigned`, `DoublePrecision`, `DoublePrecisionUnsigned`, `Bool`, `Boolean`, `Date`, `Date32`, `Time`, `Datetime`, `Datetime64`, `Timestamp`, `TimestampNtz`, `Interval`, `JSON`, `JSONB`, `Regclass`, `Text`, `TinyText`, `MediumText`, `LongText`, `String`, `FixedString`, `Bytea`, `Bit`, `BitVarying`, `VarBit`, `Custom`, `Array`, `Map`, `Tuple`, `Nested`, `Enum`, `Set`, `Struct`, `Union`, `Nullable`, `LowCardinality`, `Unspecified`, `Trigger`, `AnyType`, `GeometricType`, `TsVector`, `TsQuery`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.DataType.md).


SQL data types

---

## EnumMember

`enum` · `sqlparser::ast::data_type::EnumMember`

Also reachable as `sqlparser::ast::EnumMember`

```rust
enum EnumMember
```

**Variants**: `Name`, `NamedValue`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.EnumMember.md).


A member of an ENUM type.

---

## ExactNumberInfo

`enum` · `sqlparser::ast::data_type::ExactNumberInfo`

Also reachable as `sqlparser::ast::ExactNumberInfo`

```rust
enum ExactNumberInfo
```

**Variants**: `None`, `Precision`, `PrecisionAndScale`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.ExactNumberInfo.md).


Additional information for `NUMERIC`, `DECIMAL`, and `DEC` data types
following the 2016 [SQL Standard].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#exact-numeric-type

---

## GeometricTypeKind

`enum` · `sqlparser::ast::data_type::GeometricTypeKind`

Also reachable as `sqlparser::ast::GeometricTypeKind`

```rust
enum GeometricTypeKind
```

**Variants**: `Point`, `Line`, `LineSegment`, `GeometricBox`, `GeometricPath`, `Polygon`, `Circle`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.GeometricTypeKind.md).


Represents different types of geometric shapes which are commonly used in
PostgreSQL/Redshift for spatial operations and geometry-related computations.

[PostgreSQL]: https://www.postgresql.org/docs/9.5/functions-geometry.html

---

## IntervalFields

`enum` · `sqlparser::ast::data_type::IntervalFields`

Also reachable as `sqlparser::ast::IntervalFields`

```rust
enum IntervalFields
```

**Variants**: `Year`, `Month`, `Day`, `Hour`, `Minute`, `Second`, `YearToMonth`, `DayToHour`, `DayToMinute`, `DayToSecond`, `HourToMinute`, `HourToSecond`, `MinuteToSecond`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.IntervalFields.md).


Fields for [Postgres] `INTERVAL` type.

[Postgres]: https://www.postgresql.org/docs/17/datatype-datetime.html

---

## StructBracketKind

`enum` · `sqlparser::ast::data_type::StructBracketKind`

Also reachable as `sqlparser::ast::StructBracketKind`

```rust
enum StructBracketKind
```

**Variants**: `Parentheses`, `AngleBrackets`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.StructBracketKind.md).


Type of brackets used for `STRUCT` literals.

---

## TimezoneInfo

`enum` · `sqlparser::ast::data_type::TimezoneInfo`

Also reachable as `sqlparser::ast::TimezoneInfo`

```rust
enum TimezoneInfo
```

**Variants**: `None`, `WithTimeZone`, `WithoutTimeZone`, `Tz`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.data_type.TimezoneInfo.md).


Timestamp and Time data types information about TimeZone formatting.

This is more related to a display information than real differences between each variant. To
guarantee compatibility with the input query we must maintain its exact information.

---
