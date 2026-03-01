//! Structured error responses for the backend API.
//!
//! Every API error is serialized as `{"error": "<message>", "code": "<code>"}`.
//! Domain and core errors are mapped to appropriate HTTP status codes.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// Structured JSON error response returned by all API endpoints.
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Human-readable error description.
    pub error: String,
    /// Machine-readable error code (e.g. `"not_found"`, `"unauthorized"`).
    pub code: String,
}

/// Internal representation pairing an [`ApiError`] with its HTTP status.
#[derive(Debug)]
pub struct ApiErrorResponse {
    /// HTTP status code.
    pub status: StatusCode,
    /// Error payload.
    pub body: ApiError,
}

impl ApiErrorResponse {
    /// Build a 400 Bad Request error.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: ApiError {
                error: message.into(),
                code: "bad_request".to_string(),
            },
        }
    }

    /// Build a 401 Unauthorized error.
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            body: ApiError {
                error: message.into(),
                code: "unauthorized".to_string(),
            },
        }
    }

    /// Build a 403 Forbidden error.
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            body: ApiError {
                error: message.into(),
                code: "forbidden".to_string(),
            },
        }
    }

    /// Build a 404 Not Found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            body: ApiError {
                error: message.into(),
                code: "not_found".to_string(),
            },
        }
    }

    /// Build a 500 Internal Server Error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ApiError {
                error: message.into(),
                code: "internal_error".to_string(),
            },
        }
    }

    /// Build a 503 Service Unavailable error.
    #[allow(dead_code)]
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: ApiError {
                error: message.into(),
                code: "service_unavailable".to_string(),
            },
        }
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self.body).unwrap_or_else(|e| {
            eprintln!("error: failed to serialize API error response: {e}");
            r#"{"error":"serialization failure","code":"internal_error"}"#.to_string()
        });

        (
            self.status,
            [(
                axum::http::header::CONTENT_TYPE,
                "application/json; charset=utf-8",
            )],
            body,
        )
            .into_response()
    }
}

/// Convert a [`CoreError`](ito_core::errors::CoreError) into an [`ApiErrorResponse`].
impl From<ito_core::errors::CoreError> for ApiErrorResponse {
    fn from(err: ito_core::errors::CoreError) -> Self {
        use ito_core::DomainError;
        use ito_core::errors::CoreError;
        match &err {
            CoreError::Domain(domain_err) => match domain_err {
                DomainError::NotFound { .. } => Self::not_found(err.to_string()),
                DomainError::AmbiguousTarget { .. } => Self::bad_request(err.to_string()),
                DomainError::Io { .. } => Self::internal(err.to_string()),
            },
            CoreError::NotFound(_) => Self::not_found(err.to_string()),
            CoreError::Validation(_) => Self::bad_request(err.to_string()),
            CoreError::Parse(_) => Self::bad_request(err.to_string()),
            CoreError::Io { .. } => Self::internal(err.to_string()),
            CoreError::Process(_) => Self::internal(err.to_string()),
            CoreError::Sqlite(_) => Self::internal(err.to_string()),
            CoreError::Serde { .. } => Self::internal(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_serializes_to_json_with_error_and_code() {
        let err = ApiError {
            error: "thing not found".to_string(),
            code: "not_found".to_string(),
        };
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["error"], "thing not found");
        assert_eq!(json["code"], "not_found");
    }

    #[test]
    fn not_found_response_has_404_status() {
        let resp = ApiErrorResponse::not_found("change not found: foo");
        assert_eq!(resp.status, StatusCode::NOT_FOUND);
        assert_eq!(resp.body.code, "not_found");
    }

    #[test]
    fn unauthorized_response_has_401_status() {
        let resp = ApiErrorResponse::unauthorized("invalid token");
        assert_eq!(resp.status, StatusCode::UNAUTHORIZED);
        assert_eq!(resp.body.code, "unauthorized");
    }

    #[test]
    fn forbidden_response_has_403_status() {
        let resp = ApiErrorResponse::forbidden("not allowed");
        assert_eq!(resp.status, StatusCode::FORBIDDEN);
        assert_eq!(resp.body.code, "forbidden");
    }

    #[test]
    fn bad_request_response_has_400_status() {
        let resp = ApiErrorResponse::bad_request("invalid input");
        assert_eq!(resp.status, StatusCode::BAD_REQUEST);
        assert_eq!(resp.body.code, "bad_request");
    }

    #[test]
    fn internal_response_has_500_status() {
        let resp = ApiErrorResponse::internal("disk failure");
        assert_eq!(resp.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(resp.body.code, "internal_error");
    }

    #[test]
    fn service_unavailable_response_has_503_status() {
        let resp = ApiErrorResponse::service_unavailable("not ready");
        assert_eq!(resp.status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(resp.body.code, "service_unavailable");
    }

    #[test]
    fn core_not_found_maps_to_404() {
        let err = ito_core::errors::CoreError::not_found("missing thing");
        let resp: ApiErrorResponse = err.into();
        assert_eq!(resp.status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn core_validation_maps_to_400() {
        let err = ito_core::errors::CoreError::validation("bad field");
        let resp: ApiErrorResponse = err.into();
        assert_eq!(resp.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn into_response_produces_json_content_type() {
        let resp = ApiErrorResponse::not_found("test").into_response();
        let ct = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(ct.contains("application/json"));
    }
}
