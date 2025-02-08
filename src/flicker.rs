use bevy_asset::{Asset, Handle};
use bevy_image::Image;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_render::render_resource::{AsBindGroup, ShaderRef};

use bevy_color::LinearRgba;

use bevy_sprite::{AlphaMode2d, Material2d};

// Required for the AsBindGroup derive macro.
mod bevy {
    pub mod render {
        pub use bevy_render::*;
    }
}

#[derive(AsBindGroup, Clone, Reflect, Asset)]
pub struct FlickerMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Option<Handle<Image>>,

    /// A percentage, min-max, between 0.0 and 1.0
    /// Defines the top left corner of where to start pulling
    /// This
    /// pixels from the sprite sheet
    #[uniform(2)]
    pub offset: Vec2,

    /// A percentage, min-max, between 0.0 and 1.0
    /// Defines how large the are is to pull pixels from
    /// the sprite sheet
    #[uniform(2)]
    pub size: Vec2,

    /// A percentage, min-max, between 0.0 and 1.0
    /// TODO: Unused, what was this meant for?
    #[uniform(2)]
    pub ratio: Vec2,

    /// The color to flicker, will be mixed with each pixel
    #[uniform(2)]
    pub color: LinearRgba,
}

impl Material2d for FlickerMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://flicker_material.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl Default for FlickerMaterial {
    fn default() -> Self {
        Self {
            source_image: Default::default(),
            offset: Default::default(),
            size: Vec2::splat(1.0),
            ratio: Vec2::splat(1.0),
            color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}
