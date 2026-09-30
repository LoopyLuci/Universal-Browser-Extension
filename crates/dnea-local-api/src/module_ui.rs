//! Module panel serving.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use dnea_core::ids::ModuleId;
use serde::Deserialize;
use serde_json::json;
use tracing::warn;

use crate::state::ApiState;

const BRIDGE_JS: &str = include_str!("../assets/dnea-bridge.js");

pub async fn serve_panel(
    State(state): State<Arc<ApiState>>,
    Path((module_id, panel_id)): Path<(String, String)>,
) -> Response {
    let Ok(mid) = ModuleId::parse(&module_id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let Some(record) = state.session.module(&mid) else {
        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
    };
    let Some(manifest) = record.manifest() else {
        return (StatusCode::NOT_FOUND, "module manifest unavailable").into_response();
    };
    let Some(surface) = manifest.ui.iter().find(|s| s.id == panel_id) else {
        return (StatusCode::NOT_FOUND, "panel not declared").into_response();
    };
    if !surface.surfaces.iter().any(|s| s == "companion") {
        return (StatusCode::FORBIDDEN, "panel is not a companion surface").into_response();
    }

    let title = surface.title.as_deref().unwrap_or("DNEA Module");
    let html = format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'wasm-unsafe-eval'; style-src 'unsafe-inline'; img-src data:;">
<title>{title}</title>
</head>
<body>
<div id="dnea-root"></div>
<script>{bridge}</script>
<script>
window.dnea.onReady(function () {{
  window.dnea.invoke("render", {{ panelId: "{panel_id}" }})
    .then(function (html) {{ document.getElementById("dnea-root").innerHTML = html; }})
    .catch(function (e) {{ document.getElementById("dnea-root").textContent =
      "Panel failed to render: " + e.message; }});
}});
</script>
</body>
</html>"#,
        title = title, panel_id = panel_id, bridge = BRIDGE_JS,
    );
    Html(html).into_response()
}

#[derive(Deserialize)]
pub struct InvokeBody {
    method: String,
    #[serde(default)]
    args: serde_json::Value,
}

pub async fn invoke(
    State(state): State<Arc<ApiState>>,
    Path((module_id, _panel_id)): Path<(String, String)>,
    Json(body): Json<InvokeBody>,
) -> Response {
    let Ok(mid) = ModuleId::parse(&module_id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let Some(record) = state.session.module(&mid) else {
        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
    };
    let args = serde_json::to_vec(&body.args).unwrap_or_default();
    match record.handle.invoke(&body.method, args).await {
        Ok(bytes) => {
            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!(null));
            Json(json!({ "ok": true, "value": value })).into_response()
        }
        Err(e) => {
            warn!(module = %mid, method = %body.method, error = %e, "invoke failed");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "ok": false, "error": e.to_string()
            }))).into_response()
        }
    }
}