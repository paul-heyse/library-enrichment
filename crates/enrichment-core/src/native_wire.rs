//! Mechanical Serde boundary for the same exact values emitted by the Arrow sink.
use crate::native_union::Cell;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

pub struct Ref<'a, T: Cell>(pub &'a T);
impl<T: Cell> Serialize for Ref<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize_wire(serializer)
    }
}
pub struct Owned<T: Cell>(pub T);
impl<'de, T: Cell> Deserialize<'de> for Owned<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize_wire(deserializer).map(Self)
    }
}
pub fn serialize<T: Cell, S: Serializer>(value: &T, serializer: S) -> Result<S::Ok, S::Error> {
    value.serialize_wire(serializer)
}
pub fn deserialize<'de, T: Cell, D: Deserializer<'de>>(deserializer: D) -> Result<T, D::Error> {
    T::deserialize_wire(deserializer)
}

/// Native UInt64 has one wire representation, including values below JavaScript's safe bound.
pub fn unsigned<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    struct Unsigned;
    impl de::Visitor<'_> for Unsigned {
        type Value = u64;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a canonical decimal UInt64 string")
        }
        fn visit_str<E: de::Error>(self, value: &str) -> Result<u64, E> {
            if value.is_empty()
                || value.len() > 20
                || (value.len() > 1 && value.starts_with('0'))
                || !value.bytes().all(|byte| byte.is_ascii_digit())
            {
                return Err(E::custom("noncanonical decimal UInt64"));
            }
            value.parse().map_err(E::custom)
        }
    }
    deserializer.deserialize_any(Unsigned)
}

pub fn signed<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    let value = String::deserialize(deserializer)?;
    let parsed: i64 = value.parse().map_err(de::Error::custom)?;
    if parsed.to_string() != value {
        return Err(de::Error::custom("noncanonical decimal Int64"));
    }
    Ok(parsed)
}

/// Schemars projects the same finite bounds as Arrow admission. Do not restate bounds
/// with independent schema attributes on native declarations.
pub fn field_rule(schema: &mut schemars::Schema, rule: crate::native_union::Rule) {
    use crate::native_union::Rule;
    match rule {
        Rule::UnsignedRange { min, max } => {
            schema.insert("minimum".into(), min.into());
            schema.insert("maximum".into(), max.into());
        }
        Rule::SequenceBounds { min, max } => {
            schema.insert("minItems".into(), min.into());
            schema.insert("maxItems".into(), max.into());
        }
        Rule::Set => {
            schema.insert("uniqueItems".into(), true.into());
        }
        Rule::BinaryBytes { max } => {
            schema.insert("maxLength".into(), max.saturating_mul(2).into());
            schema.insert("pattern".into(), "^(?:[0-9a-f]{2})*$".into());
        }
        _ => {}
    }
}

