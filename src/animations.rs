use bevy::prelude::*;

pub struct OneOffAnimDespawn;


pub fn one_off_animate_sprite_and_despawn_system(
    mut commands: Commands, 
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, Entity)>,
) {
    for (timer, mut sprite, texture_atlas_handle, e) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            if sprite.index == 0 {
                commands.despawn(e);
            }
        }
    }
}