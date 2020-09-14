use bevy::prelude::*;
use std::collections::HashMap;


pub struct KaResources {
    color_material_handles: HashMap<String, Handle<ColorMaterial>>,
    pub texture_atlas_handles: HashMap<String, Handle<TextureAtlas>>,
    pub sound_handles: HashMap<String, Handle<AudioSource>>,
}

impl KaResources {
    pub fn new() -> Self {
        Self {
            color_material_handles: HashMap::new(),
            texture_atlas_handles: HashMap::new(),
            sound_handles: HashMap::new(),
        }
        
    }
    
    pub fn load_color_material(&mut self, asset_server: &Ref<AssetServer>, materials: &mut RefMut<Assets<ColorMaterial>>, path: &str) {
        let handle = materials.add(asset_server.load(path).unwrap().into());
        self.color_material_handles.insert(path.to_owned(), handle);
    }

    pub fn get_color_material(&self, path: &str) -> Handle<ColorMaterial> {
        *self.color_material_handles.get(path).unwrap()
    }

    pub fn load_texture_atlas(&mut self, asset_server: &Ref<AssetServer>, textures: &mut RefMut<Assets<Texture>>, textures_atlases: &mut RefMut<Assets<TextureAtlas>>, path: &str, col: usize, row: usize) {
        let texture_handle = asset_server.load_sync( textures, path).unwrap();
        let texture = textures.get(&texture_handle).unwrap();
        let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, col, row);
        let texture_atlas_handle = textures_atlases.add(texture_atlas);

        self.texture_atlas_handles.insert(path.to_owned(), texture_atlas_handle);
    }

    pub fn get_texture_atlas(&self, path: &str) -> Handle<TextureAtlas> {
        *self.texture_atlas_handles.get(path).unwrap()
    }

    pub fn load_sound(&mut self, asset_server: &Ref<AssetServer>, path: &str) {
        let fire_sound: Handle<AudioSource> = asset_server.load(path).unwrap();
        self.sound_handles.insert(path.to_owned(), fire_sound);
    }

    pub fn get_sound(&self, path: &str) -> Handle<AudioSource> {
        *self.sound_handles.get(path).unwrap()
    }
}




