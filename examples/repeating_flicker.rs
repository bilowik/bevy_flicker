use bevy::prelude::*;
use bevy_flicker::prelude::*;

#[derive(Component, Default)]
pub struct Marker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlickerPlugin)
        .add_system(setup.on_startup())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    for (repeating_flicker, x_pos) in [(repeating_flicker_1(), -256.0), (repeating_flicker_2(), 0.0), (repeating_flicker_3(), 256.0)] {
        let mut transform = Transform::default().with_scale(Vec3::splat(4.0));
        transform.translation.x += x_pos;
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("asteroid5.png"),
                    transform, 
                    ..default()
                },
                Marker,
                repeating_flicker
            ));
    }

}


fn repeating_flicker_1() -> RepeatingFlicker {
        RepeatingFlicker::builder()
            .with_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
            .with_flicker_time_length(0.1)
            .with_time_between_flickers(0.15)
            .with_time_between_pulses(1.0)
            .with_count(5)
            .with_pulse_count(2)
            .build()
}
fn repeating_flicker_2() -> RepeatingFlicker {
        RepeatingFlicker::builder()
            .with_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
            .build()
}
fn repeating_flicker_3() -> RepeatingFlicker {
        RepeatingFlicker::builder()
            .with_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
            .with_flicker_time_length(0.1)
            .with_time_between_flickers(0.25)
            .with_time_between_pulses(2.5)
            .with_pulse_count(5)
            .build()
}
