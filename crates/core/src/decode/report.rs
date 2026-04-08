//! Report generator.
//!
//! Assembles decoded error information, taxonomy data, and context into
//! a structured `DiagnosticReport`.

use crate::decode::host_error::ClassifiedError;
use crate::taxonomy::loader::TaxonomyDatabase;
use crate::error::PrismResult;
use crate::types::report::{DiagnosticReport, RootCause, Severity, SuggestedFix};

/// Build a diagnostic report from a classified error.
pub fn build_report(error: &ClassifiedError) -> PrismResult<DiagnosticReport> {
    // Load the taxonomy database
    let db = TaxonomyDatabase::load_embedded()?;

    // Look up the error in the taxonomy
    if let Some(entry) = db.lookup(&error.category, error.error_code) {
        let report = DiagnosticReport {
            error_category: entry.category.to_string(),
            error_code: entry.code,
            error_name: entry.name.clone(),
            summary: entry.summary.clone(),
            detailed_explanation: entry.detailed_explanation.clone(),
            severity: match entry.severity.as_str() {
                "Info" => Severity::Info,
                "Warning" => Severity::Warning,
                "Fatal" => Severity::Fatal,
                _ => Severity::Error,
            },
            root_causes: entry
                .common_causes
                .iter()
                .map(|c| RootCause {
                    description: c.description.clone(),
                    likelihood: c.likelihood.clone(),
                })
                .collect(),
            suggested_fixes: entry
                .suggested_fixes
                .iter()
                .map(|f| SuggestedFix {
                    description: f.description.clone(),
                    difficulty: f.difficulty.clone(),
                    requires_upgrade: f.requires_upgrade,
                    example: f.example.clone(),
                })
                .collect(),
            contract_error: None,
            transaction_context: None,
            related_errors: entry.related_errors.clone(),
        };

        Ok(report)
    } else {
        // Error not found in taxonomy — return a basic report
        Ok(DiagnosticReport::new(
            &error.category.to_string(),
            error.error_code,
            "Unknown",
            &format!(
                "Unknown {} error with code {}",
                error.category, error.error_code
            ),
        ))
    }
}
