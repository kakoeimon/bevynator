use bevy::prelude::*;

use bevy_kako_tools::*;

use crate::GameRes;

use crate::player_bullet;
use crate::camera::CAMERA_SPEED;
use crate::trail;

use crate::enemies::{Enemy, EnemyBullet};
use crate::explosion;

use rand::random;

pub struct PlayerPos(pub Vec2);

pub struct Player {
    pub score: u32,
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub dash: f32,
    pub max_dash: f32,
    pub dash_mult: f32,
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
            score: 0,
            health: 100.0,
            max_health: 100.0,
            speed: 300.0,
            dash: 100.0,
            max_dash: 100.0,
            dash_mult: 2.0,
            bullet_speed: 1000.0,
            fire_rate: 0.3,
            fire_rate_timer: 1.0,
            flashing: false,
            flashing_timer: 0.0,
            flashing_step_time: 0.06,
            flashing_counter: 0,
            flashing_max_counter: 12,
        }
    }

    pub fn damage(&mut self, value: f32) -> bool {
        if !self.flashing {
            self.health -= value;
            self.flashing = true;
            self.flashing_timer = 0.0;
            return true;
        }
        false
    }

    pub fn gain_health(&mut self, value: f32) {
        self.health = (self.health + value).min(self.max_health);
    }

    pub fn max_health_up(&mut self, value: f32) {
        self.max_health = (self.max_health + value).min(450.0);
    }

    pub fn dash_speed_up(&mut self, value: f32) {
        self.dash_mult = (self.dash_mult + value).min(3.0);
    }

    pub fn max_dash_up(&mut self, value: f32) {
        self.max_dash = (self.max_dash + value).min(450.0);
    }

    pub fn bullet_speed_up(&mut self, value: f32) {
        self.bullet_speed += value;
    }

    pub fn fire_rate_up(&mut self, value: f32) {
        self.fire_rate = (self.fire_rate - value).max(0.02);
    }
}


pub fn add_player(world: &mut World, res: &GameRes) {
    let player_layer = KaAABB::create_collision_layer(&[0]);
    let player_mask = KaAABB::create_collision_layer(&[2, 3]);
    let x = -300.0;
    let y = 0.0;
    let z = 10.0;
    let player = world.spawn( SpriteComponents {
        translation: Translation::new(x, y, z),
        draw: Draw {
            is_transparent: true,
            ..Default::default()
        },
        material: res.0.get_color_material("assets/ship1.png"),
        ..Default::default()
    });
    world.insert(
        player, 
        (
            KaAABB::new(x, y, 80.0, 40.0, player_layer, player_mask, true, false), 
            KaMoveable::new(0.0, 0.0, true), 
            Player::new()
        )
    ).unwrap();

}

pub fn update(mut commands: Commands,
    audio_output: Res<AudioOutput>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    res: Res<GameRes>,
    cam_pos: Res<crate::camera::CamPos>,
    mut player_pos: ResMut<PlayerPos>,
    mut query: Query<(Entity, &mut Player, &mut KaMoveable, &mut KaAABB, &Translation, &mut Draw)>,
    mut enemy_query: Query<(&mut Enemy, &Translation)>,
    mut enemy_bullet_query: Query<&EnemyBullet>,
)
{

    //KEEP PLAYER IN CAMERA
    let width = crate::SCREEN_WIDTH;
    let half_width = width / 2.0;
    let height = crate::SCREEN_HEIGHT;
    let half_height = height / 2.0;
    let cam_x = cam_pos.0.x();
    for (e, player, _movable, aabb, translation, _) in &mut query.iter() {
        if player.health <= 0.0 {
            commands.despawn(e);
            explosion::add(&mut commands, &res, translation.x(), translation.y(), 4.0);
            audio_output.play(res.0.get_sound("assets/sounds/explosion.mp3"));
            crate::status::game_over(&mut commands, &res);
            return;
        }
        let mut pos = aabb.pos.write().unwrap();
        if pos.x() <= cam_x - half_width {
            *pos.x_mut() = cam_x - half_width;
        }
        if pos.x() >= cam_x + width / 3.0 {
            *pos.x_mut() = cam_x + width / 3.0;
        }
        if pos.y() <= -half_height + aabb.half_e.y(){
            *pos.y_mut() = -half_height  + aabb.half_e.y();
        }
        if pos.y() >= half_height - aabb.half_e.y(){
            *pos.y_mut() = half_height - aabb.half_e.y();
        }

        //PLAYER POSITION
        player_pos.0 = pos.clone();

    }
    
    for (e, mut player, mut moveable, mut aabb, translation, mut draw) in &mut query.iter() {
        player.fire_rate_timer += time.delta_seconds;
        let mut speed = player.speed;

        if keyboard_input.pressed(KeyCode::X) {
            player.dash = (player.dash - time.delta_seconds * 100.0).max(0.0);
            if player.dash > 0.0 {
                speed *= player.dash_mult;
                trail::add(&mut commands, res.0.get_color_material("assets/trail.png"), translation.x() - aabb.half_e.x(), translation.y(), 0.1 + random::<f32>());
            }
            
        } else {
            player.dash = (player.dash + time.delta_seconds * 100.0).min(player.max_dash);
            
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
                let bullet_pos = Vec2::new(translation.x() + aabb.half_e.x() + moveable.velocity.x() * time.delta_seconds,
                translation.y() +  moveable.velocity.y() * time.delta_seconds);
                player_bullet::add_bullet(&mut commands, &audio_output, &res, &bullet_pos, player.bullet_speed);
            }
        }

        // COLLISIONS
        
        for col in moveable.get_collisions().iter() {
            if let Ok(mut result) = enemy_query.entity(*col) {
                if let Some((mut enemy, enemy_translation)) = result.get() {
                    if player.damage(enemy.power) {
                        explosion::add(&mut commands, &res, translation.x(), translation.y(), 1.5);
                        aabb.collision_mask = 0;
                        audio_output.play(res.0.get_sound("assets/sounds/damage.mp3"));
                        audio_output.play(res.0.get_sound("assets/sounds/explosion.mp3"));
                        let force = ( Vec2::new( player_pos.0.x() - enemy_translation.x(), player_pos.0.y() - enemy_translation.y() ) ).normalize() * 1000.0;
                        moveable.external_forces += force;
                        if let Ok(mut result) = enemy_bullet_query.entity(*col) {
                            if let Some(_) = result.get() {
                                enemy.damage(10.0);
                            }
                        }
                    }
                }
            }
        }

        //FLASHING
        if player.flashing {
            player.flashing_timer += time.delta_seconds;
            if player.flashing_timer >= player.flashing_step_time {
                player.flashing_timer = 0.0;
                //fader::add(game, "assets/bullet1.png", pos.x(), pos.y(), gen_range(0.1, 2.0));
                if !draw.is_visible {
                    draw.is_visible = true;
                    player.flashing_counter += 1;
                } else {
                    draw.is_visible = false
                }
                if player.flashing_counter >= player.flashing_max_counter {
                    aabb.collision_mask = 12;
                    player.flashing = false;
                    player.flashing_counter = 0;
                }
            }
            
        }
        
        
    }

    
    
}

