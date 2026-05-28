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
