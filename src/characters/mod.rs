mod jumper;
mod player;

use bevy::prelude::*;
use jumper::*;
use player::*;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("player_setup", SystemStage::single(spawn_player))
            .add_system(player_jumps)
            .add_system(player_movement)
            .add_system(jump_reset);
    }
}
