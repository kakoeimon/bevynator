use bevy::prelude::*;

use bevy_kako_tools::*;
use crate::GameRes;

use crate::enemies::{Enemy, EnemyBullet};
pub struct EnemyBullet1;


pub const WIDTH: f32 = 10.0;
pub const HEIGHT: f32 = 10.0;

pub const SPEED: f32 = 600.0;



pub fn add(commands: &mut Commands,
    res: &GameRes,
    x: f32, y: f32,
    dir: Vec2,
) 
{
    let layer = KaAABB::create_collision_layer(&[3]);
    let mask = 0;//KaAABB::create_collision_layer(&[1]);

    commands.spawn( SpriteComponents {
        translation: Translation::new(x, y, 5.0),
        draw: Draw {
            is_transparent: true,
            ..Default::default()
        },
        material: res.0.get_color_material("assets/enemy_bullet1.png"),
        ..Default::default()
    })
    .with(Enemy {health: 0.1, ..Default::default()})
    .with(EnemyBullet{})
    .with(KaAABB::new(x, y, WIDTH, HEIGHT, layer, mask, true, false))
    .with(KaMoveable::new(dir.x() * SPEED, dir.y() * SPEED, true))
    .with( EnemyBullet1{});
}

pub fn update(mut commands: Commands,
    audio_output: Res<AudioOutput>,
    res: Res<GameRes>,
    mut enemies_query: Query<(&Enemy, &EnemyBullet1, &Translation, Entity)>,
    
)
{
    use crate::trail;
    use rand::random;
    let trail_material = res.0.get_color_material("assets/trail.png");

    for (enemy, _, translation, e) in &mut enemies_query.iter() {
        if enemy.health <= 0.0 {
            commands.despawn(e);
            audio_output.play(res.0.get_sound("assets/sounds/hit.mp3"));
            crate::explosion::add(&mut commands, &res, translation.x(), translation.y(), 1.0);
        } else {
            trail::add(&mut commands, trail_material, translation.x(), translation.y(), 0.1 + 0.4 * random::<f32>());

        }
        
    }

    
}
