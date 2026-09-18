//! Semantic field admission before native coercion can erase extension domains.
//! This is a DataFusion analyzer over its own expressions, with no parallel query model.
use arrow::datatypes::{DataType, Field};
use datafusion::{
    common::{
        DFSchema, Result,
        config::ConfigOptions,
        tree_node::{TreeNode, TreeNodeRecursion},
    },
    logical_expr::{Expr, ExprSchemable, LogicalPlan, Operator, utils::merge_schema},
    optimizer::analyzer::AnalyzerRule,
};

#[derive(Debug, thiserror::Error)]
#[error("semantic field contract at {operation}: {left} / {right}")]
pub struct SemanticFailure {
    pub operation: String,
    pub left: String,
    pub right: String,
}

fn refuse(operation: &str, left: &Field, right: &Field) -> datafusion::common::DataFusionError {
    let describe = |field: &Field| {
        let name: String = field.name().chars().take(128).collect();
        let extension: String = field
            .metadata()
            .get("ARROW:extension:name")
            .map_or("plain", String::as_str)
            .chars()
            .take(128)
            .collect();
        let domain: String = field
            .metadata()
            .get("ARROW:extension:metadata")
            .map_or("", String::as_str)
            .chars()
            .take(256)
            .collect();
        format!("{name}: {extension}({domain})")
    };
    datafusion::common::DataFusionError::External(Box::new(SemanticFailure {
        operation: operation.into(),
        left: describe(left),
        right: describe(right),
    }))
}

/// A field may contain semantic children even if its container has no extension.
pub fn has_semantics(field: &Field) -> bool {
    field.metadata().contains_key("ARROW:extension:name") || kind_has_semantics(field.data_type())
}
fn kind_has_semantics(kind: &DataType) -> bool {
    match kind {
        DataType::Struct(fields) => fields.iter().any(|field| has_semantics(field)),
        DataType::List(field)
        | DataType::LargeList(field)
        | DataType::ListView(field)
        | DataType::LargeListView(field)
        | DataType::FixedSizeList(field, _)
        | DataType::Map(field, _) => has_semantics(field),
        DataType::Union(fields, _) => fields.iter().any(|(_, field)| has_semantics(field)),
        DataType::RunEndEncoded(ends, values) => has_semantics(ends) || has_semantics(values),
        DataType::Dictionary(_, value) => kind_has_semantics(value),
        _ => false,
    }
}

/// Compare semantic identity independently of physical nullability and identifier aliases.
/// # Errors
/// Missing or incompatible extensions refuse the operation, including nested fields.
pub fn compatible(left: &Field, right: &Field, operation: &str) -> Result<()> {
    for key in ["ARROW:extension:name", "ARROW:extension:metadata"] {
        if left.metadata().get(key) != right.metadata().get(key) {
            return Err(refuse(operation, left, right));
        }
    }
    match (left.data_type(), right.data_type()) {
        (DataType::Struct(a), DataType::Struct(b))
            if has_semantics(left) || has_semantics(right) =>
        {
            if a.len() != b.len() {
                return Err(refuse(operation, left, right));
            }
            for (a, b) in a.iter().zip(b) {
                if a.name() != b.name() {
                    return Err(refuse(operation, left, right));
                }
                compatible(a, b, operation)?;
            }
        }
        (
            DataType::List(a)
            | DataType::LargeList(a)
            | DataType::ListView(a)
            | DataType::LargeListView(a),
            DataType::List(b)
            | DataType::LargeList(b)
            | DataType::ListView(b)
            | DataType::LargeListView(b),
        )
        | (DataType::Map(a, _), DataType::Map(b, _)) => compatible(a, b, operation)?,
        (DataType::FixedSizeList(a, n), DataType::FixedSizeList(b, m)) if n == m => {
            compatible(a, b, operation)?
        }
        (DataType::Dictionary(_, a), DataType::Dictionary(_, b)) => compatible(
            &Field::new("value", a.as_ref().clone(), true),
            &Field::new("value", b.as_ref().clone(), true),
            operation,
        )?,
        (DataType::RunEndEncoded(a_ends, a_values), DataType::RunEndEncoded(b_ends, b_values)) => {
            compatible(a_ends, b_ends, operation)?;
            compatible(a_values, b_values, operation)?;
        }
        (DataType::Union(a, a_mode), DataType::Union(b, b_mode))
            if has_semantics(left) || has_semantics(right) =>
        {
            if a_mode != b_mode || a.len() != b.len() {
                return Err(refuse(operation, left, right));
            }
            for ((a_id, a_field), (b_id, b_field)) in a.iter().zip(b.iter()) {
                if a_id != b_id || a_field.name() != b_field.name() {
                    return Err(refuse(operation, left, right));
                }
                compatible(a_field, b_field, operation)?;
            }
        }
        (a, b) if (has_semantics(left) || has_semantics(right)) && a != b => {
            return Err(refuse(operation, left, right));
        }
        _ => {}
    }
    Ok(())
}

