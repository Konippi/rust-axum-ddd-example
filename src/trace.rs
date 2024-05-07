use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    tracing_subscriber::Registry::default()
        .with(tracing_opentelemetry::layer().with_tracer(build_trace_layer()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

fn build_trace_layer() -> Tracer {
    let trace_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(format!(
            "{}",
            std::env::var("OTLP_EXPORTER_ENDPOINT")
                .unwrap_or_else(|_| "http://127.0.0.1:4317".to_string()),
        ));
    let trace_config = opentelemetry_sdk::trace::config()
        .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn)
        .with_id_generator(opentelemetry_sdk::trace::RandomIdGenerator::default())
        .with_resource(opentelemetry_sdk::Resource::new(vec![
            opentelemetry::KeyValue::new("service.name", "totp-based-2fa-server"),
        ]));
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(trace_exporter)
        .with_trace_config(trace_config)
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Failed to install opentelemetry pipeline")
}
