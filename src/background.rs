use bevy::prelude::*;
use crate::GameRes;

pub struct BG;


pub fn add_bg( world: &mut World,
    material: Handle<ColorMaterial>,
    x: f32
)
{
    let bg = world.spawn( SpriteComponents {
        translation: Translation::new(x, 0.0, 0.0),
        material,
        scale: Scale(4.0),
        ..Default::default()
    });
    world.insert(bg, (BG{}, ));


}

pub fn add_background(world: &mut World, res: &GameRes)
{
    for x in 0..2 {
        add_bg(world, res.0.get_color_material("assets/bg1.png"), x as f32 * 1280.0);
    }
    
}