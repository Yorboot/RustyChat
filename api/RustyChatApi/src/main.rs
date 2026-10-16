mod structs;
mod database;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router, http};
use axum::routing::get;
use serde_json::json;
use sqlx::Row;
use structs::Adress::Address;
use structs::login_incoming::LoginIncoming;
const DATABASE_URL:&str = "mysql://root:password@localhost:3306/rustychat";
#[tokio::main]
async fn main(){
    //address struct to hold ip and port
    let address = Address{
        ip: "127.0.0.1".into(),
        port: 3000,
    };
    //initialize the database connection POOL
    let pool = database::create_pool(DATABASE_URL).await.unwrap();
    //build main route
    let app = Router::new().route("/", get(root));
    let listen_address = format!("{}:{}", address.ip, address.port);
    //start the actual api listener on port 3000 and local ip
    let listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .unwrap();
    println!("Server listening on http://{}:{}", address.ip, &address.port.to_string());
    //serve the api
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> impl IntoResponse {
    Json(json!({
        "message": "Hello, World!"
    }))
}
async fn Login(Json(body):Json<LoginIncoming>,pool:State<sqlx::mysql::MySqlPool>)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Create a new instance of LoginIncoming with the provided data
    let login_data = LoginIncoming {
        username: body.username,
        password: body.password,
        csrf_token: body.csrf_token,
    };

    let affect_rows: sqlx::mysql::MySqlRow = sqlx::query(
        "SELECT * FROM users WHERE username = ? AND password = ?"
    )
    .bind(&login_data.username)
    .bind(&login_data.password)
    .fetch_one(&*pool)
    .await
        .map_err(|error| {
        eprintln!("database error: {error}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "database error"
            })),
        )
    })?;
    if affect_rows.is_empty(){
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid username or password"
            })),
        ));

    }
    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Login successful"
        })),
    ))
}
