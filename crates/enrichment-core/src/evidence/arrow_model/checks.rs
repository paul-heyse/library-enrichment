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
            (
                "release-metadata/1",
                metadata::ReleaseDetails::KINDS
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            ),
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
pub fn required(field: &Field) -> bool {
    !field.is_nullable()
        || field
            .metadata()
            .get("enrichment.null")
            .is_some_and(|rule| rule == "forbidden")
}

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
    tagged_checks(schema.fields(), None, lit(true), &mut predicates);
    relation_checks(&frame, schema, key, &mut predicates, &mut output)?;
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

fn relation_checks(
    frame: &DataFrame,
    schema: &Schema,
    key: &str,
    predicates: &mut Vec<Expr>,
    output: &mut Vec<(String, DataFrame)>,
) -> Result<()> {
    use datafusion::functions::{string::expr_fn::concat, unicode::expr_fn::left};
    use datafusion::functions_nested::expr_fn::array_length;
    match schema
        .metadata()
        .get("enrichment.relation")
        .map(String::as_str)
    {
        Some("input_artifacts") => {
            predicates.push(col("artifact_id").not_eq(concat(vec![
                lit("art_"),
                left(
                    col("sha256"),
                    lit(super::super::ARTIFACT_ID_HEX_DIGITS as i64),
                ),
            ])));
            predicates.push(empty(col("role")));
        }
        Some("api_observations" | "relationships") => {
            predicates.push(
                col("subject")
                    .field("kind")
                    .in_list(vec![lit("symbol"), lit("definition")], true),
            );
        }
        Some("coverage") => {
            let indexed = col("outcome").eq(lit("indexed"));
            let empty = array_length(col("gaps")).eq(lit(0u64));
            predicates.push(
                indexed
                    .clone()
                    .and(empty.clone().not())
                    .or(indexed.not().and(empty)),
            );
            let gaps = frame
                .clone()
                .select(vec![col(key), col("kind"), col("gaps")])?
                .unnest_columns_with_options(
                    &["gaps"],
                    datafusion::common::UnnestOptions::new().with_preserve_nulls(false),
                )?
                .filter(col("gaps").field("kind").not_eq(col("kind")))?
                .select(vec![col(key)])?;
            output.push(("coverage gap kind".into(), gaps));
        }
        _ => {}
    }
    Ok(())
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
    match field.data_type() {
        DataType::Struct(fields) => {
            tagged_checks(fields, Some(&value), active.clone(), predicates);
            for child in fields {
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

/// Variant ownership is expressed as native predicates over the actual struct fields.
/// Physical optionality never makes a required variant member optional semantically.
fn tagged_checks(
    fields: &arrow::datatypes::Fields,
    value: Option<&Expr>,
    active: Expr,
    predicates: &mut Vec<Expr>,
) {
    let member = |name: &str| value.map_or_else(|| col(name), |value| value.clone().field(name));
    let Some(kind) = fields.iter().find(|field| field.name() == "kind") else {
        return;
    };
    let role = kind
        .metadata()
        .get("enrichment.role")
        .map(String::as_str)
        .unwrap_or("");
    let variants: &[(&str, &[&str], &[&str])] = match role {
        "vocabulary:subject/1" => &[
            ("symbol", &["symbol_id"], &[]),
            ("definition", &["definition_id"], &[]),
            ("library", &["release_id"], &[]),
            ("feature", &["feature"], &[]),
            ("document", &["artifact_id", "heading"], &[]),
            ("example", &["artifact_id", "path"], &[]),
        ],
        "vocabulary:relationship-target/1" => &[
            ("symbol", &["symbol_id"], &[]),
            ("definition", &["definition_id"], &[]),
            ("external", &["path"], &["package"]),
            ("unresolved", &["path"], &[]),
        ],
        "vocabulary:locator/1" => &[
            ("artifact", &[], &[]),
            ("lines", &["start", "end"], &["file"]),
            ("bytes", &["start", "end"], &[]),
            ("archive_member", &["file"], &[]),
            ("heading", &["heading", "ordinal"], &[]),
            ("producer_item", &["producer", "item"], &[]),
            (
                "rustdoc_item",
                &["rustdoc_id"],
                &["reported_file", "reported_line"],
            ),
            (
                "python_declaration",
                &["file", "declaration", "origin"],
                &["start", "overload"],
            ),
            ("manifest_key", &["file", "table", "key"], &[]),
            ("markdown_section", &["file", "heading", "start"], &[]),
            ("source_start", &["file", "start"], &[]),
            ("extension", &["format", "version", "extension"], &[]),
            (
                "sphinx_inventory",
                &["uri", "role", "project", "inventory_version"],
                &[],
            ),
            ("web_document", &["uri", "inventory_version"], &[]),
        ],
        "vocabulary:execution-payload/1" => &[
            ("semantic_query", &["semantic_query"], &[]),
            ("runtime_object", &["runtime_object"], &[]),
            ("usage_probe", &["usage_probe"], &[]),
        ],
        "vocabulary:execution-target/1" => &[
            ("artifact", &["artifact_id", "range"], &[]),
            ("external", &["scope", "path", "limitation"], &[]),
            ("unresolved", &["limitation"], &[]),
        ],
        "vocabulary:release-metadata/1" => &[
            ("rust_docs", &["rust_docs"], &[]),
            ("python_distribution", &["python_distribution"], &[]),
        ],
        _ => return,
    };
    coordinate_checks(role, &member, &active, predicates);
    for (tag, required, optional) in variants {
        for field in fields.iter().filter(|field| field.name() != "kind") {
            if !variants.iter().any(|(_, required, optional)| {
                required.contains(&field.name().as_str())
                    || optional.contains(&field.name().as_str())
            }) {
                continue;
            }
            let field_value = member(field.name());
            let invalid = if required.contains(&field.name().as_str()) {
                field_value.is_null()
            } else if optional.contains(&field.name().as_str()) {
                continue;
            } else {
                field_value.is_not_null()
            };
            predicates.push(
                active
                    .clone()
                    .and(member("kind").eq(lit(*tag)))
                    .and(invalid),
            );
        }
    }
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
fn coordinate_checks(
    role: &str,
    member: &impl Fn(&str) -> Expr,
    active: &Expr,
    predicates: &mut Vec<Expr>,
) {
    let mut check = |tags: &[&str], invalid: Expr| {
        predicates.push(
            active
                .clone()
                .and(member("kind").in_list(tags.iter().map(|s| lit(*s)).collect(), false))
                .and(invalid),
        );
    };
    match role {
        "vocabulary:subject/1" => {
            check(
                &["feature"],
                empty(member("feature")).or(regexp_like(member("feature"), lit(r"[\p{Cc}]"), None)),
            );
            check(&["example"], unsafe_member(member("path")));
        }
        "vocabulary:relationship-target/1" => {
            check(&["external", "unresolved"], empty(member("path")))
        }
        "vocabulary:locator/1" => {
            check(
                &["lines"],
                member("start")
                    .eq(lit(0u64))
                    .or(member("end").lt(member("start"))),
            );
            check(&["bytes"], member("end").lt(member("start")));
            check(
                &[
                    "lines",
                    "archive_member",
                    "python_declaration",
                    "manifest_key",
                    "markdown_section",
                    "source_start",
                ],
                unsafe_member(member("file")),
            );
            check(
                &["rustdoc_item"],
                member("reported_line")
                    .eq(lit(0u64))
                    .or(empty(member("reported_file")))
                    .or(regexp_like(member("reported_file"), lit(r"[\p{Cc}]"), None)),
            );
            check(
                &["python_declaration"],
                empty(member("declaration"))
                    .or(member("start").eq(lit(0u64)))
                    .or(member("origin").in_list(vec![lit("source"), lit("stub")], true)),
            );
            check(
                &["manifest_key"],
                empty(member("table")).or(empty(member("key"))),
            );
            check(
                &["markdown_section", "source_start"],
                member("start").eq(lit(0u64)),
            );
            check(
                &["producer_item"],
                empty(member("producer")).or(empty(member("item"))),
            );
            check(
                &["extension"],
                empty(member("format")).or(empty(member("version"))),
            );
            check(
                &["sphinx_inventory"],
                empty(member("role")).or(empty(member("project"))),
            );
            for (tags, field) in [
                (
                    &[
                        "lines",
                        "python_declaration",
                        "markdown_section",
                        "source_start",
                    ][..],
                    "start",
                ),
                (&["lines"][..], "end"),
                (&["heading"][..], "ordinal"),
                (&["rustdoc_item"][..], "rustdoc_id"),
                (&["rustdoc_item"][..], "reported_line"),
                (&["python_declaration"][..], "overload"),
            ] {
                check(tags, member(field).gt(lit(u64::from(u32::MAX))));
            }
        }
        _ => {}
    }
}
