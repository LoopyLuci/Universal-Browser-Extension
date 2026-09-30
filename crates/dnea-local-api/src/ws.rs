//! WebSocket event stream.

use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::Response,
};
use dnea_session::SessionEvent;
use serde_json::json;
use tokio::sync::broadcast::error::RecvError;
use tracing::debug;
use uuid::Uuid;

use crate::state::ApiState;

pub async fn handler(State(state): State<Arc<ApiState>>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| run(socket, state))
}

async fn run(mut socket: WebSocket, state: Arc<ApiState>) {
    let mut rx = state.session.subscribe();
    let mut heartbeat = tokio::time::interval(std::time::Duration::from_secs(20));

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Ok(ev) => {
                        let frame = json!({
                            "v": 1,
                            "id": Uuid::now_v7().to_string(),
                            "ts": time::OffsetDateTime::now_utc()
                                .format(&time::format_description::well_known::Rfc3339)
                                .unwrap_or_default(),
                            "kind": "event",
                            "topic": topic_of(&ev),
                            "payload": ev,
                        });
                        if socket.send(Message::Text(frame.to_string().into())).await.is_err() {
                            return;
                        }
                    }
                    Err(RecvError::Lagged(n)) => { debug!(missed = n, "ws subscriber lagged"); }
                    Err(RecvError::Closed) => return,
                }
            }
            _ = heartbeat.tick() => {
                let ping = json!({ "v": 1, "id": Uuid::now_v7().to_string(), "kind": "ping" });
                if socket.send(Message::Text(ping.to_string().into())).await.is_err() { return; }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => return,
                    Some(Ok(Message::Ping(p))) => { let _ = socket.send(Message::Pong(p)).await; }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => return,
                }
            }
        }
    }
}

fn topic_of(ev: &SessionEvent) -> &'static str {
    match ev {
        SessionEvent::ModuleLoaded { .. } => "module/loaded",
        SessionEvent::ModuleUnloaded { .. } => "module/unloaded",
        SessionEvent::GrantChanged(_) => "capability/changed",
        SessionEvent::ConsentOpened(_) => "consent/opened",
        SessionEvent::ConsentResolved { .. } => "consent/resolved",
        SessionEvent::ClientConnected { .. } => "client/connected",
        SessionEvent::ClientDisconnected { .. } => "client/disconnected",
        SessionEvent::OriginGranted { .. } => "origin/granted",
        SessionEvent::OriginRevoked { .. } => "origin/revoked",
    }
}