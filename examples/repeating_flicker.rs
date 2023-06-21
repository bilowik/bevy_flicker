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
        .spawn((
            SpriteBundle {
                texture: asset_server.load("asteroid5.png"),
                transform: Transform::default().with_scale(Vec3::splat(8.0)),
                ..default()
            },
            Marker
        ));

}


fn tick(
    query: Query<Entity, With<Marker>>,
    mut commands: Commands,
) {
    commands.entity(query.single()).insert(repeating_flicker());
}


fn repeating_flicker() -> RepeatingFlicker {
        RepeatingFlicker::builder()
            .with_color(Color::rgba(0.0, 0.0, 0.0, 0.3))
            .with_flicker_time_length(0.1)
            .with_time_between_flickers(0.15)
            .with_count(2)
            .build()
}
