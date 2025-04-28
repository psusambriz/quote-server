// axum
use axum::{
    routing::get,
    Router,
    response::Html,
};
use askama::Template;
use std::net::SocketAddr;

// html
#[derive(Template)]
#[template(path = "quote.html")]
struct QuoteTemplate {
    quote: &'static str,
}

// quote handler
async fn quote_handler() -> Html<String> {
    let template = QuoteTemplate {
        quote: "This is an inspiring quote!",
    };
    Html(template.render().unwrap())
}

// main 
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(quote_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
        .await
        .unwrap();
}
