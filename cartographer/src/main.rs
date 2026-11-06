#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod app;
mod document;
pub mod export;
mod map_list;
mod palette_lookup;
mod panels;
mod prefab;
mod shape;
mod theme;
mod tile_animation;
mod widgets;

use app::EditorApp;
use eframe::egui;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::warn;

const APP_TITLE: &str = "DA Cartographer";
const APP_ID: &str = "org.dacartographer.Cartographer";

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,egui_winit::clipboard=off".into()),
        )
        .init();

    #[cfg(target_os = "macos")]
    let bundled_maps = prepare_mac_bundle().unwrap_or_else(|error| {
        warn!(%error, "could not prepare the app's data folder");
        None
    });
    #[cfg(not(target_os = "macos"))]
    let bundled_maps: Option<PathBuf> = None;

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([960.0, 600.0])
        .with_decorations(false)
        .with_app_id(APP_ID)
        .with_title(APP_TITLE);

    let icon = load_app_icon().map(Arc::new);
    if let Some(icon) = &icon {
        viewport = viewport.with_icon(Arc::clone(icon));
    }

    let title_icon = eframe::icon_data::from_png_bytes(include_bytes!("../title-icon.png")).ok();

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        APP_TITLE,
        options,
        Box::new(move |cc| {
            Ok(Box::new(EditorApp::new(
                cc,
                title_icon.as_ref(),
                bundled_maps.as_deref(),
            )))
        }),
    )
}

#[cfg(target_os = "macos")]
fn prepare_mac_bundle() -> std::io::Result<Option<PathBuf>> {
    use std::{env, fs, io};

    let executable = env::current_exe()?;
    let Some(contents) = executable
        .parent()
        .filter(|directory| directory.ends_with("Contents/MacOS"))
        .and_then(|directory| directory.parent())
    else {
        return Ok(None);
    };

    let home = env::var_os("HOME").ok_or_else(|| io::Error::other("home folder unavailable"))?;
    let data = PathBuf::from(home)
        .join("Library/Application Support")
        .join(APP_TITLE);
    fs::create_dir_all(&data)?;
    env::set_current_dir(data)?;
    Ok(Some(contents.join("Resources/maps.ron")))
}

fn load_app_icon() -> Option<egui::IconData> {
    match eframe::icon_data::from_png_bytes(include_bytes!("../app-icon.png")) {
        Ok(icon) => Some(icon),
        Err(error) => {
            warn!(error = ?error, "failed to decode app-icon.png for viewport icon");
            None
        }
    }
}
