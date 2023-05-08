use bevy::app::{App, Plugin};
use bevy::asset::{load_internal_asset, AddAsset};
use bevy::ecs::prelude::IntoSystemConfig;
use bevy::ecs::schedule::SystemSet;
use bevy::render::prelude::Shader;

use bevy::sprite::Material2dPlugin;

pub mod components;
pub mod config;
pub mod events;
pub mod flicker;
pub mod systems;

use config::FlickerPluginConfig;
use events::{FlickerEndEvent, FlickerStartEvent};
use flicker::{FlickerMaterial, FLICKER_MATERIAL_SHADER_HANDLE};
use systems::{flicker_end, flicker_start, flicker_tick, repeating_flicker_tick};

#[derive(Default)]
pub struct FlickerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct FlickerSet;

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

        // Register events
        app.add_event::<FlickerStartEvent>()
            .add_event::<FlickerEndEvent>();

        // Register systems and systemset
        // TODO: These might need to be ordered to prevent conflicts potentially?
        app.add_system(flicker_start.in_set(FlickerSet));
        app.add_system(flicker_end.in_set(FlickerSet));
        app.add_system(flicker_tick.in_set(FlickerSet));
        app.add_system(repeating_flicker_tick.in_set(FlickerSet));
        app.init_resource::<FlickerPluginConfig>();
    }
}

pub mod prelude {
    pub use super::{config::{FlickerPluginConfig, FlickerOverlapAction}, events::*, FlickerPlugin, FlickerSet, components::RepeatingFlicker};
}
