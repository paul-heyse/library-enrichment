# `datafusion_expr::udf`

Crate `datafusion-expr` · 5 public items · structured records in [`model/datafusion_expr.udf.json`](../model/datafusion_expr.udf.json)

## ReturnFieldArgs

`struct` · `datafusion_expr::udf::ReturnFieldArgs`

Also reachable as `datafusion::logical_expr::ReturnFieldArgs`, `datafusion_expr::ReturnFieldArgs`

```rust
struct ReturnFieldArgs<'a>
```

**Fields**: `arg_fields`, `scalar_arguments`

**Implements**: `core::convert::From`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/datafusion_expr.udf.ReturnFieldArgs.md).


Information about arguments passed to the function

This structure contains metadata about how the function was called
such as the type of the arguments, any scalar arguments and if the
arguments can (ever) be null

See [`ScalarUDFImpl::return_field_from_args`] for more information

---

## ScalarFunctionArgs

`struct` · `datafusion_expr::udf::ScalarFunctionArgs`

Also reachable as `datafusion::logical_expr::ScalarFunctionArgs`, `datafusion_expr::ScalarFunctionArgs`

```rust
struct ScalarFunctionArgs
```

**Fields**: `args`, `arg_fields`, `number_rows`, `return_field`, `config_options`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn return_type(&self) -> &DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.udf.ScalarFunctionArgs.md).


Arguments passed to [`ScalarUDFImpl::invoke_with_args`] when invoking a
scalar function.

---

## ScalarUDF

`struct` · `datafusion_expr::udf::ScalarUDF`

Also reachable as `datafusion::logical_expr::ScalarUDF`, `datafusion_expr::ScalarUDF`

```rust
struct ScalarUDF
```

**Implements**: `core::convert::From`, `datafusion_expr::type_coercion::functions::UDFCoercionExt`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (29)

```rust
fn aliases(&self) -> &[String]
fn as_async(&self) -> Option<&AsyncScalarUDF>
fn call(&self, args: Vec<Expr>) -> Expr
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
fn display_name(&self, args: &[Expr]) -> Result<String>
fn documentation(&self) -> Option<&Documentation>
fn evaluate_bounds(&self, inputs: &[&Interval]) -> Result<Interval>
fn inner(&self) -> &Arc<dyn ScalarUDFImpl>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn is_nullable(&self, args: &[Expr], schema: &dyn ExprSchema) -> bool
fn is_strict(&self) -> bool
fn name(&self) -> &str
fn new_from_impl<F>(fun: F) -> ScalarUDF where F: ScalarUDFImpl + 'static
fn new_from_shared_impl(fun: Arc<dyn ScalarUDFImpl>) -> ScalarUDF
fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
fn placement(&self, args: &[ExpressionPlacement]) -> ExpressionPlacement
fn preimage(&self, args: &[Expr], lit_expr: &Expr, info: &SimplifyContext) -> Result<PreimageResult>
fn preserves_lex_ordering(&self, inputs: &[ExprProperties]) -> Result<bool>
fn propagate_constraints(&self, interval: &Interval, inputs: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn schema_name(&self, args: &[Expr]) -> Result<String>
fn short_circuits(&self) -> bool
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
fn strictly_order_preserving(&self, inputs: &[ExprProperties]) -> Result<bool>
fn struct_field_mapping(&self, literal_args: &[Option<ScalarValue>]) -> Option<StructFieldMapping>
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
```

**via `core::convert::From`**

```rust
fn from(fun: F) -> Self
```

**via `datafusion_expr::type_coercion::functions::UDFCoercionExt`**

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn name(&self) -> &str
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.udf.ScalarUDF.md).


Logical representation of a Scalar User Defined Function.

A scalar function produces a single row output for each row of input. This
struct contains the information DataFusion needs to plan and invoke
functions you supply such as name, type signature, return type, and actual
implementation.

1. For simple use cases, use [`create_udf`] (examples in [`simple_udf.rs`]).

2. For advanced use cases, use [`ScalarUDFImpl`] which provides full API
   access (examples in  [`advanced_udf.rs`]).

