use macroquad::prelude::*;
use std::f32::consts::PI;

const DRAG: f32 = 0.99;
const ABSORBTION: f32 = 0.1;

#[derive(Clone, Copy)]
pub struct Object {
    pub velocity: Vec2,
    pub position: Vec2,
    pub size: f32,
    pub fade: f64,
}

impl Object {
    pub fn new(position: Vec2, velocity: Vec2, size: f32, fade: f64) -> Self {
        Self { position, velocity, size, fade }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        self.velocity *= Vec2::splat(DRAG).powf(1.0-(0.02-self.size).powf(0.1));
        

        let self_border = -self.size+1.0;
        if self.position.y > self_border {
            self.velocity.y = -self.velocity.y.abs()
        } else {
            //self.velocity.y += 0.0000001;
        }

        if self.position.y < -self_border {
            self.velocity.y = self.velocity.y.abs()
        }

        if self.position.x > self_border {
            self.velocity.x = -self.velocity.x.abs()
        }

        else if self.position.x < -self_border {
            self.velocity.x = self.velocity.x.abs()
        }

        
    }

    pub fn modify_velocity(&mut self, rhs: &Object) {
        if self.position.distance(rhs.position) < rhs.size + self.size {
            let position_difference = self.position-rhs.position;
            let unit_position_difference = position_difference.clamp_length(1.0, 1.0);
            let combined_mass = self.size.powf(2.0) + rhs.size.powf(2.0);
            self.velocity += (
                correspondance(self.velocity, unit_position_difference)+
                correspondance(-rhs.velocity, unit_position_difference)
            )*rhs.size.powf(2.0)/combined_mass*ABSORBTION;
        } 
        
        
    }
}

fn correspondance(velocity: Vec2, direction_vector: Vec2) -> Vec2 {
    let angle_difference: f32 = velocity.angle_between(-direction_vector);

    if angle_difference.abs() < PI*0.5 {
        let length = angle_difference.cos()*velocity.length()*2.0;
        direction_vector*Vec2::splat(length)
    } else {
        Vec2::ZERO
    }
}