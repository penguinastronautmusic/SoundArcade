use bevy::prelude::*;
use crate::midware::{TrackType, StemResources};
use crate::ui::audio_controller::{BassState, DrumsState, OthersState, PlaybackState, VocalsState};

#[derive(Component)]
pub struct TrackIcon {
    pub track_type: TrackType,
}

#[derive(Component)]
pub struct AudioSquare;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct BreathingEffect {
    pub time: f32,
}

#[derive(Resource, Default)]
pub struct TickTimer(pub Timer);

#[derive(Resource)]
pub struct MicImage(pub Handle<Image>);
#[derive(Resource)]
pub struct BassImage(pub Handle<Image>);
#[derive(Resource)]
pub struct DrumImage(pub Handle<Image>);
#[derive(Resource)]
pub struct OtherImage(pub Handle<Image>);


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
    mic_image: Res<MicImage>,
    drum_image: Res<DrumImage>,
    bass_image: Res<BassImage>,
    other_image: Res<OtherImage>,
    app_start_selections: Res<crate::midware::AppStartSelections>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(TickTimer(Timer::new(app_start_selections.tick_len, TimerMode::Repeating)));
    
    // Spawn Play Button
    commands.spawn((
        Sprite {
            image: asset_server.load("play_button.png"),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        PlayButton,
    ));

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

        commands.spawn((
            Mesh2d(track_mesh.clone()),
            MeshMaterial2d(materials.add(color_dark_grey.with_alpha(0.1))),
            Transform::from_translation(Vec3::new(x, 100., -1.)),
        ));

        let icon_image = match track_type {
            TrackType::Vocals => mic_image.0.clone(),
            TrackType::Drums => drum_image.0.clone(),
            TrackType::Bass => bass_image.0.clone(),
            TrackType::Other => other_image.0.clone(),
        };

        // Spawn the track button/icon at the bottom of the screen
        let mut entity = commands.spawn((
            Sprite {
                image: icon_image,
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
    mut icon_query: Query<(Entity, &mut Sprite, &GlobalTransform, &TrackIcon)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    breathing_query: Query<&BreathingEffect>,
    mut vocals_state: ResMut<VocalsState>,
    mut drums_state: ResMut<DrumsState>,
    mut bass_state: ResMut<BassState>,
    mut other_state: ResMut<OthersState>,
) {
    let window = if let Ok(w) = window_query.single() { w } else { return };
    let (camera, camera_transform) = if let Ok(c) = camera_query.single() { c } else { return };

    let cursor_world_pos = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok());

    for (entity, mut sprite, transform, track_icon) in icon_query.iter_mut() {
        let is_active = match track_icon.track_type {
            TrackType::Vocals => vocals_state.is_active,
            TrackType::Drums => drums_state.is_active,
            TrackType::Bass => bass_state.is_active,
            TrackType::Other => other_state.is_active,
        };

        // Handle interaction
        if let Some(world_position) = cursor_world_pos {
            let icon_pos = transform.translation().truncate();
            let distance = world_position.distance(icon_pos);

            if distance < 40.0 && mouse_button_input.just_pressed(MouseButton::Left) {
                match track_icon.track_type {
                    TrackType::Vocals => {
                        if !vocals_state.has_changed {
                            vocals_state.is_active = !vocals_state.is_active;
                            vocals_state.has_changed = true;
                            info!("Toggled Vocals is_active to: {}", vocals_state.is_active);
                        }
                    }
                    TrackType::Drums => {
                        if !drums_state.has_changed {
                            drums_state.is_active = !drums_state.is_active;
                            drums_state.has_changed = true;
                            info!("Toggled Drums is_active to: {}", drums_state.is_active);
                        }
                    }
                    TrackType::Bass => {
                        if !bass_state.has_changed {
                            bass_state.is_active = !bass_state.is_active;
                            bass_state.has_changed = true;
                            info!("Toggled Bass is_active to: {}", bass_state.is_active);
                        }
                    }
                    TrackType::Other => {
                        if !other_state.has_changed {
                            other_state.is_active = !other_state.is_active;
                            other_state.has_changed = true;
                            info!("Toggled Other is_active to: {}", other_state.is_active);
                        }
                    }
                }
            }
        }

        // Update color based on state ONLY IF NOT BREATHING
        if !breathing_query.contains(entity) {
            if is_active {
                sprite.color = Color::srgb(5.0, 5.0, 5.0);
            } else {
                sprite.color = Color::srgb(0.2, 0.2, 0.2);
            }
        }
    }
}

pub fn play_button_system(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut play_button_query: Query<(Entity, &GlobalTransform), With<PlayButton>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut playback_state: ResMut<PlaybackState>,
    mut vocals_state: ResMut<VocalsState>,
    mut drums_state: ResMut<DrumsState>,
    mut bass_state: ResMut<BassState>,
    mut other_state: ResMut<OthersState>,
) {
    let window = if let Ok(w) = window_query.single() { w } else { return };
    let (camera, camera_transform) = if let Ok(c) = camera_query.single() { c } else { return };

    let cursor_world_pos = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok());

    if let Some(world_position) = cursor_world_pos {
        for (entity, transform) in play_button_query.iter_mut() {
            let pos = transform.translation().truncate();
            let distance = world_position.distance(pos);

            // 50 click radius for the play button
            if distance < 50.0 && mouse_button_input.just_pressed(MouseButton::Left) {
                vocals_state.is_active = true;
                drums_state.is_active = true;
                bass_state.is_active = true;
                other_state.is_active = true;
                playback_state.is_playing = true;
                playback_state.has_changed = true;
                commands.entity(entity).despawn();
                info!("Play button clicked.");
            }
        }
    }
}

pub fn spawn_audio_squares_system(
    mut commands: Commands,
    time: Res<Time>,
    mut tick_timer: ResMut<TickTimer>,
    mut stem_resources: ResMut<StemResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    icon_query: Query<(Entity, &TrackIcon)>,
    playback_state: Res<PlaybackState>,
    vocals_state: Res<VocalsState>,
    drums_state: Res<DrumsState>,
    bass_state: Res<BassState>,
    other_state: Res<OthersState>,
) {
    if !playback_state.is_playing {
        return;
    }

    if !tick_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let current_tick = stem_resources.current_tick;
    let spacing = 240.0;
    let start_x = -((4.0 - 1.0) * spacing) / 2.0;

    let speed = 200.0;
    let tick_duration = tick_timer.0.duration().as_secs_f32();
    let height = (speed * tick_duration - 15.0).max(5.0); // 15.0 units gap, minimum height 5.0

    let stems = [
        (&stem_resources.vocals, TrackType::Vocals, vocals_state.is_active),
        (&stem_resources.drums, TrackType::Drums, drums_state.is_active),
        (&stem_resources.bass, TrackType::Bass, bass_state.is_active),
        (&stem_resources.other, TrackType::Other, other_state.is_active),
    ];

    for (i, (stem, track_type, is_active)) in stems.iter().enumerate() {
        if !is_active {
            continue;
        }

        if let Some(&db) = stem.db_track.get(current_tick) {
            if db >= 20 {
                let x = start_x + (i as f32) * spacing;
                // Width is proportional to DB above 20. Max DB is 100.
                let width = (db as f32 - 20.0) * 3.0;
                let color = match track_type {
                    TrackType::Vocals => Color::srgb(2.0, 0.0, 1.0),
                    TrackType::Drums => Color::srgb(2.0, 2.0, 0.0),
                    TrackType::Bass => Color::srgb(1.0, 0.0, 2.0),
                    TrackType::Other => Color::srgb(0.0, 1.0, 2.0),
                };

                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(width, height))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::new(x, -180.0, 0.0)),
                    AudioSquare,
                ));

                // Trigger breathing effect on the corresponding icon
                for (icon_entity, icon) in icon_query.iter() {
                    if icon.track_type == *track_type {
                        commands.entity(icon_entity).insert(BreathingEffect {
                            time: 0.2,
                        });
                    }
                }
            }
        }
    }

    stem_resources.current_tick += 1;
}

