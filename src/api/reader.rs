use serde::{Deserialize, Serialize};

use super::librarian::Scope;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyAgentQuery {
    /// Metadata Agent Member ID used to link account with.
    pub member_id: usize,
    /// Private Server ID.
    pub server_id: String,
    /// Public Server ID.
    pub public_id: String,

    /// Unqiue ID for continuity
    pub state: String,
    /// What we were doing (server_register, member_link)
    pub scope: Scope,
}
