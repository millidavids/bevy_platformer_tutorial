mod player;
mod jumper;

use bevy::prelude::*;
use player::*;
use jumper::*;


pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "player_setup",
                SystemStage::single(spawn_player),
            )
            .add_system(jumper_jumps)
            .add_system(jump_reset);
    }
}