fn field(expr: &Expr, schema: &DFSchema) -> Result<Field> {
    Ok(expr.to_field(schema)?.1.as_ref().clone())
}

fn pair(a: &Expr, b: &Expr, schema: &DFSchema, operation: &str) -> Result<()> {
    compatible(&field(a, schema)?, &field(b, schema)?, operation)
}

fn expression(expr: &Expr, schema: &DFSchema) -> Result<()> {
    // Ordinary unannotated arithmetic retains native coercion. Only expressions touching
    // declared semantic values need this additional admission before coercion.
    let mut semantic = false;
    expr.apply(|child| {
        if field(child, schema).is_ok_and(|f| has_semantics(&f)) {
            semantic = true;
        }
        Ok(TreeNodeRecursion::Continue)
    })?;
    if !semantic {
        return Ok(());
    }
    expr.apply(|expr| {
        match expr {
            Expr::BinaryExpr(binary) => {
                pair(&binary.left, &binary.right, schema, &binary.op.to_string())?;
                if !matches!(
                    binary.op,
                    Operator::Eq
                        | Operator::NotEq
                        | Operator::Lt
                        | Operator::LtEq
                        | Operator::Gt
                        | Operator::GtEq
                        | Operator::IsDistinctFrom
                        | Operator::IsNotDistinctFrom
                ) && has_semantics(&field(&binary.left, schema)?)
                {
                    return Err(refuse(
                        &binary.op.to_string(),
                        &field(&binary.left, schema)?,
                        &field(expr, schema)?,
                    ));
                }
            }
            Expr::Alias(alias) => {
                compatible(&field(&alias.expr, schema)?, &field(expr, schema)?, "alias")?
            }
            Expr::Cast(cast) => {
                if !matches!(cast.expr.as_ref(), Expr::Literal(datafusion::common::ScalarValue::Null, _)) {
                    compatible(&field(&cast.expr, schema)?, &field(expr, schema)?, "cast")?;
                }
            }
            Expr::TryCast(cast) => {
                if !matches!(cast.expr.as_ref(), Expr::Literal(datafusion::common::ScalarValue::Null, _)) {
                    compatible(&field(&cast.expr, schema)?, &field(expr, schema)?, "try_cast")?;
                }
                // Coercion gives a literal SQL NULL its surrounding type. It contributes
                // absence, not an unannotated value that could erase a semantic domain.
            }
            Expr::InList(list) => {
                for item in &list.list {
                    pair(&list.expr, item, schema, "IN")?;
                }
            }
            Expr::Between(between) => {
                pair(&between.expr, &between.low, schema, "BETWEEN lower")?;
                pair(&between.expr, &between.high, schema, "BETWEEN upper")?;
            }
            Expr::Case(case) => {
                let output = field(expr, schema)?;
                for (when, then) in &case.when_then_expr {
                    if let Some(base) = &case.expr {
                        pair(base, when, schema, "CASE condition")?;
                    }
                    // A literal SQL NULL contributes no value or domain. The other
                    // branches still prove the complete output contract before coercion.
                    if !matches!(
                        then.as_ref(),
                        Expr::Literal(datafusion::common::ScalarValue::Null, _)
                    ) {
                        compatible(&field(then, schema)?, &output, "CASE result")?;
                    }
                }
                if let Some(other) = &case.else_expr
                    && !matches!(
                        other.as_ref(),
                        Expr::Literal(datafusion::common::ScalarValue::Null, _)
                    )
                {
                    compatible(&field(other, schema)?, &output, "CASE ELSE")?;
                }
            }
            Expr::InSubquery(query) => {
                compatible(
                    &field(&query.expr, schema)?,
                    query.subquery.subquery.schema().field(0),
                    "IN subquery",
                )?;
                validate_plan(&query.subquery.subquery)?;
            }
            Expr::SetComparison(query) => {
                compatible(
                    &field(&query.expr, schema)?,
                    query.subquery.subquery.schema().field(0),
                    "set comparison",
                )?;
                validate_plan(&query.subquery.subquery)?;
            }
            Expr::ScalarSubquery(query)
            | Expr::Exists(datafusion::logical_expr::expr::Exists {
                subquery: query, ..
            }) => validate_plan(&query.subquery)?,
            Expr::ScalarFunction(function) if function.func.inner().downcast_ref::<datafusion::functions::core::getfield::GetFieldFunc>().is_some() => {
                // DataFusion derives the selected child's full Field; no parent-domain copy.
                field(expr, schema)?;
            }
            Expr::ScalarFunction(function)
                if function.func.inner().downcast_ref::<datafusion::functions_nested::extract::ArrayElement>().is_some() =>
            {
                let input = field(&function.args[0], schema)?;
                let output = field(expr, schema)?;
                match input.data_type() {
                    DataType::List(item) | DataType::LargeList(item) | DataType::FixedSizeList(item, _) =>
                        compatible(item, &output, "array element selection")?,
                    DataType::Null => {},
                    _ => return Err(refuse("array element selection", &input, &output)),
                }
                // The pinned kernel retains nested Struct fields. A scalar element whose
                // extension metadata is lost is refused by the same compatibility check.
            }
            Expr::ScalarFunction(function)
                if crate::native_identity::is_value_encoder(&function.func)
                    || crate::native_id::is_projection(&function.func)
                    || crate::native_predicate::is_predicate(&function.func)
                    || crate::native_collections::is_projection(&function.func)
                    || crate::native_comparison::is_projection(&function.func)
                    || crate::native_runtime::is_decoder(&function.func) || crate::native_lsp::is_kernel(&function.func)
                    || crate::native_time::is_projection(&function.func)
                    || crate::search::row_page::is_encoder(&function.func) =>
            {
                // Concrete native kernels verify full declared input/output fields. Format
                // decoders, coordinate conversions and canonical bytes have explicit contracts.
                field(expr, schema)?;
            }
            Expr::ScalarFunction(function) if crate::native_transport::is_measurement(&function.func) =>
            {
                // A verified format kernel observes encoded length without reinterpreting
                // any input domain. Its full-field contract still applies before execution.
                field(expr, schema)?;
            }
            Expr::ScalarFunction(function)
                if function.func.inner().downcast_ref::<datafusion::functions::core::named_struct::NamedStructFunc>().is_some()
                    || crate::native_record::is_record(&function.func) =>
            {
                let output = field(expr, schema)?;
                let DataType::Struct(fields) = output.data_type() else {
                    return Err(refuse("named_struct", &output, &output));
                };
                for (args, output) in function.args.as_chunks::<2>().0.iter().zip(fields) {
                    compatible(&field(&args[1], schema)?, output, "named_struct child")?;
                }
            }
            Expr::ScalarFunction(function) if crate::native_selection::is_coalesce(&function.func) => {
                // The full-field return contract admits NULLs and checks every semantic
                // branch before the function lowers to native CASE.
                let _ = field(expr, schema)?;
            }
            Expr::ScalarFunction(function) if function.func.inner().downcast_ref::<datafusion::functions::core::coalesce::CoalesceFunc>().is_some() => {
                let output = field(expr, schema)?;
                for arg in &function.args {
                    compatible(&field(arg, schema)?, &output, "coalesce")?;
                }
            }
            Expr::AggregateFunction(function)
                if function
                    .func
                    .inner()
                    .downcast_ref::<datafusion::functions_aggregate::count::Count>()
                    .is_some() =>
            {
                // Native COUNT observes cardinality/nullness (and same-domain equality
                // for DISTINCT). It does not reinterpret any input as a different domain.
            }
            Expr::ScalarFunction(function)
                if function.func.inner().downcast_ref::<datafusion::functions_nested::make_array::MakeArray>().is_some() =>
            {
                let output=field(expr,schema)?;
                let DataType::List(item)=output.data_type() else {
                    return Err(refuse("array construction",&output,&output));
                };
                for input in &function.args {
                    if !matches!(input,Expr::Literal(datafusion::common::ScalarValue::Null,_)) {
                        compatible(&field(input,schema)?,item,"array construction")?;
                    }
                }
                // The upstream return_type keeps nested Struct child fields, but loses
                // top-level scalar extension metadata. Only the proven former case passes.
            }
            Expr::AggregateFunction(function)
                if function.func.inner().downcast_ref::<datafusion::functions_aggregate::array_agg::ArrayAgg>().is_some() =>
            {
                let output=field(expr,schema)?;
                let DataType::List(item)=output.data_type() else {
                    return Err(refuse("array aggregation",&output,&output));
                };
                for input in &function.params.args {
                    compatible(&field(input,schema)?,item,"array aggregation")?;
                }
                // As with make_array, native aggregation preserves nested child Fields;
                // a lost top-level scalar annotation still refuses before coercion.
            }
            Expr::ScalarFunction(function)
                if function.func.inner().downcast_ref::<datafusion::functions_nested::except::ArrayExcept>().is_some()
                    || function.func.inner().downcast_ref::<datafusion::functions_nested::concat::ArrayConcat>().is_some() =>
            {
                // Native set subtraction and concatenation keep nested record Fields.
                // Check every input against the output: scalar item metadata that the
                // pinned concat return-type calculation loses still refuses here.
                let output = field(expr, schema)?;
                for arg in &function.args {
                    if !matches!(arg, Expr::Literal(datafusion::common::ScalarValue::Null, _)) {
                        compatible(&field(arg, schema)?, &output, "collection composition")?;
                    }
                }
            }
            Expr::ScalarFunction(function)
                if function.func.inner().downcast_ref::<datafusion::functions_nested::array_has::ArrayHasAny>().is_some() =>
            {
                pair(&function.args[0], &function.args[1], schema, "collection overlap")?;
                compatible(&Field::new("predicate", DataType::Boolean, true), &field(expr, schema)?, "collection overlap")?;
            }
            Expr::ScalarFunction(function)
                if function.func.inner().downcast_ref::<datafusion::functions_nested::length::ArrayLength>().is_some()
                    || function.func.inner().downcast_ref::<datafusion::functions_nested::cardinality::Cardinality>().is_some()
                    || function.func.inner().downcast_ref::<datafusion::functions_nested::empty::ArrayEmpty>().is_some()
                    || function.func.inner().downcast_ref::<datafusion::functions::core::union_tag::UnionTagFunc>().is_some() =>
            {
                // Native collection cardinality observes structure, not element values.
                // Match concrete implementations so a same-named UDF cannot erase domains.
                field(expr, schema)?;
            }
            Expr::AggregateFunction(function)
                if function
                    .func
                    .inner()
                    .downcast_ref::<datafusion::functions_aggregate::first_last::FirstValue>()
                    .is_some()
                    || function
                        .func
                        .inner()
                        .downcast_ref::<datafusion::functions_aggregate::first_last::LastValue>()
                        .is_some() =>
            {
                // These pinned native aggregates preserve their complete argument Field.
                // Verify that guarantee instead of duplicating their accumulators.
                let output = field(expr, schema)?;
                for arg in &function.params.args {
                    compatible(&field(arg, schema)?, &output, "value selection aggregate")?;
                }
            }
            Expr::WindowFunction(function)
                if matches!(&function.fun,
                    datafusion::logical_expr::WindowFunctionDefinition::WindowUDF(native)
                    if native.inner().downcast_ref::<datafusion::functions_window::row_number::RowNumber>().is_some()) =>
            {
                // Ordering and partition expressions retain their own domains; row_number
                // emits an ordinal and does not reinterpret any partition/order value.
            }
            Expr::HigherOrderFunction(function)
                if (function.func.inner().as_ref() as &dyn std::any::Any)
                    .is::<datafusion::functions_nested::array_any_match::ArrayAnyMatch>() =>
            {
                // These native predicates bind each variable to the complete list item
                // Field. Their boolean result observes the predicate, not the item domain.
                // Continue into the lambda so every comparison still proves its operands.
                let output = field(expr, schema)?;
                compatible(&Field::new("predicate", DataType::Boolean, true), &output, "collection predicate")?;
            }
            Expr::HigherOrderFunction(function)
                if (function.func.inner().as_ref() as &dyn std::any::Any)
                    .is::<datafusion::functions_nested::array_filter::ArrayFilter>() =>
            {
                // Filtering retains the item Field; it cannot silently reinterpret it.
                compatible(&field(&function.args[0], schema)?, &field(expr, schema)?, "collection filter")?;
            }
            Expr::Lambda(_) => {
                // A lambda is a binder, not a Null-valued conversion of its body.
                // Its body and resolved variables are visited below in the same tree.
            }
            Expr::LambdaVariable(variable) => {
                if variable.field.is_none() {
                    return datafusion::common::plan_err!("unresolved semantic lambda variable");
                }
            }
            Expr::Column(_)
            | Expr::Literal(_, _)
            | Expr::ScalarVariable(_, _)
            | Expr::OuterReferenceColumn(_, _)
            | Expr::IsNull(_)
            | Expr::IsNotNull(_)
            | Expr::Not(_)
            | Expr::IsTrue(_)
            | Expr::IsFalse(_)
            | Expr::IsUnknown(_)
            | Expr::IsNotTrue(_)
            | Expr::IsNotFalse(_)
            | Expr::IsNotUnknown(_) => {}
            _ => {
                // Unproven operator derivation is a refusal, not implicit erasure. A
                // supported native expression must expose a field contract first.
                expr.apply_children(|child| {
                    let input = field(child, schema)?;
                    if has_semantics(&input) {
                        return Err(refuse(
                            "undeclared semantic operator",
                            &input,
                            &field(expr, schema)?,
                        ));
                    }
                    Ok(TreeNodeRecursion::Continue)
                })?;
            }
        }
        Ok(TreeNodeRecursion::Continue)
    })?;
    Ok(())
}

