use opentelemetry::KeyValue;
use opentelemetry::propagation::{Extractor, Injector};
use opentelemetry::trace::TraceError;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, runtime};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;

pub struct InjectorMetadataMap<'a>(pub &'a mut tonic::metadata::MetadataMap);

impl<'a> Injector for InjectorMetadataMap<'a> {
    /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = tonic::metadata::MetadataValue::try_from(&value) {
                self.0.insert(key, val);
            }
        }
    }
}

pub struct ExtractorMetadataMap<'a>(pub &'a tonic::metadata::MetadataMap);

impl<'a> Extractor for ExtractorMetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

pub fn init_tracer(endpoint: String, service_name: String) -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    // global::set_text_map_propagator(TraceContextPropagator::new());
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint),
        )
        .with_trace_config(
            opentelemetry_sdk::trace::config().with_resource(Resource::new(vec![KeyValue::new(
                SERVICE_NAME,
                service_name,
            )])),
        )
        .install_batch(runtime::Tokio)
}