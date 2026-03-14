use axum::{
    routing::get,
    Router,
    extract::State,
    response::IntoResponse,
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
};
use tokio::net::TcpListener;
use tracing_subscriber;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Shared state placeholder (we'll expand later)
    let shared_state = Arc::new(());

    let app = Router::new()
        .route("/", get(|| async { "Server is running" }))
        .route("/ws", get(ws_handler))
        .with_state(shared_state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// WebSocket handler
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<Arc<()>>,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

// Echo handler for testing
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            println!("Received: {}", text);
            // Echo back
            if let Err(e) = socket.send(Message::Text(text)).await {
                eprintln!("Failed to send message: {}", e);
            }
        }
    }
}