use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.0)),
            material: materials.add(ColorMaterial::from(Color::from(LinearRgba::RED))),
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
                .with_color(LinearRgba::new(0.0, 0.0, 1.0, 0.5).into())
                .build(),
        );
    }
}
