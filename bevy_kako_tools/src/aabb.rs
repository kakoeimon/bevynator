use std::sync::RwLock;

use bevy::{
    core::{Time},
    ecs::{Query, Res, Entity, Added},
    math::Vec2,
    transform::components::{Translation},
    
};


const MARGIN: f32 = 0.01;

pub struct KaKinematic;
pub struct KaSratic;
pub struct KaSensor;


pub struct KaMoveable {
    pub velocity: Vec2,
    pub on_wall: Option<Entity>,
    pub on_floor: Option<Entity>,
    pub on_ceilling: Option<Entity>,
    pub pushable: bool,
    pub external_forces: Vec2,
    pub external_forces_mult: f32
}

impl KaMoveable {
    pub fn new(velocity_x: f32, velocity_y: f32, pushable: bool) -> Self {
        Self {
            velocity: Vec2::new(velocity_x, velocity_y),
            on_wall: None,
            on_floor: None,
            on_ceilling: None,
            pushable,
            external_forces: Vec2::zero(),
            external_forces_mult: 50.0,
        }
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        *self.velocity.x_mut() = x;
        *self.velocity.y_mut() = y;
    }

    pub fn get_collisions(&self) -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::new();
        match self.on_floor {
            None => (),
            Some(e) => {entities.push(e.clone());}
        };

        match self.on_wall {
            None => (),
            Some(e) => {entities.push(e.clone());}
        };

        match self.on_ceilling {
            None => (),
            Some(e) => {entities.push(e.clone());}
        };

        entities
    }
    
}

pub struct KaAABB{
    pub pos: RwLock<Vec2>,
    pub half_e: Vec2,
    pub collision_layer: i32,
    pub collision_mask: i32,
    pub solid: bool,
    pub one_way: bool,
    pub collision_exceptions: Vec<u128>,
}

impl KaAABB {
    pub fn new(x: f32, y: f32, w: f32, h: f32, 
        collision_layer: i32, collision_mask: i32, solid: bool, one_way: bool
    ) -> Self 
    {
        Self {
            pos: RwLock::new(Vec2::new(x, y)),
            half_e: Vec2::new(w / 2.0, h / 2.0),
            collision_layer,
            collision_mask,
            solid,
            one_way,
            collision_exceptions: Vec::new(),
        }
    }

    pub fn overlaps(pos1: &Vec2, half_size1: &Vec2, pos2: &Vec2, half_size2: &Vec2) -> bool {
        if (pos1.x() - pos2.x()).abs() > half_size1.x() + half_size2.x() {return false;};
        if (pos1.y() - pos2.y()).abs() > half_size1.y() + half_size2.y() {return false;};
        true
    }

    pub fn overlap_min_max(min1: &Vec2, max1: &Vec2, min2: &Vec2, max2:&Vec2) -> bool {
        if max1.x() < min2.x() || min1.x() > max2.x() {return false;};
        if max1.y() < min2.y() || min1.y() > max2.y() {return false;};
        true
    }


    pub fn set_collision_layer(&mut self, bits: &[i32]) {
        for bit in bits.iter() {
            self.collision_layer |= 1<< bit;
        }
    }

    pub fn create_collision_layer(bits: &[i32]) -> i32{
        let mut layer: i32 = 0;
        for bit in bits.iter() {
            layer |= 1<< bit;
        }
        layer
    }

    pub fn set_collision_bit(layer: &mut i32, bit: i32, value: bool) {
        if value {
            *layer |= 1 << bit;
        } else {
            *layer &= !(1 << bit)
        }
    }

    pub fn get_collision_bit(layer: i32, bit: i32) -> bool {
        if layer & (1 << bit) == 0 {
            return false;
        }
        true
    }

    pub fn set_collision_layer_bit(&mut self, bit: i32, value: bool) {
        KaAABB::set_collision_bit(&mut self.collision_layer, bit, value);
    }

    pub fn get_collision_layer_bit(&self, bit: i32) -> bool {
        KaAABB::get_collision_bit(self.collision_layer, bit)
    }

    pub fn set_collision_mask_bit(&mut self, bit: i32, value: bool) {
        KaAABB::set_collision_bit(&mut self.collision_mask, bit, value);
    }
    
    pub fn get_collision_mask_bit(&self, bit: i32) -> bool {
        KaAABB::get_collision_bit(self.collision_mask, bit)
    }

    pub fn have_exception(&self, id: u128) -> bool {
        self.collision_exceptions.iter().any(|&i| i == id)
    }

    pub fn add_exception(&mut self, id: u128) {
        if !self.have_exception(id) {
            self.collision_exceptions.push(id);
        }
        
    }

    pub fn can_collide(&self, other: &KaAABB) -> bool {
        if other.collision_layer & self.collision_mask == 0 {
            return false;
        }
        true
    }
    
}


pub fn ka_aabb_initial_pos(mut query: Query<(Added<KaAABB>, &mut Translation)> ) {
    
    for (aabb, mut translation) in &mut query.iter() {
        let pos = aabb.pos.read().unwrap();
        *translation.x_mut() = pos.x();
        *translation.y_mut() = pos.y();
    }
    
}




