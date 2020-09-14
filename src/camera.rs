use bevy::prelude::*;
use bevy_kako_tools::*;
use bevy::render::camera::Camera;
use crate::GameRes;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::player;
use crate::player_bullet;
use crate::background;

pub struct CamPos(pub Vec2);

pub const CAMERA_SPEED: f32 = 200.0;
pub struct ScrollingCamera;
pub struct UiCamera;

pub fn add_camera(mut commands: Commands) {
    commands.spawn( Camera2dComponents {
        translation: Translation::new(0.0, 40.0, 999.0),
        ..Default::default()
    })
    .with(ScrollingCamera);
    commands.spawn(UiCameraComponents::default())
    .with(UiCamera{});
}

pub fn move_camera( mut commands: Commands,
    time: Res<Time>,
    mut cam_pos: ResMut<CamPos>,
    mut cam_query: Query<(&Camera, &ScrollingCamera, &mut Translation)>,
    mut bg_query: Query<(&background::BG, &mut Translation, Entity)>,
    
) 
{
    let mut cam_x = 0.0;
    let mut cam_y = 0.0;
    let width = SCREEN_WIDTH; //windows.get_primary().unwrap().width as f32;
    let height = SCREEN_HEIGHT; //windows.get_primary().unwrap().height as f32;

    //GET THE DATA FROM CAMERA
    for (_, _, mut translation) in &mut cam_query.iter() {
        *translation.x_mut() += CAMERA_SPEED * time.delta_seconds;
        cam_x = translation.x();
        cam_y = translation.y();
    }
    cam_pos.0 = Vec2::new(cam_x, cam_y);
    

    

    for (_bg, mut translation, e) in &mut bg_query.iter() {
        if translation.x() < cam_x - width {
            *translation.x_mut() += width * 2.0;
            
        }
    }

}

