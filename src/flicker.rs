use bevy::{
    prelude::*,
    sprite::Material2d,
    render::render_resource::{AsBindGroup, ShaderRef},
    reflect::TypeUuid,
};

pub const FLICKER_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3253086872234592510);

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, FromReflect)]
#[uuid = "da4a38fa-0ea5-4ced-b447-e0cf9e00f3ee"]
pub struct FlickerMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
    
    /// A percentage, min-max, between 0.0 and 1.0
    /// Defines the top left corner of where to start pulling
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
    //#[uniform(2)] TODO: Implement me
    pub color: Color,

}

impl Material2d for FlickerMaterial {
    fn fragment_shader() -> ShaderRef {
        FLICKER_MATERIAL_SHADER_HANDLE.typed().into()
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
