use bevy::ecs::event::{EntityTrigger, Trigger};
use bevy::prelude::*;

#[derive(Component)]
pub struct TrackIcon {
    pub track_type: TrackType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrackType {
    Vocals,
    Drums,
    Bass,
    Other,
}


// 1. Define a resource to hold the handle
#[derive(Resource)]
pub struct MyImage(pub Handle<Image>);

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
        commands.spawn((
            Sprite {
                image: my_image.0.clone(),
                color: Color::srgb(5.0, 5.0, 5.0),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, -250., -1.)),
        ))
            // 1. Detect Clicks
            .observe(|trigger: Trigger<Pointer<Click>>| {
                println!("Track button clicked!");
            })
            // 2. Detect Hover (Pointer Over) to make it feel like a button
            .observe(|trigger: Trigger<Pointer<Over>>, mut sprites: Query<&mut Sprite>| {
                if let Ok(mut sprite) = sprites.get_mut(trigger.entity()) {
                    sprite.color = Color::srgb(8.0, 8.0, 8.0); // Highlight color
                }
            })
            // 3. Detect Hover End (Pointer Out) to revert the color
            .observe(|trigger: Trigger<Pointer<Out>>, mut sprites: Query<&mut Sprite>| {
                if let Ok(mut sprite) = sprites.get_mut(trigger.entity()) {
                    sprite.color = Color::srgb(5.0, 5.0, 5.0); // Normal color
                }
            });
    }
}