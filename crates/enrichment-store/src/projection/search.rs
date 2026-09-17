use super::{
    cells::{RowSet, invalid},
    decode,
};
use crate::search_plan::RankedEvidence;
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::{
    evidence::{FragmentKind, SymbolKind},
    search::page::SearchKey,
    wire::data::{HitKind, ScoreFactor},
};

pub(crate) fn rows(batch: &RecordBatch) -> Result<Vec<RankedEvidence>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let ranking = r.structure("ranking")?;
            let score =
                u32::try_from(ranking.number("score")?).map_err(|e| invalid(e.to_string()))?;
            let hit_order =
                u32::try_from(r.number("hit_order")?).map_err(|e| invalid(e.to_string()))?;
            let hit = match hit_order {
                0 => HitKind::Symbol,
                1 => HitKind::Fragment,
                _ => return Err(invalid("unknown search hit kind")),
            };
            let factors = ranking
                .records("factors")?
                .into_iter()
                .map(|f| {
                    Ok(ScoreFactor {
                        name: f.text("name")?.into(),
                        points: u32::try_from(f.number("points")?)
                            .map_err(|e| invalid(e.to_string()))?,
                    })
                })
                .collect::<Result<Vec<_>, ArrowError>>()?;
            Ok(RankedEvidence {
                key: SearchKey {
                    score,
                    hit_order,
                    subject: r.text("label")?.into(),
                    candidate_id: r.text("candidate_id")?.into(),
                },
                hit,
                fact_id: r.text("fact_id")?.into(),
                subject: decode::subject(r.structure("subject_ref")?)?,
                path: r.owned("path")?,
                signature: r.owned("signature")?,
                symbol_kind: r
                    .optional_text("symbol_kind")?
                    .map(|v| SymbolKind::parse(v).ok_or_else(|| invalid("unknown symbol kind")))
                    .transpose()?,
                fragment_kind: r
                    .optional_text("fragment_kind")?
                    .map(|v| FragmentKind::parse(v).ok_or_else(|| invalid("unknown fragment kind")))
                    .transpose()?,
                excerpt: r.text("excerpt")?.into(),
                also_at: r.list("also_at")?,
                deprecated: r.boolean("deprecated")?,
                factors,
                source: decode::source(r.structure("source")?)?,
            })
        })
        .collect()
}

pub(crate) fn count(batches: &[RecordBatch]) -> Result<u64, ArrowError> {
    if batches.iter().map(RecordBatch::num_rows).sum::<usize>() != 1 {
        return Err(invalid("invalid count cardinality"));
    }
    let batch = batches
        .iter()
        .find(|b| b.num_rows() == 1)
        .ok_or_else(|| invalid("missing count"))?;
    RowSet::batch(batch)?.row(0).number("count")
}
