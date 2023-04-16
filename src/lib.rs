use bevy::app::{App, Plugin};
use bevy::asset::{load_internal_asset, AddAsset, Assets, Handle};
use bevy::ecs::prelude::IntoSystemConfig;
use bevy::ecs::schedule::SystemSet;
use bevy::render::{color::Color, prelude::Shader};

use bevy::sprite::Material2dPlugin;

mod components;
mod events;
mod flicker;
mod systems;

use flicker::{FlickerMaterial, FLICKER_MATERIAL_SHADER_HANDLE};
use events::{FlickerStartEvent, FlickerEndEvent};
use systems::{flicker_start, flicker_end, flicker_tick};

#[derive(Default)]
pub struct FlickerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct FlickerSet;

pub fn empty_system() {}

impl Plugin for FlickerPlugin {
    fn build(&self, app: &mut App) {
        // Register the flicker mateiral as an internal asset
        load_internal_asset!(
            app,
            FLICKER_MATERIAL_SHADER_HANDLE,
            "flicker_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(Material2dPlugin::<FlickerMaterial>::default())
            .register_asset_reflect::<FlickerMaterial>();

        app.world
            .resource_mut::<Assets<FlickerMaterial>>()
            .set_untracked(
                Handle::<FlickerMaterial>::default(),
                FlickerMaterial {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    ..Default::default()
                },
            );

        // Register events
        app.add_event::<FlickerStartEvent>()
            .add_event::<FlickerEndEvent>();

        // Register systems and systemset
        // TODO: These might need to be ordered to prevent conflicts potentially?
        app.add_system(flicker_start.in_set(FlickerSet));
        app.add_system(flicker_end.in_set(FlickerSet));
        app.add_system(flicker_tick.in_set(FlickerSet));
    }
}
