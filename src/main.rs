use bevy::app::{App, Startup, Update};
use bevy::camera::{Camera, Camera2d, ClearColorConfig};
use bevy::color::Color;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::DefaultPlugins;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

mod audio_processing;
mod reference;
mod ui;
mod backend;
mod midware;

use ui::app_state::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()

        .init_resource::<ui::error::ErrorTracker>()
        .add_systems(OnEnter(AppState::UnrecoverableError), ui::error::spawn_error_screen)
        .add_systems(Update, ui::error::error_screen_button_system.run_if(in_state(AppState::UnrecoverableError)))

        .add_systems(Startup, setup)

        .add_systems(OnEnter(AppState::Welcome), ui::welcome_screen::spawn_welcome_screen)
        .add_systems(Update, ui::welcome_screen::handle_welcome_screen_interaction
            .run_if(in_state(AppState::Welcome)))

        .add_systems(OnEnter(AppState::FileDialog), ui::file_handler::trigger_file_dialog)
        .add_systems(Update, ui::file_handler::poll_file_dialog
            .run_if(in_state(AppState::FileDialog)))

        .add_systems(OnEnter(AppState::ProcessingAudio), ui::process_audio::start_backend_processing)
        .add_systems(
            Update,
            (ui::process_audio::animate_spinner,
             ui::process_audio::monitor_backend).run_if(in_state(AppState::ProcessingAudio)),
        )

        .add_systems(OnEnter(AppState::CoreApplication), setup_core_app)

        .add_systems(OnEnter(AppState::TestComponentDesign), ui::test_design::setup)
        .add_systems(
            Update,
            (ui::test_design::animate_wheel,
                ui::test_design::update_duration_text).run_if(in_state(AppState::TestComponentDesign)),
        )

        .run();
}

fn setup_core_app() {
    println!("Backend processing complete! Launching core game systems...");
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