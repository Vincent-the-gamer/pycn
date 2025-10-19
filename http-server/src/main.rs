use axum::{Json, Router, routing::post};
use parser::parse_pycn;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", post(to_pycn));

    // run our app with hyper
    let host = "0.0.0.0";
    let port = 2828;
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    println!("App running at {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();
}

async fn to_pycn(Json(payload): Json<Value>) -> Json<Value> {
    let code = payload["code"].as_str().unwrap();

    if code.len() > 0 {
        let python_code = parse_pycn(code);
        Json(json!({
            "pythonCode": python_code
        }))
    } else {
        Json(json!({
            "msg": "Code parameter cannot be empty!"
        }))
    }
}
