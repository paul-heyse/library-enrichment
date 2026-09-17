//! Lossless UTC microsecond event clocks with a single declared wire representation.
use crate::{
    native_types::{ClockMeaning, ClockType, TypeMetadata},
    native_union::Cell,
};
use arrow::{
    array::{Array, ArrayRef, TimestampMicrosecondArray},
    datatypes::{DataType, Field, TimeUnit},
    error::ArrowError,
};
use arrow_schema::extension::ExtensionType;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String", extend("x-enrichment-wire-format" = "utc-microseconds", "pattern" = "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}\\.[0-9]{6}Z$"))]
pub struct EventTime(i64);
impl EventTime {
    /// Capture the clock without losing subseconds or clamping times before the epoch.
    /// # Errors
    /// A system clock outside the supported timestamp range is refused.
    pub fn now() -> Result<Self, ArrowError> {
        let micros = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(value) => i128::try_from(value.as_micros()).map_err(invalid)?,
            Err(error) => -i128::try_from(error.duration().as_micros()).map_err(invalid)?,
        };
        Self::from_micros(i64::try_from(micros).map_err(invalid)?)
    }
    /// # Errors
    /// Values outside the declared four-digit-year wire domain are refused.
    pub fn from_micros(value: i64) -> Result<Self, ArrowError> {
        let time = arrow::temporal_conversions::timestamp_us_to_datetime(value)
            .ok_or_else(|| invalid("timestamp is outside the Arrow calendar range"))?;
        let rendered = time.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();
        if rendered.len() != 27 {
            return Err(invalid("timestamp is outside the wire calendar range"));
        }
        Ok(Self(value))
    }
    pub fn micros(self) -> i64 {
        self.0
    }
}
fn invalid(error: impl std::fmt::Display) -> ArrowError {
    ArrowError::InvalidArgumentError(error.to_string())
}
impl std::fmt::Display for EventTime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&String::from(*self))
    }
}
impl From<EventTime> for String {
    fn from(value: EventTime) -> Self {
        arrow::temporal_conversions::timestamp_us_to_datetime(value.0)
            .expect("admitted timestamp")
            .format("%Y-%m-%dT%H:%M:%S%.6fZ")
            .to_string()
    }
}
impl TryFrom<String> for EventTime {
    type Error = ArrowError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 27 {
            return Err(invalid("event time requires canonical UTC microseconds"));
        }
        let utc: arrow::array::timezone::Tz = "UTC".parse()?;
        let parsed = arrow::compute::kernels::cast_utils::string_to_datetime(&utc, &value)?;
        if parsed.timestamp_subsec_nanos() % 1000 != 0 {
            return Err(invalid("event clock loses submicrosecond precision"));
        }
        let time = Self::from_micros(parsed.timestamp_micros())?;
        if String::from(time) != value {
            return Err(invalid("event time is not canonical UTC"));
        }
        Ok(time)
    }
}
impl Cell for EventTime {
    fn data_type() -> DataType {
        DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into()))
    }
    fn metadata() -> HashMap<String, String> {
        Field::new("clock", Self::data_type(), false)
            .with_extension_type(
                ClockType::try_new(&Self::data_type(), TypeMetadata::new(ClockMeaning::Event))
                    .expect("event clock contract"),
            )
            .metadata()
            .clone()
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(Arc::new(
            TimestampMicrosecondArray::from(
                values.iter().map(|v| v.map(|v| v.0)).collect::<Vec<_>>(),
            )
            .with_timezone("UTC"),
        ))
    }
    fn decode(
        row: crate::evidence::arrow_model::cells::Row<'_>,
        name: &str,
    ) -> Result<Self, ArrowError> {
        Self::from_micros(row.timestamp_micros(name)?)
    }
}

/// Distinct clock domains share the exact UTC microsecond value codec.
macro_rules! clock_domain {
    ($name:ident, $meaning:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
        )]
        #[serde(transparent)]
        pub struct $name(EventTime);
        impl $name {
            pub fn now() -> Result<Self, ArrowError> {
                EventTime::now().map(Self)
            }
            pub fn from_micros(value: i64) -> Result<Self, ArrowError> {
                EventTime::from_micros(value).map(Self)
            }
            pub fn micros(self) -> i64 {
                self.0.micros()
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
        impl From<$name> for String {
            fn from(value: $name) -> String {
                value.0.into()
            }
        }
        impl TryFrom<String> for $name {
            type Error = ArrowError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                EventTime::try_from(value).map(Self)
            }
        }
        impl Cell for $name {
            fn data_type() -> DataType {
                EventTime::data_type()
            }
            fn metadata() -> HashMap<String, String> {
                clock_metadata(ClockMeaning::$meaning)
            }
            fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
                EventTime::encode(&values.iter().map(|v| v.map(|v| &v.0)).collect::<Vec<_>>())
            }
            fn decode(
                row: crate::evidence::arrow_model::cells::Row<'_>,
                name: &str,
            ) -> Result<Self, ArrowError> {
                EventTime::decode(row, name).map(Self)
            }
        }
    };
}
clock_domain!(SubmissionTime, Submission);
clock_domain!(UpdateTime, Update);
clock_domain!(ExpiryTime, Expiry);
clock_domain!(ObservationTime, Observation);
clock_domain!(AcquisitionTime, Acquisition);
fn clock_metadata(meaning: ClockMeaning) -> HashMap<String, String> {
    Field::new("clock", EventTime::data_type(), false)
        .with_extension_type(
            ClockType::try_new(&EventTime::data_type(), TypeMetadata::new(meaning))
                .expect("clock contract"),
        )
        .metadata()
        .clone()
}

