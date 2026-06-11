use bevy::prelude::*;
use crate::midware::TrackType;

#[derive(Component)]
pub struct TrackIcon {
    pub track_type: TrackType,
}


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
    my_image: Res<MyImage>
) {
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

    let icon_mesh = meshes.add(Circle::new(40.));
    let track_mesh = meshes.add(Rectangle::new(5.0, 600.0));

    for (i, (track_type, color)) in tracks.iter().enumerate() {
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
    mut icon_query: Query<(&mut Sprite, &GlobalTransform), With<TrackIcon>>,
) {
    let window = if let Ok(w) = window_query.single() { w } else { return };
    let (camera, camera_transform) = if let Ok(c) = camera_query.single() { c } else { return };

    if let Some(cursor) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor) {
            for (mut sprite, transform) in icon_query.iter_mut() {
                let icon_pos = transform.translation().truncate();
                let distance = world_position.distance(icon_pos);

                // Assuming the icon is roughly 40 units in radius based on icon_mesh
                if distance < 40.0 {
                    sprite.color = Color::srgb(10.0, 10.0, 10.0);
                } else {
                    sprite.color = Color::srgb(5.0, 5.0, 5.0);
                }
            }
            return;
        }
    }

    // Reset colors if no hover detected
    for (mut sprite, _) in icon_query.iter_mut() {
        sprite.color = Color::srgb(5.0, 5.0, 5.0);
    }
}