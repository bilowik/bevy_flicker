use std::sync::LazyLock;
use bevy::{
    prelude::*,
    color::palettes::basic::{YELLOW, PURPLE},
};
use bevy_flicker::prelude::*;


const FIXED_TIMESTEP: f64 = 0.5;
const FLICKER_LENGTH: f32 = 3.0;

#[derive(Component, Default)]
pub struct Marker;


static RANDOM_COLORS: LazyLock<[Color; 6]> = LazyLock::new(|| {[
    Color::WHITE,
    LinearRgba::BLUE.into(),
    LinearRgba::RED.into(),
    YELLOW.into(),
    PURPLE.into(),
    Color::BLACK,
]});

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlickerPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, tick)
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("asteroid5.png"),
            transform: Transform::default().with_scale(Vec3::splat(8.0)),
            ..default()
        })
        .insert(Marker);
}

fn tick(
    query: Query<Entity, With<Marker>>,
    mut event_writer: EventWriter<FlickerStartEvent>,
    mut counter: Local<usize>,
) {
    *counter = *counter + 1;
    if *counter == RANDOM_COLORS.as_slice().len() {
        *counter = 0;
    }
    for e in query.iter() {
        event_writer.send(
            FlickerStartEvent::builder(e)
                .with_secs(FLICKER_LENGTH)
                .with_color(RANDOM_COLORS[*counter])
                .build(),
        );
    }
}
