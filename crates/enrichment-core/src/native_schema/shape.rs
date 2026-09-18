//! Bounded, borrowed traversal of every Arrow container before semantic recursion.
//! A derived join may repeat top-level names; nested records always have one field owner.
use super::{MAX_DEPTH, MAX_FIELDS, MAX_METADATA_BYTES};
use arrow::datatypes::{DataType, Field, Fields};
use datafusion::common::{DataFusionError, Result};
use std::collections::HashSet;

#[derive(Default)]
pub(super) struct Budget {
    fields: usize,
    bytes: usize,
}
impl Budget {
    pub(super) fn metadata<'a>(
        &mut self,
        pairs: impl Iterator<Item = (&'a String, &'a String)>,
    ) -> Result<()> {
        for (key, value) in pairs {
            self.bytes(key.len())?;
            self.bytes(value.len())?;
        }
        Ok(())
    }
    fn bytes(&mut self, bytes: usize) -> Result<()> {
        self.bytes = self.bytes.checked_add(bytes).ok_or_else(exhausted)?;
        if self.bytes > MAX_METADATA_BYTES {
            return Err(exhausted());
        }
        Ok(())
    }
    pub(super) fn fields(&mut self, fields: &Fields, duplicates: bool) -> Result<()> {
        self.record(fields, 0, duplicates)
    }
    fn record(&mut self, fields: &Fields, depth: usize, duplicates: bool) -> Result<()> {
        // Refuse an oversized width before allocating a name set or visiting children.
        if fields.len() > MAX_FIELDS.saturating_sub(self.fields) {
            return Err(exhausted());
        }
        let mut names = HashSet::new();
        for field in fields {
            if !duplicates && !names.insert(field.name()) {
                return datafusion::common::plan_err!("duplicate Arrow record field");
            }
            self.field(field, depth)?;
        }
        Ok(())
    }
    fn field(&mut self, field: &Field, depth: usize) -> Result<()> {
        self.fields += 1;
        if self.fields > MAX_FIELDS || depth > MAX_DEPTH {
            return Err(exhausted());
        }
        if field.name().is_empty() {
            return datafusion::common::plan_err!("empty Arrow field name");
        }
        self.bytes(field.name().len())?;
        self.metadata(field.metadata().iter())?;
        crate::native_types::validate_field(field)?;
        self.kind(field.data_type(), depth)
    }
    fn kind(&mut self, kind: &DataType, depth: usize) -> Result<()> {
        if depth > MAX_DEPTH {
            return Err(exhausted());
        }
        match kind {
            DataType::Struct(fields) => self.record(fields, depth + 1, false)?,
            DataType::List(item)
            | DataType::LargeList(item)
            | DataType::ListView(item)
            | DataType::LargeListView(item) => self.field(item, depth + 1)?,
            DataType::FixedSizeList(item, width) if *width >= 0 => {
                self.field(item, depth + 1)?;
            }
            DataType::Map(entries, _) => {
                let DataType::Struct(fields) = entries.data_type() else {
                    return datafusion::common::plan_err!("Arrow Map entries require Struct");
                };
                if entries.is_nullable() || fields.len() != 2 || fields[0].is_nullable() {
                    return datafusion::common::plan_err!(
                        "Arrow Map requires entries and non-null keys"
                    );
                }
                self.field(entries, depth + 1)?;
            }
            DataType::Union(fields, _) => {
                if fields.len() > MAX_FIELDS.saturating_sub(self.fields) {
                    return Err(exhausted());
                }
                let mut names = HashSet::new();
                for (_, field) in fields.iter() {
                    if !names.insert(field.name()) {
                        return datafusion::common::plan_err!("duplicate Arrow Union field");
                    }
                    self.field(field, depth + 1)?;
                }
            }
            DataType::Dictionary(key, value) => {
                if !key.is_dictionary_key_type() {
                    return datafusion::common::plan_err!("Arrow Dictionary requires integer keys");
                }
                // DataType cloning is recursive; inspect the borrowed value instead.
                self.kind(value, depth + 1)?;
            }
            DataType::RunEndEncoded(ends, values) => {
                if ends.is_nullable() || !ends.data_type().is_run_ends_type() {
                    return datafusion::common::plan_err!(
                        "Arrow run ends require non-null signed integer values"
                    );
                }
                self.field(ends, depth + 1)?;
                self.field(values, depth + 1)?;
            }
            DataType::Timestamp(_, Some(timezone)) => self.bytes(timezone.len())?,
            DataType::FixedSizeBinary(width) if *width < 0 => {
                return datafusion::common::plan_err!("negative Arrow binary width");
            }
            DataType::FixedSizeList(_, _) => {
                return datafusion::common::plan_err!("negative Arrow list width");
            }
            _ => {}
        }
        Ok(())
    }
}
fn exhausted() -> DataFusionError {
    DataFusionError::ResourcesExhausted(
        "Arrow field tree exceeds native depth/field/metadata bounds".into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::datatypes::{Schema, UnionFields, UnionMode};
    use std::{collections::HashMap, sync::Arc};

    #[test]
    fn every_arrow_container_checks_semantics_and_bounded_shape() {
        let unknown = Arc::new(Field::new("value", DataType::Int64, false).with_metadata(
            HashMap::from([("ARROW:extension:name".into(), "not_registered".into())]),
        ));
        let ends = Arc::new(Field::new("ends", DataType::Int32, false));
        for kind in [
            DataType::ListView(unknown.clone()),
            DataType::LargeListView(unknown.clone()),
            DataType::RunEndEncoded(ends, unknown.clone()),
            DataType::Union(
                UnionFields::try_new(vec![0], vec![unknown.clone()]).unwrap(),
                UnionMode::Dense,
            ),
            DataType::Dictionary(
                Box::new(DataType::UInt8),
                Box::new(DataType::Struct(vec![unknown].into())),
            ),
        ] {
            let fields: Fields = vec![Field::new("container", kind, true)].into();
            assert!(super::super::validate(&Schema::new(fields.clone())).is_err());
            assert!(crate::native_analysis::validate_derived_fields(&fields).is_err());
        }
        for kind in [
            DataType::Map(
                Arc::new(Field::new("entries", DataType::Int64, false)),
                false,
            ),
            DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Int64, true)), -1),
            DataType::Dictionary(Box::new(DataType::Utf8), Box::new(DataType::Int64)),
        ] {
            assert!(
                Budget::default()
                    .fields(&vec![Field::new("bad", kind, true)].into(), true)
                    .is_err()
            );
        }
    }

    #[test]
    fn nested_semantic_changes_are_captured_by_the_contract_manifest() {
        use crate::{
            native_contract::Manifest,
            native_types::{ClockMeaning, ClockType, TypeMetadata},
        };
        use arrow::datatypes::TimeUnit;
        use arrow_schema::extension::ExtensionType;
        let kind = DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into()));
        let clock = |meaning| {
            Arc::new(
                Field::new("clock", kind.clone(), false).with_extension_type(
                    ClockType::try_new(&kind, TypeMetadata::new(meaning)).unwrap(),
                ),
            )
        };
        let containers = |field: Arc<Field>| {
            vec![
                DataType::ListView(field.clone()),
                DataType::LargeListView(field.clone()),
                DataType::Union(
                    UnionFields::try_new(vec![3], vec![field.clone()]).unwrap(),
                    UnionMode::Sparse,
                ),
                DataType::RunEndEncoded(
                    Arc::new(Field::new("ends", DataType::Int32, false)),
                    field.clone(),
                ),
                DataType::Dictionary(
                    Box::new(DataType::UInt8),
                    Box::new(DataType::Struct(vec![field].into())),
                ),
            ]
        };
        for (before, after) in containers(clock(ClockMeaning::Event))
            .into_iter()
            .zip(containers(clock(ClockMeaning::Expiry)))
        {
            let schema = |kind| Schema::new(vec![Field::new("container", kind, false)]);
            let before = schema(before);
            let after = schema(after);
            assert!(crate::native_analysis::has_semantics(before.field(0)));
            assert!(
                crate::native_analysis::compatible(before.field(0), after.field(0), "fixture")
                    .is_err()
            );
            let before = Manifest::new(&before, &before).unwrap();
            let after = Manifest::new(&after, &after).unwrap();
            assert_ne!(before.identity().unwrap(), after.identity().unwrap());
            for projection in ["semantic", "storage"] {
                let row = |manifest: &Manifest| {
                    manifest
                        .fields
                        .iter()
                        .find(|row| {
                            row.projection == projection
                                && row.path.last().is_some_and(|name| name == "clock")
                        })
                        .unwrap()
                        .properties
                        .clone()
                };
                assert_ne!(row(&before), row(&after));
            }
        }
    }

    #[test]
    fn derived_aliases_allow_repeated_top_names_but_keep_shared_budgets() {
        let repeated: Fields = vec![Field::new("id", DataType::Int64, true); 2].into();
        assert!(Budget::default().fields(&repeated, true).is_ok());
        assert!(Budget::default().fields(&repeated, false).is_err());
        let nested: Fields = vec![Field::new("nested", DataType::Struct(repeated), true)].into();
        assert!(Budget::default().fields(&nested, true).is_err());
        let large: Fields =
            vec![
                Field::new("large", DataType::Int64, true).with_metadata(HashMap::from([(
                    "opaque".into(),
                    "x".repeat(MAX_METADATA_BYTES),
                )])),
            ]
            .into();
        assert!(crate::native_analysis::validate_derived_fields(&large).is_err());
        let mut deep = DataType::Int64;
        for _ in 0..=MAX_DEPTH {
            deep = DataType::Dictionary(Box::new(DataType::Int8), Box::new(deep));
        }
        assert!(
            Budget::default()
                .fields(&vec![Field::new("deep", deep, true)].into(), true)
                .is_err()
        );
        assert!(
            Budget::default()
                .fields(
                    &vec![Field::new("id", DataType::Int64, true); MAX_FIELDS + 1].into(),
                    true
                )
                .is_err()
        );
    }
}
