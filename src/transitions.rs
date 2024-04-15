use std::{f32::consts::PI, fmt::Result};
use macroquad::prelude::*;

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

pub struct Transition {
    pub class: TransitionClass,
    start_time: f64,
    position: Vec2,
}

impl Transition {
    fn new(class: TransitionClass, position: Vec2) -> Self {
        Self {
            class,
            start_time: get_time(),
            position, 
        }
    }

    fn draw(&mut self) {
        let time = get_time()-self.start_time;

        if time < 1.0 {
            match self.class {
                TransitionClass::Release(release_position) => {
                    let relative_position = release_position-self.position;
                    let release = relative_position*half_arc(time)+self.position;
                    
                    draw_line(
                        self.position.x, 
                        self.position.y, 
                        release.x, 
                        release.y, 
                        0.02, 
                        MAGENTA
                    );
                    
                    draw_circle(release.x, release.y, 0.01, MAGENTA);
                    draw_circle(self.position.x, self.position.y, 0.01, MAGENTA);
                },

                TransitionClass::Tap => {
                    draw_circle(self.position.x, self.position.y, arc(time)*0.2, MAGENTA)
                },
            }
        }
    }
}

pub enum TransitionClass {
    Tap,
    Release(Vec2),
}