pub fn move_and_collide_squares_system(
    mut commands: Commands,
    time: Res<Time>,
    mut square_query: Query<(Entity, &mut Transform), With<AudioSquare>>,
    playback_state: Res<PlaybackState>,
) {
    if !playback_state.is_playing {
        return;
    }
    let speed = 200.0;
    for (entity, mut transform) in square_query.iter_mut() {
        transform.translation.y += speed * time.delta_secs();

        // Cleanup squares that go off-screen (top)
        if transform.translation.y > 500.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn icon_breathing_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut BreathingEffect, &TrackIcon)>,
    vocals_state: Res<VocalsState>,
    drums_state: Res<DrumsState>,
    bass_state: Res<BassState>,
    other_state: Res<OthersState>,
) {
    for (entity, mut sprite, mut effect, track_icon) in query.iter_mut() {
        effect.time -= time.delta_secs();
        
        let is_active = match track_icon.track_type {
            TrackType::Vocals => vocals_state.is_active,
            TrackType::Drums => drums_state.is_active,
            TrackType::Bass => bass_state.is_active,
            TrackType::Other => other_state.is_active,
        };

        let base_color = if is_active {
            Color::srgb(5.0, 5.0, 5.0)
        } else {
            Color::srgb(0.2, 0.2, 0.2)
        };

        if effect.time <= 0.0 {
            sprite.color = base_color;
            commands.entity(entity).remove::<BreathingEffect>();
        } else {
            // "Glow" effect: make the icon slightly brighter based on the remaining time
            // effect.time goes from 0.2 to 0.0
            // We want it to be brightest at the start (t=1.0) and fade to base (t=0.0)
            let t = (effect.time / 0.2).clamp(0.0, 1.0);
            
            let base_srgba = base_color.to_srgba();

            // Increase brightness by scaling the RGB values.
            let glow_factor = 1.0 + t * 0.8;
            
            let r = base_srgba.red * glow_factor;
            let g = base_srgba.green * glow_factor;
            let b = base_srgba.blue * glow_factor;
            sprite.color = Color::srgb(r, g, b);
        }
    }
}

