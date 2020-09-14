use bevy::prelude::*;


pub struct Trail {
    pub max_time: f32,
    pub timer: f32,
}

impl Trail {
    pub fn new(max_time: f32) -> Self {
        Self {
            max_time,
            timer: 0.0
        }
    }
}

pub fn add(commands: &mut Commands,
    material: Handle<ColorMaterial>,
    x: f32, y: f32,
    trail_max_time: f32,
)
{

    commands.spawn(SpriteComponents {
        translation: Translation::new(x,y, 4.0),
        material,
        ..Default::default()
    })
    .with(Trail::new(trail_max_time));
}



pub fn update(mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Trail, &mut Scale, Entity)>
) 
{
    for (mut trail, mut scale, e) in &mut query.iter() {
        trail.timer += time.delta_seconds;

        scale.0 = (1.0 - trail.timer / trail.max_time).max(0.0);
        if trail.timer >= trail.max_time {
            commands.despawn(e);
        }
    }
}