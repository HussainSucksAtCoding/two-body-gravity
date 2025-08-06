use macroquad::{math::Vec2, prelude::*};

#[derive(Debug)]
pub struct Planet {
    pub mass: f32,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub r: f32,
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
        return pl;
    }
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, YELLOW);
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
