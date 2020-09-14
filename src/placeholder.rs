use bevy::prelude::*;

use crate::GameRes;
use crate::camera::ScrollingCamera;
use crate::enemies;


pub struct Placeholder {
    pub size: Vec2,
    pub pos: Vec2,
    pub powerup_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2),
    pub spawn_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2, powerup_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2) ),
}



impl Placeholder {
    pub fn new( pos: Vec2, size: Vec2,
        powerup_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2),
        spawn_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2, powerup_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2) )
    ) -> Self 
    {
        Self {
            pos,
            size,
            powerup_fn,
            spawn_fn,
        }
    }
}

pub fn update(mut commands: Commands,
    res: Res<GameRes>,
    mut spawners_query: Query<(&Placeholder, Entity)>,
    mut cam_query: Query<(&ScrollingCamera, &mut Translation)>
)
{
    let mut cam_x = 0.0;
    for (_, tranlsation) in &mut cam_query.iter() {
        cam_x = tranlsation.x();
    }
    for (spawner, e) in &mut spawners_query.iter() {
        if cam_x + crate::SCREEN_WIDTH / 2.0 + spawner.size.x() >= spawner.pos.x() {
            (spawner.spawn_fn)(&mut commands, &res, spawner.pos, spawner.powerup_fn);
            commands.despawn(e);
        }
    }
}
