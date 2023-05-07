use bevy::prelude::*;
use bevy_flicker::prelude::*;

const FIXED_TIMESTEP: f32 = 0.5;
const FLICKER_LENGTH: f32 = 3.0;
const MIX_SCALAR: f32 = 0.10;

#[derive(Component, Default)]
pub struct Marker;

const RANDOM_COLORS: [Color; 6] = [
    Color::WHITE,
    Color::BLUE,
    Color::RED,
    Color::YELLOW,
    Color::BLACK,
    Color::PURPLE,
];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlickerPlugin)
        .add_system(setup.on_startup())
        .add_system(tick.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
            FlickerStartEvent::new(e)
                .with_secs(FLICKER_LENGTH)
                .with_mix_scalar(MIX_SCALAR)
                .with_color(RANDOM_COLORS[*counter]),
        );
    }
}
