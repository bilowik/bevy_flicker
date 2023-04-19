use bevy::prelude::*;

#[derive(Debug)]
pub struct FlickerStartEvent {
    pub entity: Entity,
    pub secs: f32,
    pub mix_scalar: f32,
    pub color: Color,
}

impl FlickerStartEvent {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            secs: 0.1,
            mix_scalar: 1.0,
            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
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
}


#[derive(Debug)]
pub struct FlickerEndEvent {
    pub entity: Entity
}

