use bevy::prelude::*;
use rand::random;

use crate::GameRes;
use crate::placeholder::Placeholder;
use crate::enemies;
use crate::powerups::get_random_power_up;

const MAX_Y: f32 = (crate::SCREEN_HEIGHT - crate::enemies::enemy3::HEIGHT) / 2.0;

pub fn add_a_row_enemy1(world: &mut World, x: f32, y: f32, number: usize) {
    use crate::enemies::enemy1::{create_placeholder, WIDTH};
    let mut pos = Vec2::new(x, y);
    world.spawn( (create_placeholder(pos, get_random_power_up()) , ) );
    for _ in 0..number {
        *pos.x_mut() += WIDTH / 2.0;
        world.spawn( (create_placeholder(pos, get_random_power_up() ) , ) );
    }
}

pub fn add_a_row_enemy2(world: &mut World, x: f32, y: f32, number: usize) {
    use crate::enemies::enemy2::{create_placeholder, WIDTH};
    let mut pos = Vec2::new(x, y);
    world.spawn( (create_placeholder(pos, get_random_power_up()) , ) );
    for _ in 0..number {
        *pos.x_mut() += WIDTH;
        world.spawn( (create_placeholder(pos, get_random_power_up()) , ) );
    }
}

pub fn add_a_row_enemy3(world: &mut World, x: f32, y: f32, number: usize) {
    use crate::enemies::enemy3::{create_placeholder, WIDTH};
    let mut pos = Vec2::new(x, y);
    world.spawn( (create_placeholder(pos, get_random_power_up()) , ) );
    for _ in 0..number {
        *pos.x_mut() += WIDTH;
        world.spawn( (create_placeholder(pos, get_random_power_up()) , ) );
    }
}



pub fn stage1(world: &mut World) {
    let mut progress = 5; //1 progress means 2 enemy1 or enemy2 or 1 enemy3
    let mut x = 0.0;
    for _ in 0..100 {
        x += crate::SCREEN_WIDTH;
        progress += 1;
        let mut enemies = progress / 5;
        while enemies > 0 {
            let mut to_add = 1 + rand::random::<u32>() % enemies;
            //println!("progress : {} to_add: {}", progress, to_add);
            enemies -= to_add;
            let mut enemy1 = 0;
            let mut enemy2 = 0;
            let mut enemy3 = 0;
            while to_add > 0 {
                let choose = rand::random::<u32>() % 3;
                if choose == 0 {
                    enemy1 += 2;
                } else if choose == 1 {
                    enemy2 += 2;
                } else if choose == 2 {
                    enemy3 += 1
                }
                to_add -= 1;
                
            }

            if enemy1 != 0 {
                let mut y = MAX_Y * random::<f32>();
                add_a_row_enemy1(world, x, y, enemy1 / 2);
                add_a_row_enemy1(world, x, -y, enemy1 / 2);
            }

            if enemy2 != 0 {
                let mut y = MAX_Y * random::<f32>();
                add_a_row_enemy2(world, x, y, enemy2 / 2);
                add_a_row_enemy2(world, x, -y, enemy2 / 2);
            }

            if enemy3 != 0 {
                let mut y = MAX_Y * random::<f32>();
                if rand::random::<i32>() < 0 { y *= -1.0};
                add_a_row_enemy3(world, x, y, enemy3);
            }
            
        }
        

    }
}
