use bevy::app::{App, Startup, Update};
use bevy::camera::{Camera, Camera2d, ClearColorConfig};
use bevy::color::Color;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::DefaultPlugins;
use bevy::input_focus::InputDispatchPlugin;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;

mod audio_processing;
mod ui;
mod backend;
mod midware;

use ui::app_state::AppState;

fn main() {
    info!("Starting application...");

    App::new()
        .add_plugins((
            DefaultPlugins,
            UiWidgetsPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin,
        ))
        .init_state::<AppState>()

        .init_resource::<ui::error::ErrorTracker>()
        .add_systems(OnEnter(AppState::UnrecoverableError), ui::error::spawn_error_screen)
        .add_systems(Update, ui::error::error_screen_button_system.run_if(in_state(AppState::UnrecoverableError)))

        .add_systems(Startup, setup)

        .add_systems(OnEnter(AppState::Welcome), ui::welcome_screen::spawn_welcome_screen)
        .add_systems(Update, (
            ui::welcome_screen::handle_preview_interaction,
            ui::welcome_screen::handle_select_file_interaction,
            ui::bpm_slider::update_slider_visuals,
            ui::bpm_slider::update_value_labels
        ).run_if(in_state(AppState::Welcome)))

        .add_systems(OnEnter(AppState::FileDialog), ui::file_handler::trigger_file_dialog)
        .add_systems(Update, ui::file_handler::poll_file_dialog
            .run_if(in_state(AppState::FileDialog)))

        .add_systems(OnEnter(AppState::ProcessingAudio),
                     (ui::process_audio::start_backend_processing,
                            ui::spinner::setup))
        .add_systems(
            Update,
            (ui::spinner::animate_wheel,
             ui::spinner::update_duration_text,
             ui::process_audio::monitor_backend).run_if(in_state(AppState::ProcessingAudio)),
        )
        .add_systems(
        OnExit(AppState::ProcessingAudio), ui::spinner::cleanup)

        .add_systems(OnEnter(AppState::CoreApplication), setup_core_app)

        .run();
}

fn setup_core_app() {
    info!("Launching core app systems...");
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface, // 1. Using a tonemapper that desaturates to white is recommended
        Bloom::default(),           // 2. Enable bloom for the camera
        DebandDither::Enabled,      // Optional: bloom causes gradients which cause banding
    ));
}