use bevy::prelude::*;

use crate::*;
use crate::powerups::{spawn_power_up};

pub fn add(commands: &mut Commands, res: &GameRes, pos: Vec2) 
{
    spawn_power_up(commands, res, pos, "assets/powerups/bullet_speed.png", power_up_fn);
}

pub fn power_up_fn(player: &mut crate::player::Player) {
    player.bullet_speed_up( 250.0 );
}
