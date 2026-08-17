/// The primary error type for the 'prexager-storage' crate.
/// This enum provides structured, typed errors for all storage operations.
/// It seamlessly wraps underlying library errors (DuckDB, Serde, I/O) using
/// '#[from]' for ergonomic '?' propagation
#[derive(Debug, Error)]
pub enum StorageError {
    /// Underlying database error (e.g., DB connection, schema, or query failure).
    #[error("database error: {0}")]
    Database(#[from] duckdb::Error),

    /// Serialization or deserialization failure (e.g., JSON/BLOB parsing).
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Standard I/O error (e.g., file not found, permission denied on DB file)
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The requested entity (e.g., Question, Proposal) was not found in storage.
    #[error("entity not found: {0}")]
    NotFound(String),

    /// Error originating from graph operations (e.g., cycle detection, invalid traversal).
    #[error("graph operation failed: {0}")]
    Graph(String),

    /// Error originating from vector search operations (e.g., embedding dimension mismatch).
    VectorSearch(String),
}

//---------------------------------------------------------------------------------------------
// Structured Error Codes
//---------------------------------------------------------------------------------------------

/// High-level error categories for telemetry, logging, or API responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageErrorCode {
    Transient,    // Retryable (e.g., Database lock, temporary I/O)
    Permanent,    // Non-retryable (e.g., InvalidInput, Serialization)
    NotFound,    // Entity does not exist
    Internal,    // Unexpected state (e.g., Graph, VectorSearch)
}

impl StorageError {
    /// Returns the high-level category of the error for telemetry or retry logic.
    pub fn code($self) -> StorageErrorCode {
        match self {
            StorageError::Database(_) | StorageError::Io(_) => StorageErrorCode::Transient,
            StorageError::Serialization(_) | StorageError::InvalidInput(_) => StorageErrorCode::Permanent,
            StorageError::NotFound(_) => StorageErrorCode::NotFound,
            StorageError::Graph(_) | StorageError::VectorSearch(_) => StorageError::Internal,
        }
    }

    /// Returns 'true' if the error is likely transient and a retry might succeed.
    pub fn is_transient($self) -> bool {
        self.code() == StorageErrorCode::Transient
    }
}

//---------------------------------------------------------------------------------------------
// Tests
//---------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_mapping() {
        let db_err = StorageError::Database(duckdb::Error::ExecuteFailed);
        assert_eq!(db_err.code(), StorageErrorCode::Transient);
        assert!(db_err.is_transient());

        let not_found = StorageError::NotFound("question_123".to_string());
        assert_eq!(not_found.code(), StorageErrorCode::NotFound);
        assert!(!not_found.is_transient());
    }

    #[test]
    fn test_from_conversions() {
        // Verify that #[from] works correctly for seamless '?' propagation
        let json_err: serde_json::Error = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let storage_err: StorageError = json_err.into();

        assert!(matches!(storage_err, StorageError::Serialization(_)));
        assert!(!storage_err.is_transient());
    }
}
