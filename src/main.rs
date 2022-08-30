use axum::handler::Handler;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let frontend = async {
        let app = Router::new().route("/", get(html));
        let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    };

    let backend = async {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        // Build our application by creating our router.
        let app = axum::Router::new()
            .fallback(fallback.into_service())
            .route("/healthcheck", get(healthcheck));

        tracing::info!("listening on 3000");

        // Run our application as a hyper server on http://localhost:3000.
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    };

    tokio::join!(frontend, backend);
}

async fn html() -> impl IntoResponse {
    Html(
        r#"
        <script>
            fetch('http://localhost:3000/user')
              .then(response => response.json())
              .then(data => console.log(data));
        </script>
        "#,
    )
}

/// Tokio signal handler that will wait for a user to press CTRL+C or for kill/terminate signal.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Expect shutdown CTRL+C handler");
    };

    #[cfg(unix)] /* conditional compilation depending on target family = unix */ let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Expected shutdown signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::warn!("signal received, starting graceful shutdown");
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

pub async fn healthcheck() -> String {
    "OK".to_string()
}

#[cfg(test)]
mod test;
