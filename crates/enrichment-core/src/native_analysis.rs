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
    field.metadata().contains_key("ARROW:extension:name")
        || match field.data_type() {
            DataType::Struct(fields) => fields.iter().any(|field| has_semantics(field)),
            DataType::List(field)
            | DataType::LargeList(field)
            | DataType::FixedSizeList(field, _)
            | DataType::Map(field, _) => has_semantics(field),
            DataType::Dictionary(_, value) => {
                has_semantics(&Field::new("value", value.as_ref().clone(), true))
            }
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
            DataType::List(a) | DataType::LargeList(a),
            DataType::List(b) | DataType::LargeList(b),
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
                compatible(&field(&cast.expr, schema)?, &field(expr, schema)?, "cast")?
            }
            Expr::TryCast(cast) => compatible(
                &field(&cast.expr, schema)?,
                &field(expr, schema)?,
                "try_cast",
            )?,
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
            Expr::ScalarFunction(function) if function.name() == "get_field" => {
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
                    || crate::native_predicate::is_predicate(&function.func)
                    || crate::native_collections::is_projection(&function.func)
                    || crate::native_time::is_projection(&function.func) =>
            {
                // The concrete encoder verifies its declared full input fields. Its Binary
                // output is an explicit, domain-framed representation for native hashing.
                field(expr, schema)?;
            }
            Expr::ScalarFunction(function)
                if function.name() == "named_struct"
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
            Expr::ScalarFunction(function) if function.name() == "coalesce" => {
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
        let mut schema = merge_schema(&plan.inputs());
        if let LogicalPlan::TableScan(scan) = plan {
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
                for (input, output) in input.schema().fields().iter().zip(union.schema.fields()) {
                    compatible(input, output, "UNION")?;
                }
            }
        }
        // Derived schemas can contain identically named columns in different qualified
        // relations. Admission of a source declaration and validation of derived extension
        // meaning are distinct: do not reject a legal join or require projected-away siblings.
        let mut pending: Vec<_> = plan
            .schema()
            .fields()
            .iter()
            .map(|f| (f.clone(), 0))
            .collect();
        let mut visited = 0;
        while let Some((field, depth)) = pending.pop() {
            visited += 1;
            if depth > 64 || visited > 16384 {
                return datafusion::common::plan_err!("derived semantic field traversal bound");
            }
            crate::native_types::validate_field(&field)?;
            match field.data_type() {
                DataType::Struct(children) => {
                    pending.extend(children.iter().map(|f| (f.clone(), depth + 1)))
                }
                DataType::List(item)
                | DataType::LargeList(item)
                | DataType::FixedSizeList(item, _)
                | DataType::Map(item, _) => pending.push((item.clone(), depth + 1)),
                DataType::Dictionary(_, kind) => pending.push((
                    std::sync::Arc::new(Field::new(
                        "dictionary_values",
                        kind.as_ref().clone(),
                        true,
                    )),
                    depth + 1,
                )),
                _ => {}
            }
        }
        Ok(TreeNodeRecursion::Continue)
    })?;
    Ok(())
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
