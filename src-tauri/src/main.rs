#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use app::prelude::*;
use opentelemetry::global;
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
#[allow(clippy::expect_used)]
fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracer();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            app::api::login,
            app::api::signup,
            app::api::get_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

fn init_tracer() {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("mini-redis")
        .install_simple()
        .unwrap();
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(fmt::Layer::default())
        .with(filter::LevelFilter::INFO)
        .try_init()
        .unwrap();

    // Ok(tracer)
}

#[tauri::command]
fn greet(name: &str) -> String {
    tracing::info!("greet command was called");
    format!("Hello, {name}!")
}
