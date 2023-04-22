use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

pub mod librarian;
pub mod reader;

// List Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryListResponse<V> {
    pub offset: usize,
    pub limit: usize,
    pub total: usize,
    pub items: Vec<V>,
}

// Deletion Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeletionResponse {
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WrappingResponse<V> {
    Resp(V),
    Error(ApiErrorResponse),
}

impl<V> WrappingResponse<V> {
    pub fn okay(value: V) -> Self {
        Self::Resp(value)
    }

    pub fn error<S: Into<String>>(value: S) -> Self {
        Self::Error(ApiErrorResponse::new(value))
    }

    pub fn error_code<S: Into<String>>(value: S, code: ErrorCodeResponse) -> Self {
        Self::Error(ApiErrorResponse::new_with_code(value, code))
    }

    pub fn ok(self) -> std::result::Result<V, ApiErrorResponse> {
        match self {
            Self::Resp(v) => Ok(v),
            Self::Error(e) => Err(e),
        }
    }

    pub fn as_ok(&self) -> std::result::Result<&V, &ApiErrorResponse> {
        match self {
            Self::Resp(v) => Ok(v),
            Self::Error(e) => Err(e),
        }
    }

    pub fn map<N, F: Fn(V) -> N>(self, func: F) -> WrappingResponse<N> {
        match self {
            Self::Resp(v) => WrappingResponse::Resp(func(v)),
            Self::Error(e) => WrappingResponse::Error(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, thiserror::Error)]
pub struct ApiErrorResponse {
    pub code: ErrorCodeResponse,
    pub description: String,
}

impl ApiErrorResponse {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self {
            code: ErrorCodeResponse::Unset,
            description: value.into(),
        }
    }

    pub fn new_with_code<S: Into<String>>(value: S, code: ErrorCodeResponse) -> Self {
        Self {
            code,
            description: value.into(),
        }
    }
}

impl std::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api Error Occured: {}", self.description)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ErrorCodeResponse {
    Unset = 0,
    /// Only set for client errors.
    Client,
    NotLoggedIn,
}