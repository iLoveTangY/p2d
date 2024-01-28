use crate::vec2::Vec2;

pub trait Shape {
    fn mass_recip(&self) -> f32 {
        self.mass().recip()
    }

    fn mass(&self) -> f32;
}

#[derive(Clone, Copy)]
pub struct Circle {
    density: f32,
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle {
            density: 0.0,
            radius,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Shape for Circle {
    fn mass(&self) -> f32 {
        std::f32::consts::PI * (self.radius.powf(2.)) * self.density
    }
}

#[derive(Clone, Copy)]
pub struct AABB {
    density: f32,
    min: Vec2,
    max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> AABB {
        AABB {
            density: 3.0,
            min,
            max,
        }
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }

    pub fn min(&self) -> Vec2 {
        self.min
    }
}

impl Shape for AABB {
    fn mass(&self) -> f32 {
        let area = self.max - self.min;
        area.x * area.y * self.density
    }
}

#[derive(Clone, Copy)]
pub enum ShapeType {
    Circle(Circle),
    AABB(AABB),
}
