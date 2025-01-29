// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::module_inception)]

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use brt::BrtPlugin;

use crate::{controller::ControllerPlugin, model::ModelPlugin, ui::UiPlugin, view::ViewPlugin};

pub mod controller;
#[cfg(feature = "dev")]
pub mod dev;
pub mod model;
pub mod ui;
pub mod view;

mod app_constants;
pub use self::app_constants::*;

mod app_settings;
pub use self::app_settings::*;

fn main() {
    let mut app = App::new();

    let brt_plugin = BrtPlugin::new(
        AppConstants::BASE,
        AppConstants::DOMAIN,
        AppConstants::COMPANY,
        AppConstants::APP_NAME,
    )
    .with_icon(include_bytes!("../assets/icon.png"));

    // Load AppSettings
    let app_settings = AppSettings::load(brt_plugin.folders());
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: AppConstants::APP_NAME.to_string(),
                    resolution: WindowResolution::new(
                        app_settings.window_width(),
                        app_settings.window_height(),
                    ),
                    mode: if app_settings.fullscreen() {
                        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                    } else {
                        WindowMode::Windowed
                    },
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                file_path: AppConstants::BASE.to_string(),
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.insert_resource(app_settings);
    app.add_plugins(brt_plugin);

    #[cfg(feature = "dev")]
    app.add_plugins(crate::dev::DevPlugin);

    app.add_plugins((ControllerPlugin, ModelPlugin, UiPlugin, ViewPlugin));

    app.run();
}
