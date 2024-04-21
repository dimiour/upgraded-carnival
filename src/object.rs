use macroquad::prelude::*;
use std::f32::consts::PI;

const DRAG: f32 = 0.98;
const ABSORBTION: f32 = 0.2;
pub const BORDER_SIZE: f32 = 0.2;

static mut NEW_OBJECT_ID: usize = 0;

fn new_object_id() -> usize {
    unsafe {
        NEW_OBJECT_ID += 1;
        NEW_OBJECT_ID
    }
}

#[derive(Copy, Clone)]
pub enum ObjectClass {
    Bullet,
    Wall,
    Player,
}

#[derive(Clone)]
pub struct Object {
    pub velocity: Vec2,
    pub position: Vec2,
    pub size: f32,
    pub fade: f64,
    pub id: usize,
    pub class: ObjectClass,
}

impl Object {
    pub fn new(position: Vec2, velocity: Vec2, size: f32, fade: f64, class: ObjectClass) -> Self {
        Self { position, velocity, size, fade, id: new_object_id(), class }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        self.velocity *= Vec2::splat(DRAG).powf(1.0-(0.02-self.size).powf(0.1));
        

        let self_border = -self.size+BORDER_SIZE;//+(get_time() as f32*0.1).sin()*0.4;
        if self.position.y > self_border {
            self.velocity.y -= (self.position.y-self_border)*ABSORBTION;
        }

        if self.position.y < -self_border {
            self.velocity.y -= (self.position.y+self_border)*ABSORBTION;
        }

        if self.position.x > self_border {
            self.velocity.x -= (self.position.x-self_border)*ABSORBTION;
        }

        else if self.position.x < -self_border {
            self.velocity.x -= (self.position.x+self_border)*ABSORBTION;
        }

        
    }

    pub fn modify_velocity(&mut self, rhs: &Object) {
        if self.position.distance(rhs.position) < rhs.size + self.size {
            let position_difference = self.position-rhs.position;
            let unit_position_difference = position_difference.clamp_length(1.0, 1.0);
            let combined_mass = self.size.powf(2.0) + rhs.size.powf(2.0);
            self.velocity += (
                calculate_bounce(self.velocity, unit_position_difference)+
                calculate_bounce(-rhs.velocity, unit_position_difference)
            )*rhs.size.powf(2.0)/combined_mass*ABSORBTION;
        } 
        
        
    }
}

fn calculate_bounce(velocity: Vec2, direction_vector: Vec2) -> Vec2 {
    let angle_difference: f32 = velocity.angle_between(-direction_vector);

    if angle_difference.abs() < PI*0.5 {
        let length = angle_difference.cos()*velocity.length()*2.0;
        direction_vector*Vec2::splat(length)
    } else {
        Vec2::ZERO
    }
}

