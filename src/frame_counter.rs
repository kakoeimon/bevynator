use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FrameCounter;

pub fn add_frame_counter(mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    //println!("adsfasdf");
    commands.spawn(UiCameraComponents::default())
    .with(FrameCounter)
    .spawn( TextComponents {
        text: Text{
            value: "Hello".to_string(),
            font: asset_server.load("assets/FiraSans-Bold.ttf").unwrap(),
            style: TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
            },
        },
        ..Default::default()

    })
    .with(FrameCounter);
}


pub fn frame_counter_system(diagnostics: Res<Diagnostics>, mut query: Query<(&FrameCounter, &mut Text)>) {
    for (_, mut text) in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}