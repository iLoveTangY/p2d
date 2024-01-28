use std::ops::{self, AddAssign, Mul, SubAssign};

/// 2d vector
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);

    /// creates a new `Vec2`
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    /// creates a `Vec2` with all elements set to `v`
    #[inline(always)]
    pub const fn splat(v: f32) -> Vec2 {
        Vec2 { x: v, y: v }
    }

    /// Returns `self` normalized to length 1.0
    ///
    /// # Panics
    ///
    /// will panic when self is length zero or very close to zero
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        let length_recip = self.length_recip();
        assert!(length_recip.is_finite());
        self.mul(length_recip)
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`
    #[must_use]
    #[inline]
    pub fn try_normalize(self) -> Option<Self> {
        let length_recip = self.length_recip();
        if length_recip.is_finite() {
            Some(self.mul(length_recip))
        } else {
            None
        }
    }

    /// computes the dot product
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// computes the length of `self`
    #[inline]
    pub fn length(self) -> f32 {
        (self.dot(self)).sqrt()
    }

    /// computes the 1.0 / length
    #[inline]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    /// computes the squared length of `self`
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max`.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.x <= max.x && min.y <= max.y, "expected min <= max");
        self.max(min).min(max)
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y)]`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y)]`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::Add<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Vec2) -> Self::Output {
        rhs + self
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl ops::Sub<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        -(rhs - self)
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_from_into_should_work() {
        let vec2: Vec2 = [1., 2.].into();
        let expected = Vec2::new(1., 2.);
        assert_eq!(vec2, expected);

        let vec2 = Vec2::from((1., 2.));
        assert_eq!(vec2, expected);
    }

    /// Test cases for:
    /// * Vec2 + f32
    /// * f32 + Vec2
    /// * Vec2 + Vec2
    #[test]
    fn vec2_add_should_work() {
        let vec2 = Vec2::new(1., 2.);
        let number: f32 = 2.;
        let rhs_vec2 = Vec2::new(2., 3.);

        let ret = vec2 + number;
        let expected = Vec2::new(3., 4.);
        assert_eq!(ret, expected);

        let ret = number + vec2;
        assert_eq!(ret, expected);

        let ret = vec2 + rhs_vec2;
        let expected = Vec2::new(3., 5.);
        assert_eq!(ret, expected);
    }

    /// Test cases for:
    /// * Vec2 - f32
    /// * f32 - Vec2
    /// * Vec2 - Vec2
    #[test]
    fn vec2_sub_should_work() {
        let vec2 = Vec2::new(1., 2.);
        let number: f32 = 3.;
        let rhs_vec2 = Vec2::new(3., 4.);

        let ret = vec2 - number;
        let expected = Vec2::new(-2., -1.);
        assert_eq!(ret, expected);

        let ret = number - vec2;
        let expected = Vec2::new(2., 1.);
        assert_eq!(ret, expected);

        let ret = vec2 - rhs_vec2;
        let expected = Vec2::new(-2., -2.);
        assert_eq!(ret, expected);
    }

    /// Test cases for:
    /// * Vec2 * number
    /// * number * Vec2
    /// * Vec2 * Vec2
    #[test]
    fn vec2_mul_should_work() {
        let vec2 = Vec2::new(5., 6.);
        let number: f32 = 4.;
        let rhs_vec2 = Vec2::new(1., 2.);

        let ret = vec2 * number;
        let expected = Vec2::new(5. * 4., 6. * 4.);
        assert_eq!(ret, expected);

        let ret = number * vec2;
        assert_eq!(ret, expected);

        let ret = vec2 * rhs_vec2;
        let expected = Vec2::new(5. * 1., 6. * 2.);
        assert_eq!(ret, expected);
    }

    /// Test cases for:
    /// * Vec2 / number
    /// * Vec2 / Vec2
    #[test]
    fn vec2_div_should_work() {
        let vec2 = Vec2::new(5., 8.);
        let number: f32 = 4.;
        let rhs_vec2 = Vec2::new(2., 3.);

        let ret = vec2 / number;
        let expected = Vec2::new(5. / 4., 8. / 4.);
        assert_eq!(ret, expected);

        let ret = vec2 / rhs_vec2;
        let expected = Vec2::new(5. / 2., 8. / 3.);
        assert_eq!(ret, expected);
    }

    /// Test cases for:
    /// * Vec2.normalize()
    /// * Vec2.try_normalize()
    #[test]
    fn vec2_normalize_should_work() {
        let vec2 = Vec2::new(1., 0.);
        let ret = vec2.normalize();
        assert_eq!(ret, vec2);

        let vec2 = Vec2::new(1., 0.);
        let ret = vec2.try_normalize();
        assert_eq!(ret, Some(vec2));

        let vec2 = Vec2::splat(0.0);
        let ret = vec2.try_normalize();
        assert_eq!(ret, None);
    }

    /// Test cases for:
    /// * Vec2.length()
    /// * Vec2.length_recip()
    /// * Vec2.length_squared()
    #[test]
    fn vec2_length_should_work() {
        let vec2 = Vec2::new(1., 2.);
        let ret = vec2.length();
        let expected = ((1. * 1. + 2. * 2.) as f32).sqrt();
        assert_eq!(ret, expected);

        let vec2 = Vec2::splat(2.);
        let ret = vec2.length_squared();
        let expected: f32 = 2. * 2. + 2. * 2.;
        assert_eq!(ret, expected);

        let vec2 = Vec2::splat(0.);
        let ret = vec2.length_recip();
        assert!(ret.is_infinite());

        let vec2 = Vec2::new(3., 4.);
        let ret = vec2.length_recip();
        assert_eq!(ret, 1. / vec2.length());
    }

    /// Test cases for:
    /// * Vec2.dot(Vec2)
    #[test]
    fn vec2_dot_should_work() {
        let vec2 = Vec2::new(1., 2.);
        let vec2_rhs = Vec2::new(3., 4.);
        let ret = vec2.dot(vec2_rhs);
        let expected: f32 = 1. * 3. + 2. * 4.;
        assert_eq!(ret, expected);

        let ret = vec2_rhs.dot(vec2);
        assert_eq!(ret, expected);
    }

    /// Test cases for:
    /// * Vec2.clamp(Vec2, Vec2)
    #[test]
    fn vec2_clamp_should_work() {
        let vec2 = Vec2::new(4., 5.);

        let min = Vec2::new(1., 2.);
        let max = Vec2::new(7., 8.);
        let ret = vec2.clamp(min, max);
        let expected = Vec2::new(4., 5.);
        assert_eq!(ret, expected);

        let min = Vec2::new(5., 4.);
        let max = Vec2::new(7., 8.);
        let ret = vec2.clamp(min, max);
        let expected = Vec2::new(5., 5.);
        assert_eq!(ret, expected);

        let min = Vec2::new(1., 2.);
        let max = Vec2::new(3., 5.);
        let ret = vec2.clamp(min, max);
        let expected = Vec2::new(3., 5.);
        assert_eq!(ret, expected);
    }
}
