use macroquad::{math::Vec2, prelude::*};

#[derive(Debug)]
pub struct Planet {
    pub mass: f32,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub r: f32,
}

// pub struct SolarSystem {
//     planets: Vec<Planet>,
// }

pub fn calculate_force(first: &Planet, other: &Planet) -> Vec2 {
    let direction = other.pos - first.pos;
    let distance_squared = direction.length_squared().max(0.01); // Avoid division by 0
    let force_magnitude = (other.mass * first.mass) / distance_squared;
    let force = direction.normalize() * force_magnitude;
    force
}

pub fn push_planet(planet: Planet, solarsystem: &mut Vec<Planet>) {
    solarsystem.push(planet);
}

impl Planet {
    pub fn build_planet(mass: f32, pos: Vec2, velocity: Vec2, r: f32) -> Self {
        let pl = Self {
            mass,
            pos,
            velocity,
            r,
        };

        draw_circle(pl.pos.x, pl.pos.y, 20.0, WHITE);
        return pl;
    }
    pub fn draw_planet(&mut self, other: &Planet) {
        let direction = other.pos - self.pos;
        let distance_squared = direction.length_squared().max(0.01); // Avoid division by 0
        let force_magnitude = (other.mass * self.mass) / distance_squared;
        let force = direction.normalize() * force_magnitude;
        let acceleration = force / self.mass;

        self.velocity += acceleration * get_frame_time();
        self.pos += self.velocity;
        draw_circle(self.pos.x, self.pos.y, self.r, WHITE);
    }
}
