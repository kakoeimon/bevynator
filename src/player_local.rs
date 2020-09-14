use bevy::prelude::*;

use bevy_kako_tools::*;

use crate::GameRes;

use crate::player_bullet;
use crate::damageable::Damageable;
use crate::camera::CAMERA_SPEED;
use crate::trail;

use rand::random;

pub struct PlayerPos(pub Vec2);

pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub run_mult: f32,
    pub bullet_speed: f32,
    pub fire_rate: f32,
    pub fire_rate_timer: f32,
    pub flashing: bool,
    flashing_timer: f32,
    flashing_step_time: f32,
    flashing_counter: i32,
    flashing_max_counter: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            speed: 300.0,
            run_mult: 2.0,
            bullet_speed: 2000.0,
            fire_rate: 0.2,
            fire_rate_timer: 0.5,
            flashing: false,
            flashing_timer: 0.0,
            flashing_step_time: 0.06,
            flashing_counter: 0,
            flashing_max_counter: 10,
        }
    }
}


pub fn add_player(mut commands: Commands, res: Res<GameRes>) {
    let player_layer = KaAABB::create_collision_layer(&[0]);
    let player_mask = KaAABB::create_collision_layer(&[2, 3]);
    let x = -300.0;
    let y = 0.0;
    let z = 10.0;
    commands.spawn( SpriteComponents {
        translation: Translation::new(x, y, z),
        draw: Draw {
            is_transparent: true,
            ..Default::default()
        },
        material: res.0.get_color_material("assets/ship1.png"),
        ..Default::default()
    })
    .with(KaAABB::new(x, y, 80.0, 40.0, player_layer, player_mask, true, false))
    .with(KaMoveable::new(0.0, 0.0, true))
    .with(Player::new())
    .with(Damageable {health: 100.0});
}


#[allow(dead_code)]
pub fn update(world: &mut World, resources: &mut Resources)
{
    let audio_output = resources.get::<AudioOutput>().unwrap();
    let time = resources.get::<Time>().unwrap();
    let keyboard_input = resources.get::<Input<KeyCode>>().unwrap();
    let res = resources.get::<GameRes>().unwrap();
    let cam_pos = resources.get::<crate::camera::CamPos>().unwrap();
    let mut player_pos = resources.get_mut::<PlayerPos>().unwrap();

    //KEEP PLAYER IN CAMERA
    let width = crate::SCREEN_WIDTH;
    let half_width = width / 2.0;
    let height = crate::SCREEN_HEIGHT;
    let half_height = height / 2.0;
    let cam_x = cam_pos.0.x();
    let cam_y = cam_pos.0.y();


    //for (_player, _movable, aabb) in &mut query.iter() {
    for (_player, _movable, aabb) in world.query::<(&Player, &KaMoveable, &KaAABB)>().iter() {
        let mut pos = aabb.pos.write().unwrap();
        if pos.x() <= cam_x - half_width {
            *pos.x_mut() = cam_x - half_width;
        }
        if pos.x() >= cam_x + width / 3.0 {
            *pos.x_mut() = cam_x + width / 3.0;
        }
        if pos.y() <= cam_y - half_height + aabb.half_e.y(){
            *pos.y_mut() = cam_y - half_height  + aabb.half_e.y();
        }
        if pos.y() >= cam_y + half_height - aabb.half_e.y(){
            *pos.y_mut() = cam_y + half_height - aabb.half_e.y();
        }

        //PLAYER POSITION
        player_pos.0 = pos.clone();

    }
    
    for (mut player, mut moveable, aabb) in world.query::<(&mut Player, &mut KaMoveable, &KaAABB)>().iter() {
    
        player.fire_rate_timer += time.delta_seconds;
        let pos = aabb.pos.read().unwrap();
        let mut speed = player.speed;

        if keyboard_input.pressed(KeyCode::X) {
            speed *= player.run_mult;
            //trail::add(&mut commands, res.0.get_color_material("assets/trail.png"), pos.x() - aabb.half_e.x(), pos.y(), 0.1 + random::<f32>());
        }

        if keyboard_input.pressed(KeyCode::Left) {
            *moveable.velocity.x_mut() = -speed / 2.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            *moveable.velocity.x_mut() = speed;
        } else {
            *moveable.velocity.x_mut() = CAMERA_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            *moveable.velocity.y_mut() = speed;
        } else if keyboard_input.pressed(KeyCode::Down) {
            *moveable.velocity.y_mut() = -speed;
        } else {
            *moveable.velocity.y_mut() = 0.0;
        }

        
        if keyboard_input.pressed(KeyCode::C) {
            if player.fire_rate_timer >= player.fire_rate {
                player.fire_rate_timer = 0.0;
                let bullet_pos = Vec2::new(pos.x() + aabb.half_e.x() + moveable.velocity.x() * time.delta_seconds,
                    pos.y() +  moveable.velocity.y() * time.delta_seconds);
                //player_bullet::add_bullet(&mut commands, &audio_output, &res, &bullet_pos, player.bullet_speed);
            }
        }

        // COLLISIONS
        /*
        for col in moveable.get_collisions().iter() {
            match world.get::<Enemy>(*col) {
                Ok(enemy) => {
                    if player.damage(enemy.power) {
                        match world.get::<KaSprite>(*col) {
                            Ok(espr) => {
                                let force = (*player_pos - espr.pos).normalize() * 1000.0;
                                moveable.external_forces += force;
                            },
                            _=>(),
                        }
                        explosion::add(game, pos.x(), pos.y(), 1.5);
                        aabb.collision_layer = 0;
                        aabb.collision_mask = 0;
                        match world.query_one::<(&EnemyBullet, &mut Damageable)>(*col) {
                            Ok(mut bullet_query) => {
                                match bullet_query.get() {
                                    Some((_, damageable)) => {
                                        damageable.damage(10.0);
                                    },
                                    None => (),
                                }
                            },
                            _=> (),
                        }
                    }
                },
                _=> (),
            }
            
        }
        */
        
    }

    
    
}

