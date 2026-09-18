use std::sync::Arc;

use arrow_array::types::Int8Type;
use arrow_array::{Array, ArrayRef, DictionaryArray, RecordBatch};
use datafusion::arrow::ipc::reader::StreamReader;
use datafusion::arrow::ipc::writer::StreamWriter;

#[test]
fn ipc_dictionary_roundtrip_keeps_representation() {
    let dictionary: DictionaryArray<Int8Type> = [Some("b"), None, Some("a"), Some("b")]
        .into_iter()
        .collect();
    let original_type = dictionary.data_type().clone();
    let original_data = dictionary.to_data();
    let batch =
        RecordBatch::try_from_iter(vec![("key", Arc::new(dictionary) as ArrayRef)]).unwrap();
    let mut writer = StreamWriter::try_new(Vec::new(), &batch.schema()).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();
    let bytes = writer.into_inner().unwrap();
    let mut reader = StreamReader::try_new(std::io::Cursor::new(bytes), None).unwrap();
    let decoded = reader.next().unwrap().unwrap();
    assert_eq!(decoded.schema(), batch.schema());
    assert_eq!(decoded.column(0).data_type(), &original_type);
    assert_eq!(decoded.column(0).to_data(), original_data);
    assert!(reader.next().is_none());
}
