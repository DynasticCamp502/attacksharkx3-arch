//! Attack Shark X3 Linux Configuration Driver and GUI Binary Entry Point.

use anyhow::Result;
use attack_shark_x3::AttackSharkApp;
use eframe::egui::Vec2;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Starting Attack Shark X3 Linux configuration tool v{}", env!("CARGO_PKG_VERSION"));

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Attack Shark X3 — Linux Mouse Configuration")
            .with_inner_size(Vec2::new(960.0, 640.0))
            .with_min_inner_size(Vec2::new(880.0, 560.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Attack Shark X3",
        native_options,
        Box::new(|cc| Ok(Box::new(AttackSharkApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("Eframe error: {}", e))
}