/// Inspect native plans before function rewrites, and again at the analyzer entry point.
/// # Errors
/// Incompatible domains, lost annotations and unproven semantic operations refuse planning.
pub fn validate_plan(plan: &LogicalPlan) -> Result<()> {
    plan.apply(|plan| {
        // Bound the complete field tree before expression inference or schema cloning.
        validate_derived_fields(plan.schema().fields())?;
        for input in plan.inputs() {
            validate_derived_fields(input.schema().fields())?;
        }
        let mut schema = merge_schema(&plan.inputs());
        if let LogicalPlan::TableScan(scan) = plan {
            validate_derived_fields(scan.source.schema().fields())?;
            schema.merge(&DFSchema::try_from_qualified_schema(
                scan.table_name.clone(),
                &scan.source.schema(),
            )?);
        }
        for expr in plan.expressions() {
            expression(&expr, &schema)?;
        }
        if let LogicalPlan::Join(join) = plan {
            for (left, right) in &join.on {
                pair(left, right, &schema, "JOIN key")?;
            }
        }
        if let LogicalPlan::Union(union) = plan {
            for input in &union.inputs {
                if input.schema().fields().len() != union.schema.fields().len() {
                    return datafusion::common::plan_err!("UNION field count changed");
                }
                for (input, output) in input.schema().fields().iter().zip(union.schema.fields()) {
                    compatible(input, output, "UNION")?;
                }
            }
        }
        // Derived schemas can contain identically named columns in different qualified
        // relations. Admission of a source declaration and validation of derived extension
        // meaning are distinct: do not reject a legal join or require projected-away siblings.
        Ok(TreeNodeRecursion::Continue)
    })?;
    Ok(())
}

