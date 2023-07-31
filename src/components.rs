use crate::events::FlickerStartEvent;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    reflect::ReflectComponent,
};

use bevy_reflect::Reflect;

use bevy_time::{Timer, TimerMode};

use bevy_render::color::Color;

#[derive(Component, Reflect)]
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

/// An entity with this component will not react to flicker events
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct NoFlicker;

/// Marks an entity which is actively being flickered
/// An extra marker is needed since no components are added to the entity
/// being flickered.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct FlickerMarker;

/// Sends [FlickerStartEvents][crate::events::FlickerStartEvent] on an interval.
///
/// A pulse is a sequence of [RepeatingFlicker::pulse_count] flickers with a delay between
/// each in the pulse of [RepeatingFlicker::time_between_flickers]. Each pulse has a delay of
/// [RepeatingFlicker::time_between_pulses]. When [RepeatingFlicker::count] is set, after the
/// set number of pulses occur, the [RepeatingFlicker] will be removed from the Entity.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RepeatingFlicker {
    pub(crate) timer: Timer,
    pub(crate) pulse_timer: Timer,

    /// Time in seconds between each flicker.
    ///
    /// NOTE: This has no effect when pulse_count < 2, since the time between each pulse will
    /// end up being time_between_pulses.
    pub time_between_flickers: f32,

    /// Time in seconds between each pulse.
    pub time_between_pulses: f32,

    /// See [FlickerStartEvent][crate::events::FlickerStartEvent] for more information
    pub flicker_time_length: f32,

    /// See [FlickerStartEvent][crate::events::FlickerStartEvent] for more information
    pub color: Color,

    /// Number of flickers per pulse
    pub pulse_count: u32,

    /// The number of total pulses before expiring
    pub count: Option<u32>,

    pub(crate) curr_pulse_count: u32,
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
            color: self.color,
        }
    }
}

/// For implementing a builder pattern on [RepeatingFlicker][crate::components::RepeatingFlicker]
pub struct RepeatingFlickerBuilder {
    flicker_time_length: f32,
    time_between_flickers: f32,
    time_between_pulses: f32,
    color: Color,
    pulse_count: u32,
    count: Option<u32>,
}

impl Default for RepeatingFlickerBuilder {
    fn default() -> Self {
        Self {
            flicker_time_length: 0.1,
            time_between_flickers: 0.5,
            time_between_pulses: 0.5,
            color: Color::WHITE,
            pulse_count: 1,
            count: None,
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

    pub fn with_count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn with_pulse_count(mut self, pulse_count: u32) -> Self {
        self.pulse_count = pulse_count;
        self
    }

    pub fn with_time_between_pulses(mut self, time_between_pulses: f32) -> Self {
        self.time_between_pulses = time_between_pulses;
        self
    }

    pub fn build(self) -> RepeatingFlicker {
        RepeatingFlicker {
            timer: Timer::from_seconds(
                self.time_between_flickers + self.flicker_time_length,
                TimerMode::Repeating,
            ),
            pulse_timer: Timer::from_seconds(self.time_between_pulses, TimerMode::Repeating),
            flicker_time_length: self.flicker_time_length,
            time_between_flickers: self.time_between_flickers,
            color: self.color,
            count: self.count,
            time_between_pulses: self.time_between_pulses,
            pulse_count: self.pulse_count,
            curr_pulse_count: 0,
        }
    }
}
