mod collision_detection;
mod entity_renderer;
mod map_renderer;
mod player_input;
mod random_move;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collision_detection::collision_detection_system())
        .flush()
        .add_system(map_renderer::map_renderer_system())
        .add_system(entity_renderer::entity_renderer_system())
        .add_system(random_move::random_move_system())
        .build()
}
