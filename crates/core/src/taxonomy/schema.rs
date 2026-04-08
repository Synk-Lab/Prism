//! Taxonomy database schema types.
//!
//! Defines the structure for error taxonomy entries loaded from TOML files.
//! This schema is the canonical format for community contributions.

use serde::{Deserialize, Serialize};

/// A single error taxonomy entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyEntry {
    /// Unique identifier (e.g., "host.budget.limit_exceeded.cpu").
    pub id: String,
    /// Error category.
    pub category: ErrorCategory,
    /// Numeric error code within the category.
    pub code: u32,
    /// Official error name from Stellar Core source.
    pub name: String,
    /// Severity level.
    pub severity: String,
    /// Protocol version this error was introduced.
    pub since_protocol: Option<u32>,
    /// Protocol version this error was removed, if applicable.
    pub deprecated_protocol: Option<u32>,
    /// One-sentence summary.
    pub summary: String,
    /// Detailed multi-paragraph explanation.
    pub detailed_explanation: String,
    /// Root causes, ordered by likelihood.
    pub common_causes: Vec<TaxonomyCause>,
    /// Suggested fixes, ordered by relevance.
    pub suggested_fixes: Vec<TaxonomyFix>,
    /// IDs of related errors.
    pub related_errors: Vec<String>,
    /// Path in stellar-core source.
    pub source_file: Option<String>,
    /// Line number in source file.
    pub source_line: Option<u32>,
    /// Link to official documentation.
    pub documentation_url: Option<String>,
}

/// A root cause in the taxonomy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyCause {
    /// Description of the cause.
    pub description: String,
    /// Likelihood: "high", "medium", "low".
    pub likelihood: String,
}

/// A suggested fix in the taxonomy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyFix {
    /// Description of the fix.
    pub description: String,
    /// Difficulty: "easy", "medium", "hard".
    pub difficulty: String,
    /// Whether this fix requires a contract upgrade.
    pub requires_upgrade: bool,
    /// Optional code example.
    pub example: Option<String>,
    /// Unique identifier for this fix.
    pub id: Option<String>,
    /// Automated remedy code, if available.
    pub remedy_code: Option<String>,
}

/// Soroban host error categories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ErrorCategory {
    Budget,
    Storage,
    Auth,
    Context,
    Value,
    Object,
    Crypto,
    Contract,
    Wasm,
    Events,
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Budget => write!(f, "Budget"),
            Self::Storage => write!(f, "Storage"),
            Self::Auth => write!(f, "Auth"),
            Self::Context => write!(f, "Context"),
            Self::Value => write!(f, "Value"),
            Self::Object => write!(f, "Object"),
            Self::Crypto => write!(f, "Crypto"),
            Self::Contract => write!(f, "Contract"),
            Self::Wasm => write!(f, "WASM"),
            Self::Events => write!(f, "Events"),
        }
    }
}

/// A parsed TOML taxonomy file containing entries for a single category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyFile {
    /// Category metadata.
    pub category: CategoryMeta,
    /// Error entries.
    pub errors: Vec<TaxonomyEntry>,
}

/// Category-level metadata in a taxonomy TOML file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryMeta {
    /// Category name.
    pub name: String,
    /// Category description.
    pub description: String,
    /// Stellar Core source module.
    pub source_module: String,
}
