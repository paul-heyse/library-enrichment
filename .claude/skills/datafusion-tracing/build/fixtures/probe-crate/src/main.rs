//! One binary, one scenario per invocation: `probe <scenario>`.
//!
//! Each scenario builds a session, runs `SELECT`-shaped work and prints the normalised JSON
//! span capture on stdout. `build/probes.py` pairs scenarios into probe and control and decides
//! the verdict; nothing here knows what is expected, which is deliberate -- a harness that knew
//! the expected answer could produce it.

mod harness;

use std::sync::Arc;

use datafusion::arrow::array::RecordBatch;
use datafusion::error::Result;
use datafusion::execution::SessionStateBuilder;
use datafusion::prelude::*;
use datafusion_tracing::{
    InstrumentationOptions, RuleInstrumentationOptions, instrument_rules_with_debug_spans,
    instrument_rules_with_info_spans, instrument_with_debug_spans, instrument_with_info_spans,
    pretty_format_compact_batch,
};
use tracing::field;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{Registry, fmt, prelude::*};

const QUERY: &str = "SELECT a, b FROM (VALUES (1, 'alpha'), (2, 'beta'), (3, 'gamma')) AS t(a, b)";

struct Shape {
    metrics: bool,
    preview: usize,
    custom: bool,
    declare_fields: bool,
    debug_level: bool,
    rules: Option<RuleInstrumentationOptions>,
    instrument: bool,
    filter_info_only: bool,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            metrics: false,
            preview: 0,
            custom: false,
            declare_fields: true,
            debug_level: false,
            rules: None,
            instrument: true,
            filter_info_only: false,
        }
    }
}

fn options(shape: &Shape) -> InstrumentationOptions {
    let mut builder = InstrumentationOptions::builder()
        .record_metrics(shape.metrics)
        .preview_limit(shape.preview);
    if shape.preview > 0 {
        builder = builder.preview_fn(Arc::new(|batch: &RecordBatch| {
            pretty_format_compact_batch(batch, 64, 3, 10).map(|f| f.to_string())
        }));
    }
    if shape.custom {
        builder = builder
            .add_custom_field("env", "probe")
            .add_custom_field("region", "eu-west");
    }
    builder.build()
}

async fn run(shape: Shape) -> Result<String> {
    let buffer = harness::Buffer::new();
    let layer = fmt::layer()
        .with_ansi(false)
        .event_format(fmt::format().json().flatten_event(true).without_time())
        .json()
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(buffer.clone());

    let subscriber: Box<dyn tracing::Subscriber + Send + Sync> = if shape.filter_info_only {
        Box::new(Registry::default().with(layer.with_filter(
            tracing::level_filters::LevelFilter::INFO,
        )))
    } else {
        Box::new(Registry::default().with(layer))
    };
    let _guard = tracing::subscriber::set_default(subscriber);

    let mut state = SessionStateBuilder::new().with_default_features();
    if shape.instrument {
        let built = options(&shape);
        let rule = match (shape.debug_level, shape.declare_fields) {
            (false, true) => {
                instrument_with_info_spans!(options: built, env = field::Empty, region = field::Empty)
            }
            (false, false) => instrument_with_info_spans!(options: built),
            (true, true) => {
                instrument_with_debug_spans!(options: built, env = field::Empty, region = field::Empty)
            }
            (true, false) => instrument_with_debug_spans!(options: built),
        };
        state = state.with_physical_optimizer_rule(rule);
    }
    let mut session_state = state.build();
    if let Some(rule_options) = shape.rules {
        session_state = if shape.debug_level {
            instrument_rules_with_debug_spans!(options: rule_options, state: session_state)
        } else {
            instrument_rules_with_info_spans!(options: rule_options, state: session_state)
        };
    }

    let ctx = SessionContext::new_with_state(session_state);
    let _ = ctx.sql(QUERY).await?.collect().await?;
    drop(_guard);
    Ok(harness::normalise(&buffer.take()))
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    let scenario = std::env::args().nth(1).unwrap_or_default();
    let shape = match scenario.as_str() {
        "bare" => Shape { instrument: false, rules: None, ..Default::default() },
        "plain" => Shape::default(),
        "metrics-on" => Shape { metrics: true, ..Default::default() },
        "metrics-off" => Shape { metrics: false, ..Default::default() },
        "preview-on" => Shape { preview: 5, ..Default::default() },
        "preview-zero" => Shape { preview: 0, ..Default::default() },
        "custom-declared" => Shape { custom: true, declare_fields: true, ..Default::default() },
        "custom-undeclared" => Shape { custom: true, declare_fields: false, ..Default::default() },
        "rules-full" => {
            Shape { rules: Some(RuleInstrumentationOptions::full()), ..Default::default() }
        }
        "rules-phase-only" => {
            Shape { rules: Some(RuleInstrumentationOptions::phase_only()), ..Default::default() }
        }
        "rules-physical-only" => Shape {
            rules: Some(RuleInstrumentationOptions::builder().physical_optimizer().build()),
            ..Default::default()
        },
        "debug-level" => Shape { debug_level: true, ..Default::default() },
        "debug-filtered-to-info" => {
            Shape { debug_level: true, filter_info_only: true, ..Default::default() }
        }
        "info-filtered-to-info" => Shape { filter_info_only: true, ..Default::default() },
        other => {
            eprintln!("unknown scenario: {other}");
            std::process::exit(2);
        }
    };
    println!("{}", run(shape).await?);
    Ok(())
}
