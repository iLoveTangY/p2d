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

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

pub struct AABB {
    density: f32,
    // min: Vec2,
    // max: Vec2,
}

pub enum Shape {
    Circle,
    AABB,
}
