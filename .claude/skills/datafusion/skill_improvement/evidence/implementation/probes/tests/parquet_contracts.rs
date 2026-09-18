use std::io::{Cursor, Read};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use arrow_array::cast::AsArray;
use arrow_array::types::Int32Type;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use bytes::Bytes;
use parquet::arrow::ArrowWriter;
use parquet::arrow::arrow_reader::{ParquetRecordBatchReaderBuilder, RowSelection, RowSelector};
use parquet::file::reader::{ChunkReader, Length};

#[derive(Clone)]
struct CountedBytes {
    bytes: Bytes,
    count: Arc<AtomicUsize>,
}

struct CountedRead {
    cursor: Cursor<Bytes>,
    count: Arc<AtomicUsize>,
}

impl Read for CountedRead {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let n = self.cursor.read(buffer)?;
        self.count.fetch_add(n, Ordering::Relaxed);
        Ok(n)
    }
}

impl Length for CountedBytes {
    fn len(&self) -> u64 {
        self.bytes.len() as u64
    }
}

impl ChunkReader for CountedBytes {
    type T = CountedRead;
    fn get_read(&self, start: u64) -> parquet::errors::Result<Self::T> {
        Ok(CountedRead {
            cursor: Cursor::new(self.bytes.slice(start as usize..)),
            count: self.count.clone(),
        })
    }
    fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
        let bytes = <Bytes as ChunkReader>::get_bytes(&self.bytes, start, length)?;
        self.count.fetch_add(bytes.len(), Ordering::Relaxed);
        Ok(bytes)
    }
}

#[test]
fn parquet_selection_is_relative_to_retained_row_groups() {
    let batch = |start| {
        RecordBatch::try_from_iter(vec![(
            "x",
            Arc::new(Int32Array::from(vec![start, start + 1, start + 2])) as ArrayRef,
        )])
        .unwrap()
    };
    let mut writer = ArrowWriter::try_new(Vec::new(), batch(0).schema(), None).unwrap();
    for start in [0, 3, 6] {
        writer.write(&batch(start)).unwrap();
        writer.flush().unwrap();
    }
    let bytes = Bytes::from(writer.into_inner().unwrap());
    let read = |groups, selectors: Vec<RowSelector>| {
        let counter = Arc::new(AtomicUsize::new(0));
        let builder = ParquetRecordBatchReaderBuilder::try_new(CountedBytes {
            bytes: bytes.clone(),
            count: counter.clone(),
        })
        .unwrap();
        let metadata_bytes = counter.swap(0, Ordering::Relaxed);
        assert_eq!(builder.metadata().num_row_groups(), 3);
        let rows = builder
            .with_row_groups(groups)
            .with_row_selection(RowSelection::from(selectors))
            .build()
            .unwrap()
            .flat_map(|b| {
                b.unwrap()
                    .column(0)
                    .as_primitive::<Int32Type>()
                    .values()
                    .to_vec()
            })
            .collect::<Vec<_>>();
        println!(
            "local Parquet requests: metadata_bytes={metadata_bytes}, scan_bytes={}, output_rows={}; bytes requested from this in-memory ChunkReader, not network or decoder CPU",
            counter.load(Ordering::Relaxed),
            rows.len()
        );
        rows
    };
    let all = read(
        vec![0, 1, 2],
        vec![
            RowSelector::skip(7),
            RowSelector::select(1),
            RowSelector::skip(1),
        ],
    );
    let pruned = read(
        vec![0, 2],
        vec![
            RowSelector::skip(4),
            RowSelector::select(1),
            RowSelector::skip(1),
        ],
    );
    assert_eq!(all, vec![7]);
    assert_eq!(pruned, all);
    let different_coordinate = read(
        vec![0, 2],
        vec![
            RowSelector::skip(1),
            RowSelector::select(1),
            RowSelector::skip(4),
        ],
    );
    assert_eq!(different_coordinate, vec![1]);
}
