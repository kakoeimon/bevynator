use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_kako_tools::*;

pub mod animations;
pub mod explosion;
pub mod stage;
pub mod placeholder;
pub mod camera;
pub mod player;
pub mod player_bullet;
pub mod trail;
pub mod background;
pub mod frame_counter;
pub mod enemies;
pub mod powerups;
pub mod status;


const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 640.0;



pub struct Attack(f32);

pub struct GameRes(pub KaResources);




impl FromResources for GameRes{
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let mut textures = resources.get_mut::<Assets<Texture>>().unwrap();
        let mut textures_atlases = resources.get_mut::<Assets<TextureAtlas>>().unwrap();

        let mut res = Self(KaResources::new());

        res.0.load_color_material(&asset_server, &mut materials, "assets/title.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/status_bar.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/ship1.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/bg1.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/bullet1.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/trail.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/enemy_ship_1.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/enemy_ship_2.png");
        res.0.load_texture_atlas(&asset_server, &mut textures, &mut textures_atlases, "assets/bullet_explosion.png", 7, 1);
        res.0.load_color_material(&asset_server, &mut materials, "assets/enemy_bullet1.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/powerups/health.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/powerups/max_health.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/powerups/dash_up.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/powerups/dash_speed.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/powerups/bullet_speed.png");
        res.0.load_color_material(&asset_server, &mut materials, "assets/powerups/fire_rate.png");

        res.0.load_color_material(&asset_server, &mut materials, "assets/game_over.png");

        
        res.0.load_sound(&asset_server, "assets/sounds/fire.mp3");
        res.0.load_sound(&asset_server, "assets/sounds/hit.mp3");
        res.0.load_sound(&asset_server, "assets/sounds/explosion.mp3");
        res.0.load_sound(&asset_server, "assets/sounds/powerup.mp3");
        res.0.load_sound(&asset_server, "assets/sounds/damage.mp3");
        res.0.load_sound(&asset_server, "assets/sounds/powerup.mp3");
        res

    }
}


fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<GameRes>()
        .add_resource( camera::CamPos( Vec2::new(0.0, 0.0) ) )
        .add_resource( player::PlayerPos( Vec2::new(0.0, 0.0)))
        .add_resource(bevy::render::pass::ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(enemies::EnemyPlugin)
        .add_plugin(powerups::PowerUpsPlugin)
        .add_plugin(status::StatusPlugin)
        .add_startup_system(camera::add_camera.system())

        //.add_startup_system(status::add.system())
        //.add_startup_system(player::add_player.system())
        //.add_startup_system(background::add_background.system())
        //.add_startup_system(stage::stage1.system())


        //.add_startup_system(frame_counter::add_frame_counter.system())

        

        //.add_startup_system(bullet_explosion::setup.system())
        
        .add_system(camera::move_camera.system())
        //.add_system(ka_aabb_initial_pos.system())
        .add_system(ka_aabb_move.system())
        .add_system(player::update.system())
        .add_system(trail::update.system())
        .add_system(player_bullet::bullet_system.system())
        //.add_system(frame_counter::frame_counter_system.system())
        .add_system(placeholder::update.system())

        .add_system(animations::one_off_animate_sprite_and_despawn_system.system())
        
        //.add_system(count_sprites.system())
        .run();
}


fn count_sprites(mut query: Query<&Sprite>) {
    let mut num = 0;
    for sprite in &mut query.iter() {
        num += 1;
    }
    println!("number of sprites : {}", num);
} 









