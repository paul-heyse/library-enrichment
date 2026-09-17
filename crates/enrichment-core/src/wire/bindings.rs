//! Generated MCP bindings. Python forwards this finite catalog without re-declaring contracts.
use schemars::{JsonSchema, generate::SchemaSettings};
use serde_json::{Value, json};

pub enum Effect {
    Read,
    Acquire,
    Execute,
}

pub fn binding<I: JsonSchema, O: JsonSchema>(
    name: &str,
    rpc: &str,
    description: &str,
    effect: Effect,
    published: bool,
) -> Value {
    let input = SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<I>()
        .to_value();
    let mut output = super::schema::envelope_schema();
    let empty = json!({"$ref":"#/$defs/EmptyData"});
    let job = json!({"$ref":"#/$defs/JobData"});
    let mut success = vec![json!({"$ref":format!("#/$defs/{}",O::schema_name())})];
    if name == "service_status" {
        success.push(json!({"$ref":"#/$defs/LocalStatus"}));
    }
    let mut alternatives = success.clone();
    if name != "job_control" {
        alternatives.push(job.clone());
    }
    alternatives.push(empty.clone());
    output["properties"]["data"] = json!({"anyOf":alternatives});
    let mut failure = vec![empty.clone()];
    if name == "verify_usage" {
        failure.extend(success.clone());
    }
    output["allOf"].as_array_mut().expect("native outcome conditions").extend([
        json!({"if":{"properties":{"delivery":{"properties":{"mode":{"const":"artifact"}}}}},"then":{"properties":{"data":empty}}}),
        json!({"if":{"properties":{"status":{"const":"pending"}}},"then":{"properties":{"data":job,"delivery":{"properties":{"mode":{"const":"inline"}}}}}}),
        json!({"if":{"properties":{"status":{"enum":["ok","partial"]},"delivery":{"properties":{"mode":{"const":"inline"}}}}},"then":{"properties":{"data":{"anyOf":success}}}}),
        json!({"if":{"properties":{"status":{"const":"error"},"delivery":{"properties":{"mode":{"const":"inline"}}}}},"then":{"properties":{"data":{"anyOf":failure}}}}),
    ]);
    json!({
        "name":name,"rpc":rpc,"description":description,"published":published,
        "input_schema":input,"output_schema":output,
        "annotations":{"readOnlyHint":matches!(effect,Effect::Read),"destructiveHint":false,"idempotentHint":!matches!(effect,Effect::Execute),"openWorldHint":!matches!(effect,Effect::Read)},
        "timeout_seconds":if matches!(effect,Effect::Acquire) { 180 } else { 30 },
    })
}