pub fn ka_aabb_move(time: Res<Time>,
    mut query1: Query<(&KaAABB, &mut KaMoveable, Entity)>,
    mut query2: Query<(&KaAABB, Entity)>,
    mut query3: Query<(&KaAABB, &KaMoveable, &mut Translation)>,
)
{
    for (aabb1, mut moveable, e1) in &mut query1.iter() {
        moveable.on_floor = None;
        moveable.on_ceilling = None;
        moveable.on_wall = None;
        let vel = (moveable.velocity + moveable.external_forces) * time.delta_seconds;
        let moveable_force_mult = moveable.external_forces_mult * time.delta_seconds;
        moveable.external_forces *= moveable_force_mult;
        let mut pos1 = aabb1.pos.write().unwrap();
        
        let mut vel_min = Vec2::new(pos1.x() - aabb1.half_e.x(), pos1.y() - aabb1.half_e.y());
        let mut vel_max = Vec2::new(pos1.x() + aabb1.half_e.x(), pos1.y() + aabb1.half_e.y());
        
        let mut closest_entity: Option<Entity> = None;
        //Move on X
        if vel.x() != 0.0 {
            if vel.x() > 0.0 {
                *vel_max.x_mut() += vel.x();
            } else {
                *vel_min.x_mut() += vel.x();
            }
            for (aabb2, e2) in &mut query2.iter() {
                if e1.id() != e2.id() && aabb2.solid  && aabb1.can_collide(&aabb2) 
                    && !aabb1.have_exception(e2.id()) && !aabb2.have_exception(e1.id())
                {
                    let pos2 = aabb2.pos.read().unwrap();
                    let min2 = Vec2::new(pos2.x() - aabb2.half_e.x(), pos2.y() - aabb2.half_e.y());
                    let max2 = Vec2::new(pos2.x() + aabb2.half_e.x(), pos2.y() + aabb2.half_e.y());
                    if aabb2.one_way && (pos1.y() - aabb1.half_e.y()) < max2.y() { continue; };
                    if KaAABB::overlap_min_max(&vel_min, &vel_max, &min2, &max2) {
                        closest_entity = Some(e2);
                        if vel.x() > 0.0 {
                            *vel_max.x_mut() = min2.x() - MARGIN;
                        } else {
                            *vel_min.x_mut() = max2.x() + MARGIN;
                        }
                    }
                    
                }
            }
            if vel.x() > 0.0 {
                *pos1.x_mut() = vel_max.x() - aabb1.half_e.x();
            } else {
                *pos1.x_mut() = vel_min.x() + aabb1.half_e.x();
            }
            moveable.on_wall = closest_entity;
        }

        //MOVE ON Y
        vel_min = Vec2::new(pos1.x() - aabb1.half_e.x() + MARGIN, pos1.y() - aabb1.half_e.y());
        vel_max = Vec2::new(pos1.x() + aabb1.half_e.x() - MARGIN, pos1.y() + aabb1.half_e.y());
        
        let mut closest_entity: Option<Entity> = None;
        if vel.y() != 0.0 {
            if vel.y() > 0.0 {
                *vel_max.y_mut() += vel.y();
            } else {
                *vel_min.y_mut() += vel.y();
            }
            for (aabb2, e2) in &mut query2.iter() {
                if e1.id() != e2.id() && aabb2.solid  && aabb1.can_collide(&aabb2) 
                    && !aabb1.have_exception(e2.id()) && !aabb2.have_exception(e1.id())
                {
                    let pos2 = aabb2.pos.read().unwrap();
                    let min2 = Vec2::new(pos2.x() - aabb2.half_e.x(), pos2.y() - aabb2.half_e.y());
                    let max2 = Vec2::new(pos2.x() + aabb2.half_e.x(), pos2.y() + aabb2.half_e.y());
                    
                    if aabb2.one_way && (pos1.y() - aabb1.half_e.y()) < max2.y() { continue; };
                    if KaAABB::overlap_min_max(&vel_min, &vel_max, &min2, &max2) {
                        closest_entity = Some(e2);
                        if vel.y() > 0.0 {
                            *vel_max.y_mut() = min2.y() - MARGIN;
                        } else {
                            *vel_min.y_mut() = max2.y() + MARGIN;
                        }
                    }
                    
                }
            }
            if vel.y() > 0.0 {
                *pos1.y_mut() = vel_max.y() - aabb1.half_e.y();
                moveable.on_ceilling = closest_entity;
            } else {
                *pos1.y_mut() = vel_min.y() + aabb1.half_e.y();
                moveable.on_floor = closest_entity;
            }
        }
        
    }

    
    
    
    for (aabb, _moveable, mut translation) in &mut query3.iter() {
        let pos = aabb.pos.read().unwrap();
        *translation.x_mut() = pos.x();
        *translation.y_mut() = pos.y();
    }
    

}
