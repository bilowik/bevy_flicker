use bevy::prelude::*;
use bevy_flicker::prelude::*;

#[derive(Component, Default)]
pub struct Marker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FlickerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    for (repeating_flicker, x_pos) in [
        (repeating_flicker_1(), -256.0),
        (repeating_flicker_2(), 0.0),
        (repeating_flicker_3(), 256.0),
    ] {
        let mut transform = Transform::default().with_scale(Vec3::splat(4.0));
        transform.translation.x += x_pos;
        commands.spawn((
            Sprite::from_image(asset_server.load("asteroid5.png")),
            transform,
            Marker,
            repeating_flicker,
        ));
    }
}

fn repeating_flicker_1() -> RepeatingFlicker {
    RepeatingFlicker::builder()
        .with_color(LinearRgba::new(0.0, 0.0, 0.0, 0.5).into())
        .with_flicker_time_length(0.1)
        .with_time_between_flickers(0.15)
        .with_time_between_pulses(1.0)
        .with_count(5)
        .with_pulse_count(2)
        .build()
}
fn repeating_flicker_2() -> RepeatingFlicker {
    RepeatingFlicker::builder()
        .with_color(LinearRgba::new(0.0, 0.0, 0.0, 0.5).into())
        .build()
}
fn repeating_flicker_3() -> RepeatingFlicker {
    RepeatingFlicker::builder()
        .with_color(LinearRgba::new(0.0, 0.0, 0.0, 0.5).into())
        .with_flicker_time_length(0.1)
        .with_time_between_flickers(0.25)
        .with_time_between_pulses(2.5)
        .with_pulse_count(5)
        .build()
}
