use bytes::Bytes;
use futures_util::StreamExt;
use axum::{extract::BodyStream, routing::get, routing::post, Router};
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let key = "KEY";
    env::set_var(key, "VALUE");

    for (key, _value) in env::vars() {
        println!("{}", key);
    }

    let port = env::var("SETIP_LISTEN_PORT").unwrap_or_else(|_| "8099".to_string());
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;

    println!("Listening on Port: {}", port);

    // Build our application with a route
    let app = Router::new()
        .route("/", get(help))
        .route("/top", get(top))
        .route("/echo", post(echo));

    // Run it
    let tcp_listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(err) => {
            // println!("Failed to bind to port {}: {}", port, err);
            return Ok(());
        }
    };

    println!("Listening on {}", addr);
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn top() -> &'static str {
    "Winners..."
}

async fn help() -> &'static str {
    "Try POSTing data to /echo such as: `curl localhost:PORT/echo -XPOST -d 'hello world'`\n"
}

async fn echo(mut stream: BodyStream) -> Bytes {
    if let Some(Ok(s)) = stream.next().await {
        s
    } else {
        Bytes::new()
    }
}
