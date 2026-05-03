//! WASM ContractSpec decoder.
//!
//! Extracts `contractspecv0` and `SCMetaEntry` metadata from WASM custom sections.
//! Used to resolve contract-specific error enums, function signatures, and type definitions.

use crate::error::{PrismError, PrismResult};
use serde::{Deserialize, Serialize};

/// A decoded contract error enum variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractErrorEntry {
    /// Numeric error code.
    pub code: u32,
    /// Name of the error variant (e.g., "InsufficientBalance").
    pub name: String,
    /// Doc comment, if present in the contract spec.
    pub doc: Option<String>,
}

/// A decoded contract function signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFunction {
    /// Function name.
    pub name: String,
    /// Parameter names and types.
    pub params: Vec<(String, String)>,
    /// Return type description.
    pub return_type: String,
    /// Doc comment, if present.
    pub doc: Option<String>,
}

/// Fully decoded contract specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSpec {
    /// Error enum variants defined in the contract.
    pub errors: Vec<ContractErrorEntry>,
    /// Function signatures.
    pub functions: Vec<ContractFunction>,
    /// Contract name from meta, if available.
    pub name: Option<String>,
    /// Contract version from meta, if available.
    pub version: Option<String>,
}

/// Parse WASM bytecode and extract the contract specification.
///
/// # Arguments
/// * `wasm_bytes` - Raw WASM binary data
///
/// # Returns
/// A `ContractSpec` with all decoded metadata.
pub fn decode_contract_spec(wasm_bytes: &[u8]) -> PrismResult<ContractSpec> {
    let _raw_spec = SpecParser::extract_spec(wasm_bytes)?;

    // Parse WASM to find custom sections named "contractspecv0" and "contractmetav0"
    let spec = ContractSpec {
        errors: Vec::new(),
        functions: Vec::new(),
        name: None,
        version: None,
    };

    // TODO: Actually decode the raw_spec into ContractSpec
    // This will be implemented in a future PR.
    
    Ok(spec)
}

/// A parser for extracting custom sections from WASM binaries.
pub struct SpecParser;

impl SpecParser {
    /// Extracts the raw data from the `contractspecv0` custom section.
    ///
    /// # Arguments
    /// * `wasm_bytes` - Raw WASM binary data.
    ///
    /// # Returns
    /// The raw bytes of the `contractspecv0` section if found.
    pub fn extract_spec(wasm_bytes: &[u8]) -> PrismResult<Vec<u8>> {
        let parser = wasmparser::Parser::new(0);
        for payload in parser.parse_all(wasm_bytes) {
            let payload =
                payload.map_err(|e| PrismError::SpecError(format!("WASM parse error: {e}")))?;

            if let wasmparser::Payload::CustomSection(section) = payload {
                if section.name() == "contractspecv0" {
                    return Ok(section.data().to_vec());
                }
            }
        }

        Err(PrismError::SpecError(
            "contractspecv0 custom section not found".into(),
        ))
    }
}

/// Resolve a numeric error code to its named variant using a contract spec.
///
/// # Arguments
/// * `spec` - The decoded contract specification
/// * `error_code` - The numeric error code to resolve
///
/// # Returns
/// The matching error entry, if found.
pub fn resolve_error_code(spec: &ContractSpec, error_code: u32) -> Option<&ContractErrorEntry> {
    spec.errors.iter().find(|e| e.code == error_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_error_code_not_found() {
        let spec = ContractSpec {
            errors: vec![ContractErrorEntry {
                code: 1,
                name: "NotFound".to_string(),
                doc: None,
            }],
            functions: Vec::new(),
            name: None,
            version: None,
        };
        assert!(resolve_error_code(&spec, 99).is_none());
        assert!(resolve_error_code(&spec, 1).is_some());
    }

    #[test]
    fn test_extract_spec_success() {
        // Minimal WASM with contractspecv0 custom section
        // Header: \0asm\1\0\0\0
        // Custom section: id=0, payload_len=..., name_len=14, name="contractspecv0", data=[1, 2, 3]
        let mut wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        let section_name = "contractspecv0";
        let section_data = vec![1, 2, 3, 4];
        
        let mut custom_payload = Vec::new();
        // Name length as leb128 (14 is 0x0E)
        custom_payload.push(section_name.len() as u8);
        custom_payload.extend_from_slice(section_name.as_bytes());
        custom_payload.extend_from_slice(&section_data);
        
        wasm.push(0); // Custom section ID
        // Section length as leb128
        wasm.push(custom_payload.len() as u8);
        wasm.extend(custom_payload);

        let result = SpecParser::extract_spec(&wasm).expect("Should find section");
        assert_eq!(result, section_data);
    }

    #[test]
    fn test_extract_spec_not_found() {
        let wasm = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        let result = SpecParser::extract_spec(&wasm);
        assert!(result.is_err());
        match result {
            Err(PrismError::SpecError(msg)) => assert!(msg.contains("not found")),
            _ => panic!("Expected SpecError"),
        }
    }
}
