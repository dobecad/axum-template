use axum::{response::Html, routing::get, Router};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
