use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;
//address struct to hold ip and port
struct Address{
    ip: String,
    port: u16,
}
#[tokio::main]
async fn main(){
    let address = Address{
        ip: "127.0.0.1".into(),
        port: 3000,
    };
    //build main route
    let app = Router::new().route("/", get(root));
    let listen_address = format!("{}:{}", address.ip, address.port);
    //start the actual api listener on port 3000 and local ip
    let listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .unwrap();
    println!("Server listening on http://{}:{}", address.ip, &address.port.to_string());
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> impl IntoResponse {
    Json(json!({
        "message": "Hello, World!"
    }))
}