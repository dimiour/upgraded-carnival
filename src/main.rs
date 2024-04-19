mod object; mod game; mod window; mod weapons; mod transitions;
use game::*;
use window::*;
use weapons::*;
use transitions::*;

use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    game().await;
}

async fn game() {
    let mut last_tick = 0.0;
    
    let mut game = Game::new();

    let loadout = (ROCKET, SNIPER);
    let mut equipped = true;

    let mut transitions: Vec<Transition> = vec![];

    loop {
        clear_background(DARKBROWN);
        //print_stats(game.map.len());

        let center = game.center();
        let screen_mouse_position = screen_mouse_position();
        let mouse_position = real_mouse_position(center);
        
        set_window(center);
        
        //background
        for x in -9..10 {
            for y in -9..10 {
                draw_circle(0.1*x as f32,0.1*y as f32,0.003,WHITE);
            }
        }
        let size = object::BORDER_SIZE;//+(get_time() as f32*0.1).sin()*0.4;
        draw_rectangle_lines(-size, -size, size*2.0, size*2.0, 0.01, WHITE);

        //draw map
        game.draw_map();

        //
        let weapon = if equipped {loadout.0} else {loadout.1};
        
        //draw control
        if is_mouse_button_pressed(MouseButton::Left) {
            game.interaction = Some((get_time(), screen_mouse_position, false));
        }

        if is_mouse_button_down(MouseButton::Left) {
            let click_position = game.interaction.unwrap().1+ui_position(center);
            let time_since_click = get_time()-game.interaction.unwrap().0;
            let fade_in = half_arc(time_since_click/weapon.fire_rate);
            let fade_color = Color::new(1.0-fade_in, fade_in, 0.0, 1.0);
            
            //if click_position.distance(mouse_position) > weapon.min_stretch {
                let capped_position = 
                    (mouse_position-click_position)
                    .clamp_length_max(weapon.max_stretch*fade_in)
                    +click_position;
                
                draw_line(
                    click_position.x, 
                    click_position.y, 
                    capped_position.x, 
                    capped_position.y, 
                    0.02, 
                    fade_color
                );
                
                draw_circle(capped_position.x, capped_position.y, 0.01, fade_color);
            //}
            draw_circle(click_position.x, click_position.y, 0.01, fade_color);
        }

        if is_mouse_button_released(MouseButton::Left) {
            game.interaction.as_mut().unwrap().2 = true;
        }

        //check tick
        let time = get_time();
        while time >= last_tick+TICK_LENGTH {
            game.tick();

            let (new_equipped, new_transition) = game.interaction(loadout, equipped);
            equipped = new_equipped;
            if let Some(transition) = new_transition {
                transitions.insert(0, transition);
            }
            
            last_tick += TICK_LENGTH;
        }

        for index in 0..transitions.len() {
            if transitions[index].draw(center) {
                transitions.remove(index);
            }
        }

        //new frame
        next_frame().await
    }
}

//CONFIG
