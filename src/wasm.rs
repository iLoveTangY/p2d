use std::cell::RefCell;
use std::rc::Rc;

use crate::shape::{ShapeType, AABB};
use crate::vec2::Vec2;
use crate::{body::Body, shape::Circle, world::World};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct P2DWorld {
    world: World,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl P2DWorld {
    pub fn new(dt: f32, iterations: i32, gravity_scale: f32) -> P2DWorld {
        P2DWorld {
            world: World::new(dt, iterations, gravity_scale),
        }
    }

    pub fn add_body(&mut self, p2d_body: P2DBody) {
        self.world.add_rc_body(p2d_body.body);
    }

    pub fn get_bodies(&self) -> Vec<P2DBody> {
        let bodies = self.world.get_bodies();
        let mut result: Vec<P2DBody> = Vec::with_capacity(bodies.len());
        for body in bodies {
            let shape_type = match body.borrow().shape() {
                ShapeType::AABB(_) => P2DShapeType::AABB,
                ShapeType::Circle(_) => P2DShapeType::Circle,
            };
            result.push(P2DBody { body: body.clone(), shape_type })
        }

        result
    }

    pub fn step(&mut self) {
        self.world.step();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone)]
pub enum P2DShapeType {
    Circle,
    AABB,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone)]
pub struct P2DCircle {
    pub radius: f32
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy)]
pub struct P2DAABB {
    pub min: Vec2,
    pub max: Vec2,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct P2DBody {
    pub(crate) body: Rc<RefCell<Body>>,
    shape_type: P2DShapeType,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl P2DBody {
    pub fn new_circle(radius: f32, position: Vec2, restitution: f32) -> P2DBody {
        P2DBody {
            body: Rc::new(RefCell::new(Body::new_circle(
                Circle::new(radius),
                position, 
                restitution,
            ))),
            shape_type: P2DShapeType::Circle,
        }
    }

    pub fn new_aabb(min: Vec2, max: Vec2, position: Vec2, restitution: f32) -> P2DBody {
        P2DBody {
            body: Rc::new(RefCell::new(Body::new_aabb(
                AABB::new(min, max),
                position,
                restitution,
            ))),
            shape_type: P2DShapeType::AABB,
        }
    }

    pub fn make_static(&mut self) {
        self.body.borrow_mut().make_static();
    }

    pub fn get_shape_type(&self) -> P2DShapeType {
        self.shape_type
    }

    pub fn get_position(&self) -> Vec2 {
        self.body.borrow().position()
    }

    pub fn get_circle(&self) -> P2DCircle {
        match self.body.borrow().shape() {
            ShapeType::Circle(circle) => P2DCircle { radius: circle.radius() },
            _ => panic!("Invalid call for get circle"),
        }
    }

    pub fn get_aabb(&self) -> P2DAABB {
        match self.body.borrow().shape() {
            ShapeType::AABB(aabb) => P2DAABB {
                min: aabb.min(),
                max: aabb.max(),
            },
            _ => panic!("Invalid call for get aabb"),
        }
    }

    pub fn is_static(&self) -> bool {
        self.body.borrow().is_static()
    }
}

