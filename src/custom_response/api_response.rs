use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: "success".to_string(),
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str) -> Self {
        ApiResponse {
            status: "error".to_string(),
            data: None,
            error: Some(message.to_string()),
        }
    }
}
