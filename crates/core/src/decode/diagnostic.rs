//! Diagnostic event analyzer.
//!
//! Processes diagnostic events from transaction results to extract additional
//! context such as which budget category was exceeded, which auth check failed,
//! or which storage key was inaccessible.

use crate::error::PrismResult;
use crate::types::report::DiagnosticReport;

/// Enrich a diagnostic report with information from diagnostic events.
///
/// Diagnostic events are internal host events that reveal execution internals
/// beyond what the top-level error code provides.
pub fn enrich_report(
    report: &mut DiagnosticReport,
    tx_data: &serde_json::Value,
) -> PrismResult<()> {
    // Extract diagnostic events from the transaction result metadata
    if let Some(events) = tx_data.get("diagnosticEvents").and_then(|e| e.as_array()) {
        for event in events {
            if let Some(event_type) = event.get("type").and_then(|t| t.as_str()) {
                match event_type {
                    "budget" => analyze_budget_event(report, event),
                    "storage" => analyze_storage_event(report, event),
                    "auth" => analyze_auth_event(report, event),
                    _ => {
                        tracing::debug!("Unknown diagnostic event type: {event_type}");
                    }
                }
            }
        }
    }

    Ok(())
}

/// Analyze a budget-related diagnostic event.
fn analyze_budget_event(report: &mut DiagnosticReport, event: &serde_json::Value) {
    // TODO: Extract CPU vs memory budget details
    let _ = (report, event);
}

/// Analyze a storage-related diagnostic event.
fn analyze_storage_event(report: &mut DiagnosticReport, event: &serde_json::Value) {
    // TODO: Extract which storage key was accessed/denied
    let _ = (report, event);
}

/// Analyze an auth-related diagnostic event.
fn analyze_auth_event(report: &mut DiagnosticReport, event: &serde_json::Value) {
    // TODO: Extract which auth check failed and what was expected
    let _ = (report, event);
}
