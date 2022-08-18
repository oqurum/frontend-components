use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthFormLink {
    pub server_owner_name: Option<String>,
    pub server_name: Option<String>,
    pub server_id: Option<String>,

    pub redirect_uri: String,

    pub state: String,
    pub scope: Scope,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct AuthQueryHandshake {
    /// Used for verifying
    pub state: Option<String>,

    /// Private Server ID
    pub server_id: String,
    /// Public Server ID
    pub public_id: String,

    pub scope: Scope,
}



#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    ServerRegister,
}