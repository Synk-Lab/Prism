//! What-If engine — modify inputs and re-simulate from any execution point.
//!
//! Accepts state/input patches, forks from a checkpoint, replays with
//! modifications, and produces a comparison trace.

use crate::error::PrismResult;
use crate::types::trace::ExecutionTrace;
use serde::{Deserialize, Serialize};

/// A patch to apply to the execution state or inputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhatIfPatch {
    /// Modify a function argument.
    ModifyArgument {
        /// Argument index to modify.
        index: usize,
        /// New value (as decoded string representation).
        new_value: String,
    },
    /// Modify a ledger entry.
    ModifyLedgerEntry {
        /// Ledger key.
        key: String,
        /// New value (as hex-encoded bytes).
        new_value: String,
    },
    /// Modify the resource limits.
    ModifyResourceLimits {
        /// New CPU instruction limit.
        cpu_limit: Option<u64>,
        /// New memory byte limit.
        memory_limit: Option<u64>,
    },
    /// Add or remove an auth signer.
    ModifyAuth {
        /// Signer to add (if Some) or remove (if None).
        add_signer: Option<String>,
        /// Signer to remove.
        remove_signer: Option<String>,
    },
}

/// Result of a what-if simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfResult {
    /// The original execution trace.
    pub original: ExecutionTrace,
    /// The modified execution trace.
    pub modified: ExecutionTrace,
    /// Point where the two traces first diverge.
    pub divergence_point: Option<usize>,
    /// Summary of differences.
    pub summary: String,
}

/// Run a what-if simulation with the given patches.
pub async fn simulate_whatif(
    _tx_hash: &str,
    _patches: &[WhatIfPatch],
    _network: &crate::types::config::NetworkConfig,
) -> PrismResult<WhatIfResult> {
    // TODO: Implement what-if simulation
    // 1. Replay the original transaction
    // 2. Apply patches to state/inputs
    // 3. Re-execute from the patch point
    // 4. Compare the two traces

    Err(crate::error::PrismError::Internal(
        "What-if simulation not yet implemented".to_string(),
    ))
}
