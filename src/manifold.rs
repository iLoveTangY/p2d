use crate::{
    body::Body,
    shape::{Circle, ShapeType, AABB},
    vec2::Vec2,
};

pub struct Manifold<'a> {
    a: &'a mut Body,
    b: &'a mut Body,

    // A 的碰撞法线，单位向量
    normal: Vec2,
    // 物体碰撞时的侵入量
    penetration: f32,
    // 碰撞求解使用的恢复系数
    e: f32,
    // 所有的碰撞点
    contacts: Vec<Vec2>,
}

impl<'a> Manifold<'a> {
    pub fn new(a: &'a mut Body, b: &'a mut Body) -> Manifold<'a> {
        Manifold {
            a,
            b,
            normal: Vec2::new(0., 1.),
            penetration: 0.,
            e: 0.,
            contacts: vec![],
        }
    }
    /// 碰撞求解
    /// 解出碰撞点和碰撞法向量
    pub fn solve(&mut self) {
        // let a: &Body = self.a.borrow();
        // let b: &Body = self.b.borrow()
        match (self.a.shape_type(), self.b.shape_type()) {
            (ShapeType::Circle(ref circle_a), ShapeType::Circle(ref circle_b)) => {
                self.circle_2_circle(circle_a, circle_b);
            }
            (ShapeType::Circle(ref circle), ShapeType::AABB(ref aabb)) => {
                self.circle_2_aabb(circle, aabb);
            }
            (ShapeType::AABB(ref aabb), ShapeType::Circle(ref circle)) => {
                self.aabb_2_circle(aabb, circle);
            }
            (ShapeType::AABB(ref aabb_a), ShapeType::AABB(ref aabb_b)) => {
                self.aabb_2_aabb(aabb_a, aabb_b);
            }
        }
    }

    pub fn initialize(&mut self) {
        self.e = self.a.restitution().min(self.b.restitution());
    }

    pub fn apply_impulse(&mut self) {
        // 两个物体的质量都是无穷大
        if (self.a.restitution() + self.b.restitution()).abs() < 0.00001 {
            self.infinite_mass_correction();
            return;
        }
        // 相对速度在碰撞法线方向的分量
        let rv = (self.b.velocity() - self.a.velocity()).dot(self.normal);
        if rv > 0. {
            // 物体有分离的趋势
            return;
        }
        // 计算冲量
        let inv_mass_sum = self.a.inverse_mass() + self.b.inverse_mass();
        let mut j = -(1.0 + self.e) * rv;
        j /= inv_mass_sum;
        let impulse = self.normal * j;
        self.a.apply_impulse(impulse);
        self.b.apply_impulse(impulse);
    }

    fn infinite_mass_correction(&mut self) {
        self.a.set_velocity(Vec2::ZERO);
        self.b.set_velocity(Vec2::ZERO);
    }

    fn circle_2_circle(&mut self, circle_a: &Circle, circle_b: &Circle) {
        let n = self.b.position() - self.a.position();
        let r = circle_a.radius() - circle_b.radius();
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
            self.contacts.push(self.a.position());
        } else {
            self.penetration = r - dist;
            self.normal = n / dist;
            self.contacts.push(self.normal * circle_a.radius() + self.a.position());
        }
    }

    fn circle_2_aabb(&mut self, circle: &Circle, aabb: &AABB) {
        std::mem::swap(self.a, self.b);
        self.aabb_2_circle(aabb, circle);
        self.normal = -self.normal;
        std::mem::swap(self.a, self.b);
    }

    // fn aabb_2_circle_impl()

    fn aabb_2_circle(&mut self, aabb: &AABB, circle: &Circle) {
        // let mut difference = self.b.position() - self.a.position();
        let half_extend = (aabb.max() - aabb.min()) / 2.;

        let clamped = half_extend.clamp(-half_extend, half_extend);
        let closet = self.a.position() + clamped;
        let difference = closet - self.b.position();
        if difference.length_squared() < circle.radius() * circle.radius() {
            self.contacts.push(closet);
            self.normal = self.b.position() - closet;
            self.normal = self.normal.normalize();
            self.penetration = 0.;
        }
    }

    fn aabb_2_aabb(&mut self, _a: &AABB, _b: &AABB) {
        todo!("Not implementaion yet!")
    }

}