/// Explicit constructors and the common instant projection are the only semantic clock conversions.
pub fn function(meaning: Option<ClockMeaning>) -> datafusion::logical_expr::ScalarUDF {
    datafusion::logical_expr::ScalarUDF::from(ClockProjection {
        meaning,
        signature: datafusion::logical_expr::Signature::variadic_any(
            datafusion::logical_expr::Volatility::Immutable,
        ),
    })
}
pub fn expression(
    meaning: ClockMeaning,
    value: datafusion::logical_expr::Expr,
) -> datafusion::logical_expr::Expr {
    function(Some(meaning)).call(vec![value])
}
/// Deliberate service clock resolution, before assigning a semantic meaning.
pub fn now_instant() -> datafusion::logical_expr::Expr {
    datafusion::functions::datetime::expr_fn::date_trunc(
        datafusion::prelude::lit("microsecond"),
        datafusion::functions::datetime::expr_fn::now(),
    )
}
pub fn now(meaning: ClockMeaning) -> datafusion::logical_expr::Expr {
    expression(meaning, now_instant())
}
pub(crate) fn is_projection(function: &datafusion::logical_expr::ScalarUDF) -> bool {
    function.inner().downcast_ref::<ClockProjection>().is_some()
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct ClockProjection {
    meaning: Option<ClockMeaning>,
    signature: datafusion::logical_expr::Signature,
}
impl datafusion::logical_expr::ScalarUDFImpl for ClockProjection {
    fn name(&self) -> &str {
        match self.meaning {
            None => "clock_instant",
            Some(ClockMeaning::Event) => "event_time",
            Some(ClockMeaning::Acquisition) => "acquisition_time",
            Some(ClockMeaning::Observation) => "observation_time",
            Some(ClockMeaning::Submission) => "submission_time",
            Some(ClockMeaning::Update) => "update_time",
            Some(ClockMeaning::Expiry) => "expiry_time",
        }
    }
    fn signature(&self) -> &datafusion::logical_expr::Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> datafusion::common::Result<DataType> {
        datafusion::common::internal_err!("clock projection requires full fields")
    }
    fn return_field_from_args(
        &self,
        args: datafusion::logical_expr::ReturnFieldArgs,
    ) -> datafusion::common::Result<arrow::datatypes::FieldRef> {
        let [input] = args.arg_fields else {
            return datafusion::common::plan_err!("clock projection requires one value");
        };
        if !matches!(input.data_type(), DataType::Timestamp(_, zone) if zone.as_deref().is_none_or(|z| z == "UTC" || z == "+00:00"))
        {
            return datafusion::common::plan_err!("clock projection requires a UTC timestamp");
        }
        if input.metadata().contains_key("ARROW:extension:name") {
            crate::native_types::validate_field(input)?;
            if input
                .metadata()
                .get("ARROW:extension:name")
                .map(String::as_str)
                != Some("enrichment.clock")
            {
                return datafusion::common::plan_err!("clock projection requires a clock domain");
            }
        } else if self.meaning.is_none() {
            return datafusion::common::plan_err!("instant projection requires a declared clock");
        }
        let mut field = Field::new(self.name(), EventTime::data_type(), input.is_nullable());
        if let Some(meaning) = self.meaning {
            field = field.with_metadata(clock_metadata(meaning));
        }
        Ok(Arc::new(field))
    }
    fn invoke_with_args(
        &self,
        args: datafusion::logical_expr::ScalarFunctionArgs,
    ) -> datafusion::common::Result<datafusion::logical_expr::ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let input = args.args[0].to_array(args.number_rows)?;
        let options = arrow::compute::CastOptions {
            safe: false,
            ..Default::default()
        };
        let projected =
            arrow::compute::cast_with_options(input.as_ref(), &EventTime::data_type(), &options)?;
        let restored =
            arrow::compute::cast_with_options(projected.as_ref(), input.data_type(), &options)?;
        if arrow::compute::kernels::cmp::eq(&input.as_ref(), &restored.as_ref())?.false_count() != 0
        {
            return datafusion::common::exec_err!(
                "clock conversion would discard submicrosecond precision"
            );
        }
        let values = projected
            .as_any()
            .downcast_ref::<TimestampMicrosecondArray>()
            .ok_or_else(|| {
                datafusion::common::DataFusionError::Internal("clock projection layout".into())
            })?;
        for bound in [
            arrow::compute::kernels::aggregate::min(values),
            arrow::compute::kernels::aggregate::max(values),
        ]
        .into_iter()
        .flatten()
        {
            EventTime::from_micros(bound)?;
        }
        Ok(datafusion::logical_expr::ColumnarValue::Array(projected))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_microseconds_and_calendar_validation() {
        for (micros, wire) in [
            (-1, "1969-12-31T23:59:59.999999Z"),
            (0, "1970-01-01T00:00:00.000000Z"),
            (1, "1970-01-01T00:00:00.000001Z"),
        ] {
            assert_eq!(String::from(EventTime::from_micros(micros).unwrap()), wire);
            assert_eq!(
                EventTime::try_from(wire.to_owned()).unwrap().micros(),
                micros
            );
        }
        for value in [
            "2026-02-31T00:00:00.000000Z",
            "2026-09-16T00:00:00.0000001Z",
            "2026-09-16T00:00:00Z",
        ] {
            assert!(EventTime::try_from(value.to_owned()).is_err());
        }
        assert!(EventTime::from_micros(i64::MAX).is_err());
    }
}
