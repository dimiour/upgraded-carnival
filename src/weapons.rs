pub const SNIPER: Weapon = Weapon {
    recoil_scale: 0.01,
    max_stretch: 0.05,
    min_stretch: 0.025,
    speed_scale: 0.2,
    bullet_size: 0.014,
    gun_size: 0.02,
    fade_time: 1.0,
};

pub const LEAP: Weapon = Weapon {
    recoil_scale: -0.1,
    max_stretch: 0.05,
    min_stretch: 0.025,
    speed_scale: 0.1,
    bullet_size: 0.009,
    gun_size: 0.02,
    fade_time: 0.2,
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
}