/// Derived schemas may have repeated names after joins, but every nested annotation must
/// remain known and bounded. Shared by logical planning and every physical operator schema.
pub fn validate_derived_fields(fields: &arrow::datatypes::Fields) -> Result<()> {
    crate::native_schema::validate_derived_fields(fields)
}

#[derive(Debug)]
pub struct SemanticAnalyzer;
impl AnalyzerRule for SemanticAnalyzer {
    fn name(&self) -> &str {
        "enrichment_semantic_fields"
    }
    fn analyze(&self, plan: LogicalPlan, _: &ConfigOptions) -> Result<LogicalPlan> {
        // Native rewrites can change nested field metadata and nullability. Rebind
        // lambda variables from their actual arguments before semantic admission.
        let plan = plan.resolve_lambda_variables()?.data;
        validate_plan(&plan)?;
        Ok(plan)
    }
}

/// The same rule after DataFusion's coercion and function rewrites. This is installed in
/// SessionState, so native Delta builders and direct DataFrame planning share admission.
#[derive(Debug)]
pub struct SemanticAfterAnalysis;
impl AnalyzerRule for SemanticAfterAnalysis {
    fn name(&self) -> &str {
        "enrichment_semantic_analyzed_fields"
    }
    fn analyze(&self, plan: LogicalPlan, options: &ConfigOptions) -> Result<LogicalPlan> {
        SemanticAnalyzer.analyze(plan, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn containers(child: Field) -> Vec<DataType> {
        let child = Arc::new(child);
        vec![
            DataType::Struct(vec![child.clone()].into()),
            DataType::List(child.clone()),
            DataType::LargeList(child.clone()),
            DataType::ListView(child.clone()),
            DataType::LargeListView(child.clone()),
            DataType::FixedSizeList(child.clone(), 2),
            DataType::Dictionary(
                Box::new(DataType::Int8),
                Box::new(DataType::Struct(vec![child.clone()].into())),
            ),
            DataType::RunEndEncoded(
                Arc::new(Field::new("ends", DataType::Int32, false)),
                child.clone(),
            ),
            DataType::Union(
                arrow::datatypes::UnionFields::try_new([3], [child]).unwrap(),
                arrow::datatypes::UnionMode::Dense,
            ),
        ]
    }

    #[test]
    fn semantic_container_matrix_allows_null_widening_and_rejects_domain_erasure() -> Result<()> {
        let original = crate::native_union::field::<crate::identity::SnapshotId>(
            "identity",
            crate::native_union::Rule::Text,
        );
        let other = crate::native_union::field::<crate::identity::EnvironmentId>(
            "identity",
            crate::native_union::Rule::Text,
        );
        let plain = Field::new("identity", DataType::FixedSizeBinary(32), true);
        for (((original, nullable), other), plain) in containers(original.clone())
            .into_iter()
            .zip(containers(original.with_nullable(true)))
            .zip(containers(other))
            .zip(containers(plain))
        {
            let field = Field::new("values", original, false);
            compatible(&field, &Field::new("renamed", nullable, true), "outer join")?;
            assert!(compatible(&field, &Field::new("values", other, false), "UNION").is_err());
            assert!(compatible(&field, &Field::new("values", plain, false), "CAST").is_err());
        }
        Ok(())
    }

    #[test]
    fn semantic_container_union_discriminants_cannot_be_reassigned() {
        let child = crate::native_union::field::<crate::identity::SnapshotId>(
            "identity",
            crate::native_union::Rule::Text,
        );
        let field = |tag| {
            Field::new(
                "choice",
                DataType::Union(
                    arrow::datatypes::UnionFields::try_new([tag], [child.clone()]).unwrap(),
                    arrow::datatypes::UnionMode::Dense,
                ),
                true,
            )
        };
        assert!(compatible(&field(3), &field(4), "CASE").is_err());
    }
}
