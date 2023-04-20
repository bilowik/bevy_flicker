use bevy::prelude::*;


#[derive(Component)]
pub struct Flickered(pub Timer);


impl Default for Flickered {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

#[allow(dead_code)]
impl Flickered {
    pub fn with_secs(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Once))
    }
}


// An entity with this component will not react to flicker events
#[derive(Component, Debug, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct NoFlicker;



/// Used to temporarily store image handles that are not being used at the moment on a component
/// such as when a shader is being applied in place (in the case of flickering)
#[derive(Component, Debug, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ImageSave(pub Handle<Image>);


/// Used to temporarily store image handles that are not being used at the moment on a component
/// such as when a shader is being applied in place (in the case of flickering)
#[derive(Component, Debug, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct TextureAtlasSave(pub Handle<TextureAtlas>);

/// Used to temporarily store the original color of a mesh that is flickering.
/// Flickering a mesh involves changing its underlying ColorMaterial
#[derive(Component, Debug, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct MeshColorSave(pub Color);

