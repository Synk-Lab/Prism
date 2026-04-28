//! XDR codec - thin wrapper over `stellar-xdr` with convenience methods.
//!
//! Handles serialization/deserialization of transaction envelopes, results,
//! ledger entries, ScVal, and ScSpecEntry types.

use crate::error::{PrismError, PrismResult};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use stellar_xdr::curr::{
    LedgerEntry, Limits, ReadXdr, TransactionEnvelope, TransactionMeta, 
    WriteXdr, TransactionResult, ScMap, ScVal, ScSpecEntry,
};

/// Uniform base64-XDR encode/decode interface for Stellar XDR types.
pub trait XdrCodec: Sized {
    /// The short name used in error messages (e.g. `"TransactionMeta"`).
    const TYPE_NAME: &'static str;

    /// Decode from XDR bytes.
    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self>;

    /// Encode to XDR bytes.
    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>>;

    /// Decode from a base64-encoded XDR string.
    fn from_base64(b64: &str) -> PrismResult<Self> {
        let bytes = decode_xdr_base64(b64)?;
        Self::from_xdr_bytes(&bytes)
    }

    /// Encode to a base64-encoded XDR string.
    fn to_base64(&self) -> PrismResult<String> {
        let bytes = self.to_xdr_bytes()?;
        Ok(encode_xdr_base64(&bytes))
    }
}

// ── Trait Implementations ───────────────────────────────────────────────────

impl XdrCodec for TransactionMeta {
    const TYPE_NAME: &'static str = "TransactionMeta";

    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self> {
        TransactionMeta::from_xdr(bytes, Limits::none()).map_err(|e| {
            PrismError::XdrDecodingFailed {
                type_name: Self::TYPE_NAME,
                reason: e.to_string(),
            }
        })
    }

    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>> {
        self.to_xdr(Limits::none()).map_err(|e| {
            PrismError::XdrError(format!("Failed to encode {}: {}", Self::TYPE_NAME, e))
        })
    }
}

impl XdrCodec for TransactionEnvelope {
    const TYPE_NAME: &'static str = "TransactionEnvelope";

    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self> {
        TransactionEnvelope::from_xdr(bytes, Limits::none()).map_err(|e| {
            PrismError::XdrDecodingFailed {
                type_name: Self::TYPE_NAME,
                reason: e.to_string(),
            }
        })
    }

    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>> {
        self.to_xdr(Limits::none()).map_err(|e| {
            PrismError::XdrError(format!("Failed to encode {}: {}", Self::TYPE_NAME, e))
        })
    }
}

impl XdrCodec for TransactionResult {
    const TYPE_NAME: &'static str = "TransactionResult";

    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self> {
        TransactionResult::from_xdr(bytes, Limits::none()).map_err(|e| {
            PrismError::XdrDecodingFailed {
                type_name: Self::TYPE_NAME,
                reason: e.to_string(),
            }
        })
    }

    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>> {
        self.to_xdr(Limits::none()).map_err(|e| {
            PrismError::XdrError(format!("Failed to encode {}: {}", Self::TYPE_NAME, e))
        })
    }
}

impl XdrCodec for LedgerEntry {
    const TYPE_NAME: &'static str = "LedgerEntry";

    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self> {
        LedgerEntry::from_xdr(bytes, Limits::none()).map_err(|e| {
            PrismError::XdrDecodingFailed {
                type_name: Self::TYPE_NAME,
                reason: e.to_string(),
            }
        })
    }

    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>> {
        self.to_xdr(Limits::none()).map_err(|e| {
            PrismError::XdrError(format!("Failed to encode {}: {}", Self::TYPE_NAME, e))
        })
    }
}

impl XdrCodec for ScMap {
    const TYPE_NAME: &'static str = "ScMap";

    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self> {
        ScMap::from_xdr(bytes, Limits::none()).map_err(|e| {
            PrismError::XdrDecodingFailed {
                type_name: Self::TYPE_NAME,
                reason: e.to_string(),
            }
        })
    }

    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>> {
        self.to_xdr(Limits::none()).map_err(|e| {
            PrismError::XdrError(format!("Failed to encode {}: {}", Self::TYPE_NAME, e))
        })
    }
}

impl XdrCodec for ScVal {
    const TYPE_NAME: &'static str = "ScVal";

    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self> {
        ScVal::from_xdr(bytes, Limits::none()).map_err(|e| {
            PrismError::XdrDecodingFailed {
                type_name: Self::TYPE_NAME,
                reason: e.to_string(),
            }
        })
    }

    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>> {
        self.to_xdr(Limits::none()).map_err(|e| {
            PrismError::XdrError(format!("Failed to encode {}: {}", Self::TYPE_NAME, e))
        })
    }
}

impl XdrCodec for ScSpecEntry {
    const TYPE_NAME: &'static str = "ScSpecEntry";

    fn from_xdr_bytes(bytes: &[u8]) -> PrismResult<Self> {
        ScSpecEntry::from_xdr(bytes, Limits::none()).map_err(|e| {
            PrismError::XdrDecodingFailed {
                type_name: Self::TYPE_NAME,
                reason: e.to_string(),
            }
        })
    }

    fn to_xdr_bytes(&self) -> PrismResult<Vec<u8>> {
        self.to_xdr(Limits::none()).map_err(|e| {
            PrismError::XdrError(format!("Failed to encode {}: {}", Self::TYPE_NAME, e))
        })
    }
}

// ── Low-level helpers ───────────────────────────────────────────────────────

