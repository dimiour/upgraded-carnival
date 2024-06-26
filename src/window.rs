use macroquad::prelude::*;

static mut CAMERA: Vec2 = Vec2 {x: 0.0, y: 0.0};

pub fn ui_position() -> Vec2 {
    unsafe {
        CAMERA
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_height: 2000,
        window_width: 1000,
        window_resizable: true,
        window_title: String::from("Macroquad Hacking"),
        ..Default::default()
    }
}

pub fn real_mouse_position() -> Vec2 {screen_mouse_position()+ui_position()}

pub fn screen_mouse_position() -> Vec2 {(mouse_position_local()+Vec2::ONE)*0.5*window_size()}

pub fn is_vertical() -> bool {screen_height() > screen_width()}

pub fn window_size() -> Vec2 {
    if is_vertical() {
        vec2(screen_width()/screen_height(), 1.0)
    } else {
        vec2(1.0, screen_height()/screen_width())
    }
}

pub fn set_window(center: Vec2) {
    let window_size = window_size();
    let window_rect = 
        Rect::new(0.0, 0.0, window_size.x, window_size.y)
            .offset(center-window_size*0.5);

    set_camera(&Camera2D::from_display_rect(window_rect));
    unsafe {
        CAMERA = center-window_size*0.5;
    }
}

pub fn draw_centered_text(text: &str, center: Vec2, font_size: f32, color: Color) {
    let font_scale = camera_font_scale(font_size);
    let text_center = get_text_center(text, None, font_scale.0, font_scale.1, 0.0);
    
    let position = center-text_center;

    draw_text_ex(text, position.x, position.y, TextParams{
        font_size: font_scale.0, 
        font_scale: font_scale.1, 
        font_scale_aspect: font_scale.2, 
        color, 
        ..Default::default()
    })
}