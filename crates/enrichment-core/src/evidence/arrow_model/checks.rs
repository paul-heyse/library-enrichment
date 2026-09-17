//! Native field admission compiled from the authoritative Arrow and Rust vocabularies.
//! Schema traversal builds plans once; no evidence row is decoded or validated in Rust.
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::{core::expr_ext::FieldAccessor, regex::expr_fn::regexp_like},
    logical_expr::Expr,
    prelude::{col, lit},
};
use std::{collections::BTreeMap, ops::Not, sync::OnceLock};

fn enum_values<T: schemars::JsonSchema>() -> Vec<String> {
    let schema = schemars::schema_for!(T);
    let root = schema.as_value();
    if let Some(values) = root.get("enum").and_then(serde_json::Value::as_array) {
        return values
            .iter()
            .map(|v| v.as_str().expect("string vocabulary").to_owned())
            .collect();
    }
    root.get("oneOf")
        .or_else(|| root.get("anyOf"))
        .and_then(serde_json::Value::as_array)
        .expect("tagged vocabulary")
        .iter()
        .map(|variant| {
            let tag = &variant["properties"]["kind"];
            tag.get("const")
                .or_else(|| tag.get("enum").and_then(|e| e.get(0)))
                .and_then(serde_json::Value::as_str)
                .expect("declared kind tag")
                .to_owned()
        })
        .collect()
}
fn vocabularies() -> &'static BTreeMap<&'static str, Vec<String>> {
    use crate::{
        evidence::execution::*,
        evidence::relational::*,
        evidence::*,
        execution::*,
        identity::Ecosystem,
        policy::ExecutionProfile,
        producer::{RunOutcome, python::ObservationOrigin},
        wire::{EvidenceClass, SourceVersionMatch},
    };
    static VALUES: OnceLock<BTreeMap<&'static str, Vec<String>>> = OnceLock::new();
    VALUES.get_or_init(|| {
        BTreeMap::from([
            ("symbol-kind/1", enum_values::<SymbolKind>()),
            ("ecosystem/1", enum_values::<Ecosystem>()),
            (
                "environment-resolution/1",
                enum_values::<crate::identity::EnvironmentResolution>(),
            ),
            (
                "research-mode/1",
                enum_values::<crate::identity::ResearchMode>(),
            ),
            (
                "published-job/1",
                enum_values::<crate::evidence::catalog::PublishedJobKind>(),
            ),
            (
                "terminal-state/1",
                crate::wire::JobState::TERMINAL
                    .into_iter()
                    .map(|state| {
                        serde_json::to_value(state)
                            .expect("state vocabulary")
                            .as_str()
                            .expect("string state")
                            .to_owned()
                    })
                    .collect(),
            ),
            ("api-origin/1", enum_values::<ApiOrigin>()),
            ("subject/1", enum_values::<SubjectRef>()),
            ("locator/1", enum_values::<Locator>()),
            ("evidence-class/1", enum_values::<EvidenceClass>()),
            (
                "source-version-match/1",
                enum_values::<SourceVersionMatch>(),
            ),
            ("execution-outcome/1", enum_values::<ExecutionOutcome>()),
            ("execution-payload/1", enum_values::<ExecutionPayload>()),
            ("execution-target/1", enum_values::<ExecutionTarget>()),
            ("probe-mode/1", enum_values::<ProbeMode>()),
            ("process-end/1", enum_values::<ProcessEnd>()),
            ("semantic-method/1", enum_values::<SemanticMethod>()),
            ("fragment-kind/1", enum_values::<FragmentKind>()),
            ("relation-kind/1", enum_values::<RelationKind>()),
            ("relationship-target/1", enum_values::<TargetRef>()),
            ("python-origin/1", enum_values::<ObservationOrigin>()),
            ("artifact-kind/1", enum_values::<ArtifactKind>()),
            ("coverage-outcome/1", enum_values::<CoverageOutcome>()),
            ("evidence-kind/1", enum_values::<EvidenceKind>()),
            ("execution-profile/1", enum_values::<ExecutionProfile>()),
            ("gap-reason/1", enum_values::<GapReason>()),
            ("run-outcome/1", enum_values::<RunOutcome>()),
        ])
    })
}

/// The semantic rule is independent of nullable storage/read layouts.
pub use crate::native_schema::required;

/// Bounded violation relations for root fields and each nested list. Arrow nullability,
/// declared vocabularies, reference shape and digests have one native enforcement path.
/// The caller also applies relation-specific identities and cross-relation constraints.
pub fn violations(
    frame: DataFrame,
    schema: &Schema,
    key: &str,
) -> Result<Vec<(String, DataFrame)>> {
    let mut output = Vec::new();
    let mut predicates = Vec::new();
    tagged_checks(schema.fields(), None, lit(true), &mut predicates)?;
    for field in schema.fields() {
        field_checks(
            &frame,
            field,
            col(field.name()),
            lit(true),
            field.name(),
            key,
            &mut predicates,
            &mut output,
        )?;
    }
    add_violations(frame, key, "fields", predicates, &mut output)?;
    Ok(output)
}

