use super::TraceLabels;
use opentelemetry_api::trace::TraceContextExt;
use std::iter::FromIterator;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub fn get_exemplar() -> Option<TraceLabels> {
    // Get the OpenTelemetry Context from the tracing span
    let context = Span::current().context();

    // Now get the OpenTelemetry "span" from the Context
    // (it's confusing because the word "span" is used by both tracing and OpenTelemetry
    // to mean slightly different things)
    let span = context.span();
    let span_context = span.span_context();

    if span_context.is_valid() {
        Some(TraceLabels::from_iter([
            ("trace_id", span_context.trace_id().to_string()),
            ("span_id", span_context.span_id().to_string()),
        ]))
    } else {
        None
    }
}
