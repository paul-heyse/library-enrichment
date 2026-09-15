use super::cells::{column, record_list, structure, text};
use arrow::{
    array::{ArrayRef, UInt32Array},
    error::ArrowError,
};
use enrichment_core::search::Factor;
use std::sync::Arc;

pub(crate) fn result(rows: &[Option<(u32, Vec<Factor>)>]) -> Result<ArrayRef, ArrowError> {
    let factors: Vec<_> = rows
        .iter()
        .flat_map(|r| {
            r.as_ref()
                .into_iter()
                .flat_map(|(_, factors)| factors.iter())
        })
        .collect();
    let values = structure(
        vec![
            column(
                "name",
                text(factors.iter().map(|f| f.name)),
                false,
                "vocabulary:search-factor/2",
            ),
            column(
                "points",
                Arc::new(UInt32Array::from_iter_values(
                    factors.iter().map(|f| f.points),
                )),
                false,
                "search-factor-points",
            ),
        ],
        None,
    )?;
    structure(
        vec![
            column(
                "score",
                Arc::new(UInt32Array::from_iter_values(
                    rows.iter()
                        .map(|r| r.as_ref().map_or(0, |(score, _)| *score)),
                )),
                false,
                "lexical-score/2",
            ),
            column(
                "factors",
                record_list(
                    rows.iter()
                        .map(|r| r.as_ref().map_or(0, |(_, factors)| factors.len())),
                    values,
                )?,
                false,
                "ordered:score-factors",
            ),
        ],
        Some(rows.iter().map(Option::is_some).collect()),
    )
}
