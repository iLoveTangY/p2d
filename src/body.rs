use crate::{shape::{Circle, Shape, ShapeType, AABB}, vec2::Vec2};

pub struct Body {
    shape: ShapeType,
    position: Vec2,
    velocity: Vec2,
    restitution: f32,
    force: Vec2,
    mass: f32,
    inverse_mass: f32,
}

impl Body {
    #[inline]
    pub fn new_circle(shape: Circle, position: Vec2, restitution: f32) -> Body {
        let mass = shape.mass();
        let inverse_mass = shape.mass_recip();
        Body {
            shape: ShapeType::Circle(shape),
            position,
            restitution,
            velocity: Vec2::ZERO,
            force: Vec2::ZERO,
            mass,
            inverse_mass,
        }
    }

    #[inline]
    pub fn new_aabb(shape: AABB, position: Vec2, restitution: f32) -> Body {
        let mass = shape.mass();
        let inverse_mass = shape.mass_recip();
        Body {
            shape: ShapeType::AABB(shape),
            position,
            restitution,
            velocity: Vec2::ZERO,
            force: Vec2::ZERO,
            mass,
            inverse_mass,
        }
    }

    #[inline(always)]
    pub fn restitution(&self) -> f32 {
        self.restitution
    }

    #[inline(always)]
    pub fn mass(&self) -> f32 {
        self.mass
    }

    #[inline(always)]
    pub fn position(&self) -> Vec2 {
        self.position
    }

    #[inline(always)]
    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    #[inline(always)]
    pub fn inverse_mass(&self) -> f32 {
        self.inverse_mass
    }

    #[inline(always)]
    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    #[inline(always)]
    pub fn set_velocity(&mut self, v: Vec2) {
        self.velocity = v;
    }

    #[inline(always)]
    pub fn force(&self) -> Vec2 {
        self.force
    }

    pub fn shape(&self) -> ShapeType {
        self.shape
    }

    #[inline(always)]
    pub fn apply_force(&mut self, f: Vec2) {
        self.force += f;
    }

    #[inline(always)]
    pub fn clear_force(&mut self) {
        self.force = Vec2::ZERO;
    }

    #[inline(always)]
    pub fn apply_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse * self.inverse_mass;
    }

    #[inline(always)]
    pub fn make_static(&mut self) {
        self.mass = 0.;
        self.inverse_mass = 0.;
    }
}