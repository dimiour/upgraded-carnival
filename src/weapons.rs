use crate::object::ObjectClass;

pub const SNIPER: Weapon = Weapon {
    recoil_scale: 0.01, // scale depending on the stretch of the gun
    max_stretch: 0.05,
    min_stretch: 0.025,
    speed_scale: 0.4,
    bullet_size: 0.014,
    gun_size: 0.02,
    fade_time: 1.0,
    fire_rate: 0.5,
    spread_count: 1,
    angle_spread: 0.1,
    bullet_class: ObjectClass::Bullet,
};

pub const LEAP: Weapon = Weapon {
    recoil_scale: -0.5,
    max_stretch: 0.05,
    min_stretch: 0.025,
    speed_scale: 0.1,
    bullet_size: 0.009,
    gun_size: 0.02,
    fade_time: 0.0,
    fire_rate: 0.2,
    spread_count: 1,
    angle_spread: 0.0,
    bullet_class: ObjectClass::Bullet,
};

pub const ROCKET: Weapon = Weapon {
    recoil_scale: 0.1,
    max_stretch: 0.05,
    min_stretch: 0.025,
    speed_scale: 0.2,
    bullet_size: 0.01,
    gun_size: 0.03,
    fade_time: 0.5,
    fire_rate: 0.3,
    spread_count: 3,
    angle_spread: 0.3,
    bullet_class: ObjectClass::Bullet,
};

pub const BUILDER: Weapon = Weapon {
    recoil_scale: 0.001,
    max_stretch: 0.05,
    min_stretch: 0.025,
    speed_scale: 0.001,
    bullet_size: 0.015,
    gun_size: 0.04,
    fade_time: 1.0,
    fire_rate: 0.5,
    spread_count: 6,
    angle_spread: 0.6,
    bullet_class: ObjectClass::Wall,
};

#[derive(Copy, Clone)]
pub struct Weapon {
    pub gun_size: f32, 
    pub recoil_scale: f32, 
    pub max_stretch: f32,
    pub min_stretch: f32, 
    pub speed_scale: f32,
    pub bullet_size: f32,
    pub fade_time: f64, 
    pub fire_rate: f64,
    pub spread_count: u8, // spread_count cant equal 0 or error will occur
    pub angle_spread: f32,
    pub bullet_class: ObjectClass,
}