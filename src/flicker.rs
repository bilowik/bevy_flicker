use bevy_reflect::TypeUuid;
use bevy_render::{
    render_resource::{AsBindGroup, ShaderRef},
    texture::Image,
    color::Color,
};
use bevy_sprite::Material2d;
use bevy_asset::{Handle, Asset};
use bevy_reflect::Reflect;
use bevy_math::Vec2;

// Required for the AsBindGroup derive macro.
mod bevy {
    pub mod render {
        pub use bevy_render::*;
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, Asset)]
#[uuid = "da4a38fa-0ea5-4ced-b447-e0cf9e00f3ee"]
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
    pub color: Color,
}

impl Material2d for FlickerMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://flicker_material.wgsl".into()
    }
}

impl Default for FlickerMaterial {
    fn default() -> Self {
        Self {
            source_image: Default::default(),
            offset: Default::default(),
            size: Vec2::splat(1.0),
            ratio: Vec2::splat(1.0),
            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        }
    }
}
