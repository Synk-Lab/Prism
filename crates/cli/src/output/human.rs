//! Human-readable colored terminal output formatter.

use prism_core::types::report::DiagnosticReport;

/// Print a diagnostic report in human-readable colored format.
pub fn print_report(report: &DiagnosticReport) -> anyhow::Result<()> {
    // TODO: Implement rich colored terminal output
    println!(
        "Error: {} ({}:{})",
        report.error_name, report.error_category, report.error_code
    );
    println!("Summary: {}", report.summary);
    Ok(())
}
