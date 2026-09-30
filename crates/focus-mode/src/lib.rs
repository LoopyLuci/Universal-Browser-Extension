//! Focus Mode — reference DNEA module.

wit_bindgen::generate!({
    path: "../../wit",
    world: "module",
});

use exports::dnea::host::lifecycle::Guest as LifecycleGuest;
use exports::dnea::host::module_api::Guest as ModuleApiGuest;

struct FocusMode;

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct State {
    #[serde(default)]
    block_list: Vec<String>,
    #[serde(default)]
    allow_list: Vec<String>,
    #[serde(default)]
    strict_mode: bool,
    #[serde(default)]
    sessions_started: u64,
}

const STATE_KEY: &str = "state";

fn load_state() -> State {
    match dnea::host::storage::get(STATE_KEY) {
        Ok(Some(bytes)) => serde_json::from_slice(&bytes).unwrap_or_default(),
        _ => State::default(),
    }
}

fn save_state(state: &State) -> Result<(), dnea::host::types::HostError> {
    let bytes = serde_json::to_vec(state)
        .map_err(|e| dnea::host::types::HostError::Internal(e.to_string()))?;
    dnea::host::storage::put(STATE_KEY, &bytes)
}

impl LifecycleGuest for FocusMode {
    fn activate() -> Result<(), dnea::host::types::HostError> {
        let state = load_state();
        dnea::host::log::emit(
            dnea::host::types::LogLevel::Info,
            "focus-mode activated",
            Some(&format!(r#"{{"sessions":{}}}"#, state.sessions_started)),
        );
        Ok(())
    }

    fn deactivate() -> Result<(), dnea::host::types::HostError> {
        dnea::host::log::emit(
            dnea::host::types::LogLevel::Info,
            "focus-mode deactivated",
            None,
        );
        Ok(())
    }

    fn upgrade(from_version: String, _from_storage_schema: u32) -> Result<(), dnea::host::types::HostError> {
        dnea::host::log::emit(
            dnea::host::types::LogLevel::Info,
            "focus-mode upgraded",
            Some(&format!(r#"{{"from":"{}"}}"#, from_version)),
        );
        Ok(())
    }

    fn health_check() -> Result<(), dnea::host::types::HostError> {
        let _ = load_state();
        Ok(())
    }
}

impl ModuleApiGuest for FocusMode {
    fn invoke(method: String, args_json: String) -> Result<String, dnea::host::types::HostError> {
        match method.as_str() {
            "getState" => {
                let state = load_state();
                Ok(serde_json::to_string(&state).unwrap_or_else(|_| "{}".into()))
            }
            "setBlockList" => {
                let args: serde_json::Value = serde_json::from_str(&args_json)
                    .map_err(|e| dnea::host::types::HostError::Internal(e.to_string()))?;
                let mut state = load_state();
                state.block_list = args
                    .get("list")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                save_state(&state)?;
                Ok(r#"{"ok":true}"#.into())
            }
            "startSession" => {
                let mut state = load_state();
                state.sessions_started += 1;
                save_state(&state)?;
                Ok(format!(
                    r#"{{"ok":true,"session":{}}}"#,
                    state.sessions_started
                ))
            }
            "onTabNavigated" => {
                let args: serde_json::Value = serde_json::from_str(&args_json)
                    .map_err(|e| dnea::host::types::HostError::Internal(e.to_string()))?;
                let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("");
                let state = load_state();
                let host = host_of(url);
                let blocked = state.block_list.iter().any(|h| host.ends_with(h.as_str()))
                    && !state.allow_list.iter().any(|h| host.ends_with(h.as_str()));
                if blocked {
                    dnea::host::log::emit(
                        dnea::host::types::LogLevel::Info,
                        "blocked host",
                        Some(&format!(r#"{{"host":"{}"}}"#, host)),
                    );
                }
                Ok(format!(r#"{{"blocked":{}}}"#, blocked))
            }
            "render" => {
                let state = load_state();
                let blocked = state
                    .block_list
                    .iter()
                    .map(|h| format!("<li>{}</li>", html_escape(h)))
                    .collect::<Vec<_>>()
                    .join("");
                Ok(format!(
                    r#"<h2>Focus Mode</h2>
<p>Strict mode: {strict}</p>
<p>Sessions: {sessions}</p>
<ul>{blocked}</ul>"#,
                    strict = state.strict_mode,
                    sessions = state.sessions_started,
                    blocked = blocked
                ))
            }
            other => Err(dnea::host::types::HostError::Unavailable(format!("unknown method: {other}"))),
        }
    }
}

fn host_of(url: &str) -> &str {
    url.split("://")
        .nth(1)
        .and_then(|s| s.split('/').next())
        .unwrap_or("")
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
