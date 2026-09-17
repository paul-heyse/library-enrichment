//! Shared native schema traversal at declaration and format boundaries.
//!
//! Field names and semantic requiredness survive physical nullable read layouts. The same
//! traversal supplies native validation expressions and refuses lossy struct projections.
use std::collections::HashSet;

use arrow::datatypes::{DataType, Field, FieldRef, Fields, Schema};
use datafusion::{
    common::{Column, DataFusionError, Result},
    dataframe::DataFrame,
    functions::core::expr_ext::FieldAccessor,
    logical_expr::{Expr, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDFImpl},
    prelude::lit,
};

/// Carry outer-join absence into a derived projection's semantic requiredness.
/// Native nullability alone cannot erase an inherited required-field annotation.
/// Only nullable outputs depending on the nullable side are widened; child contracts
/// remain intact and become conditional on the containing record's presence.
/// # Errors
/// Native expression/schema errors refuse construction of the derived relation.
pub fn outer_join_projection(frame: DataFrame) -> Result<DataFrame> {
    use datafusion::{
        common::{
            metadata::FieldMetadata,
            tree_node::{TreeNode, TreeNodeRecursion},
        },
        logical_expr::{ExprSchemable, JoinType, LogicalPlan, Projection},
    };
    let LogicalPlan::Projection(projection) = frame.logical_plan() else {
        return Ok(frame);
    };
    let mut nullable = HashSet::new();
    projection.input.apply(|plan| {
        if let LogicalPlan::Join(join) = plan {
            if matches!(join.join_type, JoinType::Left | JoinType::Full) {
                nullable.extend(join.right.schema().columns());
            }
            if matches!(join.join_type, JoinType::Right | JoinType::Full) {
                nullable.extend(join.left.schema().columns());
            }
        }
        // Qualifiers below aliases belong to a different expression scope.
        Ok(if matches!(plan, LogicalPlan::SubqueryAlias(_)) {
            TreeNodeRecursion::Jump
        } else {
            TreeNodeRecursion::Continue
        })
    })?;
    let expressions = projection
        .expr
        .iter()
        .zip(projection.schema.fields())
        .map(|(expr, field)| {
            let mut widened = false;
            expr.apply(|part| {
                widened |= matches!(part, Expr::Column(column) if nullable.contains(column));
                Ok(TreeNodeRecursion::Continue)
            })?;
            let mut field = field.as_ref().clone();
            if widened && expr.nullable(projection.input.schema())? {
                let mut metadata = field.metadata().clone();
                metadata.insert("enrichment.null".into(), "outer_join_absent".into());
                field = field.with_nullable(true).with_metadata(metadata);
            }
            Ok(if widened {
                expr.clone()
                    .unalias()
                    .alias_with_metadata(field.name(), Some(FieldMetadata::from(&field)))
            } else {
                expr.clone()
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let input = projection.input.clone();
    let (state, _) = frame.into_parts();
    Ok(DataFrame::new(
        state,
        LogicalPlan::Projection(Projection::try_new(expressions, input)?),
    ))
}

/// Check full argument fields at both logical and physical UDF preparation.
/// Names are declaration-local bindings; nested record names and semantic extensions are exact.
/// # Errors
/// Cardinality, value shape or conflicting semantic types refuse preparation.
pub fn function_arguments(actual: &[FieldRef], expected: &[FieldRef]) -> Result<()> {
    if actual.len() != expected.len() {
        return Err(DataFusionError::Plan(
            "native function argument cardinality".into(),
        ));
    }
    for (actual, expected) in actual.iter().zip(expected) {
        check_projection(actual, expected)?;
        if !same_value_type(actual.data_type(), expected.data_type()) {
            return Err(DataFusionError::Plan(format!(
                "native function argument {} requires {}, received {}",
                expected.name(),
                expected.data_type(),
                actual.data_type()
            )));
        }
        for key in ["ARROW:extension:name", "ARROW:extension:metadata"] {
            if actual.metadata().get(key) != expected.metadata().get(key) {
                return Err(DataFusionError::Plan(format!(
                    "native function argument {} changes {key}",
                    expected.name()
                )));
            }
        }
    }
    Ok(())
}

/// A UDF's field contract is also its execution contract; no separate row policy lives here.
/// # Errors
/// Invalid argument fields or scalar-argument cardinality refuse preparation.
pub fn function_output(
    args: &ReturnFieldArgs<'_>,
    expected: &[FieldRef],
    name: &str,
    data_type: DataType,
    nullable: bool,
) -> Result<FieldRef> {
    validate(&Schema::new(expected.to_vec()))?;
    function_arguments(args.arg_fields, expected)?;
    if args.scalar_arguments.len() != args.arg_fields.len() {
        return Err(DataFusionError::Plan(
            "native function literal cardinality".into(),
        ));
    }
    Ok(std::sync::Arc::new(
        Field::new(name, data_type, nullable).with_metadata(std::collections::HashMap::from([(
            "enrichment.function".into(),
            name.into(),
        )])),
    ))
}

/// Full-field parser contract for plain textual protocol inputs.
/// # Errors
/// A semantic identifier/domain cannot be laundered through a text parser.
pub fn text_function_output(
    args: &ReturnFieldArgs<'_>,
    arity: usize,
    name: &str,
    data_type: DataType,
    nullable: bool,
) -> Result<FieldRef> {
    let fields: Vec<_> = (0..arity)
        .map(|index| {
            std::sync::Arc::new(Field::new(
                format!("argument_{index}"),
                DataType::Utf8,
                true,
            ))
        })
        .collect();
    function_output(args, &fields, name, data_type, nullable)
}

/// Reuse the UDF's planning contract at direct and physical invocation boundaries.
/// # Errors
/// Inconsistent argument or output fields refuse execution before a kernel runs.
pub fn function_call(function: &impl ScalarUDFImpl, args: &ScalarFunctionArgs) -> Result<()> {
    if args.args.len() != args.arg_fields.len() {
        return Err(DataFusionError::Plan(
            "native function execution cardinality".into(),
        ));
    }
    let scalars: Vec<_> = args
        .args
        .iter()
        .map(|arg| match arg {
            datafusion::logical_expr::ColumnarValue::Scalar(value) => Some(value),
            datafusion::logical_expr::ColumnarValue::Array(_) => None,
        })
        .collect();
    let expected = function.return_field_from_args(ReturnFieldArgs {
        arg_fields: &args.arg_fields,
        scalar_arguments: &scalars,
    })?;
    if args.return_field.data_type() != expected.data_type()
        || args.return_field.metadata() != expected.metadata()
        || (!expected.is_nullable() && args.return_field.is_nullable())
    {
        return Err(DataFusionError::Plan(
            "native function execution output contract".into(),
        ));
    }
    Ok(())
}

/// Requiredness is semantic; a scan may expose a nullable physical field.
#[must_use]
pub fn required(field: &Field) -> bool {
    !field.is_nullable()
        || field
            .metadata()
            .get("enrichment.null")
            .is_some_and(|v| v == "forbidden")
}

/// Validate the finite field tree before recursive planning or encoding.
/// # Errors
/// Duplicate names and excessive field depth/count/metadata are rejected.
pub fn validate(schema: &Schema) -> Result<()> {
    let mut count = 0;
    let mut bytes = schema
        .metadata()
        .iter()
        .map(|(k, v)| k.len() + v.len())
        .sum();
    validate_fields(schema.fields(), 0, &mut count, &mut bytes)
}

fn validate_fields(
    fields: &Fields,
    depth: usize,
    count: &mut usize,
    bytes: &mut usize,
) -> Result<()> {
    if depth > 64 {
        return Err(DataFusionError::ResourcesExhausted(
            "schema nesting exceeds 64".into(),
        ));
    }
    let mut names = HashSet::new();
    let mut wire_names = HashSet::new();
    let mut sections = HashSet::new();
    for field in fields {
        *count += 1;
        *bytes += field.name().len()
            + field
                .metadata()
                .iter()
                .map(|(k, v)| k.len() + v.len())
                .sum::<usize>();
        if *count > 16_384 || *bytes > 1_048_576 {
            return Err(DataFusionError::ResourcesExhausted(
                "schema declaration exceeds field/metadata bounds".into(),
            ));
        }
        if field.name().is_empty() || !names.insert(field.name()) {
            return Err(DataFusionError::Plan(format!(
                "empty or duplicate schema field {:?}",
                field.name()
            )));
        }
        crate::native_types::validate_field(field)?;
        let mut flattened = false;
        if let Some(encoded) = field.metadata().get("enrichment.rule") {
            use crate::native_union::Rule;
            let rule: Rule = serde_json::from_str(encoded)
                .map_err(|error| DataFusionError::Plan(format!("invalid field rule: {error}")))?;
            match rule {
                Rule::Flatten => {
                    flattened = true;
                    let DataType::Struct(children) = field.data_type() else {
                        return datafusion::common::plan_err!("flattened field requires a record");
                    };
                    if !required(field) {
                        return datafusion::common::plan_err!("flattened record must be present");
                    }
                    for name in flattened_names(children, 0)? {
                        if !wire_names.insert(name) {
                            return datafusion::common::plan_err!("flattened wire field collision");
                        }
                    }
                }
                Rule::Section(name) => {
                    if crate::wire::research::ResultSectionName::parse(&name).is_none()
                        || !sections.insert(name.clone())
                        || field.metadata().get("enrichment.section") != Some(&name)
                    {
                        return datafusion::common::plan_err!(
                            "invalid or repeated result section declaration"
                        );
                    }
                }
                Rule::ScopedReference { scope, .. } => {
                    if scope.is_empty()
                        || scope.len() > 16
                        || scope.iter().any(|binding| {
                            [&binding.source, &binding.target].into_iter().any(|path| {
                                path.is_empty()
                                    || path.len() > 64
                                    || path.iter().any(|part| part.is_empty() || part.len() > 256)
                            })
                        })
                    {
                        return Err(DataFusionError::Plan(
                            "invalid bounded reference scope".into(),
                        ));
                    }
                    for binding in scope {
                        let mut local = fields.clone();
                        for (index, segment) in binding.source.iter().enumerate() {
                            let (_, member) = local.find(segment).ok_or_else(|| {
                                DataFusionError::Plan("reference scope source is absent".into())
                            })?;
                            if index + 1 < binding.source.len() {
                                let DataType::Struct(children) = member.data_type() else {
                                    return Err(DataFusionError::Plan(
                                        "reference scope traverses a non-record".into(),
                                    ));
                                };
                                local = children.clone();
                            }
                        }
                    }
                }
                Rule::SequenceBounds { min, max } | Rule::UnsignedRange { min, max }
                    if min > max =>
                {
                    return datafusion::common::plan_err!("inverted declared bounds");
                }
                Rule::UnsignedRange { .. }
                    if !matches!(
                        field.data_type(),
                        DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64
                    ) =>
                {
                    return datafusion::common::plan_err!("unsigned bound requires unsigned value");
                }
                Rule::UnsignedRange { max, .. } => {
                    let largest = match field.data_type() {
                        DataType::UInt8 => u64::from(u8::MAX),
                        DataType::UInt16 => u64::from(u16::MAX),
                        DataType::UInt32 => u64::from(u32::MAX),
                        DataType::UInt64 => u64::MAX,
                        _ => unreachable!("unsigned datatype checked above"),
                    };
                    if max > largest {
                        return datafusion::common::plan_err!(
                            "unsigned bound exceeds declared representation"
                        );
                    }
                }
                Rule::Vocabulary(values) => {
                    let unique: HashSet<_> = values.iter().collect();
                    if values.is_empty()
                        || values.len() > 4096
                        || unique.len() != values.len()
                        || values
                            .iter()
                            .any(|value| value.is_empty() || value.len() > 256)
                    {
                        return Err(DataFusionError::Plan(
                            "invalid bounded vocabulary declaration".into(),
                        ));
                    }
                }
                Rule::RangeEnd { start, .. } | Rule::ArtifactIdentity { digest: start } => {
                    if !fields.iter().any(|other| {
                        other.name() == &start
                            && same_value_type(other.data_type(), field.data_type())
                    }) {
                        return Err(DataFusionError::Plan(
                            "range endpoint names an absent or incompatible start".into(),
                        ));
                    }
                }
                Rule::Sequence | Rule::Set | Rule::SequenceBounds { .. }
                    if !matches!(
                        field.data_type(),
                        DataType::List(_) | DataType::LargeList(_)
                    ) =>
                {
                    return Err(DataFusionError::Plan(
                        "collection rule requires a list".into(),
                    ));
                }
                Rule::Map if !matches!(field.data_type(), DataType::Map(_, _)) => {
                    return Err(DataFusionError::Plan(
                        "map rule requires a native Map".into(),
                    ));
                }
                _ => {}
            }
        }
        if !flattened && !wire_names.insert(field.name().clone()) {
            return datafusion::common::plan_err!("wire field collides with a flattened member");
        }
        if let Some(encoded) = field.metadata().get("enrichment.union.tags") {
            let tags: Vec<String> = serde_json::from_str(encoded)
                .map_err(|error| DataFusionError::Plan(error.to_string()))?;
            let unique: HashSet<_> = tags.iter().collect();
            if field.data_type() != &DataType::Utf8
                || tags.is_empty()
                || tags.len() > 256
                || unique.len() != tags.len()
                || tags.iter().any(|tag| tag.is_empty() || tag.len() > 128)
            {
                return Err(DataFusionError::Plan(
                    "invalid bounded variant declaration".into(),
                ));
            }
        }
        match field.data_type() {
            DataType::Struct(children) => validate_fields(children, depth + 1, count, bytes)?,
            DataType::List(item)
            | DataType::LargeList(item)
            | DataType::FixedSizeList(item, _)
            | DataType::Map(item, _) => {
                validate_fields(&vec![item.clone()].into(), depth + 1, count, bytes)?
            }
            DataType::Dictionary(_, kind) => {
                validate_fields(
                    &vec![std::sync::Arc::new(Field::new(
                        "dictionary_value",
                        kind.as_ref().clone(),
                        true,
                    ))]
                    .into(),
                    depth + 1,
                    count,
                    bytes,
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}

/// The field rule is the single flattening declaration for both schema admission and JSON output.
fn flattened_names(fields: &Fields, depth: usize) -> Result<Vec<String>> {
    if depth > 64 {
        return datafusion::common::plan_err!("flattened record exceeds nesting bound");
    }
    let mut names = Vec::new();
    for field in fields {
        let rule = field
            .metadata()
            .get("enrichment.rule")
            .map(|rule| serde_json::from_str::<crate::native_union::Rule>(rule))
            .transpose()
            .map_err(|error| DataFusionError::Plan(error.to_string()))?;
        if matches!(rule, Some(crate::native_union::Rule::Flatten)) {
            let DataType::Struct(children) = field.data_type() else {
                return datafusion::common::plan_err!("flattened field requires a record");
            };
            names.extend(flattened_names(children, depth + 1)?);
        } else {
            names.push(field.name().clone());
        }
    }
    Ok(names)
}

/// Refuse missing or renamed struct children before DataFusion can fill them with NULL.
/// Container item names are physical Arrow/Delta conventions, not record-field identities.
/// # Errors
/// Different record field sets or conflicting semantic extension metadata are rejected.
pub fn check_projection(actual: &Field, expected: &Field) -> Result<()> {
    for key in ["ARROW:extension:name", "ARROW:extension:metadata"] {
        if let (Some(a), Some(e)) = (actual.metadata().get(key), expected.metadata().get(key))
            && a != e
        {
            return Err(DataFusionError::Plan(format!(
                "conflicting {key} for {}",
                expected.name()
            )));
        }
    }
    match (actual.data_type(), expected.data_type()) {
        (DataType::Struct(a), DataType::Struct(e)) => {
            let names: HashSet<_> = a.iter().map(|f| f.name()).collect();
            let expected_names: HashSet<_> = e.iter().map(|f| f.name()).collect();
            if a.len() != e.len()
                || names.len() != a.len()
                || expected_names.len() != e.len()
                || names != expected_names
            {
                return Err(DataFusionError::Plan(format!(
                    "struct projection changes fields of {}",
                    expected.name()
                )));
            }
            for field in e {
                let source = a
                    .iter()
                    .find(|a| a.name() == field.name())
                    .expect("equal field sets");
                check_projection(source, field)?;
            }
        }
        (
            DataType::List(a) | DataType::LargeList(a),
            DataType::List(e) | DataType::LargeList(e),
        ) => check_projection(a, e)?,
        (DataType::Map(a, _), DataType::Map(e, _)) => {
            let (DataType::Struct(a), DataType::Struct(e)) = (a.data_type(), e.data_type()) else {
                return Err(DataFusionError::Plan(
                    "map entries must be a key/value record".into(),
                ));
            };
            if a.len() != 2 || e.len() != 2 {
                return Err(DataFusionError::Plan(
                    "map entry cardinality must be two".into(),
                ));
            }
            // Arrow's map key/value field names are positional format conventions, unlike
            // the named children of a user record nested inside a key or value.
            check_projection(&a[0], &e[0])?;
            check_projection(&a[1], &e[1])?;
        }
        (DataType::FixedSizeList(a, an), DataType::FixedSizeList(e, en)) if an == en => {
            check_projection(a, e)?
        }
        (DataType::Struct(_), _) | (_, DataType::Struct(_)) => {
            return Err(DataFusionError::Plan(format!(
                "struct projection changes shape of {}",
                expected.name()
            )));
        }
        _ => {}
    }
    Ok(())
}

// After a native cast, metadata restoration must not change record order. Before the cast,
// check_projection permits native name-based reordering, including records inside Map values.
fn positional_records(actual: &DataType, expected: &DataType) -> Result<()> {
    match (actual, expected) {
        (DataType::Struct(a), DataType::Struct(e)) => {
            if a.iter().map(|f| f.name()).ne(e.iter().map(|f| f.name())) {
                return Err(DataFusionError::Plan(
                    "array metadata restoration would reorder record values".into(),
                ));
            }
            for (a, e) in a.iter().zip(e) {
                positional_records(a.data_type(), e.data_type())?;
            }
        }
        (
            DataType::List(a) | DataType::LargeList(a),
            DataType::List(e) | DataType::LargeList(e),
        )
        | (DataType::Map(a, _), DataType::Map(e, _)) => {
            positional_records(a.data_type(), e.data_type())?
        }
        (DataType::FixedSizeList(a, _), DataType::FixedSizeList(e, _)) => {
            positional_records(a.data_type(), e.data_type())?
        }
        (DataType::Dictionary(_, a), DataType::Dictionary(_, e)) => positional_records(a, e)?,
        _ => {}
    }
    Ok(())
}

/// Restore an exact field layout after native value casting. The pinned DataFusion scalar-list
/// re-encoding drops item metadata in SingleRowListArrayBuilder. Rebuild checked ArrayData metadata
/// without interpreting values or permitting a representation/record-order change.
/// # Errors
/// Different value layouts, child order or invalid required-value buffers refuse restoration.
pub fn array_layout(
    array: arrow::array::ArrayRef,
    expected: &DataType,
) -> Result<arrow::array::ArrayRef> {
    if array.data_type() == expected {
        return Ok(array);
    }
    fn same_layout(actual: &DataType, expected: &DataType) -> bool {
        match (actual, expected) {
            (DataType::Struct(a), DataType::Struct(e)) => {
                a.len() == e.len()
                    && a.iter().zip(e).all(|(a, e)| {
                        a.name() == e.name() && same_layout(a.data_type(), e.data_type())
                    })
            }
            (DataType::List(a), DataType::List(e))
            | (DataType::LargeList(a), DataType::LargeList(e)) => {
                same_layout(a.data_type(), e.data_type())
            }
            (DataType::FixedSizeList(a, n), DataType::FixedSizeList(e, m)) => {
                n == m && same_layout(a.data_type(), e.data_type())
            }
            (DataType::Map(a, n), DataType::Map(e, m)) => {
                n == m && same_layout(a.data_type(), e.data_type())
            }
            (DataType::Dictionary(a, b), DataType::Dictionary(c, d)) => a == c && same_layout(b, d),
            _ => actual == expected,
        }
    }
    if !same_layout(array.data_type(), expected) {
        return Err(DataFusionError::Plan(
            "native cast did not establish the declared array layout".into(),
        ));
    }
    positional_records(array.data_type(), expected)?;
    fn rebuild(data: arrow::array::ArrayData, kind: &DataType) -> Result<arrow::array::ArrayData> {
        let types: Vec<&DataType> = match kind {
            DataType::Struct(fields) => fields.iter().map(|field| field.data_type()).collect(),
            DataType::List(field)
            | DataType::LargeList(field)
            | DataType::FixedSizeList(field, _)
            | DataType::Map(field, _) => vec![field.data_type()],
            DataType::Dictionary(_, value) => vec![value.as_ref()],
            _ => Vec::new(),
        };
        if data.child_data().len() != types.len() {
            return Err(DataFusionError::Plan(
                "native array child cardinality changed".into(),
            ));
        }
        let children = data
            .child_data()
            .iter()
            .zip(types)
            .map(|(child, kind)| rebuild(child.clone(), kind))
            .collect::<Result<Vec<_>>>()?;
        Ok(data
            .into_builder()
            .data_type(kind.clone())
            .child_data(children)
            .build()?)
    }
    Ok(arrow::array::make_array(rebuild(
        array.to_data(),
        expected,
    )?))
}

/// A producer may conservatively declare nullability, but cannot change the value type.
/// # Errors
/// Missing fields, different value types and conflicting semantic metadata are rejected.
pub fn check_input(actual: &Schema, expected: &Schema) -> Result<()> {
    validate(actual)?;
    validate(expected)?;
    if actual.fields().len() != expected.fields().len() {
        return Err(DataFusionError::Plan(
            "input schema field count differs".into(),
        ));
    }
    for field in expected.fields() {
        let source = actual.field_with_name(field.name())?;
        check_projection(source, field)?;
        if !same_value_type(source.data_type(), field.data_type()) {
            return Err(DataFusionError::Plan(format!(
                "input value type differs for {}",
                field.name()
            )));
        }
    }
    Ok(())
}

/// Temporary native cast layout. Full field contracts remain attached and actual child
/// validity is checked when restoring the declared array after casting. Map keys stay required.
pub fn nullable_layout(field: &Field) -> Field {
    let kind = match field.data_type() {
        DataType::Struct(fields) => DataType::Struct(
            fields
                .iter()
                .map(|field| nullable_layout(field))
                .collect::<Vec<_>>()
                .into(),
        ),
        DataType::List(field) => DataType::List(std::sync::Arc::new(nullable_layout(field))),
        DataType::LargeList(field) => {
            DataType::LargeList(std::sync::Arc::new(nullable_layout(field)))
        }
        DataType::FixedSizeList(field, size) => {
            DataType::FixedSizeList(std::sync::Arc::new(nullable_layout(field)), *size)
        }
        DataType::Map(entries, sorted) => {
            let DataType::Struct(fields) = entries.data_type() else {
                unreachable!("validated map declaration")
            };
            DataType::Map(
                std::sync::Arc::new(
                    entries
                        .as_ref()
                        .clone()
                        .with_data_type(DataType::Struct(
                            vec![
                                nullable_layout(&fields[0]).with_nullable(false),
                                nullable_layout(&fields[1]),
                            ]
                            .into(),
                        ))
                        .with_nullable(false),
                ),
                *sorted,
            )
        }
        other => other.clone(),
    };
    field.clone().with_data_type(kind).with_nullable(true)
}

/// Resolve declared requiredness into Arrow fields for a typed syntax producer.
/// Producer facts have no nullable Delta read-layout requirement. Removing the redundant
/// requiredness annotation lets native joins and recursive unions derive nullability while
/// retaining the actual coordinate, vocabulary and domain declarations.
pub fn producer_field(field: &Field) -> Field {
    let kind = match field.data_type() {
        DataType::Struct(fields) => {
            DataType::Struct(fields.iter().map(|f| producer_field(f)).collect())
        }
        DataType::List(item) => DataType::List(std::sync::Arc::new(producer_field(item))),
        DataType::LargeList(item) => DataType::LargeList(std::sync::Arc::new(producer_field(item))),
        DataType::FixedSizeList(item, size) => {
            DataType::FixedSizeList(std::sync::Arc::new(producer_field(item)), *size)
        }
        DataType::Map(item, sorted) => {
            DataType::Map(std::sync::Arc::new(producer_field(item)), *sorted)
        }
        other => other.clone(),
    };
    let mut metadata = field.metadata().clone();
    metadata.remove("enrichment.null");
    // Text carries no intrinsic admission or comparison meaning. Propagating its empty
    // rule through recursive SQL would incorrectly require concat to retain source metadata.
    if metadata.get("enrichment.rule").is_some_and(|encoded| {
        matches!(
            serde_json::from_str(encoded),
            Ok(crate::native_union::Rule::Text)
        )
    }) {
        metadata.remove("enrichment.rule");
    }
    field
        .clone()
        .with_data_type(kind)
        .with_nullable(!required(field))
        .with_metadata(metadata)
}

pub(crate) fn same_value_type(actual: &DataType, expected: &DataType) -> bool {
    if actual.is_string() && expected.is_string() {
        return true;
    }
    match (actual, expected) {
        (DataType::Struct(a), DataType::Struct(e)) => {
            a.len() == e.len()
                && e.iter().all(|field| {
                    a.iter()
                        .find(|source| source.name() == field.name())
                        .is_some_and(|source| {
                            same_value_type(source.data_type(), field.data_type())
                        })
                })
        }
        (
            DataType::List(a) | DataType::LargeList(a),
            DataType::List(e) | DataType::LargeList(e),
        )
        | (DataType::Map(a, _), DataType::Map(e, _)) => {
            same_value_type(a.data_type(), e.data_type())
        }
        _ => actual == expected,
    }
}

/// Derive parent-aware scalar requiredness as total-Boolean native predicates.
/// Collection members need relational UNNEST admission; they are not scalar SQL columns.
/// # Errors
/// Invalid/unbounded schema declarations are rejected before traversal.
pub fn scalar_requirements(schema: &Schema) -> Result<Vec<Expr>> {
    validate(schema)?;
    let mut predicates = Vec::new();
    for field in schema.fields() {
        scalar_field(
            field,
            Expr::Column(Column::from_name(field.name())),
            Vec::new(),
            None,
            &mut predicates,
        );
    }
    Ok(predicates)
}

fn scalar_field(
    field: &Field,
    base: Expr,
    path: Vec<Expr>,
    parent_absent: Option<Expr>,
    output: &mut Vec<Expr>,
) {
    let value = if path.is_empty() {
        base.clone()
    } else {
        datafusion::functions::core::expr_fn::get_field_path(base.clone(), path.clone())
    };
    if required(field)
        && let Some(absent) = &parent_absent
    {
        output.push(absent.clone().or(value.clone().is_not_null()));
    }
    if let DataType::FixedSizeBinary(width) = field.data_type() {
        // Persisted checks use registered native functions on Delta Binary. octet_length
        // accepts strings at this pin; hex encoding preserves every byte without UTF-8 loss.
        let encoded = datafusion::functions::encoding::expr_fn::encode(value.clone(), lit("hex"));
        let length = datafusion::functions::unicode::expr_fn::character_length(encoded);
        let valid = length.eq(lit(i64::from(*width) * 2));
        output.push(if required(field) && parent_absent.is_none() {
            valid
        } else {
            value.clone().is_null().or(valid)
        });
    }
    if let DataType::Struct(children) = field.data_type() {
        let absent = if required(field) {
            parent_absent
        } else {
            let absent = value.clone().is_null();
            Some(parent_absent.map_or(absent.clone(), |parent| parent.or(absent)))
        };
        for child in children {
            let mut path = path.clone();
            path.push(lit(child.name().clone()));
            scalar_field(child, base.clone(), path, absent.clone(), output);
        }
    }
}

/// Native intrinsic admission with one input scan, including variants and repeated records.
/// Built-in higher-order array expressions keep each element attached to its parent. A
/// relation with many collections must not execute its complete input plan once per child.
/// # Errors
/// Invalid declarations or native planning failures refuse admission before any write.
pub fn intrinsic_violations(frame: DataFrame, schema: &Schema) -> Result<Option<DataFrame>> {
    validate(schema)?;
    let mut checks = Vec::new();
    let mut flags = Vec::new();
    for field in schema.fields() {
        let value = Expr::Column(Column::from_name(field.name()));
        let mut predicates = collection_invalid(field, value.clone(), true, 0)?
            .into_iter()
            .collect::<Vec<_>>();
        let mut contextual = false;
        if let Some(encoded) = field.metadata().get("enrichment.rule") {
            use crate::native_union::Rule;
            let rule: Rule =
                serde_json::from_str(encoded).map_err(|e| DataFusionError::Plan(e.to_string()))?;
            match rule {
                Rule::RangeEnd { start, .. } => {
                    contextual = true;
                    predicates.push(value.lt(datafusion::prelude::col(start)));
                }
                Rule::ArtifactIdentity { digest } => {
                    contextual = true;
                    predicates.push(value.not_eq(datafusion::functions::string::expr_fn::concat(
                        vec![lit("art_"), datafusion::prelude::col(digest)],
                    )));
                }
                Rule::ScopedReference { .. } => contextual = true,
                _ => {}
            }
        }
        if let Some(predicate) = any_invalid(predicates) {
            // Contextual declarations keep the full sibling schema; each ordinary field
            // remains an independent shallow predicate over shared Arrow buffers.
            let inputs = if contextual {
                schema.fields().to_vec()
            } else {
                vec![field.clone()]
            };
            checks
                .push(crate::native_predicate::bind_fields(inputs, predicate)?.alias(field.name()));
            flags.push(Expr::Column(Column::from_name(field.name())));
        }
    }
    let mut tags = Vec::new();
    crate::evidence::arrow_model::checks::tagged_checks(
        schema.fields(),
        None,
        lit(true),
        &mut tags,
    )?;
    if let Some(predicate) = any_invalid(tags) {
        // Names cannot collide with declared fields.
        let name = (0..)
            .map(|index| format!("native_variant_{index}"))
            .find(|name| schema.field_with_name(name).is_err())
            .expect("finite schema");
        checks.push(
            crate::native_predicate::bind_fields(schema.fields().to_vec(), predicate)?.alias(&name),
        );
        flags.push(Expr::Column(Column::from_name(name)));
    }
    let Some(invalid) = any_invalid(flags) else {
        return Ok(None);
    };
    Ok(Some(
        frame.select(checks)?.filter(invalid)?.limit(0, Some(1))?,
    ))
}

fn any_invalid(predicates: impl IntoIterator<Item = Expr>) -> Option<Expr> {
    // Schema width must not become expression recursion depth during planning or execution.
    let mut layer = predicates.into_iter().collect::<Vec<_>>();
    while layer.len() > 1 {
        let mut next = Vec::with_capacity(layer.len().div_ceil(2));
        let mut values = layer.into_iter();
        while let Some(left) = values.next() {
            next.push(if let Some(right) = values.next() {
                left.or(right)
            } else {
                left
            });
        }
        layer = next;
    }
    layer.pop()
}

fn repeated_invalid(value: Expr, item: &FieldRef, depth: usize) -> Result<Option<Expr>> {
    use datafusion::functions_nested::expr_fn::{array_has, array_transform};
    let name = format!("collection_member_{depth}");
    let member = Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
        name.clone(),
        Some(item.clone()),
    ));
    Ok(
        collection_invalid(item, member, true, depth + 1)?.map(|invalid| {
            value.clone().is_not_null().and(array_has(
                array_transform(
                    value,
                    datafusion::logical_expr::expr_fn::lambda(vec![name], invalid),
                ),
                lit(true),
            ))
        }),
    )
}

fn collection_invalid(
    field: &Field,
    value: Expr,
    inside: bool,
    depth: usize,
) -> Result<Option<Expr>> {
    use datafusion::functions_nested::expr_fn::{
        array_distinct, array_length, map_keys, map_values,
    };
    let mut invalid = Vec::new();
    if inside && required(field) {
        invalid.push(value.clone().is_null());
    }
    if let Some(predicate) =
        crate::evidence::arrow_model::checks::declared_invalid(field, value.clone())?
    {
        invalid.push(value.clone().is_not_null().and(predicate));
    }
    let nested = match field.data_type() {
        DataType::Struct(children) => {
            let mut predicates = Vec::new();
            crate::evidence::arrow_model::checks::tagged_checks(
                children,
                Some(&value),
                lit(true),
                &mut predicates,
            )?;
            for child in children {
                if let Some(encoded) = child.metadata().get("enrichment.rule") {
                    let rule: crate::native_union::Rule = serde_json::from_str(encoded)
                        .map_err(|error| DataFusionError::Plan(error.to_string()))?;
                    match rule {
                        crate::native_union::Rule::RangeEnd { start, .. } => predicates.push(
                            value
                                .clone()
                                .field(child.name())
                                .lt(value.clone().field(start)),
                        ),
                        crate::native_union::Rule::ArtifactIdentity { digest } => {
                            predicates.push(value.clone().field(child.name()).not_eq(
                                datafusion::functions::string::expr_fn::concat(vec![
                                    lit("art_"),
                                    value.clone().field(digest),
                                ]),
                            ))
                        }
                        _ => {}
                    }
                }
                if let Some(predicate) =
                    collection_invalid(child, value.clone().field(child.name()), inside, depth)?
                {
                    predicates.push(predicate);
                }
            }
            any_invalid(predicates)
        }
        DataType::List(item) | DataType::LargeList(item) | DataType::FixedSizeList(item, _) => {
            repeated_invalid(value.clone(), item, depth)?
        }
        DataType::Map(item, _) => {
            let DataType::Struct(children) = item.data_type() else {
                return Err(DataFusionError::Plan("map entry shape".into()));
            };
            if children.len() != 2 {
                return Err(DataFusionError::Plan("map entry cardinality".into()));
            }
            let keys = map_keys(value.clone());
            let mut predicates =
                vec![array_length(keys.clone()).not_eq(array_length(array_distinct(keys.clone())))];
            // Avoid map_entries: its fixed key/value names can disagree with legal Arrow
            // entry field names at the pinned implementation. Keys and values retain offsets.
            predicates.extend(repeated_invalid(keys, &children[0], depth)?);
            predicates.extend(repeated_invalid(
                map_values(value.clone()),
                &children[1],
                depth,
            )?);
            any_invalid(predicates)
        }
        _ => None,
    };
    if let Some(nested) = nested {
        invalid.push(value.is_not_null().and(nested));
    }
    Ok(any_invalid(invalid))
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn metadata_restoration_never_relabels_record_slots_inside_containers() {
        let original = DataType::Struct(
            vec![
                Field::new("a", DataType::Int32, true),
                Field::new("b", DataType::Int32, true),
            ]
            .into(),
        );
        let renamed = DataType::Struct(
            vec![
                Field::new("b", DataType::Int32, true),
                Field::new("a", DataType::Int32, true),
            ]
            .into(),
        );
        for (source, target) in [
            (
                DataType::FixedSizeList(Arc::new(Field::new("item", original.clone(), true)), 2),
                DataType::FixedSizeList(Arc::new(Field::new("item", renamed.clone(), true)), 2),
            ),
            (
                DataType::Dictionary(Box::new(DataType::Int32), Box::new(original)),
                DataType::Dictionary(Box::new(DataType::Int32), Box::new(renamed)),
            ),
        ] {
            assert!(array_layout(arrow::array::new_empty_array(&source), &target).is_err());
        }
    }
}
