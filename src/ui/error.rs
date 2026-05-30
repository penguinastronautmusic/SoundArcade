//! A set of components and functions used to display an error to the user that require
//! the app to go to recovery state.
//!
//! Usually a result of something wrong being caught with <Result> rust object.


use bevy::prelude::*;
use crate::ui::app_state::AppState;


#[derive(Resource, Default, Debug)]
pub(crate) struct ErrorTracker {
    pub message: String,
}


#[derive(Component)]
pub(crate) struct ErrorUiRoot;


pub(crate) fn spawn_error_screen(
    mut commands: Commands,
    error_tracker: Res<ErrorTracker>,
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        DespawnOnExit(AppState::UnrecoverableError),
        ErrorUiRoot,
    )).with_children(|parent| {
        parent.spawn(Text::new("Oops! An unrecoverable error occurred."));

        parent.spawn((
            Node {
                max_width: Val::Percent(80.0),
                max_height: Val::Percent(50.0),
                padding: UiRect::all(Val::Px(15.0)),
                overflow: Overflow::scroll(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.1, 0.1, 1.0)),
        )).with_children(|p| {
            p.spawn(Text::new(error_tracker.message.clone()));
        });

        parent.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
        )).with_children(|p| {
            p.spawn(Text::new("OK"));
        });
    });
}

pub(crate) fn error_screen_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut error_tracker: ResMut<ErrorTracker>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Clear the old error data
            error_tracker.message.clear();
            // Route user back to welcome screen
            next_state.set(AppState::Welcome);
        }
    }
}

