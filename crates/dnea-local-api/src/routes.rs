//! HTTP routes.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use dnea_core::ids::ModuleId;
use dnea_session::consent::{ConsentDecision, ConsentRequest};
use dnea_session::install::InstallSource;
use serde::Deserialize;
use serde_json::json;
use tower_http::{compression::CompressionLayer, limit::RequestBodyLimitLayer, trace::TraceLayer};
use uuid::Uuid;

use crate::{
    auth::{mint_token, verify_token, Claims, Scope},
    state::ApiState,
    ws,
};

pub fn router(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/v1/version", get(version))
        .route("/v1/auth/token", post(mint))
        .route("/v1/auth/refresh", post(refresh))
        .route("/", get(crate::static_assets::index))
        .route("/app.js", get(crate::static_assets::app_js))
        .route("/style.css", get(crate::static_assets::style_css))
        .route("/v1/status", get(status))
        .route("/v1/modules", get(list_modules).post(install_module))
        .route("/v1/modules/:id", get(get_module))
        .route("/v1/modules/:id/capabilities", get(get_capabilities))
        .route("/v1/modules/:id/capabilities/:cap/origins",
            get(list_origins).post(add_origin))
        .route("/v1/modules/:id/capabilities/:cap/origins/:origin",
            delete(remove_origin))
        .route("/v1/modules/installs/:install_id/resume", post(resume_install))
        .route("/v1/consent/pending", get(list_consent))
        .route("/v1/consent/:id/respond", post(respond_consent))
        .route("/modules/:id/ui/:panel", get(crate::module_ui::serve_panel))
        .route("/modules/:id/ui/:panel/invoke", post(crate::module_ui::invoke))
        .route("/ws", get(ws::handler))
        .layer(RequestBodyLimitLayer::new(1 << 20))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

struct Authed(Claims);

#[axum::async_trait]
impl axum::extract::FromRequestParts<Arc<ApiState>> for Authed {
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<ApiState>,
    ) -> Result<Self, Self::Rejection> {
        let token = parts.headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "missing bearer token").into_response())?;
        let claims = verify_token(&state.secret, token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid token").into_response())?;
        Ok(Authed(claims))
    }
}

impl Authed {
    fn require(&self, scope: Scope) -> Result<(), Response> {
        if self.0.scopes.contains(&scope) { Ok(()) } else {
            Err((StatusCode::FORBIDDEN, Json(json!({ "error": {
                "code": "E_NOT_GRANTED",
                "message": format!("missing scope {}", scope.as_str())
            }}))).into_response())
        }
    }
}

async fn version() -> Json<serde_json::Value> {
    Json(json!({
        "runtimeVersion": env!("CARGO_PKG_VERSION"),
        "protocolVersion": dnea_core::PROTOCOL_VERSION,
    }))
}

#[derive(Deserialize)]
struct MintRequest {
    #[serde(default)] grant_type: Option<String>,
    #[serde(default)] device_id: Option<String>,
    #[serde(default)] requested_scopes: Option<Vec<String>>,
}

async fn mint(State(state): State<Arc<ApiState>>, Json(req): Json<MintRequest>) -> Response {
    let subject = req.device_id.clone().unwrap_or_else(|| "dnea-client".to_string());
    let scopes: Vec<Scope> = req.requested_scopes.as_ref()
        .map(|s| s.iter().filter_map(|s| parse_scope(s)).collect())
        .unwrap_or_else(Scope::companion_set);
    let session = state.session.session_id().to_string();
    match mint_token(&state.secret, &subject, &session, scopes, state.token_ttl) {
        Ok(token) => Json(json!({ "token": token, "expiresIn": state.token_ttl.as_secs() }))
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn refresh(State(state): State<Arc<ApiState>>, headers: axum::http::HeaderMap) -> Response {
    let Some(token) = headers.get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer ")) else {
        return (StatusCode::UNAUTHORIZED, "missing bearer token").into_response();
    };
    let Ok(claims) = verify_token(&state.secret, token) else {
        return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
    };
    match mint_token(&state.secret, &claims.sub, &claims.session, claims.scopes, state.token_ttl) {
        Ok(token) => Json(json!({ "token": token, "expiresIn": state.token_ttl.as_secs() }))
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn status(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let clients: Vec<_> = state.session.clients().into_iter()
        .map(|c| json!({
            "clientId": c.id.to_string(),
            "browserFamily": c.browser_family,
            "extensionId": c.extension_id,
            "bridgePid": c.bridge_pid,
        })).collect();
    let modules: Vec<_> = state.session.modules().into_iter()
        .map(|m| json!({ "id": m.id.to_string(), "version": m.version })).collect();
    Json(json!({
        "runtimeVersion": env!("CARGO_PKG_VERSION"),
        "protocolVersion": dnea_core::PROTOCOL_VERSION,
        "sessionId": state.session.session_id().to_string(),
        "clients": clients,
        "modules": modules,
    })).into_response()
}

async fn list_modules(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let list: Vec<_> = state.session.modules().into_iter()
        .map(|m| {
            let grants = state.session.grants(&m.id);
            json!({
                "id": m.id.to_string(),
                "version": m.version,
                "capabilityCount": grants.as_ref().map(|g| g.len()).unwrap_or(0),
            })
        }).collect();
    Json(json!({ "modules": list })).into_response()
}

async fn get_module(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(id): Path<String>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module ID").into_response();
    };
    let Some(m) = state.session.module(&module_id) else {
        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
    };
    let grants: Vec<_> = state.session.grants(&module_id).map(|g| {
        g.iter().map(|(cap, grant)| json!({
            "capability": cap,
            "constraints": grant.constraints,
            "source": grant.source,
        })).collect()
    }).unwrap_or_default();
    Json(json!({ "id": m.id.to_string(), "version": m.version, "grants": grants }))
        .into_response()
}

async fn get_capabilities(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(id): Path<String>,
) -> Response {
    get_module(State(state), Authed(claims), Path(id)).await
}

async fn list_origins(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path((id, cap)): Path<(String, String)>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let origins = state.session.grant_store().origins_for(&module_id, &cap);
    Json(json!({ "origins": origins })).into_response()
}

#[derive(Deserialize)]
struct AddOriginRequest {
    origin: String,
    #[serde(default)] remember_for: Option<String>,
}

async fn add_origin(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path((id, cap)): Path<(String, String)>,
    Json(req): Json<AddOriginRequest>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let expires_at = match req.remember_for.as_deref() {
        Some("always") => None,
        Some("session") => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(86400)),
        Some("module-version") => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(90 * 86400)),
        _ => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(3600)),
    };
    let grant = state.session.grant_store()
        .add_origin(&module_id, &cap, &req.origin, expires_at);
    state.session.broadcast_origin_granted(&grant);
    Json(grant).into_response()
}

