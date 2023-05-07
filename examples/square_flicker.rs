use bevy::prelude::*;
use bevy_flicker::prelude::*;

const FIXED_TIMESTEP: f32 = 1.0;
const FLICKER_LENGTH: f32 = 0.5;
const MIX_SCALAR: f32 = 0.05;

#[derive(Component, Default)]
pub struct Marker;

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

fn tick(query: Query<Entity, With<Marker>>, mut event_writer: EventWriter<FlickerStartEvent>) {
    for e in query.iter() {
        info!("Flickering the square!");
        event_writer.send(
            FlickerStartEvent::new(e)
                .with_secs(FLICKER_LENGTH)
                .with_mix_scalar(MIX_SCALAR)
                .with_color(Color::BLUE),
        );
    }
}
