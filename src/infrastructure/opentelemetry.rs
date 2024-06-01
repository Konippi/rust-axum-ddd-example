use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::CONFIG;

pub struct OpenTelemetry {}

impl OpenTelemetry {
    /// Initialize
    pub fn init() {
        tracing_subscriber::Registry::default()
            .with(tracing_opentelemetry::layer().with_tracer(Self::build_tracer()))
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    }

    /// Build tracer for OpenTelemetry
    fn build_tracer() -> Tracer {
        let trace_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(format!("{}", CONFIG.otlp_exporter_endpoint,));
        let trace_config = opentelemetry_sdk::trace::config()
            .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn)
            .with_id_generator(opentelemetry_sdk::trace::RandomIdGenerator::default())
            .with_resource(opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", "rust-axum-ddd-example"),
            ]));
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(trace_exporter)
            .with_trace_config(trace_config)
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .expect("Failed to install opentelemetry pipeline")
    }
}
