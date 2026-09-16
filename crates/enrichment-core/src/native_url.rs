//! Bounded URL syntax decoding. Scheme, network and endpoint policy are native expressions.
use arrow::{
    array::{Array, StringArray, StringBuilder, StructArray, UInt64Builder},
    datatypes::{DataType, Field, Fields},
};
use datafusion::{
    error::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
    },
};
use std::sync::Arc;

fn fields() -> Fields {
    vec![
        Field::new("scheme", DataType::Utf8, true),
        Field::new("host", DataType::Utf8, true),
        Field::new("authority", DataType::Utf8, true),
        Field::new("ipv4", DataType::UInt64, true),
        Field::new("ipv6", DataType::Utf8, true),
    ]
    .into()
}
pub fn parts() -> ScalarUDF {
    ScalarUDF::from(UrlParts {
        signature: Signature::exact(vec![DataType::Utf8], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct UrlParts {
    signature: Signature,
}
impl ScalarUDFImpl for UrlParts {
    fn name(&self) -> &str {
        "url_parts_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Struct(fields()))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        if args.args.len() != 1 {
            return Err(DataFusionError::Plan(
                "URL parser requires one argument".into(),
            ));
        }
        let input = args.args[0].clone().into_array(args.number_rows)?;
        let input = input
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                DataFusionError::Plan("URL parser requires native Utf8 coercion".into())
            })?;
        let mut scheme = StringBuilder::new();
        let mut host = StringBuilder::new();
        let mut authority = StringBuilder::new();
        let mut ipv4 = UInt64Builder::new();
        let mut ipv6 = StringBuilder::new();
        let mut validity = Vec::with_capacity(input.len());
        for value in input {
            if value.is_some_and(|v| v.len() > 65_536) {
                return Err(DataFusionError::ResourcesExhausted(
                    "URL exceeds 64 KiB".into(),
                ));
            }
            let parsed = value.and_then(|v| url::Url::parse(v).ok());
            scheme.append_option(parsed.as_ref().map(url::Url::scheme));
            host.append_option(parsed.as_ref().and_then(url::Url::host_str));
            authority.append_option(parsed.as_ref().and_then(|u| {
                u.host_str().map(|host| match u.port() {
                    Some(port) => format!("{host}:{port}"),
                    None => host.to_owned(),
                })
            }));
            let address = parsed.as_ref().and_then(url::Url::host);
            let v4 = match address {
                Some(url::Host::Ipv4(v4)) => Some(v4),
                Some(url::Host::Ipv6(v6)) => v6.to_ipv4_mapped(),
                _ => None,
            };
            ipv4.append_option(v4.map(|v| u64::from(u32::from(v))));
            ipv6.append_option(match address {
                Some(url::Host::Ipv6(v6)) if v4.is_none() => {
                    Some(format!("{:032x}", u128::from(v6)))
                }
                _ => None,
            });
            validity.push(parsed.is_some());
        }
        Ok(ColumnarValue::Array(Arc::new(StructArray::try_new(
            fields(),
            vec![
                Arc::new(scheme.finish()),
                Arc::new(host.finish()),
                Arc::new(authority.finish()),
                Arc::new(ipv4.finish()),
                Arc::new(ipv6.finish()),
            ],
            Some(arrow::buffer::NullBuffer::from(validity)),
        )?)))
    }
}
