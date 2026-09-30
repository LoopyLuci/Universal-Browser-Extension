//! Companion end-to-end test.

use std::sync::Arc;
use std::time::Duration;

use dnea_core::ids::ModuleId;
use dnea_local_api::{ApiConfig, LocalApi};
use dnea_module_host::{grant::GrantSet, InstanceHandle};
use dnea_session::{
    consent::{ConsentRequest, RememberFor},
    registry::ModuleRecord,
    Session,
};
use serde_json::json;
use tempfile::TempDir;

fn dummy_handle() -> Arc<InstanceHandle> {
    InstanceHandle::new_for_test(
        "test".into(), "0.0.0-test".into(), Arc::new(GrantSet::new()))
}

#[tokio::test(flavor = "multi_thread")]
async fn companion_can_list_modules_and_respond_to_consent() {
    let session = Session::new();
    session.register_module(Arc::new(ModuleRecord {
        id: ModuleId::parse("com.example.focus-mode").unwrap(),
        version: "0.0.0-test".into(),
        handle: dummy_handle(),
        manifest: None,
        loaded_at: std::time::SystemTime::now(),
    }));

    let tmp = TempDir::new().unwrap();
    let config = ApiConfig {
        bind: "127.0.0.1:0".parse().unwrap(),
        discovery_path: tmp.path().join("api.json"),
        token_ttl: Duration::from_secs(600),
    };
    let api = LocalApi::bind(session.clone(), config).await.unwrap();
    let addr = api.local_addr().unwrap();
    let server = tokio::spawn(api.serve());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let token_res = client
        .post(format!("http://{addr}/v1/auth/token"))
        .json(&json!({
            "grantType": "companion",
            "requestedScopes": ["ui.read", "ui.consent", "ui.modules"]
        }))
        .send().await.unwrap();
    assert_eq!(token_res.status(), 200);
    let token_body: serde_json::Value = token_res.json().await.unwrap();
    let token = token_body["token"].as_str().unwrap().to_string();

    let modules_res = client
        .get(format!("http://{addr}/v1/modules"))
        .bearer_auth(&token)
        .send().await.unwrap();
    assert_eq!(modules_res.status(), 200);
    let modules: serde_json::Value = modules_res.json().await.unwrap();
    assert_eq!(modules["modules"].as_array().unwrap().len(), 1);

    let prompt_id = uuid::Uuid::now_v7();
    let request = ConsentRequest {
        prompt_id,
        module: "com.example.focus-mode".into(),
        module_name: "Focus Mode".into(),
        capability: "browser:tabs:query".into(),
        tier: "prompt".into(),
        reason: "Detect when you open a blocked site.".into(),
        constraints: json!({ "tabScopes": ["all-windows"] }),
        target_description: Some("Query open tabs".into()),
        expires_at: std::time::SystemTime::now() + Duration::from_secs(300),
        can_remember: vec![RememberFor::Session, RememberFor::ModuleVersion],
    };
    let session_clone = session.clone();
    let consent_task = tokio::spawn(async move {
        session_clone.request_consent(request).await
    });

    tokio::time::sleep(Duration::from_millis(50)).await;
    let pending_res = client
        .get(format!("http://{addr}/v1/consent/pending"))
        .bearer_auth(&token)
        .send().await.unwrap();
    let pending: serde_json::Value = pending_res.json().await.unwrap();
    assert_eq!(pending["prompts"].as_array().unwrap().len(), 1);

    let respond_res = client
        .post(format!("http://{addr}/v1/consent/{prompt_id}/respond"))
        .bearer_auth(&token)
        .json(&json!({ "granted": true, "remember_for": "module-version" }))
        .send().await.unwrap();
    assert_eq!(respond_res.status(), 204);

    tokio::time::timeout(Duration::from_secs(2), consent_task)
        .await.expect("consent decision").unwrap().unwrap();

    let module_id = ModuleId::parse("com.example.focus-mode").unwrap();
    let grants = session.grants(&module_id).expect("grants");
    assert!(grants.get("browser:tabs:query").is_some());

    server.abort();
}