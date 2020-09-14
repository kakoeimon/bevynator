use bevy::prelude::*;
use bevy_kako_tools::*;
use crate::GameRes;
use crate::player::Player;

pub mod health_up;
pub mod max_health_up;
pub mod max_dash_up;
pub mod bullet_speed_up;
pub mod fire_rate_up;
pub mod dash_speed_up;

const WIDTH: f32 = 40.0;
const HEIGHT: f32 = 40.0;


pub struct PowerUp {
    pub side: f32,
    pub time_1: f32,
    pub time_2: f32,
    pub timer: f32,
    pub power_up_fn: fn(player: &mut Player),
}

impl PowerUp {
    pub fn new(y: f32, power_up_fn: fn(player: &mut Player) ) -> Self {
        let mut side = -1.0;
        if y < 0.0 { side = 1.0}
        Self {
            side,
            time_1: 1.0,
            time_2: 1.5,
            timer: 0.0,
            power_up_fn,

        }
    }
}

pub struct PowerUpsPlugin;

impl Plugin for PowerUpsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        
        app.add_system(update.system());
        
    }
}

pub fn spawn_power_up(commands: &mut Commands,
    res: &GameRes,
    pos: Vec2,
    asset_path: &str,
    power_up_fn: fn(player: &mut Player),
) 
{
    let layer = KaAABB::create_collision_layer(&[4]);
    let mask = KaAABB::create_collision_layer(&[0]);

    commands.spawn( SpriteComponents {
        translation: Translation::new(pos.x(), pos.y(), 2.0),
        draw: Draw {
            is_transparent: true,
            ..Default::default()
        },
        material: res.0.get_color_material(asset_path),
        ..Default::default()
    })
    .with(PowerUp::new(pos.y(), power_up_fn))
    .with(KaAABB::new(pos.x(), pos.y(), WIDTH, HEIGHT, layer, mask, true, false))
    .with(KaMoveable::new(-1.0, 0.0, true));
}


pub fn no_power_up(_commands: &mut Commands, _res: &GameRes, _pos: Vec2) {}


pub fn update(mut commands: Commands,
    audio_output: Res<AudioOutput>,
    time: Res<Time>,
    res: Res<GameRes>,
    cam_pos: Res<crate::camera::CamPos>,
    mut query_powerup: Query<(Entity, &mut KaMoveable, &mut PowerUp, &KaAABB, &mut Translation)>,
    mut player_query: Query<(&mut Player, &Translation)>,
)
{
    for (e, mut moveable, mut powerup, aabb, mut translation) in &mut query_powerup.iter() {
        //MOVEMENT
        powerup.timer += time.delta_seconds;
        if powerup.timer < powerup.time_1 {
            let force = powerup.time_1 - powerup.timer;
            *moveable.velocity.y_mut() = 200.0 * force * powerup.side;
        } else if powerup.timer < powerup.time_2 {
            let force = powerup.time_2 - powerup.timer;
            *moveable.velocity.y_mut() = -100.0 * force * powerup.side;
        } else if powerup.timer >= powerup.time_2 {
            powerup.timer = 0.0;
        }

        if translation.y().abs() - aabb.half_e.y() > crate::SCREEN_HEIGHT / 2.0 {
            commands.despawn(e);
        } else {
            for col in moveable.get_collisions().iter() {
                if let Ok(mut result) = player_query.entity(*col) {
                    if let Some((mut player, player_translation)) = result.get() {
                        *translation.x_mut() = player_translation.x();
                        *translation.y_mut() = player_translation.y();
                        audio_output.play(res.0.get_sound("assets/sounds/powerup.mp3"));
                        (powerup.power_up_fn)(&mut player);
                        commands.despawn(e);
                    }
                }
            }
        }

    }
}

pub fn get_random_power_up() -> fn(commands: &mut Commands, res: &GameRes, pos: Vec2)
{
    if rand::random::<u32>() % 2 == 0 {
        let power_up = rand::random::<u32>() % 9;
        if power_up < 4 {
            return health_up::add;
        } else if power_up == 4 {
            return max_health_up::add;
        } else if power_up == 5 {
            return max_dash_up::add;
        } else if power_up == 6 {
            return bullet_speed_up::add;
        } else if power_up == 7 {
            return fire_rate_up::add;
        } else if power_up == 8 {
            return dash_speed_up::add;
        }
    }
    no_power_up
} 