use bevy::prelude::*;
use crate::events::FlickerStartEvent;

#[derive(Component, Reflect, FromReflect)]
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


#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct RepeatingFlicker {
    pub(crate) timer: Timer,

    /// Time in seconds between each flicker, including the time passed during the flicker
    pub time_between_flickers: f32,
    
    /// See `FlickerStartEvent` for more information
    pub flicker_time_length: f32,

    /// See `FlickerStartEvent` for more information
    pub mix_scalar: f32,


    /// See `FlickerStartEvent` for more information
    pub color: Color,
}


impl Default for RepeatingFlicker {
    fn default() -> Self {
        RepeatingFlickerBuilder::default().build()
    }

}


impl RepeatingFlicker {
    pub fn builder() -> RepeatingFlickerBuilder {
        Default::default() 
    }

    pub(crate) fn generate_start_event(&self, entity: Entity) -> FlickerStartEvent {
        FlickerStartEvent {
           entity,
           secs: self.flicker_time_length,
           mix_scalar: self.mix_scalar,
           color: self.color,
        }
    }
}

pub struct RepeatingFlickerBuilder {
    flicker_time_length: f32,
    time_between_flickers: f32,
    mix_scalar: f32,
    color: Color,
}

impl Default for RepeatingFlickerBuilder {
    fn default() -> Self {
        Self {
            flicker_time_length: 0.1,
            time_between_flickers: 0.5,
            mix_scalar: 0.5,
            color: Color::WHITE
        }
    }
}

impl RepeatingFlickerBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_flicker_time_length(mut self, flicker_time_length: f32) -> Self {
        self.flicker_time_length = flicker_time_length;
        self
    }

    pub fn with_time_between_flickers(mut self, time_between_flickers: f32) -> Self {
        self.time_between_flickers = time_between_flickers;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_mix_scalar(mut self, mix_scalar: f32) -> Self {
        self.mix_scalar = mix_scalar;
        self
    }

    pub fn build(self) -> RepeatingFlicker {
        RepeatingFlicker {
            timer: Timer::from_seconds(self.time_between_flickers, TimerMode::Repeating),
            flicker_time_length: self.flicker_time_length,
            time_between_flickers: self.time_between_flickers, 
            mix_scalar: self.mix_scalar,
            color: self.color,
        }
    }
}


