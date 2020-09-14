use bevy::prelude::*;
use crate::GameRes;
use crate::camera::CamPos;

const BAR_SIZE: f32 = 18.0;
const BAR_SCALE: f32 = 2.0;
const BAR_MARGIN: f32 = 4.0;

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        
        app.add_startup_system(start_game.system());
        app.add_system(title_screen_system.thread_local_system());
        app.add_system(update.system());
        app.add_system(gameover_system.thread_local_system());
        
    }
}

pub struct StatusBack;
pub struct StatusBar;
pub struct HealthBarStatic;
pub struct HealthBar;
pub struct DashBarStatic;
pub struct DashBar;

pub fn add(world: &mut World, res: &GameRes, materials: &mut Assets<ColorMaterial>,) {
    
    //HEALTH MOVING BAR
    let e = world.spawn(
        NodeComponents {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(BAR_SIZE)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(16.0),
                    left: Val::Px(16.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
            ..Default::default()
        }
    );
    world.insert_one(e, HealthBar{}).unwrap();

    //HEALTH STATIC BAR
    let e = world.spawn(
        NodeComponents {
            style: Style {
                size: Size::new(Val::Px(200.0 + BAR_MARGIN * 2.0), Val::Px(BAR_SIZE + BAR_MARGIN * 2.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(16.0 - BAR_MARGIN),
                    left: Val::Px(16.0 - BAR_MARGIN),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.3, 0.05, 0.05).into()),
            ..Default::default()
        }
    );
    world.insert_one(e, HealthBarStatic{}).unwrap();

    //DASH MOVING BAR
    let e = world.spawn(
        NodeComponents {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(BAR_SIZE)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(44.0),
                    left: Val::Px(16.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.1, 0.6, 0.1).into()),
            ..Default::default()
        }
    );
    world.insert_one(e, DashBar{}).unwrap();

    //DASH STATIC BAR
    let e = world.spawn(
        NodeComponents {
            style: Style {
                size: Size::new(Val::Px(200.0 + BAR_MARGIN * 2.0), Val::Px(BAR_SIZE + BAR_MARGIN * 2.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(44.0 - BAR_MARGIN),
                    left: Val::Px(16.0 - BAR_MARGIN),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.05, 0.2, 0.05).into()),
            ..Default::default()
        }
    );
    world.insert_one(e, DashBarStatic{}).unwrap();
    
    //BACKGROUND
    
    let e = world.spawn(
        ImageComponents {
            style: Style {
                size: Size::new(Val::Px(crate::SCREEN_WIDTH), Val::Auto),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()

            },
            material: res.0.get_color_material("assets/status_bar.png"),
            ..Default::default()
        }
    );
    world.insert_one(e, StatusBack{}).unwrap();
    
    
}

pub struct GameOver;
pub fn game_over(commands: &mut Commands, res: &GameRes)
{
    commands.spawn(
        ImageComponents {
            style: Style {
                size: Size::new(Val::Px(crate::SCREEN_WIDTH), Val::Auto),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(80.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()

            },
            material: res.0.get_color_material("assets/game_over.png"),
            ..Default::default()
        }
    ).with(GameOver{});
}

pub fn update(
    mut health_query: Query<(&HealthBar, &mut Style)>,
    mut health_static_query: Query<(&HealthBarStatic, &mut Style)>,
    mut dash_query: Query<(&DashBar, &mut Style)>,
    mut dash_static_query: Query<(&DashBarStatic, &mut Style)>,
    mut player_query: Query<&crate::player::Player>,


)
{
    for player in &mut player_query.iter() {
        for (_health, mut style) in &mut health_query.iter() {
            style.size.width = Val::Px(player.health * BAR_SCALE);
        }
        for (_health_static, mut style) in &mut health_static_query.iter() {
            style.size.width = Val::Px(player.max_health * BAR_SCALE + BAR_MARGIN * 2.0);
        }

        for (_dash, mut style) in &mut dash_query.iter() {
            style.size.width = Val::Px(player.dash * BAR_SCALE);
        }
        for (_dash_static, mut style) in &mut dash_static_query.iter() {
            style.size.width = Val::Px(player.max_dash * BAR_SCALE + BAR_MARGIN * 2.0);
        }

        
        
    }
    
}

fn gameover_system(world: &mut World, resources: &mut Resources) {
    let keyboard_input = resources.get::<Input<KeyCode>>().unwrap();
    let mut the_game_is_over = false;
    let mut to_despawn: Vec<Entity> = Vec::new();
    for (e, _) in world.query::<(Entity, &GameOver)>().iter() {
        if keyboard_input.pressed(KeyCode::Escape) {
            the_game_is_over = true;
            to_despawn.push(e);
        }
        
    }
    if the_game_is_over{
        for (e, _) in world.query::<(Entity, &Sprite)>().iter() {
            to_despawn.push(e);
        }
    
        for (e, _) in world.query::<(Entity, &crate::placeholder::Placeholder)>().iter() {
            to_despawn.push(e);
        }
    
        for (e, _) in world.query::<(Entity, &Style)>().iter() {
            to_despawn.push(e);
        }
    
    
        for entity in to_despawn.iter() {
            match world.despawn(*entity) {
                Ok(()) => (),
                Err(_) => (),
            }
        }

        let res = resources.get::<GameRes>().unwrap();
        let e = world.spawn(
            ImageComponents {
                style: Style {
                    size: Size::new(Val::Px(crate::SCREEN_WIDTH), Val::Auto),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
    
                },
                material: res.0.get_color_material("assets/title.png"),
                ..Default::default()
            }
        );
        world.insert_one(e, TitleScreen{}).unwrap();
        //.with(TitleScreen{});
    }
    
}


fn start_game(mut commands: Commands, res: Res<GameRes>) {
    title_screen(&mut commands, &res);
}
pub struct TitleScreen;
fn title_screen(commands: &mut Commands, res: &GameRes) {

    commands.spawn(
        ImageComponents {
            style: Style {
                size: Size::new(Val::Px(crate::SCREEN_WIDTH), Val::Auto),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()

            },
            material: res.0.get_color_material("assets/title.png"),
            ..Default::default()
        }
    ).with(TitleScreen{});

}

fn title_screen_system(world: &mut World, resources: &mut Resources) {
    let res = resources.get::<GameRes>().unwrap();
    let keyboard_input = resources.get::<Input<KeyCode>>().unwrap();
    let mut start_game = false;
    let mut to_despawn: Vec<Entity> = Vec::new();
    for (e, _) in world.query::<(Entity, &TitleScreen)>().iter() {
        if keyboard_input.pressed(KeyCode::Space) {
            start_game = true;
            to_despawn.push(e);
        }
        
    }
    if start_game{
        use bevy::render::camera::Camera;
        for (e, _) in world.query::<(Entity, &Sprite)>().iter() {
            to_despawn.push(e);
        }
    
        for (e, _) in world.query::<(Entity, &crate::placeholder::Placeholder)>().iter() {
            to_despawn.push(e);
        }
    
        for (e, _) in world.query::<(Entity, &Style)>().iter() {
            to_despawn.push(e);
        }
    
    
        for entity in to_despawn.iter() {
            match world.despawn(*entity) {
                Ok(()) => (),
                Err(_) => (),
            }
        }

        crate::stage::stage1(world);
        for (_, _, mut translation) in world.query::<(&crate::camera::ScrollingCamera, &Camera, &mut Translation)>().iter() {
            translation.set_x(0.0);
        }
        let mut cam_pos = resources.get_mut::<CamPos>().unwrap();
        cam_pos.0.set_x(0.0);

        crate::player::add_player(world, &res);
        crate::background::add_background(world, &res);
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        add(world, &res, &mut materials);

    }
    
}


