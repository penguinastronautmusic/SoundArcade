//! The first UI element that the User sees.
//!
//! This is meant to be an element that explains the general flow of the app.

use bevy::{
    prelude::*,
    ui_widgets::{SliderValue,
    },
};
use bevy::ui_widgets::{observe, slider_self_update};
use crate::audio_processing::bpm::Bpm;
use crate::midware::AppStartSelections;
use crate::ui::app_state::AppState;
use crate::ui::bpm_slider;
use crate::ui::bpm_slider::{horizontal_slider, ValueLabel};


#[derive(Component)]
pub struct WelcomeScreenUi;

#[derive(Component)]
pub struct SelectFileButton;

#[derive(Component)]
pub struct PreviewButton;

pub fn spawn_welcome_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            WelcomeScreenUi,
            DespawnOnExit(AppState::Welcome)
        ))
        .with_children(|parent| {
            // Main App Header
            parent.spawn((
                Text::new("Song Arcade"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.45, 0.18, 1.0)),
            ));

            // Tool Instructions Paragraph
            parent.spawn((
                Text::new("
                Adjust the BPM (beats per minute) slider to a higher or lower frequency.\n\
                This will determine how fast or how slow the bars are going to move. \n\
                \n\
                Click 'Select file' to parse a track into drums, vocals, bass and others. \n\
                Note that this can take some time. If you want to see it in action with dummy data,\n\
                Select 'Preview' to preview your track with a random wave. \n\
                \n\
                Supported formats include MP3, WAV, and OGG."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                TextLayout::new_with_justify(Justify::Center),
            ));

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|slider_container| {
                // Live BPM Text Label
                let label_entity = slider_container.spawn((
                    Text::new("120 BPM"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )).id();

                // Horizontal Slider Bundle Linked to Label Entity
                slider_container.spawn((
                    horizontal_slider(),
                    ValueLabel(label_entity),
                    observe(slider_self_update)
                ));
            });

            let button_node = Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };
            let button_borders = BorderColor {
                top: Color::WHITE,
                right: Color::WHITE,
                bottom: Color::WHITE,
                left: Color::WHITE,
            };

            // Select file button
            parent
                .spawn((
                    Button,
                    SelectFileButton,
                    button_node.clone(),
                    button_borders.clone(),
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Select file to start"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Preview button
            parent
                .spawn((
                    Button,
                    PreviewButton,
                    button_node.clone(),
                    button_borders.clone(),
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Preview"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}


pub fn handle_select_file_interaction(
    mut commands: Commands,
    slider_query: Query<&SliderValue, With<bpm_slider::HorizontalSlider>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SelectFileButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let current_bpm = slider_query.single().map(|s| s.0).unwrap_or(120.0);
    let bpm = Bpm::from_f32(current_bpm);

    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(AppStartSelections {
                    tick_len: bpm.to_duration(),
                    is_dummy_backend: false,
                });

                next_state.set(AppState::FileDialog);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgba(0.16, 0.13, 0.18, 1.0));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 1.0));
            }
        }
    }
}

pub fn handle_preview_interaction(
    mut commands: Commands,
    slider_query: Query<&SliderValue, With<bpm_slider::HorizontalSlider>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PreviewButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let current_bpm = slider_query.single().map(|s| s.0).unwrap_or(120.0);
    let bpm = Bpm::from_f32(current_bpm);

    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(AppStartSelections {
                    tick_len: bpm.to_duration(),
                    is_dummy_backend: true,
                });

                next_state.set(AppState::FileDialog);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 1.0));
            }
        }
    }
}
