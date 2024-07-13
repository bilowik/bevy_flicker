use bevy_ecs::{entity::Entity, event::Event};

use bevy_color::Color;

#[derive(Debug, Event)]
pub struct FlickerStartEvent {
    /// Entity to apply the flicker to
    pub entity: Entity,

    /// The length in seconds the flicker should last
    pub secs: f32,

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
    color: Color,
}

impl Default for FlickerStartEventBuilder {
    fn default() -> Self {
        Self {
            entity: None,
            secs: 0.1,
            color: Color::WHITE,
        }
    }
}

impl FlickerStartEventBuilder {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            ..Default::default()
        }
    }

    pub fn with_secs(mut self, secs: f32) -> Self {
        self.secs = secs;
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
            color: self.color,
        }
    }
}
