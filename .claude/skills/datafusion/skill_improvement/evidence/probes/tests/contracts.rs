use std::collections::HashMap;
use std::sync::Arc;

use arrow_array::cast::AsArray;
use arrow_array::types::{Int8Type, Int32Type};
use arrow_array::{
    Array, ArrayRef, BooleanArray, DictionaryArray, Int32Array, RecordBatch, RecordBatchOptions,
    StringArray, UInt32Array,
};
use arrow_cast::{CastOptions, cast_with_options};
use arrow_row::{RowConverter, SortField};
use arrow_schema::{DataType, Field, Schema, SortOptions};
use arrow_select::filter::{FilterBuilder, filter_record_batch};
use arrow_select::take::{TakeOptions, take, take_record_batch};

#[test]
fn filter_null_is_not_selected_and_metadata_is_retained() {
    let metadata = HashMap::from([("domain".to_owned(), "example".to_owned())]);
    let field = Field::new("x", DataType::Int32, true).with_metadata(metadata.clone());
    let schema = Arc::new(Schema::new_with_metadata(vec![field], metadata));
    let batch = RecordBatch::try_new(
        Arc::clone(&schema),
        vec![Arc::new(Int32Array::from(vec![Some(10), Some(20), None]))],
    )
    .unwrap();
    let predicate = BooleanArray::from(vec![Some(true), None, Some(true)]);
    let selected = filter_record_batch(&batch, &predicate).unwrap();
    assert_eq!(selected.schema(), schema);
    assert_eq!(
        selected.column(0).as_primitive::<Int32Type>(),
        &Int32Array::from(vec![Some(10), None])
    );
    let reused = FilterBuilder::new(&predicate).optimize().build();
    assert_eq!(selected, reused.filter_record_batch(&batch).unwrap());
    let keep_middle = BooleanArray::from(vec![Some(true), Some(true), Some(true)]);
    assert_eq!(
        filter_record_batch(&batch, &keep_middle)
            .unwrap()
            .num_rows(),
        3
    );
}

#[test]
fn take_preserves_requested_order_duplicates_and_null_indices() {
    let values = Int32Array::from(vec![10, 20, 30]);
    let indices = UInt32Array::from(vec![Some(2), Some(0), Some(2), None]);
    let selected = take(&values, &indices, Some(TakeOptions { check_bounds: true })).unwrap();
    assert_eq!(
        selected.as_primitive::<Int32Type>(),
        &Int32Array::from(vec![Some(30), Some(10), Some(30), None])
    );
    let out_of_range = UInt32Array::from(vec![3]);
    assert!(
        take(
            &values,
            &out_of_range,
            Some(TakeOptions { check_bounds: true })
        )
        .is_err()
    );
}

#[test]
fn cast_safe_true_nulls_failed_parses_but_false_errors() {
    let input = StringArray::from(vec![Some("42"), Some("bad"), None]);
    let null_on_failure =
        cast_with_options(&input, &DataType::Int32, &CastOptions::default()).unwrap();
    assert_eq!(
        null_on_failure.as_primitive::<Int32Type>(),
        &Int32Array::from(vec![Some(42), None, None])
    );
    let strict = CastOptions {
        safe: false,
        ..Default::default()
    };
    assert!(cast_with_options(&input, &DataType::Int32, &strict).is_err());
    assert!(cast_with_options(&StringArray::from(vec!["42"]), &DataType::Int32, &strict).is_ok());
}

#[test]
fn row_order_changes_with_null_placement() {
    let arrays = vec![Arc::new(Int32Array::from(vec![Some(2), None, Some(1)])) as ArrayRef];
    let sorted = |nulls_first| {
        let converter = RowConverter::new(vec![SortField::new_with_options(
            DataType::Int32,
            SortOptions {
                descending: false,
                nulls_first,
            },
        )])
        .unwrap();
        let rows = converter.convert_columns(&arrays).unwrap();
        let mut positions: Vec<_> = (0..rows.num_rows()).collect();
        positions.sort_unstable_by(|a, b| rows.row(*a).cmp(&rows.row(*b)));
        positions
    };
    assert_eq!(sorted(true), vec![1, 2, 0]);
    assert_eq!(sorted(false), vec![2, 0, 1]);
}

#[test]
fn row_decode_hydrates_dictionary_and_preserves_values() {
    let dictionary: DictionaryArray<Int8Type> = ["a", "b", "a"].into_iter().collect();
    let converter =
        RowConverter::new(vec![SortField::new(dictionary.data_type().clone())]).unwrap();
    let rows = converter.convert_columns(&[Arc::new(dictionary)]).unwrap();
    assert_eq!(rows.row(0), rows.row(2));
    assert_ne!(rows.row(0), rows.row(1));
    let decoded = converter.convert_rows(&rows).unwrap();
    assert_eq!(decoded[0].data_type(), &DataType::Utf8);
    assert_eq!(
        decoded[0].as_string::<i32>(),
        &StringArray::from(vec!["a", "b", "a"])
    );
}

#[test]
fn empty_schema_exposes_different_batch_selection_contracts() {
    let batch = RecordBatch::try_new_with_options(
        Arc::new(Schema::empty()),
        vec![],
        &RecordBatchOptions::new().with_row_count(Some(3)),
    )
    .unwrap();
    let filtered =
        filter_record_batch(&batch, &BooleanArray::from(vec![true, false, true])).unwrap();
    assert_eq!(filtered.num_rows(), 2);
    assert_eq!(filtered.num_columns(), 0);
    // In this release take_record_batch reconstructs with try_new, losing the explicit row count.
    assert!(take_record_batch(&batch, &UInt32Array::from(vec![0, 2])).is_err());
    let ordinary = RecordBatch::try_from_iter(vec![(
        "x",
        Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef,
    )])
    .unwrap();
    assert_eq!(
        take_record_batch(&ordinary, &UInt32Array::from(vec![0, 2]))
            .unwrap()
            .num_rows(),
        2
    );
}

#[test]
fn batch_construction_checks_top_level_nullability() {
    let values = Arc::new(Int32Array::from(vec![Some(1), None])) as ArrayRef;
    let strict = Arc::new(Schema::new(vec![Field::new("x", DataType::Int32, false)]));
    let nullable = Arc::new(Schema::new(vec![Field::new("x", DataType::Int32, true)]));
    assert!(RecordBatch::try_new(strict, vec![Arc::clone(&values)]).is_err());
    assert!(RecordBatch::try_new(nullable, vec![values]).is_ok());
}
