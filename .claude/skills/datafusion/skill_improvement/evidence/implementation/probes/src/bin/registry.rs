use datafusion::prelude::SessionContext;
use serde_json::json;

fn main() {
    let state = SessionContext::new().state();
    let mut rows = Vec::new();
    for (name, function) in state.scalar_functions() {
        rows.push(json!({"family":"scalar", "registered_name":name, "canonical_name":function.name(), "aliases":function.aliases(), "signature":format!("{:?}", function.signature())}));
    }
    for (name, function) in state.aggregate_functions() {
        rows.push(json!({"family":"aggregate", "registered_name":name, "canonical_name":function.name(), "aliases":function.aliases(), "signature":format!("{:?}", function.signature())}));
    }
    for (name, function) in state.window_functions() {
        rows.push(json!({"family":"window", "registered_name":name, "canonical_name":function.name(), "aliases":function.aliases(), "signature":format!("{:?}", function.signature())}));
    }
    for name in state.table_functions().keys() {
        rows.push(json!({"family":"table", "registered_name":name}));
    }
    rows.sort_by_key(|v| {
        (
            v["family"].as_str().unwrap().to_owned(),
            v["registered_name"].as_str().unwrap().to_owned(),
        )
    });
    let mut settings = state
        .config_options()
        .entries()
        .into_iter()
        .map(|e| json!({"key":e.key, "value":e.value, "description":e.description}))
        .collect::<Vec<_>>();
    settings.sort_by_key(|e| e["key"].as_str().unwrap().to_owned());
    println!("{}", serde_json::to_string_pretty(&json!({"profile":"df55-sql-parquet-nested-string", "datafusion":"55.1.0", "arrow":"59.3.0", "scope":"fresh SessionContext, this manifest and Cargo.lock only; not all possible features or user registrations", "functions": rows,"configuration": settings})).unwrap());
}
