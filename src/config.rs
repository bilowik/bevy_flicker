use bevy::prelude::*;

/// Used to determine what to do when a flicker event is received for an
/// entity that is already in a flickering state
#[derive(PartialEq, Eq)]
pub enum FlickerOverlapAction {
    /// Flicker events targetted at entities that are already flickering will overwrite the
    /// existing flicker.
    Overwrite,

    /// Flicker events targetted at entities that are already flickering will be ignored.
    Ignore,
}

impl Default for FlickerOverlapAction {
    fn default() -> Self {
        FlickerOverlapAction::Overwrite
    }
}

#[derive(Resource, Default)]
pub struct FlickerPluginConfig {
    /// See [FlickerOverlapAction]
    pub overlap_action: FlickerOverlapAction,
}

impl FlickerPluginConfig {
    pub(crate) fn ignore_overlap(&self) -> bool {
        match self.overlap_action {
            FlickerOverlapAction::Ignore => true,
            _ => false,
        }
    }
}
