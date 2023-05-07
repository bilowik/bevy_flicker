use bevy::prelude::*;

#[derive(Debug)]
pub struct FlickerStartEvent {
    /// Entity to apply the flicker to 
    pub entity: Entity,

    /// The length in seconds the flicker should last
    pub secs: f32,
    
    /// The value used in the linear blending of the flicker color and original color between 0.0
    /// and 1.0, where 0.50 is equal weights to both the flicker and original colors, < 0.50
    /// results in a smaller weight for the flicker color.
    pub mix_scalar: f32,

    /// The flicker color that will be blending with the original color
    pub color: Color,
}

impl FlickerStartEvent {
    pub fn builder(entity: Entity) -> FlickerStartEventBuilder {
        FlickerStartEventBuilder::new(entity)
    }
}

pub struct FlickerStartEventBuilder {
    entity: Option<Entity>, // Entity cannot have a default, so Option is used.
    secs: f32,
    mix_scalar: f32,
    color: Color,
}

impl Default for FlickerStartEventBuilder {
    fn default() -> Self {
        Self {
            entity: None,
            secs: 0.1,
            mix_scalar: 0.5,
            color: Color::WHITE,
        }
    }
}

impl FlickerStartEventBuilder {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            ..default()
        }
    }

    pub fn with_secs(mut self, secs: f32) -> Self {
        self.secs = secs;
        self
    }
    pub fn with_mix_scalar(mut self, mix_scalar: f32) -> Self {
        self.mix_scalar = mix_scalar;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn build(self) -> FlickerStartEvent {
        FlickerStartEvent {
            entity: self.entity.unwrap(), // Guaranteed to not be None
            secs: self.secs,
            mix_scalar: self.mix_scalar,
            color: self.color,
        }
    }
}

#[derive(Debug)]
pub struct FlickerEndEvent {
    pub entity: Entity,
}



