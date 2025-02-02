use bevy::prelude::*;

/// Bevy [`Resource`] to store the current pointer position in world coordinates
#[derive(Resource, Default)]
pub struct PointerWorldPosition(pub Vec2);