/// Translate the generator's explicit native width into its exact wire representation.
/// This is a format projection, not an independent field-name or semantic-policy registry.
pub fn schema(mut value: serde_json::Value) -> serde_json::Value {
    fn visit(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(object) => {
                for child in object.values_mut() {
                    visit(child);
                }
                if object.get("format").and_then(serde_json::Value::as_str) == Some("int64") {
                    let min = object
                        .get("minimum")
                        .and_then(serde_json::Value::as_i64)
                        .unwrap_or(i64::MIN);
                    let max = object
                        .get("maximum")
                        .and_then(serde_json::Value::as_i64)
                        .unwrap_or(i64::MAX);
                    let mut branches = Vec::new();
                    if max >= 0 {
                        let pattern = decimal_pattern(min.max(0) as u64, max as u64);
                        branches.push(pattern[1..pattern.len() - 1].to_owned());
                    }
                    if min < 0 {
                        let pattern =
                            decimal_pattern(max.min(-1).unsigned_abs(), min.unsigned_abs());
                        branches.push(format!("-{}", &pattern[1..pattern.len() - 1]));
                    }
                    let nullable = object
                        .get("type")
                        .and_then(serde_json::Value::as_array)
                        .is_some_and(|types| types.iter().any(|kind| kind == "null"));
                    object.insert(
                        "type".into(),
                        if nullable {
                            serde_json::json!(["string", "null"])
                        } else {
                            serde_json::json!("string")
                        },
                    );
                    object.insert("format".into(), serde_json::json!("decimal-int64"));
                    object.insert(
                        "pattern".into(),
                        serde_json::json!(format!("^(?:{})$", branches.join("|"))),
                    );
                    object.remove("minimum");
                    object.remove("maximum");
                    if let Some(default) = object.get_mut("default")
                        && let Some(number) = default.as_i64()
                    {
                        *default = serde_json::json!(number.to_string());
                    }
                }
                if matches!(
                    object.get("format").and_then(serde_json::Value::as_str),
                    Some("uint64" | "uint")
                ) {
                    let min = object
                        .get("minimum")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(0);
                    let max = object
                        .get("maximum")
                        .and_then(serde_json::Value::as_u64)
                        .unwrap_or(u64::MAX);
                    let nullable = object
                        .get("type")
                        .and_then(serde_json::Value::as_array)
                        .is_some_and(|types| types.iter().any(|kind| kind == "null"));
                    object.insert(
                        "type".into(),
                        if nullable {
                            serde_json::json!(["string", "null"])
                        } else {
                            serde_json::json!("string")
                        },
                    );
                    object.insert("format".into(), serde_json::json!("decimal-uint64"));
                    object.insert(
                        "pattern".into(),
                        serde_json::json!(decimal_pattern(min, max)),
                    );
                    object.remove("minimum");
                    object.remove("maximum");
                    if let Some(default) = object.get_mut("default")
                        && let Some(number) = default.as_u64()
                    {
                        *default = serde_json::json!(number.to_string());
                    }
                }
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    visit(value);
                }
            }
            _ => {}
        }
    }
    visit(&mut value);
    value
}

/// Exact bounded decimal syntax for JSON Schema validators, including JavaScript clients.
/// Numeric comparison cannot be used on the string representation without losing precision.
fn decimal_pattern(min: u64, max: u64) -> String {
    fn interval(low: &str, high: &str) -> String {
        if low == high {
            return low.into();
        }
        if low.bytes().all(|v| v == b'0') && high.bytes().all(|v| v == b'9') {
            return format!("[0-9]{{{}}}", low.len());
        }
        let a = low.as_bytes()[0];
        let b = high.as_bytes()[0];
        if a == b {
            return format!("{}{}", char::from(a), interval(&low[1..], &high[1..]));
        }
        let suffix = low.len() - 1;
        if low[1..].bytes().all(|v| v == b'0') && high[1..].bytes().all(|v| v == b'9') {
            return format!("[{}-{}][0-9]{{{suffix}}}", char::from(a), char::from(b));
        }
        let mut branches = vec![format!(
            "{}{}",
            char::from(a),
            interval(&low[1..], &"9".repeat(suffix))
        )];
        if b > a + 1 {
            branches.push(format!(
                "[{}-{}][0-9]{{{suffix}}}",
                char::from(a + 1),
                char::from(b - 1)
            ));
        }
        branches.push(format!(
            "{}{}",
            char::from(b),
            interval(&"0".repeat(suffix), &high[1..])
        ));
        format!("(?:{})", branches.join("|"))
    }
    if min > max {
        return "^(?!)$".into();
    }
    let mut branches = Vec::new();
    let mut complete = Vec::new();
    let mut base = 1u64;
    for digits in 1..=20 {
        let low = min.max(if digits == 1 { 0 } else { base });
        let end = base.checked_mul(10).map(|next| next - 1);
        let high = max.min(end.unwrap_or(u64::MAX));
        if low <= high {
            if end == Some(high) && low <= base {
                if low == 0 {
                    branches.push("0".into());
                }
                complete.push(digits);
            } else {
                branches.push(interval(&low.to_string(), &high.to_string()));
            }
        }
        let Some(next) = base.checked_mul(10) else {
            break;
        };
        base = next;
    }
    if let (Some(first), Some(last)) = (complete.first(), complete.last()) {
        branches.push(format!("[1-9][0-9]{{{},{}}}", first - 1, last - 1));
    }
    format!("^(?:{})$", branches.join("|"))
}
