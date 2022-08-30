use axum::routing::get;
use axum::handler::Handler;
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt};

fn init_tracing() {
    use axum_tracing_opentelemetry::{make_resource, otlp};
    use tracing_subscriber::fmt::format::FmtSpan;
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or("INFO".to_string()),
    );

    let otel_rsrc = make_resource(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let otel_tracer = otlp::init_tracer(otel_rsrc, otlp::identity).expect("setup of Tracer");
    let otel_layer = tracing_opentelemetry::layer().with_tracer(otel_tracer);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(EnvFilter::from_default_env())
        .with(otel_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() {
    init_tracing();

    // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(
            fallback.into_service()
        )
        .route("/healthcheck",
            get(healthcheck)
        )
        .layer(opentelemetry_tracing_layer());

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C or for kill/terminate signal.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Expect shutdown CTRL+C handler");
    };

    #[cfg(unix)] /* conditional compilation depending on target family = unix */
    let terminate = async {
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

    trac!("signal shutdown");
}

pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri)
    )
}

pub async fn healthcheck() -> String {
    "OK".to_string()
}