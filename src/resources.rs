use bevy::prelude::*;

/// Bevy [`Resource`] to store the current pointer position in world coordinates
#[derive(Resource, Default, Debug)]
pub struct PointerWorldPosition(pub Vec2);

impl std::fmt::Display for PointerWorldPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PointerWorldPosition({:.2}, {:.2})", self.0.x, self.0.y)
    }
}
