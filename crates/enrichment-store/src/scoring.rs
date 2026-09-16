//! Optimizer-visible lexical ranking. One native factor list supplies scores and explanations.

use arrow::datatypes::{DataType, Field, FieldRef};
use datafusion::{
    common::ScalarValue,
    error::{DataFusionError, Result},
    functions::{
        core::{
            expr_ext::FieldAccessor,
            expr_fn::{coalesce, least, named_struct},
        },
        string::expr_fn::{concat, ends_with, lower, octet_length, starts_with, trim},
        unicode::expr_fn::strpos,
    },
    functions_nested::expr_fn::{
        array_compact, array_distinct, array_filter, array_has, array_sum, array_transform,
        make_array, string_to_array,
    },
    logical_expr::{
        Expr,
        expr::Cast,
        expr_fn::{lambda, lambda_var, when},
    },
    prelude::{col, lit},
};
use enrichment_core::search::{FACTORS, spec::SearchSpec};
use std::{ops::Not, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScoreKind {
    Symbol,
    Fragment,
}

fn null(kind: &DataType) -> Result<Expr> {
    Ok(lit(ScalarValue::try_from(kind)?))
}
fn typed(expr: Expr, field: FieldRef) -> Expr {
    Expr::Cast(Cast::new_from_field(Box::new(expr), field))
}
fn any(expressions: impl IntoIterator<Item = Expr>) -> Expr {
    expressions
        .into_iter()
        .reduce(Expr::or)
        .unwrap_or_else(|| lit(false))
}
fn present(expr: Expr) -> Expr {
    coalesce(vec![expr, lit(false)])
}
fn contains(value: Expr, token: Expr) -> Expr {
    present(strpos(value, token).gt(lit(0i64)))
}
fn query_text(spec: &SearchSpec) -> Expr {
    lower(trim(vec![lit(spec.query.as_str())]))
}
fn token_var() -> Expr {
    // The native string_to_array input is a Utf8 literal. Bind its nullable element field
    // before composing nested higher-order expressions; the native resolver validates/rebinds
    // it against the actual argument when the complete plan is prepared.
    Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
        "token".into(),
        Some(Arc::new(Field::new("token", DataType::Utf8, true))),
    ))
}
fn tokens_expr(spec: &SearchSpec) -> Expr {
    let words = datafusion::functions::regex::expr_fn::regexp_replace(
        query_text(spec),
        lit(r"[^\p{Alphabetic}\p{Number}_]+"),
        lit(" "),
        Some(lit("g")),
    );
    array_distinct(array_filter(
        string_to_array(words, lit(" "), lit(ScalarValue::Utf8(None))),
        lambda(vec!["token"], octet_length(token_var()).gt_eq(lit(2_i32))),
    ))
}
fn any_token(tokens: Expr, predicate: impl FnOnce(Expr) -> Expr) -> Expr {
    array_has(
        array_transform(tokens, lambda(vec!["token"], predicate(token_var()))),
        lit(true),
    )
}

/// Decode only the bounded query token list at the transport boundary.
pub async fn tokens(runtime: &crate::runtime::QueryRuntime, query: &str) -> Result<Vec<String>> {
    use arrow::array::{Array, ListArray};
    let session = runtime.session();
    let frame = session
        .sql("SELECT 1")
        .await?
        .select(vec![tokens_expr(&SearchSpec::new(query)).alias("tokens")])?;
    let result = runtime.execute(frame).await?;
    let array = result.batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<ListArray>()
        .ok_or_else(|| DataFusionError::Internal("native tokens are not an Arrow list".into()))?
        .value(0);
    if array.len() > 64 {
        return Err(DataFusionError::Plan(
            "search query exceeds 64 terms".into(),
        ));
    }
    let values = crate::projection::TextColumn::new(array.as_ref())?;
    (0..array.len())
        .map(|i| {
            values
                .get(i)
                .map(ToOwned::to_owned)
                .ok_or_else(|| DataFusionError::Internal("null native query token".into()))
        })
        .collect()
}

