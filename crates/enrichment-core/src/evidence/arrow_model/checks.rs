//! Native field admission compiled from the authoritative Arrow and Rust vocabularies.
//! Schema traversal builds plans once; no evidence row is decoded or validated in Rust.
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::{core::expr_ext::FieldAccessor, regex::expr_fn::regexp_like},
    logical_expr::Expr,
    prelude::lit,
};
use std::ops::Not;

/// Requiredness and every intrinsic value rule have one native compiler.
pub use crate::native_schema::required;

/// One scan validates the field declarations and retains a row identity for diagnostics.
/// Cross-relation references are separately compiled against the bound provider namespace.
pub fn violations(
    frame: DataFrame,
    schema: &Schema,
    key: &str,
) -> Result<Vec<(String, DataFrame)>> {
    Ok(crate::native_schema::intrinsic_witness(frame, schema, key)?
        .map(|frame| ("declared fields".into(), frame))
        .into_iter()
        .collect())
}

/// A declaration's intrinsic value predicate, shared by evidence and every native mutation.
/// Requiredness/presence and container traversal are applied by the caller.
pub(crate) fn declared_invalid(field: &Field, value: Expr) -> Result<Option<Expr>> {
    let vocabulary = field
        .metadata()
        .get("enrichment.vocabulary")
        .map(|encoded| {
            let values: Vec<String> = serde_json::from_str(encoded)
                .map_err(|error| DataFusionError::Plan(error.to_string()))?;
            Ok::<_, DataFusionError>(
                value
                    .clone()
                    .in_list(values.iter().map(lit).collect(), true),
            )
        })
        .transpose()?;
    if let Some(encoded) = field.metadata().get("enrichment.rule") {
        use crate::native_union::{Rule, Unit};
        let rule: Rule = serde_json::from_str(encoded)
            .map_err(|error| DataFusionError::Plan(error.to_string()))?;
        let declared = match rule {
            Rule::Text
            | Rule::Flatten
            | Rule::Section(_)
            | Rule::ArtifactIdentity { .. }
            | Rule::Documentation
            | Rule::Sequence
            | Rule::Map
            | Rule::Json
            | Rule::Coordinate(
                Unit::ByteOffset
                | Unit::Ordinal
                | Unit::RustdocItem
                | Unit::LineZeroBased
                | Unit::Utf8Byte
                | Unit::Utf16CodeUnit,
            )
            | Rule::RangeEnd {
                unit:
                    Unit::ByteOffset
                    | Unit::Ordinal
                    | Unit::RustdocItem
                    | Unit::LineZeroBased
                    | Unit::Utf8Byte
                    | Unit::Utf16CodeUnit,
                ..
            } => None,
            Rule::UnsignedRange { min, max } => {
                let bound =
                    |number| datafusion::logical_expr::cast(lit(number), field.data_type().clone());
                Some(
                    value
                        .clone()
                        .lt(bound(min))
                        .or(value.clone().gt(bound(max))),
                )
            }
            Rule::SequenceBounds { min, max } => {
                let length = datafusion::functions_nested::expr_fn::array_length(value.clone());
                Some(length.clone().lt(lit(min)).or(length.gt(lit(max))))
            }
            Rule::Set => {
                use datafusion::functions_nested::expr_fn::{array_distinct, array_length};
                Some(
                    array_length(value.clone()).not_eq(array_length(array_distinct(value.clone()))),
                )
            }
            Rule::BinaryBytes { max } => {
                let encoded =
                    datafusion::functions::encoding::expr_fn::encode(value.clone(), lit("hex"));
                // Physical intrinsic checks do not enter the SQL coercion analyzer.
                // Hex length is Int32/Int64; compare bytes in the declared unsigned domain.
                let bytes = datafusion::logical_expr::cast(
                    datafusion::functions::unicode::expr_fn::character_length(encoded),
                    arrow::datatypes::DataType::UInt64,
                ) / lit(2u64);
                Some(bytes.gt(lit(max)))
            }
            Rule::Sha256 => Some(regexp_like(value.clone(), lit("^[0-9a-f]{64}$"), None).not()),
            Rule::Vocabulary(values) => Some(
                value
                    .clone()
                    .in_list(values.iter().map(lit).collect(), true),
            ),
            Rule::Reference(domain) | Rule::ScopedReference { domain, .. }
                if matches!(field.data_type(), DataType::FixedSizeBinary(32)) =>
            {
                use crate::native_types::IdentityType;
                use arrow_schema::extension::ExtensionType;
                crate::native_types::validate_field(field)?;
                let metadata = IdentityType::deserialize_metadata(
                    field
                        .metadata()
                        .get("ARROW:extension:metadata")
                        .map(String::as_str),
                )?;
                if field
                    .metadata()
                    .get("ARROW:extension:name")
                    .map(String::as_str)
                    != Some(IdentityType::NAME)
                    || metadata.meaning() != &domain
                {
                    return datafusion::common::plan_err!(
                        "reference domain disagrees with binary identity declaration"
                    );
                }
                None
            }
            Rule::ForeignKey { .. } => match field.data_type() {
                arrow::datatypes::DataType::Utf8
                | arrow::datatypes::DataType::LargeUtf8
                | arrow::datatypes::DataType::Utf8View => Some(
                    empty(value.clone()).or(regexp_like(value.clone(), lit(r"[\p{Cc}]"), None)),
                ),
                _ => None,
            },
            Rule::NonEmpty | Rule::Reference(_) | Rule::ScopedReference { .. } => {
                Some(empty(value.clone()).or(regexp_like(value.clone(), lit(r"[\p{Cc}]"), None)))
            }
            Rule::Coordinate(Unit::LineOneBased)
            | Rule::RangeEnd {
                unit: Unit::LineOneBased,
                ..
            } => Some(value.clone().eq(lit(0u32))),
            Rule::MemberPath => Some(unsafe_member(value.clone())),
            Rule::PythonDeclarationOrigin => Some(
                value
                    .clone()
                    .in_list(vec![lit("source"), lit("stub")], true),
            ),
            Rule::DocumentUri => {
                let parsed = crate::native_url::parts().call(vec![value.clone()]);
                Some(
                    parsed
                        .clone()
                        .is_null()
                        .or(parsed
                            .clone()
                            .field("scheme")
                            .in_list(vec![lit("http"), lit("https")], true))
                        .or(parsed.clone().field("host").is_null())
                        .or(parsed.field("has_credentials")),
                )
            }
        };
        return Ok(match (vocabulary, declared) {
            (Some(vocabulary), Some(declared)) => Some(vocabulary.or(declared)),
            (vocabulary, declared) => vocabulary.or(declared),
        });
    }
    Ok(vocabulary)
}

