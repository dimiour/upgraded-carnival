mod object;
mod game;
use game::*;
mod window;
use window::*;

use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    game().await;
}

async fn game() {
    let mut last_tick = 0.0;
    
    let mut game = Game::new();

    let loadout = (SNIPER, SNIPER);
    let mut equipped = true;

    loop {
        print_stats(game.map.len());

        let center = game.center();
        let screen_mouse_position = screen_mouse_position();
        let mouse_position = real_mouse_position(center);
        
        set_window(center);
        
        //background
        for x in -4..5 {
            for y in -4..5 {
                draw_circle(0.1*x as f32,0.1*y as f32,0.003,WHITE);
            }
        }
        
        draw_rectangle_lines(-1.0, -1.0, 2.0, 2.0, 0.01, WHITE);

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
            
            if click_position.distance(mouse_position) > weapon.min_stretch {
                let capped_position = 
                    (mouse_position-click_position)
                    .clamp_length_max(weapon.max_stretch)
                    +click_position;
                
                draw_line(
                    click_position.x, 
                    click_position.y, 
                    capped_position.x, 
                    capped_position.y, 
                    0.02, 
                    MAGENTA
                );
                
                draw_circle(capped_position.x, capped_position.y, 0.01, MAGENTA);
            }
            draw_circle(click_position.x, click_position.y, 0.01, MAGENTA);
        }

        if is_mouse_button_released(MouseButton::Left) {
            game.interaction.as_mut().unwrap().2 = true;
        }

        //check tick
        let time = get_time();
        while time >= last_tick+TICK_LENGTH {
            game.tick();

            game.interaction(weapon);

            last_tick += TICK_LENGTH;
        }

        //new frame
        next_frame().await
    }
}

//CONFIG
