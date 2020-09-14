use bevy::prelude::*;
use bevy_kako_tools::*;
use crate::GameRes;
pub mod enemy_bullet1;
pub mod enemy1;
pub mod enemy2;
pub mod enemy3;


pub struct Enemy {
    pub health: f32,
    pub power: f32,
    pub powerup_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2),
}

pub fn no_power_up(_commands: &mut Commands, _res: &GameRes, _pos: Vec2) {}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            power: 10.0,
            health: 10.0,
            powerup_fn: no_power_up,
        }
    }
}

impl Enemy {
    pub fn damage(&mut self, power: f32) {
        self.health -= power;
        self.health = self.health.max(0.0);
    }
}

pub struct EnemyBullet;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update.system());
        app.add_system(enemy1::update.system());
        app.add_system(enemy2::update.system());
        app.add_system(enemy3::update.system());

        app.add_system(enemy_bullet1::update.system());
    }
}

//Despawn Enemies and EnemyBullets when are out of screen.
pub fn update(mut commands: Commands,
    cam_pos: Res<crate::camera::CamPos>,
    mut enemies_query: Query<(&Enemy, &Translation, &KaAABB, Entity)>,
    mut bullets_query: Query<(&EnemyBullet, &Translation, &KaAABB, Entity)>
)
{
    use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
    let cam_x = cam_pos.0.x();
    let left_bounds = cam_x - SCREEN_WIDTH / 2.0;
    let right_bounds = cam_x + SCREEN_WIDTH / 2.0;
    let up_bounds = SCREEN_HEIGHT / 2.0;
    let down_bounds = -up_bounds;

    for (_, translation, aabb, e) in &mut enemies_query.iter() {
        if translation.x() + aabb.half_e.x() < left_bounds ||
            translation.y() - aabb.half_e.y() > up_bounds ||
            translation.y() + aabb.half_e.y() < down_bounds
        {
            commands.despawn(e);
        }
    }

    for (_, translation, aabb, e) in &mut bullets_query.iter() {
        if translation.x() - aabb.half_e.x() > right_bounds {
            commands.despawn(e);
        }
    }


}
