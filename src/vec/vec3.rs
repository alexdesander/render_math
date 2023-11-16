use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f32 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    pub fn normalized(&mut self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(&self, v: Self) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: Self) -> Vec3f32 {
        Vec3f32 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    /// Generates an arbitrary unit (normalized) vector that is perpendicular to self.
    /// Make sure self is not 0
    pub fn perpendicular(&self) -> Self {
        debug_assert!(self.magnitude() != 0.0);

        // 1
        let mut result = Self::new(0.0, 0.0, 0.0);

        // 2
        let m = if self.x != 0.0 {
            0
        } else if self.y != 0.0 {
            1
        } else {
            2
        };
        let n = (m + 1) % 3;

        // 3
        let x_m = match m {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(),
        };
        let x_n = match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(),
        };
        match n {
            0 => result.x = x_m,
            1 => result.y = x_m,
            2 => result.z = x_m,
            _ => {}
        }
        match m {
            0 => result.x = -x_n,
            1 => result.y = -x_n,
            2 => result.z = -x_n,
            _ => {}
        }

        // 4
        result.normalize();
        result
    }
}

impl Add for Vec3f32 {
    type Output = Vec3f32;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3f32::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Div<f32> for Vec3f32 {
    type Output = Vec3f32;

    fn div(mut self, rhs: f32) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

impl Mul<f32> for Vec3f32 {
    type Output = Vec3f32;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perpendicular() {
        let vec = Vec3f32::new(0.0, 1.0, 0.0);
        let perpendicular = vec.perpendicular();
        assert!((-0.0001..0.0001).contains(&vec.dot(perpendicular)));
        assert!((0.9999..1.00001).contains(&perpendicular.magnitude()));

        let vec = Vec3f32::new(1.0, 1.0, 0.0);
        let perpendicular = vec.perpendicular();
        assert!((-0.0001..0.0001).contains(&vec.dot(perpendicular)));
        assert!((0.9999..1.00001).contains(&perpendicular.magnitude()));

        let vec = Vec3f32::new(1.0, 1.0, 1.0);
        let perpendicular = vec.perpendicular();
        assert!((-0.0001..0.0001).contains(&vec.dot(perpendicular)));
        assert!((0.9999..1.00001).contains(&perpendicular.magnitude()));

        let vec = Vec3f32::new(0.0, 0.0, -1.0);
        let perpendicular = vec.perpendicular();
        assert!((-0.0001..0.0001).contains(&vec.dot(perpendicular)));
        assert!((0.9999..1.00001).contains(&perpendicular.magnitude()));

        let vec = Vec3f32::new(0.1, -1.0, 9999.0);
        let perpendicular = vec.perpendicular();
        assert!((-0.0001..0.0001).contains(&vec.dot(perpendicular)));
        assert!((0.9999..1.00001).contains(&perpendicular.magnitude()));
    }
}
