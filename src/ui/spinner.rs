//! A simple old-school style spinner that shows the duration of the current process.
//! 
//! This is meant to be used during file processing at the start of the application.

use bevy::prelude::*;
use bevy::time::Stopwatch;
use crate::ui::app_state::AppState;

const SQUARE_SIZE: f32 = 24.0;
const TICK_TIME: f32 = 0.12;

const LEN_SQUARES: usize = 8;

const COLOR_GREY: Color = Color::srgb(0.25, 0.25, 0.25);
const COLOR_PURPLE: Color = Color::srgb(0.63, 0.48, 0.77);


#[derive(Resource)]
pub(crate) struct WheelTracker {
    tick_timer: Timer,
    duration_clock: Stopwatch,
    active_index: usize,
}

impl Default for WheelTracker {
    fn default() -> Self {
        Self {
            tick_timer: Timer::from_seconds(TICK_TIME, TimerMode::Repeating),
            duration_clock: Stopwatch::new(),
            active_index: 0,
        }
    }
}

#[derive(Component)]
pub(crate) struct WheelSquare {
    index: usize,
}

#[derive(Component)]
pub(crate) struct DurationText;

pub(crate) fn setup(mut commands: Commands) {
    commands.insert_resource(WheelTracker {
        tick_timer: Timer::from_seconds(TICK_TIME, TimerMode::Repeating),
        duration_clock: Stopwatch::new(),
        active_index: 0
    },);

    let square_grid : Vec<(i8, i8)> = vec!((-2, 2), (0, 2), (2, 2),
                                           (2, 0),
                                           (2, -2), (0, -2), (-2, -2),
                                           (-2, 0));

    let mut i = 0;
    for (x, y) in square_grid {
        commands.spawn((
            Sprite {
                color: COLOR_GREY,
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..default()
            },
            Transform::from_xyz(x as f32*SQUARE_SIZE*2.0, y as f32*SQUARE_SIZE*2.0, 0.0),
            WheelSquare { index: i },
            DespawnOnExit(AppState::ProcessingAudio)
        ));
        i += 1;
    }

    commands.spawn((
        Text::new("Processing..."),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(200.0),
            justify_self: JustifySelf::Center,
            ..default()
        },
        DespawnOnExit(AppState::ProcessingAudio)
    ));

    commands.spawn((
        Text::new("Duration: 0.00s"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(150.0),
            justify_self: JustifySelf::Center,
            ..default()
        },
        DurationText,
        DespawnOnExit(AppState::ProcessingAudio)
    ));
}

pub(crate) fn cleanup(mut commands: Commands) {
    commands.remove_resource::<WheelTracker>();
}

pub(crate) fn animate_wheel(
    time: Res<Time>,
    mut tracker: ResMut<WheelTracker>,
    mut query: Query<(&WheelSquare, &mut Sprite)>,
) {
    tracker.tick_timer.tick(time.delta());
    tracker.duration_clock.tick(time.delta());

    if tracker.tick_timer.is_finished() {
        tracker.active_index = (tracker.active_index + 1) % LEN_SQUARES;

        for (wheel_square, mut sprite) in query.iter_mut() {
            if wheel_square.index == tracker.active_index {
                sprite.color = COLOR_PURPLE;
            } else {
                sprite.color = COLOR_GREY;
            }
        }
    }
}


pub(crate) fn update_duration_text(
    tracker: Res<WheelTracker>,
    mut query: Query<&mut Text, With<DurationText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        let elapsed = tracker.duration_clock.elapsed_secs();
        text.0 = format!("Duration: {:.2}s", elapsed);
    }
}