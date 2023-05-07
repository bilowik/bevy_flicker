use bevy::prelude::*;
use bevy_flicker::prelude::*;

const FIXED_TIMESTEP: f32 = 1.0;
const FLICKER_LENGTH: f32 = 0.5;
const MIX_SCALAR: f32 = 0.15;

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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("asteroid_sheet_test.png");
    let atlas = texture_atlases.add(TextureAtlas::from_grid(texture, Vec2::new(64.0, 64.0), 1, 4, None, None));
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 2,
                ..default()
            },
            transform: Transform::default().with_scale(Vec3::splat(8.0)),
            texture_atlas: atlas,
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
                .with_mix_scalar(MIX_SCALAR)
                .with_color(Color::RED)
                .build(),
        );
    }
}
