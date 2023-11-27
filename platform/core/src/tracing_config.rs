use error_stack::{Report, ResultExt};
use opentelemetry_otlp::WithExportConfig;
use thiserror::Error;
use tonic::metadata::{Ascii, MetadataValue};
use tracing::subscriber::set_global_default;
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

/// Configuration for sending telemetry to Honeycomb
pub struct HoneycombConfig {
    /// The Honeycomb team to export to
    pub team: String,
    /// The Honeycomb dataset to export to. This is also used as the service name
    pub dataset: String,
}

/// Configuration for sending telemetry to Jaeger
pub struct JaegerConfig {
    /// The Jaeger service name
    pub service_name: String,
    /// The Jaeger endpoint to send tracing to
    pub endpoint: String,
}

/// Configuration to define how to export telemetry
pub enum TracingExportConfig {
    /// Do not export tracing to an external service. This still prints it to the console.
    None,
    /// Export traces to Honeycomb
    Honeycomb(HoneycombConfig),
    /// Export traces to Jaeger
    Jaeger(JaegerConfig),
}

/// The error returned when tracing setup fails
#[derive(Error, Debug)]
#[error("Failed to configure tracing")]
pub struct TraceConfigureError;

/// Configure and enable tracing, potentially with export to an external service.
pub fn configure(export_config: TracingExportConfig) -> Result<(), Report<TraceConfigureError>> {
    LogTracer::builder()
        .ignore_crate("rustls")
        .with_max_level(log::LevelFilter::Debug)
        .init()
        .expect("Failed to create logger");

    let env_filter = EnvFilter::try_from_env("LOG").unwrap_or_else(|_| EnvFilter::new("info"));

    let tree = HierarchicalLayer::new(2)
        .with_targets(true)
        .with_bracketed_fields(true);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(tree)
        .with(ErrorLayer::default());

    match export_config {
        TracingExportConfig::Honeycomb(honeycomb_config) => {
            let mut otlp_meta = tonic::metadata::MetadataMap::new();
            otlp_meta.insert(
                "x-honeycomb-team",
                honeycomb_config
                    .team
                    .parse::<MetadataValue<Ascii>>()
                    .change_context(TraceConfigureError)?,
            );

            let exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("api.honeycomb.io:443")
                .with_metadata(otlp_meta);

            let otlp = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_trace_config(opentelemetry_sdk::trace::config().with_resource(
                    opentelemetry_sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                        "service.name",
                        honeycomb_config.dataset,
                    )]),
                ))
                .with_exporter(exporter)
                .install_batch(opentelemetry_sdk::runtime::TokioCurrentThread)
                .change_context(TraceConfigureError)?;
            let telemetry = tracing_opentelemetry::layer().with_tracer(otlp);

            let subscriber = subscriber.with(telemetry);
            set_global_default(subscriber).expect("Setting subscriber");
        }
        TracingExportConfig::Jaeger(config) => {
            let tracer = opentelemetry_jaeger::new_agent_pipeline()
                .with_service_name(&config.service_name)
                .with_endpoint(config.endpoint.as_str())
                .install_batch(opentelemetry_sdk::runtime::TokioCurrentThread)
                .unwrap();
            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

            let subscriber = subscriber.with(telemetry);
            set_global_default(subscriber).expect("Setting subscriber");
        }
        TracingExportConfig::None => {
            set_global_default(subscriber).expect("Setting subscriber");
        }
    };

    Ok(())
}

/// Shut down the active tracing exporter
pub fn teardown() {
    opentelemetry::global::shutdown_tracer_provider();
}