async fn remove_origin(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path((id, cap, origin)): Path<(String, String, String)>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    if state.session.grant_store().remove_origin(&module_id, &cap, &origin) {
        state.session.broadcast_origin_revoked(&module_id.to_string(), &cap, &origin);
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[derive(Deserialize)]
struct InstallRequest {
    source: String,
    #[serde(default, rename = "manifestPath")] manifest_path: Option<String>,
    #[serde(default, rename = "artifactDir")] artifact_dir: Option<String>,
}

async fn install_module(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Json(req): Json<InstallRequest>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiModules) { return r; }
    let Some(pipeline) = state.install_pipeline.clone() else {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "error": {
            "code": "E_INTERNAL", "message": "install pipeline not configured" } })))
            .into_response();
    };
    let source = match req.source.as_str() {
        "local" => {
            let manifest_path = match req.manifest_path {
                Some(p) => p.into(),
                None => return (StatusCode::BAD_REQUEST, "manifestPath required").into_response(),
            };
            let artifact_dir = req.artifact_dir.map(Into::into)
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            InstallSource::Local { manifest_path, artifact_dir }
        }
        other => return (StatusCode::BAD_REQUEST, format!("unsupported source: {other}")).into_response(),
    };
    match pipeline.install(source).await {
        Ok(report) => Json(report).into_response(),
        Err(e) => {
            let status = match &e {
                dnea_session::install::InstallError::Signature(_) => StatusCode::UNAUTHORIZED,
                dnea_session::install::InstallError::ConsentDenied { .. } => StatusCode::FORBIDDEN,
                dnea_session::install::InstallError::AdminRequired { .. } => StatusCode::FORBIDDEN,
                dnea_session::install::InstallError::Conflict { .. } => StatusCode::CONFLICT,
                dnea_session::install::InstallError::MissingDependency { .. } => StatusCode::FAILED_DEPENDENCY,
                _ => StatusCode::BAD_REQUEST,
            };
            (status, Json(json!({ "error": {
                "code": "E_INSTALL_FAILED", "message": e.to_string() } })))
                .into_response()
        }
    }
}

async fn resume_install(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(install_id): Path<Uuid>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiModules) { return r; }
    let Some(pipeline) = state.install_pipeline.clone() else {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "error": {
            "code": "E_INTERNAL", "message": "install pipeline not configured" } })))
            .into_response();
    };
    match pipeline.resume(install_id).await {
        Ok(report) => Json(report).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "error": {
            "code": "E_INSTALL_FAILED", "message": e.to_string() } })))
            .into_response(),
    }
}

async fn list_consent(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let list: Vec<ConsentRequest> = state.session.open_consents();
    Json(json!({ "prompts": list })).into_response()
}

#[derive(Deserialize)]
struct ConsentResponseBody {
    granted: bool,
    #[serde(default)] scope: Option<serde_json::Value>,
    #[serde(default)] remember_for: Option<String>,
}

async fn respond_consent(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(id): Path<Uuid>,
    Json(body): Json<ConsentResponseBody>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
    let decision = ConsentDecision {
        prompt_id: id,
        granted: body.granted,
        scope: body.scope,
        remember_for: body.remember_for.and_then(|s| match s.as_str() {
            "session" => Some(dnea_session::consent::RememberFor::Session),
            "module-version" => Some(dnea_session::consent::RememberFor::ModuleVersion),
            "always" => Some(dnea_session::consent::RememberFor::Always),
            _ => None,
        }),
    };
    match state.session.resolve_consent(decision) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

fn parse_scope(s: &str) -> Option<Scope> {
    Some(match s {
        "ui.read" => Scope::UiRead,
        "ui.consent" => Scope::UiConsent,
        "ui.modules" => Scope::UiModules,
        "ui.settings" => Scope::UiSettings,
        "ui.audit" => Scope::UiAudit,
        "ui.admin" => Scope::UiAdmin,
        _ => return None,
    })
}