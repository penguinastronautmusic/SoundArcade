use bevy::prelude::*;
use crate::midware::{TrackType, StemResources};

#[derive(Component)]
pub struct TrackIcon {
    pub track_type: TrackType,
}

#[derive(Component)]
pub struct AudioSquare;

#[derive(Resource, Default)]
pub struct TickTimer(pub Timer);


// 1. Define a resource to hold the handle
#[derive(Resource)]
pub struct MyImage(pub Handle<Image>);


#[derive(Component)]
pub(crate) struct VocalsTrackIcon;
#[derive(Component)]
pub(crate) struct DrumsTrackIcon;
#[derive(Component)]
pub(crate) struct BassTrackIcon;
#[derive(Component)]
pub(crate) struct OtherTrackIcon;

pub fn setup_audio_tracks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    my_image: Res<MyImage>,
    app_start_selections: Res<crate::midware::AppStartSelections>,
) {
    commands.insert_resource(TickTimer(Timer::new(app_start_selections.tick_len, TimerMode::Repeating)));
    let spacing = 240.0;
    // Center the 4 tracks horizontally
    let start_x = -((4.0 - 1.0) * spacing) / 2.0;

    let color_dark_grey = Color::srgb(1.0, 1.0, 1.0);

    let tracks = [
        (TrackType::Vocals, Color::srgb(1.0, 1.0, 1.0)),
        (TrackType::Drums, Color::srgb(1.0, 1.0, 1.0)),
        (TrackType::Bass, Color::srgb(1.0, 1.0, 1.0)),
        (TrackType::Other, Color::srgb(1.0, 1.0, 1.0)),
    ];

    let _icon_mesh = meshes.add(Circle::new(40.));
    let track_mesh = meshes.add(Rectangle::new(5.0, 600.0));

    for (i, (track_type, _color)) in tracks.iter().enumerate() {
        let x = start_x + (i as f32) * spacing;

        // Spawn the track line (faded highway)
        commands.spawn((
            Mesh2d(track_mesh.clone()),
            MeshMaterial2d(materials.add(color_dark_grey.with_alpha(0.1))),
            Transform::from_translation(Vec3::new(x, 100., -1.)),
        ));

        // Spawn the track button/icon at the bottom of the screen
        let mut entity = commands.spawn((
            Sprite {
                image: my_image.0.clone(),
                color: Color::srgb(5.0, 5.0, 5.0),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, -250., -1.)),
            TrackIcon {
                track_type: *track_type,
            },
        ));

        match track_type {
            TrackType::Vocals => entity.insert(VocalsTrackIcon),
            TrackType::Drums => entity.insert(DrumsTrackIcon),
            TrackType::Bass => entity.insert(BassTrackIcon),
            TrackType::Other => entity.insert(OtherTrackIcon),
        };
    }
}

pub fn track_icon_interaction_system(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut icon_query: Query<(&mut Sprite, &GlobalTransform, &TrackIcon)>,
    mut stem_resources: ResMut<StemResources>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let window = if let Ok(w) = window_query.single() { w } else { return };
    let (camera, camera_transform) = if let Ok(c) = camera_query.single() { c } else { return };

    if let Some(cursor) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor) {
            for (mut sprite, transform, track_icon) in icon_query.iter_mut() {
                let icon_pos = transform.translation().truncate();
                let distance = world_position.distance(icon_pos);

                // Assuming the icon is roughly 40 units in radius based on icon_mesh
                if distance < 40.0 {
                    sprite.color = Color::srgb(10.0, 10.0, 10.0);

                    if mouse_button_input.just_pressed(MouseButton::Left) {
                        match track_icon.track_type {
                            TrackType::Vocals => {
                                stem_resources.vocals.is_active = !stem_resources.vocals.is_active;
                                info!("Toggled Vocals is_active to: {}", stem_resources.vocals.is_active);
                            }
                            TrackType::Drums => {
                                stem_resources.drums.is_active = !stem_resources.drums.is_active;
                                info!("Toggled Drums is_active to: {}", stem_resources.drums.is_active);
                            }
                            TrackType::Bass => {
                                stem_resources.bass.is_active = !stem_resources.bass.is_active;
                                info!("Toggled Bass is_active to: {}", stem_resources.bass.is_active);
                            }
                            TrackType::Other => {
                                stem_resources.other.is_active = !stem_resources.other.is_active;
                                info!("Toggled Other is_active to: {}", stem_resources.other.is_active);
                            }
                        }
                    }
                } else {
                    sprite.color = Color::srgb(5.0, 5.0, 5.0);
                }
            }
            return;
        }
    }

    // Reset colors if no hover detected
    for (mut sprite, _, _) in icon_query.iter_mut() {
        sprite.color = Color::srgb(5.0, 5.0, 5.0);
    }
}

pub fn spawn_audio_squares_system(
    mut commands: Commands,
    time: Res<Time>,
    mut tick_timer: ResMut<TickTimer>,
    mut stem_resources: ResMut<StemResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !tick_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let current_tick = stem_resources.current_tick;
    let spacing = 240.0;
    let start_x = -((4.0 - 1.0) * spacing) / 2.0;

    let speed = 200.0;
    let tick_duration = tick_timer.0.duration().as_secs_f32();
    let height = (speed * tick_duration - 10.0).max(5.0); // 10.0 units gap, minimum height 5.0

    let stems = [
        (&stem_resources.vocals, TrackType::Vocals),
        (&stem_resources.drums, TrackType::Drums),
        (&stem_resources.bass, TrackType::Bass),
        (&stem_resources.other, TrackType::Other),
    ];

    for (i, (stem, track_type)) in stems.iter().enumerate() {
        if !stem.is_active {
            continue;
        }

        if let Some(&db) = stem.db_track.get(current_tick) {
            if db >= 20 {
                let x = start_x + (i as f32) * spacing;
                // Width is proportional to DB above 20. Max DB is 100.
                let width = (db as f32 - 20.0) * 3.0 + 10.0;
                let color = match track_type {
                    TrackType::Vocals => Color::srgb(0.0, 1.5, 0.0), // Green
                    TrackType::Drums => Color::srgb(1.5, 0.0, 0.0),  // Red
                    TrackType::Bass => Color::srgb(0.0, 0.0, 1.5),   // Blue
                    TrackType::Other => Color::srgb(1.5, 1.5, 0.0),  // Yellow
                };

                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::new(x, 400.0, 0.0)),
                    AudioSquare,
                ));
            }
        }
    }

    stem_resources.current_tick += 1;
}

pub fn move_and_collide_squares_system(
    mut commands: Commands,
    time: Res<Time>,
    mut square_query: Query<(Entity, &mut Transform), With<AudioSquare>>,
    icon_query: Query<(&GlobalTransform, &TrackIcon)>,
) {
    let speed = 200.0;
    for (entity, mut transform) in square_query.iter_mut() {
        transform.translation.y -= speed * time.delta_secs();

        // Collision check
        let square_pos = transform.translation.truncate();
        for (icon_transform, _icon) in icon_query.iter() {
            let icon_pos = icon_transform.translation().truncate();
            let distance = square_pos.distance(icon_pos);

            if distance < 60.0 {
                commands.entity(entity).despawn();
                break;
            }
        }

        // Cleanup squares that go off-screen
        if transform.translation.y < -500.0 {
            commands.entity(entity).despawn();
        }
    }
}