/// Variant ownership is expressed as native predicates over the actual struct fields.
/// Physical optionality never makes a required variant member optional semantically.
pub(crate) fn tagged_checks(
    fields: &arrow::datatypes::Fields,
    value: Option<&Expr>,
    active: Expr,
    predicates: &mut Vec<Expr>,
) -> Result<()> {
    let member = |name: &str| {
        value.map_or_else(
            || Expr::Column(datafusion::common::Column::from_name(name)),
            |value| value.clone().field(name),
        )
    };
    let Some(kind) = fields
        .iter()
        .find(|field| field.metadata().contains_key("enrichment.union.tags"))
    else {
        return Ok(());
    };
    if let Some(tags) = kind.metadata().get("enrichment.union.tags") {
        let tags: Vec<String> =
            serde_json::from_str(tags).map_err(|error| DataFusionError::Plan(error.to_string()))?;
        predicates.push(
            active
                .clone()
                .and(member(kind.name()).in_list(tags.iter().map(lit).collect(), true)),
        );
        for payload in fields.iter().filter(|field| field.name() != kind.name()) {
            let selected = member(kind.name()).eq(lit(payload.name()));
            predicates.push(
                active.clone().and(
                    selected
                        .clone()
                        .and(member(payload.name()).is_null())
                        .or(selected.not().and(member(payload.name()).is_not_null())),
                ),
            );
        }
        return Ok(());
    }
    Ok(())
}

fn safe_member(value: Expr) -> Expr {
    regexp_like(value, lit(r"^(?:[^/\\\p{Cc}]+/)*[^/\\\p{Cc}]+$"), None)
}
fn unsafe_member(value: Expr) -> Expr {
    safe_member(value.clone())
        .not()
        .or(regexp_like(value, lit(r"(^|/)\.{1,2}(/|$)"), None))
}
fn empty(value: Expr) -> Expr {
    value.eq(lit(""))
}