/// Decode a base64-encoded XDR string to raw bytes.
pub fn decode_xdr_base64(xdr_base64: &str) -> PrismResult<Vec<u8>> {
    STANDARD.decode(xdr_base64).map_err(|e| {
        PrismError::XdrError(format!("Base64 decode failed: {}", e))
    })
}

/// Encode raw bytes to a base64 XDR string.
pub fn encode_xdr_base64(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

/// Decode a transaction hash from hex string.
pub fn decode_tx_hash(hash_hex: &str) -> PrismResult<[u8; 32]> {
    let bytes = hex_decode(hash_hex)
        .map_err(|e| PrismError::XdrError(format!("Invalid tx hash hex: {}", e)))?;
    
    if bytes.len() != 32 {
        return Err(PrismError::XdrError(format!(
            "Transaction hash must be 32 bytes, got {}",
            bytes.len()
        )));
    }
    
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_xdr::curr::{
        Memo, MuxedAccount, Preconditions, SequenceNumber, Transaction, 
        TransactionExt, TransactionV1Envelope, Uint256,
    };

    fn make_test_envelope() -> TransactionEnvelope {
        TransactionEnvelope::Tx(TransactionV1Envelope {
            tx: Transaction {
                source_account: MuxedAccount::Ed25519(Uint256([0; 32])),
                fee: 100,
                seq_num: SequenceNumber(1),
                cond: Preconditions::None,
                memo: Memo::None,
                operations: vec![].try_into().unwrap(),
                ext: TransactionExt::V0,
            },
            signatures: vec![].try_into().unwrap(),
        })
    }

    #[test]
    fn test_xdr_codec_round_trip() {
        let envelope = make_test_envelope();
        let b64 = envelope.to_base64().expect("encode");
        let decoded = TransactionEnvelope::from_base64(&b64).expect("decode");
        assert_eq!(envelope, decoded);
    }

    #[test]
    fn test_transaction_meta_v3_decoding() {
        // Minimal TransactionMetaV3 XDR bytes (big-endian).
        let xdr_bytes: Vec<u8> = vec![
            0, 0, 0, 3, // V3 discriminant
            0, 0, 0, 0, // ext = ExtensionPoint::V0
            0, 0, 0, 0, // txChangesBefore = []
            0, 0, 0, 1, // operations length = 1
            0, 0, 0, 0, // OperationMeta.changes = []
            0, 0, 0, 0, // txChangesAfter = []
            0, 0, 0, 1, // sorobanMeta present
            0, 0, 0, 0, // SorobanTransactionMetaExt::V0
            0, 0, 0, 1, // events length = 1
            0, 0, 0, 0, // ContractEvent.ext = V0
            0, 0, 0, 0, // contractId absent
            0, 0, 0, 1, // type = CONTRACT
            0, 0, 0, 0, // body discriminant V0
            0, 0, 0, 0, // topics = []
            0, 0, 0, 1, // data = ScVal::Void
            0, 0, 0, 1, // returnValue = ScVal::Void
            0, 0, 0, 0, // diagnosticEvents = []
        ];
        
        let b64 = encode_xdr_base64(&xdr_bytes);
        let meta = TransactionMeta::from_base64(&b64).expect("decode V3");

        if let TransactionMeta::V3(v3) = meta {
            assert_eq!(v3.operations.len(), 1);
            let soroban = v3.soroban_meta.expect("soroban_meta");
            assert_eq!(soroban.events.len(), 1);
        } else {
            panic!("expected V3");
        }
    }

    #[test]
    fn test_decode_tx_hash_valid() {
        let hash = "a".repeat(64);
        assert!(decode_tx_hash(&hash).is_ok());
    }

    #[test]
    fn test_transaction_result_round_trip() {
        // Minimal valid TransactionResult: feeCharged=0, txSUCCESS=0, results=[], ext=V0
        // 8 bytes (fee), 4 bytes (code), 4 bytes (results len), 4 bytes (ext)
        let xdr_bytes = vec![0u8; 20];
        let bytes = encode_xdr_base64(&xdr_bytes);
        
        let decoded = TransactionResult::from_base64(&bytes).expect("decode");
        let encoded = decoded.to_base64().expect("encode");
        
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_scmap_round_trip() {
        let scmap = ScMap(vec![].try_into().unwrap());
        let b64 = scmap.to_base64().expect("encode");
        let decoded = ScMap::from_base64(&b64).expect("decode");
        assert_eq!(scmap, decoded);
    }

    #[test]
    fn test_scval_round_trip() {
        let scval = ScVal::Void;
        let b64 = scval.to_base64().expect("encode");
        let decoded = ScVal::from_base64(&b64).expect("decode");
        assert_eq!(scval, decoded);
    }

    #[test]
    fn test_scspecentry_round_trip() {
        use stellar_xdr::curr::ScSpecFunctionV0;
        let entry = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "test".try_into().unwrap(),
            inputs: vec![].try_into().unwrap(),
            outputs: vec![].try_into().unwrap(),
        });
        let b64 = entry.to_base64().expect("encode");
        let decoded = ScSpecEntry::from_base64(&b64).expect("decode");
        assert_eq!(entry, decoded);
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn hex_decode(input: &str) -> Result<Vec<u8>, String> {
    if input.len() % 2 != 0 {
        return Err("Hex input must have an even length".to_string());
    }

    (0..input.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&input[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex at position {i}: {e}"))
        })
        .collect()
}
