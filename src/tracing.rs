use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::registry::Registry;

fn build_trace_layer() -> OpenTelemetryLayer<Registry, Tracer> {
    let trace_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317");
    let trace_config = opentelemetry_sdk::trace::config()
        .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn)
        .with_id_generator(opentelemetry_sdk::trace::RandomIdGenerator::default())
        .with_resource(opentelemetry_sdk::Resource::new(vec![
            opentelemetry::KeyValue::new("service.name", "totp-based-2fa-server"),
        ]));
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(trace_exporter)
        .with_trace_config(trace_config)
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("pipeline install failed");
    tracing_opentelemetry::layer().with_tracer(tracer)
}

fn init() {
    let trace_layer = build_trace_layer();
    // let metrics_layer = tracing_opentelemetry::MetricsLayer::new();
}
