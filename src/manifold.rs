use std::{cell::RefCell, rc::Rc};

use crate::{
    body::Body,
    shape::{Circle, ShapeType, AABB},
    vec2::Vec2,
};

pub(crate) struct Manifold {
    a: Rc<RefCell<Body>>,
    b: Rc<RefCell<Body>>,

    // A 的碰撞法线，单位向量
    normal: Vec2,
    // 物体碰撞时的侵入量
    penetration: f32,
    // 碰撞求解使用的恢复系数
    e: f32,
    // 所有的碰撞点
    contacts: Vec<Vec2>,
    // 碰撞计算时要使用的静摩擦力
    sf: f32,
    // 碰撞计算时要使用的动摩擦力
    df: f32,
}

impl Manifold {
    fn new(a: Rc<RefCell<Body>>, b: Rc<RefCell<Body>>) -> Manifold {
        Manifold {
            a,
            b,
            normal: Vec2::new(0., 1.),
            penetration: 0.,
            e: 0.,
            contacts: vec![],
            sf: 0.,
            df: 0.,
        }
    }
    /// 碰撞求解
    /// 解出碰撞点和碰撞法向量
    pub(crate) fn solve(a: Rc<RefCell<Body>>, b: Rc<RefCell<Body>>) -> Manifold {
        // let a = self.a.borrow();
        let a_type = a.borrow().shape();
        let b_type = b.borrow().shape();
        // let b = self.b.borrow();
        let mut m = Manifold::new(a, b);
        match (a_type, b_type) {
            (ShapeType::Circle(ref circle_a), ShapeType::Circle(ref circle_b)) => {
                m.circle_2_circle(circle_a, circle_b);
            }
            (ShapeType::Circle(ref circle), ShapeType::AABB(ref aabb)) => {
                m.circle_2_aabb(circle, aabb);
            }
            (ShapeType::AABB(ref aabb), ShapeType::Circle(ref circle)) => {
                m.aabb_2_circle(aabb, circle);
            }
            (ShapeType::AABB(ref aabb_a), ShapeType::AABB(ref aabb_b)) => {
                m.aabb_2_aabb(aabb_a, aabb_b);
            }
        }
        m
    }

    pub(crate) fn get_contacts(&self) -> &Vec<Vec2> {
        &self.contacts
    }

    pub(crate) fn initialize(&mut self) {
        let a = self.a.borrow();
        let b = self.b.borrow();
        self.e = a.restitution().min(b.restitution());
        self.sf = (a.static_fraction * a.static_fraction + b.static_fraction * b.static_fraction).sqrt();
        self.df = (a.dynamic_fraction * a.dynamic_fraction + b.dynamic_fraction * b.dynamic_fraction).sqrt();
    }

    pub(crate) fn apply_impulse(&mut self) {
        let mut a = self.a.borrow_mut();
        let mut b = self.b.borrow_mut();
        // 两个物体的质量都是无穷大
        if (a.restitution() + b.restitution()).abs() < 0.00001 {
            // let mut a = self.a.borrow_mut();
            // let mut b = self.b.borrow_mut();
            a.set_velocity(Vec2::ZERO);
            b.set_velocity(Vec2::ZERO);   
            return;
        }
        // 相对速度在碰撞法线方向的分量
        let rv = (b.velocity() - a.velocity()).dot(self.normal);
        if rv > 0. {
            // 物体有分离的趋势
            return;
        }
        // 计算冲量
        let inv_mass_sum = a.inverse_mass() + b.inverse_mass();
        let mut j = -(1.0 + self.e) * rv;
        j /= inv_mass_sum;
        let impulse = self.normal * j;
        // let mut a = self.a.borrow_mut();
        // let mut b = self.b.borrow_mut();
        a.apply_impulse(-impulse);
        b.apply_impulse(impulse);

        // 应用摩擦力
        let rv_2 = b.velocity() - a.velocity();
        let mut t = rv_2 - self.normal * (rv_2.dot(self.normal));
        // 如果 t 为 0，不需要计算摩擦力
        if (t.length_squared() - 0.).abs() <= 0.0001 {
            return;
        }
        t = t.normalize();
        // 计算切线方向冲量幅值
        let mut jt = -rv_2.dot(t);
        jt /= inv_mass_sum;
        if jt.abs() < 0.00001 {
            return;
        }
        // 库仑定律
        let tangent_impulse;
        if jt.abs() < j * self.sf {
            tangent_impulse = t * jt;
        } else {
            tangent_impulse = t * (-j * self.df);
        }
        a.apply_impulse(-tangent_impulse);
        b.apply_impulse(tangent_impulse);
    }

    fn circle_2_circle(&mut self, circle_a: &Circle, circle_b: &Circle) {
        let a = self.a.borrow();
        let b = self.b.borrow();
        let n = b.position() - a.position();
        let r = circle_a.radius() + circle_b.radius();
        let dist_sqr = n.length_squared();
        if dist_sqr >= r * r {
            // 无碰撞发生
            return;
        }
        let dist = dist_sqr.sqrt();
        if (dist - 0.).abs() < 0.00001 {
            // 两个圆处于同一位置
            self.penetration = circle_a.radius();
            self.normal = Vec2::new(1., 0.);
            self.contacts.push(a.position());
        } else {
            self.penetration = r - dist;
            self.normal = n / dist;
            self.contacts.push(self.normal * circle_a.radius() + a.position());
        }
    }

    fn circle_2_aabb(&mut self, circle: &Circle, aabb: &AABB) {
        std::mem::swap(&mut self.a, &mut self.b);
        self.aabb_2_circle(aabb, circle);
        self.normal = -self.normal;
        std::mem::swap(&mut self.a, &mut self.b);
    }

    // fn aabb_2_circle_impl()

    fn aabb_2_circle(&mut self, aabb: &AABB, circle: &Circle) {
        let a = self.a.borrow();
        let b = self.b.borrow();
        let mut difference = b.position() - a.position();
        let half_extend = (aabb.max() - aabb.min()) / 2.;

        let clamped = difference.clamp(-half_extend, half_extend);
        let closet = a.position() + clamped;
        difference = closet - b.position();
        if difference.length_squared() < circle.radius() * circle.radius() {
            self.contacts.push(closet);
            self.normal = b.position() - closet;
            self.normal = self.normal.normalize();
            self.penetration = 0.;
        }
    }

    fn aabb_2_aabb(&mut self, first: &AABB, second: &AABB) {
        let a = self.a.borrow();
        let b = self.b.borrow();

        let n = b.position() - a.position();
        let mut a_extend = (first.max().x - first.min().x) / 2.;
        let mut b_extend = (second.max().x - second.min().x) / 2.;
        let x_overlap = a_extend + b_extend - n.x.abs();
        if x_overlap > 0. {
            a_extend = (first.max().y - first.min().y) / 2.;
            b_extend = (second.max().y - second.min().y) / 2.;
            let y_overlap = a_extend + b_extend - n.y.abs();
            // x y 方向都得发生重叠才会发生碰撞
            if y_overlap > 0. {
                // 重叠小的方向是碰撞发生的方向
                if x_overlap < y_overlap {
                    if n.x < 0. {
                        self.normal = Vec2::new(-1., 0.);
                    } else {
                        self.normal = Vec2::new(1., 0.);
                    }
                    self.penetration = x_overlap;
                } else {
                    if n.y < 0. {
                        self.normal = Vec2::new(0., -1.);
                    } else {
                        self.normal = Vec2::new(0., 1.);
                    }
                    self.penetration = y_overlap;
                }
                self.contacts.push(Vec2::new(0., 0.));
            }
        }
    }

}
