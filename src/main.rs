pub mod planets;
use macroquad::prelude::*;
use planets::*;

#[macroquad::main("idk")]
async fn main() {
    //let mut solarsystem: Vec<Planet> = vec![];
    let (center_x, center_y) = (screen_width() / 2.0, screen_height() / 2.0);

    let mut planet = Planet::build_planet(
        10.0,
        Vec2 { x: 0.0, y: 0.0 },
        Vec2 { x: -0.5, y: 1.0 },
        10.0,
    );

    let mut sun = Planet::build_planet(
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
        let (center_x, center_y) = (screen_width() / 2.0, screen_height() / 2.0);

        sun.draw_planet(&planet);
        planet.draw_planet(&sun);

        next_frame().await
    }
}

// draw_circle(center_x, center_y, sun.r, YELLOW);
// draw_circle(planet.pos.x, planet.pos.y, planet.r, WHITE);

// println!("{}, {} // {}", planet.pos.x, planet.pos.y, force);
