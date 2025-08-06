pub mod planets;
use macroquad::prelude::*;
use planets::*;

#[macroquad::main("idk")]
async fn main() {
    let mut solarsystem: Vec<Planet> = vec![];
    let (center_x, center_y) = (screen_width() / 2.0, screen_height() / 2.0);

    let sun = Planet::build_planet(
        100000.0,
        Vec2 {
            x: center_x,
            y: center_y,
        },
        Vec2 { x: 0.0, y: 0.0 },
        20.0,
    );

    loop {
        clear_background(BLACK);

        if is_mouse_button_released(MouseButton::Left) {
            let mouse_pos: Vec2 = mouse_position().into();

            let planet = Planet::build_planet(100.0, mouse_pos, Vec2 { x: -3.0, y: 1.0 }, 10.0);

            solarsystem.push(planet);
        }

        sun.draw();

        for planets in solarsystem.iter_mut() {
            planets.draw_planet(&sun);
        }

        next_frame().await
    }
}
