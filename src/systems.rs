use bevy::prelude::*;

use crate::{PointerToWorldCamera, PointerWorldPosition};

/// Bevy [`Update`] system to update the pointer world position
pub fn update_pointer_world_position(
    camera_q: Query<(&Camera, &GlobalTransform), With<PointerToWorldCamera>>,
    windows: Query<&Window>,
    mut pointer_pos: ResMut<PointerWorldPosition>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(screen_pos) = window.cursor_position() {
        // Convert screen position to world coordinates
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            pointer_pos.0 = world_pos;
        }
    }
}
