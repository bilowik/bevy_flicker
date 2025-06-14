// In an earlier version of this plugin, the inner mechanics of flickering
// were much more complex and so flickering a TextureAtlasSprite was a much
// more significant chore code-wise, which is why this example existed to test
// that.

use bevy::prelude::*;
use bevy_flicker::prelude::*;

const FIXED_TIMESTEP: f64 = 1.0;
const FLICKER_LENGTH: f32 = 0.5;

#[derive(Component, Default)]
pub struct Marker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlickerPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, tick)
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("asteroid_sheet_test.png");
    let atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),
        1,
        4,
        None,
        None,
    ));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            texture_atlas: Some(TextureAtlas {
                index: 2,
                layout: atlas_layout,
                ..default()
            }),
            image: texture,
            ..default()
        },
        Transform::default().with_scale(Vec3::splat(8.0)),
        Marker,
    ));
}

fn tick(query: Query<Entity, With<Marker>>, mut event_writer: EventWriter<FlickerStartEvent>) {
    for e in query.iter() {
        info!("Flickering the square!");
        event_writer.write(
            FlickerStartEvent::builder(e)
                .with_secs(FLICKER_LENGTH)
                .with_color(LinearRgba::new(1.0, 0.0, 0.0, 0.2).into())
                .build(),
        );
    }
}
