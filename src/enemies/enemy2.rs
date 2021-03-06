use bevy::prelude::*;

use bevy_kako_tools::*;
use crate::GameRes;
use crate::placeholder::Placeholder;

use crate::player::PlayerPos;
use crate::enemies::enemy_bullet1;

use crate::enemies::Enemy;
pub struct Enemy2;


pub const WIDTH: f32 = 80.0;
pub const HEIGHT: f32 = 40.0; 



pub fn add(commands: &mut Commands,
    res: &GameRes,
    pos: Vec2,
    powerup_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2),
) 
{
    let layer = KaAABB::create_collision_layer(&[2]);
    let mask = 0;//KaAABB::create_collision_layer(&[1]);

    commands.spawn( SpriteComponents {
        translation: Translation::new(pos.x(), pos.y(), 2.0),
        draw: Draw {
            is_transparent: true,
            ..Default::default()
        },
        material: res.0.get_color_material("assets/enemy_ship_1.png"),
        ..Default::default()
    })
    .with(Enemy{powerup_fn, ..Default::default()})
    .with(KaAABB::new(pos.x(), pos.y(), WIDTH, HEIGHT, layer, mask, true, false))
    .with(KaMoveable::new(-100.0, 0.0, true))
    .with( Enemy2{});
}

pub fn update(mut commands: Commands,
    audio_output: Res<AudioOutput>,
    time: Res<Time>,
    cam_pos: Res<crate::camera::CamPos>,
    res: Res<GameRes>,
    player_pos: Res<PlayerPos>,
    mut enemies_query: Query<(&Enemy, &Enemy2, &Translation, &KaAABB, &mut KaMoveable, Entity)>,
    
)
{
    use crate::SCREEN_WIDTH;
    use crate::trail;
    use rand::random;
    let cam_x = cam_pos.0.x();

    for (enemy, _, translation, aabb, mut movable, e) in &mut enemies_query.iter() {
        if enemy.health <= 0.0 {
            commands.despawn(e);
            (enemy.powerup_fn)(&mut commands, &res, Vec2::new(translation.x(), translation.y()));
            audio_output.play(res.0.get_sound("assets/sounds/explosion.mp3"));
            crate::explosion::add(&mut commands, &res, translation.x(), translation.y(), 2.0);
        } else {
            let pos = aabb.pos.read().unwrap();
                
            if pos.x() < cam_x + SCREEN_WIDTH / 4.0 {
                *movable.velocity.x_mut() -= 2400.0 * time.delta_seconds;
                trail::add(&mut commands, res.0.get_color_material("assets/trail.png"), translation.x() + aabb.half_e.x(), translation.y() , 0.1 + 0.5 * random::<f32>());
            
                    //FIRE
                if random::<u32>() % 100 == 0 {
                    let fire_pos = Vec2::new(pos.x() - aabb.half_e.x(), pos.y());
                    let dir = Vec2::new(player_pos.0.x() - fire_pos.x(), player_pos.0.y() - fire_pos.y()).normalize();
                    enemy_bullet1::add(&mut commands, &res, fire_pos.x(), fire_pos.y(), dir);
                }
            } else {
                //FIRE
                if random::<u32>() % 1000 == 0 {
                    let fire_pos = Vec2::new(pos.x() - aabb.half_e.x(), pos.y());
                    let dir = Vec2::new(player_pos.0.x() - fire_pos.x(), player_pos.0.y() - fire_pos.y()).normalize();
                    enemy_bullet1::add(&mut commands, &res, fire_pos.x(), fire_pos.y(), dir);
                }
            }

        }
        
    }

    
}


pub fn create_placeholder(pos: Vec2, powerup_fn: fn(commands: &mut Commands, res: &GameRes, pos: Vec2) ) -> Placeholder {
    Placeholder {
        pos,
        size: Vec2::new(WIDTH, HEIGHT),
        powerup_fn,
        spawn_fn: add,
    }

}