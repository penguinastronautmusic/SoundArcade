//! The first UI element that the User sees.
//!
//! This is meant to be an element that explains the general flow of the app.

use bevy::prelude::*;
use crate::ui::app_state::AppState;

#[derive(Component)]
pub struct WelcomeScreenUi;

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
            BackgroundColor(Color::BLACK), // Dark background theme
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
                Text::new("Instructions:\n1. Click the button below to choose an audio file.\n2. Supported formats include MP3, WAV, and OGG.\n3."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)), // Dimmed text color
                TextLayout::new_with_justify(Justify::Center),
            ));

            // Interactive Button Container
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(60.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor{
                        top:Color::WHITE,
                        right:Color::WHITE,
                        bottom:Color::WHITE,
                        left:Color::WHITE
                    },
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
        });
}

pub fn handle_welcome_screen_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
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
