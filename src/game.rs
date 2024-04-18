use crate::screen_mouse_position;
use crate::object::*;
use crate::weapons::*;
use crate::transitions::*;

use macroquad::prelude::*;
use std::f32::consts::PI;
use std::f64::INFINITY;


const TPS: f64 = 60.0;
pub const TICK_LENGTH: f64 = 1.0/TPS;


//GAME
#[derive(Clone)]
pub struct Game {
    pub interaction: Option<(f64, Vec2, bool)>,
    pub player: usize,
    pub map: Vec<Object>,
}

impl Game {
    pub fn new() -> Self {
        let mut game: Self = Self {
            interaction: None,
            player: 0,
            map: vec![]
        };
    
        for x in -3..4 {
            for y in -3..4 {
                game.player = game.map.len();
                game.map.push(Object::new(
                    vec2(0.05*x as f32, 0.05*y as f32), 
                    vec2(0.0, 0.0), 
                    0.02, 
                    INFINITY
                ));
            }
        }

        game
    }

    pub fn tick(&mut self) {
        let rhs_map = self.map.clone();

        let mut delete_list = vec![];
        for (index, object) in self.map.iter_mut().enumerate() {
            for rhs in rhs_map.iter() {
                object.modify_velocity(rhs);
                
            }

            if object.fade <= 0.0 {
                delete_list.insert(0, index)
            } else {
                object.fade -= TICK_LENGTH;
            }
            
            object.update();
        }

        for object_index in delete_list.into_iter() {
            self.map.remove(object_index);
        }
    }

    pub fn interaction(&mut self, loadout: (Weapon, Weapon), equipped: bool) -> (bool, Option<Transition>) {
        if let Some((click_time, click_position, true)) = self.interaction {
            let time = get_time()-click_time;
            let weapon = if equipped {loadout.0} else {loadout.1};

            self.interaction = None;
            
            let release = 
                (screen_mouse_position()-click_position)
                .clamp_length(0.0, weapon.max_stretch);
            
            if release.length() > weapon.min_stretch {
                if time >= weapon.fire_rate {
                    self.perform_shot(weapon, release)
                }
                return (equipped, Some(Transition::new(TransitionClass::Release(release+click_position), click_position)))
            } else if time < 0.3 {
                return (!equipped, Some(Transition::new(TransitionClass::Tap, click_position)))
            }

            return (equipped, None)
        }

        (equipped, None)
    }

    pub fn draw_map(&self) {
        for (index, object) in self.map.iter().enumerate() {
            let mut color = if self.player == index {
                BLUE
            } else {
                Color::new(
                    1.0, 
                    object.size*50.0-0.5, 
                    0.5-object.size*50.0, 
                    if object.fade > 0.25 {1.0} else {object.fade as f32/0.25}
                ) 
            };
            
            if object.size > 0.015 {
                draw_circle(object.position.x, object.position.y, object.size, color);
                color.r -= 0.1;color.g -= 0.1;color.b -= 0.1;
                draw_circle(object.position.x, object.position.y, object.size*0.8, color)
            } else {
                draw_poly_lines(
                    object.position.x, 
                    object.position.y,
                    3,
                    object.size, 
                    (get_time() as f32%1.0)*360.0,
                    0.004,
                    color
                );
            }  
        }
    }
    
    pub fn center(&self) -> Vec2 {
        self.map[self.player].position
    }

    fn perform_shot(&mut self, weapon: Weapon, release: Vec2) {
        let velocity = self.map[self.player].velocity;
        let initial_bullet_position = release.clamp_length(weapon.gun_size, weapon.gun_size);
                    
        for spread in 0..weapon.spread_count {
        //if (get_time()/get_frame_time() as f64).floor()%5.0 == 0.0 {
            let offset = 
                spread as f32
                -((weapon.spread_count/2) as f32)
                -0.5*(weapon.spread_count%2) as f32;
            let bullet_velocity = (release*weapon.speed_scale);
            
            let velocity_angle = 
            if bullet_velocity.x == 0.0 {
                if bullet_velocity.y > 0.0 {
                    PI/2.0
                } else {
                    -PI/2.0
                }
            } else if bullet_velocity.x > 0.0 {
                (bullet_velocity.y/bullet_velocity.x).atan()
            } else {
                (bullet_velocity.y/bullet_velocity.x).atan() + PI
            };
            
            
            
            let bullet_angle = Vec2::from_angle(weapon.angle_spread*offset+velocity_angle);
            self.map.push(Object::new(
                self.center()+bullet_angle*initial_bullet_position.length(),
                velocity+bullet_angle*bullet_velocity.length(), //
                weapon.bullet_size,
                weapon.fade_time
            ));
        }

        self.map[self.player].velocity -= release*weapon.recoil_scale;
    }
}