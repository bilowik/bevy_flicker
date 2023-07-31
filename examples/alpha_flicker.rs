use bevy::prelude::*;
use bevy_flicker::prelude::*;

const FIXED_TIMESTEP: f32 = 1.0;
const FLICKER_LENGTH: f32 = 0.5;

#[derive(Component, Default)]
pub struct Marker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlickerPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, tick)
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("alpha.png"),
            ..default()
        })
        .insert(Marker);
}

fn tick(query: Query<Entity, With<Marker>>, mut event_writer: EventWriter<FlickerStartEvent>) {
    for e in query.iter() {
        info!("Flickering the square!");
        event_writer.send(
            FlickerStartEvent::builder(e)
                .with_secs(FLICKER_LENGTH)
                .with_color(Color::rgba(1.0, 0.0, 0.0, 0.2))
                .build(),
        );
    }
}
