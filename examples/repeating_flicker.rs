use bevy::prelude::*;
use bevy_flicker::prelude::*;

const FIXED_TIMESTEP: f32 = 1.0;

#[derive(Component, Default)]
pub struct Marker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlickerPlugin)
        .add_system(setup.on_startup())
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("asteroid5.png"),
                transform: Transform::default().with_scale(Vec3::splat(8.0)),
                ..default()
            },
            RepeatingFlicker::builder()
                .with_color(Color::rgba(0.0, 0.0, 1.0, 0.2))
                .build(),
        ));

}
