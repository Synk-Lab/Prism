//! Diagnostic report types — the primary output of the decode engine.

use serde::{Deserialize, Serialize};

/// Severity level of the decoded error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// A single root cause suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCause {
    /// Human-readable description of the root cause.
    pub description: String,
    /// Likelihood of this root cause: high, medium, low.
    pub likelihood: String,
}

/// A single suggested fix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFix {
    /// Human-readable description of the fix.
    pub description: String,
    /// Difficulty level: easy, medium, hard.
    pub difficulty: String,
    /// Whether this fix requires a contract upgrade.
    pub requires_upgrade: bool,
    /// Optional code example or reference.
    pub example: Option<String>,
    /// Unique identifier for this fix.
    pub id: String,
    /// Automated remedy code, if available.
    pub remedy_code: Option<String>,
}

/// Decoded contract-specific error information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractErrorInfo {
    /// The contract address.
    pub contract_id: String,
    /// The numeric error code.
    pub error_code: u32,
    /// The error enum name from the contract spec (e.g., "InsufficientBalance").
    pub error_name: Option<String>,
    /// Doc comment from the contract spec, if available.
    pub doc_comment: Option<String>,
}

/// Transaction context information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionContext {
    /// The transaction hash.
    pub tx_hash: String,
    /// The ledger sequence number.
    pub ledger_sequence: u32,
    /// The function that was called.
    pub function_name: Option<String>,
    /// Decoded function arguments.
    pub arguments: Vec<String>,
    /// Fee breakdown.
    pub fee: FeeBreakdown,
    /// Resource usage summary.
    pub resources: ResourceSummary,
}

/// Fee breakdown for a transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeBreakdown {
    pub inclusion_fee: i64,
    pub resource_fee: i64,
    pub refundable_fee: i64,
    pub non_refundable_fee: i64,
}

/// Resource usage summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSummary {
    pub cpu_instructions_used: u64,
    pub cpu_instructions_limit: u64,
    pub memory_bytes_used: u64,
    pub memory_bytes_limit: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

/// The complete diagnostic report — the primary output of Tier 1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    /// The decoded error category and code.
    pub error_category: String,
    /// The numeric error code.
    pub error_code: u32,
    /// Official error name.
    pub error_name: String,
    /// Human-readable one-line summary.
    pub summary: String,
    /// Detailed explanation.
    pub detailed_explanation: String,
    /// Severity level.
    pub severity: Severity,
    /// Ordered root causes.
    pub root_causes: Vec<RootCause>,
    /// Ordered suggested fixes.
    pub suggested_fixes: Vec<SuggestedFix>,
    /// Contract-specific error info, if applicable.
    pub contract_error: Option<ContractErrorInfo>,
    /// Transaction context.
    pub transaction_context: Option<TransactionContext>,
    /// Related error IDs.
    pub related_errors: Vec<String>,
}

impl DiagnosticReport {
    /// Create a new empty diagnostic report with the given error info.
    pub fn new(category: &str, code: u32, name: &str, summary: &str) -> Self {
        Self {
            error_category: category.to_string(),
            error_code: code,
            error_name: name.to_string(),
            summary: summary.to_string(),
            detailed_explanation: String::new(),
            severity: Severity::Error,
            root_causes: Vec::new(),
            suggested_fixes: Vec::new(),
            contract_error: None,
            transaction_context: None,
            related_errors: Vec::new(),
        }
    }
}
