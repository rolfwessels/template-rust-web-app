use app_core::StringListApp;
use rand::distr::{Alphanumeric, SampleString};
use std::{env, error::Error, net::SocketAddr};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use template_rust_web_app::{graphql::schema, memory_list::InMemoryList, router};

const VERSION: &str = match option_env!("APP_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if env::args_os().nth(1).as_deref() == Some("--version".as_ref()) {
        println!("{} {VERSION}", env!("CARGO_PKG_NAME"));
        return Ok(());
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .json()
        .init();

    let addr: SocketAddr = env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
        .parse()?;
    let listener = TcpListener::bind(addr).await?;
    let list = InMemoryList::new(vec!["river-spark".into(), "paper-kite".into()]);
    let app = StringListApp::new(list, || Alphanumeric.sample_string(&mut rand::rng(), 10));

    tracing::info!(address = %listener.local_addr()?, version = VERSION, "Listening");
    axum::serve(listener, router(schema(app))).await?;
    Ok(())
}