fn add_violations(
    frame: DataFrame,
    key: &str,
    label: &str,
    predicates: Vec<Expr>,
    output: &mut Vec<(String, DataFrame)>,
) -> Result<()> {
    if let Some(predicate) = disjunction(predicates) {
        output.push((
            label.to_owned(),
            frame.filter(predicate)?.select(vec![col(key)])?,
        ));
    }
    Ok(())
}

fn disjunction(mut expressions: Vec<Expr>) -> Option<Expr> {
    // Keep plan recursion logarithmic in the contract size. A left fold over every
    // nested field produces a needlessly deep Expr tree before native simplification.
    while expressions.len() > 1 {
        let mut input = expressions.into_iter();
        expressions = std::iter::from_fn(|| {
            input.next().map(|left| match input.next() {
                Some(right) => left.or(right),
                None => left,
            })
        })
        .collect();
    }
    expressions.pop()
}

fn field_checks(
    frame: &DataFrame,
    field: &Field,
    value: Expr,
    present: Expr,
    path: &str,
    key: &str,
    predicates: &mut Vec<Expr>,
    output: &mut Vec<(String, DataFrame)>,
) -> Result<()> {
    if required(field) {
        predicates.push(present.clone().and(value.clone().is_null()));
    }
    let active = present.and(value.clone().is_not_null());
    let role = field
        .metadata()
        .get("enrichment.role")
        .map(String::as_str)
        .unwrap_or("");
    if let Some(vocabulary) = role.strip_prefix("vocabulary:") {
        let values = vocabularies().get(vocabulary).ok_or_else(|| {
            DataFusionError::Internal(format!("unbound native vocabulary {vocabulary}"))
        })?;
        predicates.push(
            active.clone().and(
                value
                    .clone()
                    .in_list(values.iter().map(lit).collect(), true),
            ),
        );
    } else if role.starts_with("key:") || role.starts_with("ref:") {
        predicates.push(active.clone().and(value.clone().eq(lit("")).or(regexp_like(
            value.clone(),
            lit(r"[\p{Cc}]"),
            None,
        ))));
    } else if matches!(role, "extractor-name" | "extractor-version" | "input-role") {
        predicates.push(active.clone().and(empty(value.clone())));
    } else if role == "sha256" {
        predicates.push(
            active
                .clone()
                .and(regexp_like(value.clone(), lit("^[0-9a-f]{64}$"), None).not()),
        );
    }
    if let Some(invalid) = declared_invalid(field, value.clone())? {
        predicates.push(active.clone().and(invalid));
    }
    match field.data_type() {
        DataType::Struct(fields) => {
            tagged_checks(fields, Some(&value), active.clone(), predicates)?;
            for child in fields {
                if let Some(encoded) = child.metadata().get("enrichment.rule") {
                    let rule: crate::native_union::Rule = serde_json::from_str(encoded)
                        .map_err(|error| DataFusionError::Plan(error.to_string()))?;
                    if let crate::native_union::Rule::RangeEnd { start, .. } = rule {
                        predicates.push(
                            active.clone().and(
                                value
                                    .clone()
                                    .field(child.name())
                                    .lt(value.clone().field(start)),
                            ),
                        );
                    }
                }
                field_checks(
                    frame,
                    child,
                    value.clone().field(child.name()),
                    active.clone(),
                    &format!("{path}.{}", child.name()),
                    key,
                    predicates,
                    output,
                )?;
            }
        }
        DataType::List(item) | DataType::LargeList(item) => {
            let nested = frame
                .clone()
                .filter(active)?
                .select(vec![col(key).alias("row_id"), value.alias("item")])?
                .unnest_columns_with_options(
                    &["item"],
                    datafusion::common::UnnestOptions::new().with_preserve_nulls(false),
                )?;
            let mut checks = Vec::new();
            field_checks(
                &nested,
                item,
                col("item"),
                lit(true),
                path,
                "row_id",
                &mut checks,
                output,
            )?;
            add_violations(nested, "row_id", path, checks, output)?;
        }
        _ => {}
    }
    Ok(())
}

/// A declaration's intrinsic value predicate, shared by evidence and every native mutation.
/// Requiredness/presence and container traversal are applied by the caller.
pub(crate) fn declared_invalid(field: &Field, value: Expr) -> Result<Option<Expr>> {
    if let Some(encoded) = field.metadata().get("enrichment.rule") {
        use crate::native_union::{Rule, Unit};
        let rule: Rule = serde_json::from_str(encoded)
            .map_err(|error| DataFusionError::Plan(error.to_string()))?;
        return Ok(match rule {
            Rule::Text
            | Rule::Flatten
            | Rule::Section(_)
            | Rule::ArtifactIdentity { .. }
            | Rule::Documentation
            | Rule::Sequence
            | Rule::Set
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
        });
    }
    Ok(None)
}

/// Variant ownership is expressed as native predicates over the actual struct fields.
/// Physical optionality never makes a required variant member optional semantically.
pub(crate) fn tagged_checks(
    fields: &arrow::datatypes::Fields,
    value: Option<&Expr>,
    active: Expr,
    predicates: &mut Vec<Expr>,
) -> Result<()> {
    let member = |name: &str| value.map_or_else(|| col(name), |value| value.clone().field(name));
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
