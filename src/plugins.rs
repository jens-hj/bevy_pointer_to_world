use crate::systems::*;
use bevy::prelude::*;

use crate::PointerWorldPosition;

pub struct PointerToWorldPlugin;

impl Plugin for PointerToWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointerWorldPosition>()
            .add_systems(Update, update_pointer_world_position);
    }
}
