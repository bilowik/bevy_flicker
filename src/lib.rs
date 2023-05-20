//! This plugin facilitates creating a brief overlay/mix of a specific color over a sprite or mesh.
//!
//! To trigger a flicker, you can send a [FlickerStartEvent][events::FlickerStartEvent], which will contain the parameters
//! that dictate the color, length, and strength of the flicker. 
//!
//! Included is also a [RepeatingFlicker][components::RepeatingFlicker] component that will send 
//! [FlickerStartEvents][events::FlickerStartEvent] on an interval.
//!
//! This also works on textures with alpha, the overlay takes into account the alpha of the underlying texture 
//! and will adjust the overlay alpha so that it's intensity is proportional between different underlying 
//! alpha values. So an underlying 0.2 alpha value will reduce the alpha of the overlay by 80%. For alpha 
//! values of 0, the overlay's alpha will also be 0.
//!
//! See more, complete examples [here](https://github.com/bilowik/bevy_flicker/tree/main/examples)
//!
//!
//! ```no_run
//! use bevy_flicker::prelude::*;
//! 
//! fn tick(query: Query<Entity>, mut event_writer: EventWriter<FlickerStartEvent>) {
//!     for e in query.iter() {
//!         event_writer.send(
//!             FlickerStartEvent::builder(e)
//!                 .with_secs(0.5)
//!                 .with_color(Color::rgba(0.0, 0.0, 1.0, 0.2))
//!                 .build(),
//!         );
//!     }
//! }
//!
//!
//! ```

use bevy::app::{App, Plugin};
use bevy::asset::{load_internal_asset, AddAsset};
use bevy::ecs::prelude::IntoSystemConfig;
use bevy::ecs::schedule::SystemSet;
use bevy::render::prelude::Shader;

use bevy::sprite::Material2dPlugin;

pub mod components;
pub mod config;
pub mod events;
mod flicker;
mod systems;

use config::FlickerPluginConfig;
use events::FlickerStartEvent;
use flicker::{FlickerMaterial, FLICKER_MATERIAL_SHADER_HANDLE};
use systems::{flicker_start, flicker_tick, repeating_flicker_tick};

/// The bevy plugin to include during App initialization
#[derive(Default)]
pub struct FlickerPlugin;

/// The SystemSet that the flicker systems belong to.
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
        app.add_event::<FlickerStartEvent>();

        // Register systems and systemset
        // TODO: These might need to be ordered to prevent conflicts potentially?
        app.add_system(flicker_start.in_set(FlickerSet));
        app.add_system(flicker_tick.in_set(FlickerSet));
        app.add_system(repeating_flicker_tick.in_set(FlickerSet));
        app.init_resource::<FlickerPluginConfig>();
    }
}

pub mod prelude {
    pub use super::{config::{FlickerPluginConfig, FlickerOverlapAction}, events::*, FlickerPlugin, FlickerSet, components::RepeatingFlicker};
}
