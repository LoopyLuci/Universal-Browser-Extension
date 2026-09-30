//! Embedded companion UI assets.

use axum::http::header;
use axum::response::{IntoResponse, Response};

pub fn index() -> Response {
    ([(header::CONTENT_TYPE, "text/html; charset=utf-8")],
     dnea_companion_web::INDEX_HTML).into_response()
}

pub fn app_js() -> Response {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
     dnea_companion_web::APP_JS).into_response()
}

pub fn style_css() -> Response {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")],
     dnea_companion_web::STYLE_CSS).into_response()
}