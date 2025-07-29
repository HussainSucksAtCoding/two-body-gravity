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

pub fn calculate_gravity(first: &Planet, other: &Planet) -> Vec2 {
    let direction = other.pos - first.pos;
    let distance_squared = direction.length_squared().max(0.01); // Avoid division by 0
    let force_magnitude = (other.mass * first.mass) / distance_squared;
    let force = direction.normalize() * force_magnitude;
    force
}

impl Planet {
    pub fn build_planet(mass: f32, pos: Vec2, velocity: Vec2, r: f32) -> Planet {
        let pl = Planet {
            mass,
            pos,
            velocity,
            r,
        };
        return pl;
    }
    pub fn push_planet(self, solarsystem: &mut Vec<Planet>) {
        solarsystem.push(self);
    }
}
