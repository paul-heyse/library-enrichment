//! Transfer paid handoff capacity into Arrow's native shared-buffer reservations.
//! This accounts retained buffers, not a claim that an upstream decoder allocated under a cap.
use arrow::{array::ArrayData, record_batch::RecordBatch};
use datafusion::common::{Result, resources_datafusion_err};
use datafusion_execution::memory_pool::{MemoryConsumer, MemoryPool, MemoryReservation};
use std::sync::Arc;

/// Arrow's native claim API replaces each buffer's reservation and preserves that reservation
/// through its final clone/slice. Its ordinary DataFusion adapter calls infallible `grow`.
/// Prepay the entire claim traversal instead, then split that paid capacity at the native seam.
pub(crate) fn claim(batch: &RecordBatch, pool: &Arc<dyn MemoryPool>, name: &str) -> Result<()> {
    enrichment_core::native_analysis::validate_derived_fields(batch.schema().fields())?;
    let arrays: Vec<_> = batch
        .columns()
        .iter()
        .map(|array| array.to_data())
        .collect();
    let size = arrays.iter().try_fold(0usize, |size, array| {
        size.checked_add(capacity(array)?)
            .ok_or_else(|| resources_datafusion_err!("Arrow claim capacity overflow"))
    })?;
    let paid = MemoryConsumer::new(name).register(pool);
    paid.try_grow(size)?;
    let prepaid = Prepaid {
        paid,
        capacity: size,
    };
    // The capacity traversal mirrors ArrayData::claim's buffers/nulls/children exactly.
    // Repeated aliases are conservatively prepaid; replacing a prior claim releases it.
    for array in arrays {
        array.claim(&prepaid);
    }
    Ok(())
}

fn capacity(array: &ArrayData) -> Result<usize> {
    let mut size = array.buffers().iter().try_fold(0usize, |size, buffer| {
        size.checked_add(buffer.capacity())
            .ok_or_else(|| resources_datafusion_err!("Arrow buffer capacity overflow"))
    })?;
    if let Some(nulls) = array.nulls() {
        size = size
            .checked_add(nulls.buffer().capacity())
            .ok_or_else(|| resources_datafusion_err!("Arrow validity capacity overflow"))?;
    }
    for child in array.child_data() {
        size = size
            .checked_add(capacity(child)?)
            .ok_or_else(|| resources_datafusion_err!("Arrow child capacity overflow"))?;
    }
    Ok(size)
}

#[derive(Debug)]
struct Prepaid {
    paid: MemoryReservation,
    capacity: usize,
}
impl arrow_buffer::MemoryPool for Prepaid {
    fn reserve(&self, size: usize) -> Box<dyn arrow_buffer::MemoryReservation> {
        Box::new(self.paid.split(size))
    }
    fn capacity(&self) -> usize {
        self.capacity
    }
    fn used(&self) -> usize {
        self.capacity - self.paid.size()
    }
    fn available(&self) -> isize {
        isize::try_from(self.paid.size()).unwrap_or(isize::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{
        array::{ArrayRef, Int32Array, ListArray, StringArray, StructArray, types::Int32Type},
        datatypes::{DataType, Field, Schema},
    };
    use datafusion::execution::memory_pool::GreedyMemoryPool;

    #[test]
    fn native_claim_tracks_nested_aliases_reclaims_and_refuses_without_releasing_prior_owner()
    -> Result<()> {
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(1 << 20));
        let values: ArrayRef = Arc::new(StringArray::from(vec![Some("one"), None, Some("three")]));
        let list: ArrayRef = Arc::new(ListArray::from_iter_primitive::<Int32Type, _, _>([
            Some(vec![Some(1), None]),
            None,
            Some(vec![Some(3)]),
        ]));
        let fields = vec![
            Field::new("text", DataType::Utf8, true),
            Field::new("list", list.data_type().clone(), true),
        ];
        let nested: ArrayRef = Arc::new(StructArray::new(
            fields.into(),
            vec![values.clone(), list],
            None,
        ));
        let batch = RecordBatch::try_new(
            Arc::new(Schema::new(vec![
                Field::new("nested", nested.data_type().clone(), false),
                Field::new("alias", DataType::Utf8, true),
            ])),
            vec![nested, values],
        )?;
        claim(&batch, &pool, "first")?;
        let actual = pool.reserved();
        assert!(actual > 0);
        let alias = batch.slice(1, 1);
        claim(&alias, &pool, "second")?;
        assert_eq!(
            pool.reserved(),
            actual,
            "aliases replace, never accumulate charges"
        );
        let exhausted: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(1));
        assert!(claim(&batch, &exhausted, "refused").is_err());
        assert_eq!(exhausted.reserved(), 0);
        assert_eq!(
            pool.reserved(),
            actual,
            "failed admission leaves the prior owner"
        );
        drop(batch);
        assert_eq!(pool.reserved(), actual);
        drop(alias);
        assert_eq!(pool.reserved(), 0);
        let empty = RecordBatch::try_from_iter([(
            "value",
            Arc::new(Int32Array::from(Vec::<i32>::new())) as ArrayRef,
        )])?;
        claim(&empty, &exhausted, "empty")?;
        assert_eq!(exhausted.reserved(), 0);
        Ok(())
    }
}
