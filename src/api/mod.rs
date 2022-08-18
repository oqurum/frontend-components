use serde::{Serialize, Deserialize};


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



// TODO: Could just be an enum.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WrappingResponse<V> {
    pub resp: Option<V>,
    pub error: Option<ApiErrorResponse>,
}

impl<V> WrappingResponse<V> {
    pub fn okay(value: V) -> Self {
        Self { resp: Some(value), error: None }
    }

    pub fn error<S: Into<String>>(value: S) -> Self {
        Self { resp: None, error: Some(ApiErrorResponse::new(value)) }
    }

    pub fn ok(self) -> std::result::Result<V, ApiErrorResponse> {
        if let Some(resp) = self.resp {
            Ok(resp)
        } else if let Some(err) = self.error {
            Err(err)
        } else {
            unreachable!()
        }
    }

    pub fn as_ok(&self) -> std::result::Result<&V, &ApiErrorResponse> {
        if let Some(resp) = self.resp.as_ref() {
            Ok(resp)
        } else if let Some(err) = self.error.as_ref() {
            Err(err)
        } else {
            unreachable!()
        }
    }

    pub fn map<N, F: Fn(V) -> N>(self, func: F) -> WrappingResponse<N> {
        WrappingResponse {
            resp: self.resp.map(func),
            error: self.error,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, thiserror::Error)]
pub struct ApiErrorResponse {
    pub description: String,
}

impl ApiErrorResponse {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self { description: value.into() }
    }
}

impl std::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Api Error Occured: {}", self.description)
    }
}