use bevy::prelude::*;

use crate::GameRes;

pub fn add(commands: &mut Commands, res: &GameRes, x:f32 , y:f32, scale: f32)
{
    commands.spawn(
        SpriteSheetComponents {
            translation: Translation::new(x, y, 4.0),
            scale: Scale(scale),
            texture_atlas: res.0.get_texture_atlas("assets/bullet_explosion.png"),
            ..Default::default()
        }
    )
    .with(Timer::from_seconds(0.033, true))
    .with(crate::animations::OneOffAnimDespawn);

}