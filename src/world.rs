use std::cell::RefCell;

use crate::{body::Body, manifold::Manifold, vec2::Vec2};

pub struct World {
    dt: i32,  // 每次循环的时间间隔
    iterations: i32,  // 每次循环迭代次数
    bodies: Vec<Body>,  // 场景中的所有物体
    gravity_scale: f32,  // 重力放大倍数
    gravity: Vec2,  // 重力大小
}

impl World {
    /// 创建一个新的物理世界
    /// * `dt`: 物理世界的更新频率
    /// * `iterations`: 每次 step 的循环次数
    /// * `gravity_scale`: 重力放大倍数
    pub fn new(dt: i32, iterations: i32, gravity_scale: f32) -> World {
        World {
            dt,
            iterations,
            bodies: vec![],
            gravity_scale: gravity_scale,
            gravity: Vec2::new(0., 10.0 * gravity_scale)
        }
    }

    pub fn get_bodies(&self) -> &Vec<Body> {
        &self.bodies
    }

    pub fn step(&mut self) {
        let mut contacts = vec![];
        for (i, a) in self.bodies.iter().enumerate() {
            for b in self.bodies[i + 1..].iter() {
                if a.inverse_mass() == 0. && b.inverse_mass() == 0. {
                    continue;
                }
                let mut m = Manifold::new(RefCell::new(a), RefCell::new(b));
                m.solve();
                if m.get_contacts().len() > 0 {
                    contacts.push(m);
                }
            }
        }
    }
}