use std::{cell::RefCell, rc::Rc};

use crate::{body::Body, manifold::Manifold, vec2::Vec2};

pub struct World {
    dt: f32,                        // 每次循环的时间间隔
    iterations: i32,                // 每次循环迭代次数
    bodies: Vec<Rc<RefCell<Body>>>, // 场景中的所有物体
    gravity_scale: f32,             // 重力放大倍数
    gravity: Vec2,                  // 重力大小
}

impl World {
    /// 创建一个新的物理世界
    /// * `dt`: 物理世界的更新频率
    /// * `iterations`: 每次 step 的循环次数
    /// * `gravity_scale`: 重力放大倍数
    pub fn new(dt: f32, iterations: i32, gravity_scale: f32) -> World {
        World {
            dt,
            iterations,
            bodies: vec![],
            gravity_scale: gravity_scale,
            gravity: Vec2::new(0., 10.0 * gravity_scale),
        }
    }

    /// 获取 world 中所有刚体
    pub fn get_bodies(&self) -> &Vec<Rc<RefCell<Body>>> {
        &self.bodies
    }

    /// world 中添加一个刚体
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(Rc::new(RefCell::new(body)));
    }

    /// world 推进一步，并更新每个物体的位置
    pub fn step(&mut self) {
        // 碰撞检测
        // Broad Phase + Narrow Phase
        let mut contacts = vec![];
        for (i, a) in self.bodies.iter().enumerate() {
            for b in self.bodies[i + 1..].iter() {
                if a.borrow().inverse_mass() == 0. && b.borrow().inverse_mass() == 0. {
                    // 两个物体的质量都是无穷大，不会发生位置的变化
                    continue;
                }
                let mut m = Manifold::solve(a.clone(), b.clone());
                if m.get_contacts().len() > 0 {
                    contacts.push(m);
                }
            }
        }

        for body in &self.bodies {
            self.integrate_forces(body.clone());
        }

        for contact in &mut contacts {
            contact.initialize();
        }

        for _ in 0..self.iterations {
            for contact in &mut contacts {
                contact.apply_impulse();
            }
        }

        for body in &self.bodies {
            self.integrate_velocity(body.clone());
        }

        for body in &self.bodies {
            body.borrow_mut().clear_force();
        }
    }

    // 把计算出来的力应用到物体上
    fn integrate_forces(&self, body: Rc<RefCell<Body>>) {
        let internal_body = body.borrow();
        if internal_body.inverse_mass() == 0. {
            return;
        }
        // v1 = v0 + F / m * dt
        // TODO: 这里不使用 dt / 2 是否可以？
        let new_velocity = internal_body.velocity()
            + (self.gravity + internal_body.force() * internal_body.inverse_mass()) * (self.dt as f32 / 2.);
        body.borrow_mut().set_velocity(new_velocity);
    }

    // 根据速度计算新的位置
    fn integrate_velocity(&self, body: Rc<RefCell<Body>>) {
        {
            let internal_body = body.borrow();
            if internal_body.inverse_mass() == 0. {
                return;
            }
            let new_pos = internal_body.position() + internal_body.velocity() * self.dt as f32;
            body.borrow_mut().set_position(new_pos);
        }
        // 为了稳定？
        self.integrate_forces(body);
    }
}