See [`Self::call`] to create an `Expr` which invokes a `ScalarUDF` with arguments.

# API Note

This is a separate struct from [`ScalarUDFImpl`] to maintain backwards
compatibility with the older API.

[`create_udf`]: crate::expr_fn::create_udf
[`simple_udf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/simple_udf.rs
[`advanced_udf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udf.rs

---

## StructFieldMapping

`struct` · `datafusion_expr::udf::StructFieldMapping`

Also reachable as `datafusion::logical_expr::StructFieldMapping`, `datafusion_expr::StructFieldMapping`

```rust
struct StructFieldMapping
```

**Fields**: `field_accessor`, `fields`

[Full member, field, variant and typed contracts](../operations/datafusion_expr.udf.StructFieldMapping.md).


Describes how a struct-producing UDF's output fields correspond to its
input arguments. This enables the optimizer to propagate orderings
through struct projections (e.g., so that sorting by a struct field
can be recognized as equivalent to sorting by the source column).

See [`ScalarUDFImpl::struct_field_mapping`] for details.

---

## ScalarUDFImpl

`trait` · `datafusion_expr::udf::ScalarUDFImpl`

Also reachable as `datafusion::logical_expr::ScalarUDFImpl`, `datafusion_expr::ScalarUDFImpl`

```rust
trait ScalarUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any
```

**Implementors** (235)

- `datafusion_expr::async_udf::AsyncScalarUDF`
- `datafusion_expr::expr_fn::SimpleScalarUDF`
- `datafusion_ffi::udf::ForeignScalarUDF`
- `datafusion_functions::core::arrow_cast::ArrowCastFunc`
- `datafusion_functions::core::arrow_field::ArrowFieldFunc`
- `datafusion_functions::core::arrow_metadata::ArrowMetadataFunc`
- `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc`
- `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc`
- `datafusion_functions::core::cast_to_type::CastToTypeFunc`
- `datafusion_functions::core::coalesce::CoalesceFunc`
- `datafusion_functions::core::file_row_index::FileRowIndexFunc`
- `datafusion_functions::core::getfield::GetFieldFunc`
- `datafusion_functions::core::greatest::GreatestFunc`
- `datafusion_functions::core::input_file_name::InputFileNameFunc`
- `datafusion_functions::core::least::LeastFunc`
- `datafusion_functions::core::named_struct::NamedStructFunc`
- `datafusion_functions::core::nullif::NullIfFunc`
- `datafusion_functions::core::nvl2::NVL2Func`
- `datafusion_functions::core::nvl::NVLFunc`
- `datafusion_functions::core::overlay::OverlayFunc`
- `datafusion_functions::core::struct::StructFunc`
- `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc`
- `datafusion_functions::core::union_extract::UnionExtractFun`
- `datafusion_functions::core::union_tag::UnionTagFunc`
- `datafusion_functions::core::version::VersionFunc`
- `datafusion_functions::core::with_metadata::WithMetadataFunc`
- `datafusion_functions::crypto::digest::DigestFunc`
- `datafusion_functions::crypto::md5::Md5Func`
- `datafusion_functions::crypto::sha::SHAFunc`
- `datafusion_functions::datetime::current_date::CurrentDateFunc`
- `datafusion_functions::datetime::current_time::CurrentTimeFunc`
- `datafusion_functions::datetime::date_bin::DateBinFunc`
- `datafusion_functions::datetime::date_part::DatePartFunc`
- `datafusion_functions::datetime::date_trunc::DateTruncFunc`
- `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc`
- `datafusion_functions::datetime::make_date::MakeDateFunc`
- `datafusion_functions::datetime::make_time::MakeTimeFunc`
- `datafusion_functions::datetime::now::NowFunc`
- `datafusion_functions::datetime::to_char::ToCharFunc`
- `datafusion_functions::datetime::to_date::ToDateFunc`
- `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc`
- `datafusion_functions::datetime::to_time::ToTimeFunc`
- `datafusion_functions::datetime::to_timestamp::ToTimestampFunc`
- `datafusion_functions::datetime::to_timestamp::ToTimestampMicrosFunc`
- `datafusion_functions::datetime::to_timestamp::ToTimestampMillisFunc`
- `datafusion_functions::datetime::to_timestamp::ToTimestampNanosFunc`
- `datafusion_functions::datetime::to_timestamp::ToTimestampSecondsFunc`
- `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc`
- `datafusion_functions::encoding::inner::DecodeFunc`
- `datafusion_functions::encoding::inner::EncodeFunc`
- `datafusion_functions::math::abs::AbsFunc`
- `datafusion_functions::math::ceil::CeilFunc`
- `datafusion_functions::math::cot::CotFunc`
- `datafusion_functions::math::factorial::FactorialFunc`
- `datafusion_functions::math::floor::FloorFunc`
- `datafusion_functions::math::gcd::GcdFunc`
- `datafusion_functions::math::iszero::IsZeroFunc`
- `datafusion_functions::math::lcm::LcmFunc`
- `datafusion_functions::math::log::LogFunc`
- `datafusion_functions::math::nans::IsNanFunc`
- `datafusion_functions::math::nanvl::NanvlFunc`
- `datafusion_functions::math::pi::PiFunc`
- `datafusion_functions::math::power::PowerFunc`
- `datafusion_functions::math::random::RandomFunc`
- `datafusion_functions::math::round::RoundFunc`
- `datafusion_functions::math::signum::SignumFunc`
- `datafusion_functions::math::trunc::TruncFunc`
- `datafusion_functions::regex::regexpcount::RegexpCountFunc`
- `datafusion_functions::regex::regexpinstr::RegexpInstrFunc`
- `datafusion_functions::regex::regexplike::RegexpLikeFunc`
- `datafusion_functions::regex::regexpmatch::RegexpMatchFunc`
- `datafusion_functions::regex::regexpreplace::RegexpReplaceFunc`
- `datafusion_functions::string::ascii::AsciiFunc`
- `datafusion_functions::string::bit_length::BitLengthFunc`
- `datafusion_functions::string::btrim::BTrimFunc`
- `datafusion_functions::string::chr::ChrFunc`
- `datafusion_functions::string::concat::ConcatFunc`
- `datafusion_functions::string::concat_ws::ConcatWsFunc`
- `datafusion_functions::string::contains::ContainsFunc`
- `datafusion_functions::string::ends_with::EndsWithFunc`
- `datafusion_functions::string::levenshtein::LevenshteinFunc`
- `datafusion_functions::string::lower::LowerFunc`
- `datafusion_functions::string::ltrim::LtrimFunc`
- `datafusion_functions::string::octet_length::OctetLengthFunc`
- `datafusion_functions::string::repeat::RepeatFunc`
- `datafusion_functions::string::replace::ReplaceFunc`
- `datafusion_functions::string::rtrim::RtrimFunc`
- `datafusion_functions::string::split_part::SplitPartFunc`
- `datafusion_functions::string::starts_with::StartsWithFunc`
- `datafusion_functions::string::to_hex::ToHexFunc`
- `datafusion_functions::string::upper::UpperFunc`
- `datafusion_functions::string::uuid::UuidFunc`
- `datafusion_functions::unicode::character_length::CharacterLengthFunc`
- `datafusion_functions::unicode::find_in_set::FindInSetFunc`
- `datafusion_functions::unicode::initcap::InitcapFunc`
- `datafusion_functions::unicode::left::LeftFunc`
- `datafusion_functions::unicode::lpad::LPadFunc`
- `datafusion_functions::unicode::reverse::ReverseFunc`
- `datafusion_functions::unicode::right::RightFunc`
- `datafusion_functions::unicode::rpad::RPadFunc`
- `datafusion_functions::unicode::strpos::StrposFunc`
- `datafusion_functions::unicode::substr::SubstrFunc`
- `datafusion_functions::unicode::substrindex::SubstrIndexFunc`
- `datafusion_functions::unicode::translate::TranslateFunc`
- `datafusion_functions_nested::array_add::ArrayAdd`
- `datafusion_functions_nested::array_avg::ArrayAvg`
- `datafusion_functions_nested::array_compact::ArrayCompact`
- `datafusion_functions_nested::array_has::ArrayHas`
- `datafusion_functions_nested::array_has::ArrayHasAll`
- `datafusion_functions_nested::array_has::ArrayHasAny`
- `datafusion_functions_nested::array_normalize::ArrayNormalize`
- `datafusion_functions_nested::array_product::ArrayProduct`
- `datafusion_functions_nested::array_scale::ArrayScale`
- `datafusion_functions_nested::array_subtract::ArraySubtract`
- `datafusion_functions_nested::array_sum::ArraySum`
- `datafusion_functions_nested::arrays_zip::ArraysZip`
- `datafusion_functions_nested::cardinality::Cardinality`
- `datafusion_functions_nested::concat::ArrayAppend`
- `datafusion_functions_nested::concat::ArrayConcat`
- `datafusion_functions_nested::concat::ArrayPrepend`
- `datafusion_functions_nested::cosine_distance::CosineDistance`
- `datafusion_functions_nested::dimension::ArrayDims`
- `datafusion_functions_nested::distance::ArrayDistance`
- `datafusion_functions_nested::empty::ArrayEmpty`
- `datafusion_functions_nested::except::ArrayExcept`
- `datafusion_functions_nested::extract::ArrayElement`
- `datafusion_functions_nested::flatten::Flatten`
- `datafusion_functions_nested::inner_product::InnerProduct`
- `datafusion_functions_nested::length::ArrayLength`
- `datafusion_functions_nested::make_array::MakeArray`
- `datafusion_functions_nested::map::MapFunc`
- `datafusion_functions_nested::map_entries::MapEntriesFunc`
- `datafusion_functions_nested::map_extract::MapExtract`
- `datafusion_functions_nested::map_keys::MapKeysFunc`
- `datafusion_functions_nested::min_max::ArrayMax`
- `datafusion_functions_nested::position::ArrayPosition`
- `datafusion_functions_nested::position::ArrayPositions`
- `datafusion_functions_nested::range::Range`
- `datafusion_functions_nested::remove::ArrayRemove`
- `datafusion_functions_nested::remove::ArrayRemoveAll`
- `datafusion_functions_nested::remove::ArrayRemoveN`
- `datafusion_functions_nested::repeat::ArrayRepeat`
- `datafusion_functions_nested::replace::ArrayReplace`
- `datafusion_functions_nested::resize::ArrayResize`
- `datafusion_functions_nested::reverse::ArrayReverse`
- `datafusion_functions_nested::set_ops::ArrayDistinct`
- `datafusion_functions_nested::set_ops::ArrayIntersect`
- `datafusion_functions_nested::set_ops::ArrayUnion`
- `datafusion_functions_nested::sort::ArraySort`
- `datafusion_functions_nested::string::ArrayToString`
- `datafusion_functions_nested::string::StringToArray`
- `datafusion_spark::function::array::array_contains::SparkArrayContains`
- `datafusion_spark::function::array::repeat::SparkArrayRepeat`
- `datafusion_spark::function::array::shuffle::SparkShuffle`
- `datafusion_spark::function::array::slice::SparkSlice`
- `datafusion_spark::function::array::spark_array::SparkArray`
- `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition`
- `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber`
- `datafusion_spark::function::bitmap::bitmap_count::BitmapCount`
- `datafusion_spark::function::bitwise::bit_count::SparkBitCount`
- `datafusion_spark::function::bitwise::bit_get::SparkBitGet`
- `datafusion_spark::function::bitwise::bit_shift::SparkBitShift`
- `datafusion_spark::function::bitwise::bitwise_not::SparkBitwiseNot`
- `datafusion_spark::function::collection::size::SparkSize`
- `datafusion_spark::function::datetime::add_months::SparkAddMonths`
- `datafusion_spark::function::datetime::date_add::SparkDateAdd`
- `datafusion_spark::function::datetime::date_diff::SparkDateDiff`
- `datafusion_spark::function::datetime::date_part::SparkDatePart`
- `datafusion_spark::function::datetime::date_sub::SparkDateSub`
- `datafusion_spark::function::datetime::date_trunc::SparkDateTrunc`
- `datafusion_spark::function::datetime::extract::SparkHour`
- `datafusion_spark::function::datetime::extract::SparkMinute`
- `datafusion_spark::function::datetime::extract::SparkSecond`
- `datafusion_spark::function::datetime::from_utc_timestamp::SparkFromUtcTimestamp`
- `datafusion_spark::function::datetime::last_day::SparkLastDay`
- `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval`
- `datafusion_spark::function::datetime::make_interval::SparkMakeInterval`
- `datafusion_spark::function::datetime::monthname::SparkMonthName`
- `datafusion_spark::function::datetime::next_day::SparkNextDay`
- `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc`
- `datafusion_spark::function::datetime::to_utc_timestamp::SparkToUtcTimestamp`
- `datafusion_spark::function::datetime::trunc::SparkTrunc`
- `datafusion_spark::function::datetime::unix::SparkUnixDate`
- `datafusion_spark::function::datetime::unix::SparkUnixTimestamp`
- `datafusion_spark::function::datetime::weekday::SparkWeekDay`
- `datafusion_spark::function::hash::crc32::SparkCrc32`
- `datafusion_spark::function::hash::sha1::SparkSha1`
- `datafusion_spark::function::hash::sha2::SparkSha2`
- `datafusion_spark::function::hash::xxhash64::SparkXxhash64`
- `datafusion_spark::function::json::json_tuple::JsonTuple`
- `datafusion_spark::function::map::map_from_arrays::MapFromArrays`
- `datafusion_spark::function::map::map_from_entries::MapFromEntries`
- `datafusion_spark::function::map::str_to_map::SparkStrToMap`
- `datafusion_spark::function::math::abs::SparkAbs`
- `datafusion_spark::function::math::atan2::SparkAtan2`
- `datafusion_spark::function::math::bin::SparkBin`
- `datafusion_spark::function::math::ceil::SparkCeil`
- `datafusion_spark::function::math::expm1::SparkExpm1`
- `datafusion_spark::function::math::factorial::SparkFactorial`
- `datafusion_spark::function::math::floor::SparkFloor`
- `datafusion_spark::function::math::hex::SparkHex`
- `datafusion_spark::function::math::hypot::SparkHypot`
- `datafusion_spark::function::math::modulus::SparkMod`
- `datafusion_spark::function::math::modulus::SparkPmod`
- `datafusion_spark::function::math::negative::SparkNegative`
- `datafusion_spark::function::math::pow::SparkPow`
- `datafusion_spark::function::math::rint::SparkRint`
- `datafusion_spark::function::math::round::SparkRound`
- `datafusion_spark::function::math::trigonometry::SparkCsc`
- `datafusion_spark::function::math::trigonometry::SparkSec`
- `datafusion_spark::function::math::unhex::SparkUnhex`
- `datafusion_spark::function::math::width_bucket::SparkWidthBucket`
- `datafusion_spark::function::string::ascii::SparkAscii`
- `datafusion_spark::function::string::base64::SparkBase64`
- `datafusion_spark::function::string::base64::SparkUnBase64`
- `datafusion_spark::function::string::char::CharFunc`
- `datafusion_spark::function::string::concat::SparkConcat`
- `datafusion_spark::function::string::concat_ws::SparkConcatWs`
- `datafusion_spark::function::string::elt::SparkElt`
- `datafusion_spark::function::string::format_string::FormatStringFunc`
- `datafusion_spark::function::string::ilike::SparkILike`
- `datafusion_spark::function::string::is_valid_utf8::SparkIsValidUtf8`
- `datafusion_spark::function::string::length::SparkLengthFunc`
- `datafusion_spark::function::string::like::SparkLike`
- `datafusion_spark::function::string::luhn_check::SparkLuhnCheck`
- `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8`
- `datafusion_spark::function::string::quote::SparkQuote`
- `datafusion_spark::function::string::soundex::SparkSoundex`
- `datafusion_spark::function::string::space::SparkSpace`
- `datafusion_spark::function::string::substring::SparkSubstring`
- `datafusion_spark::function::url::parse_url::ParseUrl`
- `datafusion_spark::function::url::try_parse_url::TryParseUrl`
- `datafusion_spark::function::url::try_url_decode::TryUrlDecode`
- `datafusion_spark::function::url::url_decode::UrlDecode`
- `datafusion_spark::function::url::url_encode::UrlEncode`

**Methods** (24)

```rust
fn aliases(&self) -> &[String]
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
fn display_name(&self, args: &[Expr]) -> Result<String>
fn documentation(&self) -> Option<&Documentation>
fn evaluate_bounds(&self, _input: &[&Interval]) -> Result<Interval>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn is_nullable(&self, _args: &[Expr], _schema: &dyn ExprSchema) -> bool
fn is_strict(&self) -> bool
fn name(&self) -> &str
fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
fn placement(&self, _args: &[ExpressionPlacement]) -> ExpressionPlacement
fn preimage(&self, _args: &[Expr], _lit_expr: &Expr, _info: &SimplifyContext) -> Result<PreimageResult>
fn preserves_lex_ordering(&self, _inputs: &[ExprProperties]) -> Result<bool>
fn propagate_constraints(&self, _interval: &Interval, _inputs: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn schema_name(&self, args: &[Expr]) -> Result<String>
fn short_circuits(&self) -> bool
fn signature(&self) -> &Signature
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
fn strictly_order_preserving(&self, _inputs: &[ExprProperties]) -> Result<bool>
fn struct_field_mapping(&self, _literal_args: &[Option<ScalarValue>]) -> Option<StructFieldMapping>
fn with_updated_config(&self, _config: &ConfigOptions) -> Option<ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.udf.ScalarUDFImpl.md).


Trait for implementing user defined scalar functions.

This trait exposes the full API for implementing user defined functions and
can be used to implement any function.

See [`advanced_udf.rs`] for a full example with complete implementation and
[`ScalarUDF`] for other available options.

[`advanced_udf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udf.rs

# Basic Example
```
# use std::any::Any;
# use std::sync::LazyLock;
# use arrow::datatypes::DataType;
# use datafusion_common::{DataFusionError, plan_err, Result};
# use datafusion_expr::{col, ColumnarValue, Documentation, ScalarFunctionArgs, Signature, Volatility};
# use datafusion_expr::{ScalarUDFImpl, ScalarUDF};
# use datafusion_expr::scalar_doc_sections::DOC_SECTION_MATH;
/// This struct for a simple UDF that adds one to an int32
#[derive(Debug, PartialEq, Eq, Hash)]
struct AddOne {
  signature: Signature,
}

impl AddOne {
  fn new() -> Self {
    Self {
      signature: Signature::uniform(1, vec![DataType::Int32], Volatility::Immutable),
     }
  }
}

static DOCUMENTATION: LazyLock<Documentation> = LazyLock::new(|| {
        Documentation::builder(DOC_SECTION_MATH, "Add one to an int32", "add_one(2)")
            .with_argument("arg1", "The int32 number to add one to")
            .build()
    });

fn get_doc() -> &'static Documentation {
    &DOCUMENTATION
}

/// Implement the ScalarUDFImpl trait for AddOne
impl ScalarUDFImpl for AddOne {
   fn name(&self) -> &str { "add_one" }
   fn signature(&self) -> &Signature { &self.signature }
   fn return_type(&self, args: &[DataType]) -> Result<DataType> {
     if !matches!(args.get(0), Some(&DataType::Int32)) {
       return plan_err!("add_one only accepts Int32 arguments");
     }
     Ok(DataType::Int32)
   }
   // The actual implementation would add one to the argument
   fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        unimplemented!()
   }
   fn documentation(&self) -> Option<&Documentation> {
        Some(get_doc())
    }
}

// Create a new ScalarUDF from the implementation
let add_one = ScalarUDF::from(AddOne::new());

// Call the function `add_one(col)`
let expr = add_one.call(vec![col("a")]);
```

---
