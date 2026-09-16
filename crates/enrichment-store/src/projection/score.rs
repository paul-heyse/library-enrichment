//! Ranking schema; values are produced by native expressions in `scoring`.
use arrow::datatypes::{DataType, Field, FieldRef};
use std::{collections::HashMap, sync::Arc};
fn field_with_role(name: &str, kind: DataType, role: &str) -> Field {
    Field::new(name, kind, true).with_metadata(HashMap::from([
        ("enrichment.role".into(), role.into()),
        ("enrichment.contract".into(), super::VERSION.into()),
        ("enrichment.null".into(), "unobserved".into()),
    ]))
}
pub(crate) fn factor_type() -> DataType {
    DataType::Struct(
        vec![
            field_with_role("name", DataType::Utf8, "vocabulary:search-factor/2"),
            field_with_role("points", DataType::UInt32, "search-factor-points"),
        ]
        .into(),
    )
}
pub(crate) fn field() -> FieldRef {
    Arc::new(Field::new(
        "ranking",
        DataType::Struct(
            vec![
                field_with_role("score", DataType::UInt32, "lexical-score/2"),
                field_with_role(
                    "factors",
                    DataType::List(Arc::new(Field::new("item", factor_type(), true))),
                    "ordered:score-factors",
                ),
            ]
            .into(),
        ),
        true,
    ))
}