/// Compile a bounded request into built-in expressions, including higher-order list reduction.
/// No product UDF computes a row score; changing a factor changes its explanation and sum together.
/// # Errors
/// Incorrect argument counts or invalid typed expression construction are rejected.
pub fn ranking(kind: ScoreKind, spec: &SearchSpec, args: Vec<Expr>) -> Result<Expr> {
    let expected = if kind == ScoreKind::Symbol { 6 } else { 2 };
    if args.len() != expected {
        return Err(DataFusionError::Plan(
            "incorrect ranking argument count".into(),
        ));
    }
    let q = query_text(spec);
    let nonempty = q.clone().not_eq(lit(""));
    let tokens = tokens_expr(spec);
    let mut rules: Vec<(&str, Expr, Expr)> = Vec::new();
    let weight = |name: &str| -> Result<Expr> {
        FACTORS
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, p)| lit(*p))
            .ok_or_else(|| DataFusionError::Plan(format!("unknown ranking factor {name}")))
    };
    match kind {
        ScoreKind::Symbol => {
            let text: Vec<_> = args[..5].iter().cloned().map(lower).collect();
            let exact = present(text[0].clone().eq(q.clone())).and(nonempty.clone());
            let suffix = present(
                ends_with(text[0].clone(), concat(vec![lit("::"), q.clone()])).or(ends_with(
                    text[0].clone(),
                    concat(vec![lit("."), q.clone()]),
                )),
            )
            .and(nonempty.clone())
            .and(exact.clone().not());
            let name = present(text[1].clone().eq(q.clone()))
                .and(nonempty.clone())
                .and(exact.clone().not())
                .and(suffix.clone().not());
            let prefix = any_token(tokens.clone(), |token| {
                present(starts_with(text[1].clone(), token))
            })
            .and(exact.clone().or(name.clone()).not());
            for (label, condition) in [
                ("exact_path", exact),
                ("path_suffix", suffix),
                ("name_exact", name),
                ("name_prefix", prefix),
            ] {
                rules.push((label, condition, weight(label)?));
            }
            for (label, index) in [
                ("path_token", 0),
                ("signature_token", 2),
                ("summary_token", 3),
                ("docs_token", 4),
            ] {
                rules.push((
                    label,
                    any_token(tokens.clone(), |token| contains(text[index].clone(), token)),
                    weight(label)?,
                ));
            }
            let matched = any(rules.iter().map(|(_, condition, _)| condition.clone()));
            rules.push((
                "definition_path",
                matched.and(args[5].clone().not()),
                weight("definition_path")?,
            ));
        }
        ScoreKind::Fragment => {
            let subject = lower(args[0].clone());
            let text = lower(args[1].clone());
            rules.push((
                "subject_exact",
                present(subject.clone().eq(q.clone())).and(nonempty.clone()),
                weight("subject_exact")?,
            ));
            rules.push((
                "subject_token",
                any_token(tokens.clone(), |token| contains(subject.clone(), token)),
                weight("subject_token")?,
            ));
            let hits = array_sum(array_transform(
                tokens.clone(),
                lambda(
                    vec!["token"],
                    when(contains(text, token_var()), lit(1u32)).otherwise(lit(0u32))?,
                ),
            ));
            rules.push((
                "text_token",
                hits.clone().gt(lit(0u32)),
                least(vec![hits, lit(3u32)]) * weight("text_token")?,
            ));
        }
    }
    let matched = any(rules.iter().map(|(_, condition, _)| condition.clone()));
    let factor_type = crate::projection::score::factor_type();
    let factors = rules
        .into_iter()
        .map(|(name, condition, points)| {
            when(
                condition,
                typed(
                    named_struct(vec![lit("name"), lit(name), lit("points"), points]),
                    Arc::new(Field::new("factor", factor_type.clone(), true)),
                ),
            )
            .otherwise(null(&factor_type)?)
        })
        .collect::<Result<Vec<_>>>()?;
    let factors = array_compact(make_array(factors));
    let score = typed(
        array_sum(array_transform(
            factors.clone(),
            lambda(["factor"], lambda_var("factor").field("points")),
        )),
        Arc::new(Field::new("score", DataType::UInt32, true)),
    );
    let result = crate::projection::score::field();
    let value = typed(
        named_struct(vec![lit("score"), score, lit("factors"), factors]),
        result.clone(),
    );
    when(matched, value).otherwise(null(result.data_type())?)
}

/// Eligibility shares the native matching definition with scoring.
pub fn eligibility(spec: &SearchSpec) -> Result<Expr> {
    Ok(ranking(
        ScoreKind::Symbol,
        spec,
        vec![
            col("path"),
            col("name"),
            col("signature"),
            col("doc_summary"),
            col("docs"),
            lit(false),
        ],
    )?
    .is_not_null())
}
/// Fragment eligibility consumes the native domain label and exact text.
pub fn fragment_eligibility(spec: &SearchSpec) -> Result<Expr> {
    Ok(ranking(ScoreKind::Fragment, spec, vec![col("label"), col("text")])?.is_not_null())
}
