mod structs;
mod database;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use axum::routing::get;
use serde_json::json;
use structs::Adress::Address;
use structs::login_incoming::LoginIncoming;
//address struct to hold ip and port

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
async fn Login(Json(body):Json<LoginIncoming>)-> Result<impl IntoResponse, Json<serde_json::Value>> {
    let login_data = LoginIncoming {
        username: body.username,
        password: body.password,
        csrf_token: body.csrf_token,
    };
    // Process the login data
    Ok(Json(json!({
        "message": "Login successful"
    })))
}
