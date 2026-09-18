use std::hint::black_box;
use std::time::Instant;

use arrow_array::{BooleanArray, Int32Array};
use arrow_select::filter::{FilterBuilder, filter};
use serde_json::json;

fn main() {
    let mut samples = Vec::new();
    for n in [1024, 65536] {
        let values = Int32Array::from_iter_values(0..n);
        let mask = BooleanArray::from_iter(
            (0..n).map(|i| if i % 7 == 0 { None } else { Some(i % 3 == 0) }),
        );
        for reuse in [1, 20] {
            for repetition in 0..10 {
                let t = Instant::now();
                let prepared = FilterBuilder::new(&mask).optimize().build();
                let setup_ns = t.elapsed().as_nanos();
                assert_eq!(
                    filter(&values, &mask).unwrap().to_data(),
                    prepared.filter(&values).unwrap().to_data()
                );
                let t = Instant::now();
                for _ in 0..reuse {
                    black_box(prepared.filter(black_box(&values)).unwrap());
                }
                let reused_ns = t.elapsed().as_nanos();
                let t = Instant::now();
                for _ in 0..reuse {
                    black_box(filter(black_box(&values), &mask).unwrap());
                }
                let direct_ns = t.elapsed().as_nanos();
                samples.push(json!({"rows":n,"reuse":reuse,"repetition":repetition,"setup_ns":setup_ns,"reused_apply_ns":reused_ns,"direct_total_ns":direct_ns}));
            }
        }
    }
    println!("{}", serde_json::to_string_pretty(&json!({"debug_assertions":cfg!(debug_assertions), "scope":"Exploratory local timing, fixed method order and warm caches; not an optimized throughput benchmark or a universal crossover threshold.","samples":samples})).unwrap());
}
