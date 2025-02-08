use bevy::{
    color::palettes::basic::{PURPLE, YELLOW},
    prelude::*,
};
use bevy_flicker::prelude::*;
use std::sync::LazyLock;

const FIXED_TIMESTEP: f64 = 0.5;
const FLICKER_LENGTH: f32 = 3.0;

#[derive(Component, Default)]
pub struct Marker;

static RANDOM_COLORS: LazyLock<[Color; 6]> = LazyLock::new(|| {
    [
        Color::WHITE,
        LinearRgba::BLUE.into(),
        LinearRgba::RED.into(),
        YELLOW.into(),
        PURPLE.into(),
        Color::BLACK,
    ]
});

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlickerPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, tick)
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP))
        .insert_resource(FlickerPluginConfig {
            overlap_action: FlickerOverlapAction::Ignore,
        })
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite::from_image(asset_server.load("asteroid5.png")),
        Marker,
        Transform::default().with_scale(Vec3::splat(8.0)),
    ));
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
