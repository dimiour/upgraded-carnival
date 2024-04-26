use std::f32::consts::PI;
use macroquad::prelude::*;
use crate::window::*;

const TRANSITION_SPEED: f64 = 0.25;

pub fn arc(t: f64) -> f32 {
    if t < 1.0 {
        (t as f32*PI).sin()
    } else {
        0.0
    }
}

pub fn half_arc(t: f64) -> f32 {
    if t < 1.0 {
        (t as f32*PI*0.5).sin()
    } else {
        1.0
    }
}

#[derive(Debug)]
pub struct Transition {
    pub class: TransitionClass,
    start_time: f64,
    position: Vec2,
}

impl Transition {
    pub fn new(class: TransitionClass, position: Vec2) -> Self {
        Self {
            class,
            start_time: get_time(),
            position, 
        }
    }

    pub fn draw(&mut self) -> bool {
        let time = get_time()-self.start_time;

        if time < TRANSITION_SPEED {
            match self.class {
                TransitionClass::Release(release_position) => {
                    
                    let relative_position = release_position-self.position;
                    let position = self.position+ui_position();
                    let release = relative_position*(1.0-half_arc(time/TRANSITION_SPEED))+position;
                    
                    draw_line(
                        position.x, 
                        position.y, 
                        release.x, 
                        release.y, 
                        0.02, 
                        GREEN
                    );
                    
                    draw_circle(release.x, release.y, 0.01, GREEN);
                    draw_circle(position.x, position.y, 0.01, GREEN);
                },

                TransitionClass::Tap => {
                    let position = self.position+ui_position();
                    draw_circle(position.x, position.y, arc(time/TRANSITION_SPEED)*0.03, BLUE)
                },
            }
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
pub enum TransitionClass {
    Tap,
    Release(Vec2),
}
