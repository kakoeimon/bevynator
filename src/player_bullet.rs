use bevy::prelude::*;
use bevy_kako_tools::*;

use crate::GameRes;

use crate::{
    trail,
    enemies::Enemy,
};

const WIDTH: f32 = 10.0;
const HEIGHT: f32 = 10.0;

pub struct PlayerBullet;


pub fn add_bullet(commands: &mut Commands,
    audio_output: &AudioOutput,
    res: &GameRes,
    bullet_pos: &Vec2,
    bullet_speed: f32,
)
{
    let bullet_layer = KaAABB::create_collision_layer(&[1]);
    let bullet_mask = KaAABB::create_collision_layer(&[2]);
    audio_output.play(res.0.get_sound("assets/sounds/fire.mp3"));
    
    commands.spawn(SpriteComponents {
        draw: Draw {
            is_transparent: true,
            ..Default::default()
        },
        translation: Translation::new(bullet_pos.x(), bullet_pos.y(), 1.0),
        material: res.0.get_color_material("assets/bullet1.png"),
        ..Default::default()
    })
    .with(KaAABB::new(bullet_pos.x(), bullet_pos.y(), WIDTH, HEIGHT, bullet_layer, bullet_mask, true, false))
    .with(KaMoveable::new(bullet_speed, 0.0, true))
    .with(PlayerBullet);

    
}

pub fn bullet_system(mut commands: Commands,
    res: Res<GameRes>,
    camera_pos: Res<crate::camera::CamPos>,
    audio_output: Res<AudioOutput>,
    mut query: Query<(&PlayerBullet, &Translation, &KaMoveable, Entity)>,
    enemy_query: Query<&mut Enemy>
) {
    let right_bound = camera_pos.0.x() + crate::SCREEN_WIDTH / 2.0 + WIDTH / 2.0;
    for (_bullet, translation, movable, e) in &mut query.iter() {
        
        
        if translation.x() > right_bound {
            commands.despawn(e);
        } else {
            let mut die = false;
            trail::add(&mut commands, res.0.get_color_material("assets/trail.png"), translation.x(), translation.y(), 0.05);
            for col in movable.get_collisions().iter() {
                die = true;
                if let Ok(mut result) = enemy_query.entity(*col) {
                    if let Some(mut enemy) = result.get() {
                        enemy.damage(10.0);
                    }
                }
            }
            if die {
                commands.despawn(e);
                audio_output.play(res.0.get_sound("assets/sounds/hit.mp3"));
                crate::explosion::add(&mut commands, &res, translation.x(), translation.y(), 1.0);
            }
        }
        
    }

}