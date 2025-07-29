pub mod planets;
use macroquad::prelude::*;
use planets::*;

#[macroquad::main("idk")]
async fn main() {
    //let mut solarsystem: Vec<Planet> = vec![];
    let (center_x, center_y) = (screen_width() / 2.0, screen_height() / 2.0);

    let sun = Planet::build_planet(
        10000.0,
        Vec2 {
            x: center_x,
            y: center_y,
        },
        Vec2 { x: 0.0, y: 0.0 },
        20.0,
    );
    let mut planet: Planet = Planet::build_planet(
        1.0,
        Vec2 { x: 200.0, y: 200.0 },
        Vec2 { x: -0.5, y: 0.5 }, // INITIAL VELOCITYYYY
        10.0,
    );

    loop {
        let (center_x, center_y) = (screen_width() / 2.0, screen_height() / 2.0);

        let force = calculate_gravity(&planet, &sun);
        let acceleration = force / planet.mass;

        clear_background(BLACK);

        planet.velocity += acceleration * get_frame_time();
        planet.pos += planet.velocity;

        draw_circle(center_x, center_y, sun.r, YELLOW);
        draw_circle(planet.pos.x, planet.pos.y, planet.r, WHITE);

        println!("{}, {} // {}", planet.pos.x, planet.pos.y, force);

        next_frame().await
    }
}
