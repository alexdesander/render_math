use crate::{mat::mat4f32::Mat4f32, vec::vec3::Vec3f32};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Rot3Df32 {
    // Scalar
    pub s: f32,
    // Bivector
    pub xy: f32,
    pub yz: f32,
    pub zx: f32,
}

impl Rot3Df32 {
    /// Returns the identity of a rotor (basically no rotation)
    pub fn identity() -> Self {
        Rot3Df32 {
            s: 1.0,
            xy: 0.0,
            yz: 0.0,
            zx: 0.0,
        }
    }

    /// Construct a new Rotor that rotates vectors by DOUBLE the angle and
    /// direction between vector a and b.
    /// Make sure a and b are normalized vectors and not 0.
    pub fn new(a: Vec3f32, b: Vec3f32) -> Self {
        if cfg!(debug_assertions) {
            debug_assert!(
                (0.9999..1.0001).contains(&a.magnitude())
                    && (0.9999..1.0001).contains(&b.magnitude()),
                "Construction of a rotor requires normalized vectors!"
            );
        }

        Rot3Df32 {
            s: b.x * a.x + b.y * a.y + b.z * a.z,
            xy: b.x * a.y - b.y * a.x,
            yz: b.y * a.z - b.z * a.y,
            zx: b.z * a.x - b.x * a.z,
        }
    }

    /// Construct a new Rotor that rotates vectors by the angle and
    /// direction between vector a and b.
    /// Make sure a and b are normalized vectors and not 0.
    pub fn new_exact(a: Vec3f32, mut b: Vec3f32) -> Self {
        if cfg!(debug_assertions) {
            debug_assert!(
                (0.9999..1.0001).contains(&a.magnitude())
                    && (0.9999..1.0001).contains(&b.magnitude()),
                "Construction of a rotor requires normalized vectors!"
            );
        }

        // Check if a ~= -b using the dot product
        if (-1.00001..-0.99999).contains(&a.dot(b)) {
            b = a.perpendicular();
        } else {
            let a_plus_b = a + b;
            b = a_plus_b / a_plus_b.magnitude();
        }

        Self::new(a, b)
    }

    /// Returns self but inverted (reverse rotation)
    pub fn inverted(&self) -> Self {
        let mut result = *self;
        result.invert();
        result
    }

    pub fn invert(&mut self) {
        self.xy = -self.xy;
        self.yz = -self.yz;
        self.zx = -self.zx;
    }

    /// Returns the resulting vector after rotating v (this is RvR^(-1))
    pub fn rotated_vec(&self, mut v: Vec3f32) -> Vec3f32 {
        self.rotate_vec(&mut v);
        v
    }

    /// Rotates v (this is RvR^(-1))
    pub fn rotate_vec(&self, v: &mut Vec3f32) {
        let tx = self.s * v.x + self.xy * v.y - self.zx * v.z;
        let ty = self.s * v.y - self.xy * v.x + self.yz * v.z;
        let tz = self.s * v.z - self.yz * v.y + self.zx * v.x;
        let txyz = self.xy * v.z + self.yz * v.x + self.zx * v.y;

        v.x = tx * self.s + ty * self.xy - tz * self.zx + txyz * self.yz;
        v.y = ty * self.s - tx * self.xy + tz * self.yz + txyz * self.zx;
        v.z = tz * self.s + tx * self.zx - ty * self.yz + txyz * self.xy;
    }

    /// Creates a new rotor which is the combination of self and r
    /// (First self then r)
    pub fn appended(&self, r: Rot3Df32) -> Self {
        let mut result = *self;
        result.append(r);
        result
    }

    /// Appends a rotor to this rotor
    /// The new rotation is the combination of both
    pub fn append(&mut self, r: Rot3Df32) {
        let s = self.s * r.s - self.xy * r.xy - self.yz * r.yz - self.zx * r.zx;
        let xy = self.s * r.xy + self.xy * r.s - self.yz * r.zx + self.zx * r.yz;
        let yz = self.s * r.yz + self.yz * r.s + self.xy * r.zx - self.zx * r.xy;
        let zx = self.s * r.zx + self.zx * r.s - self.xy * r.yz + self.yz * r.xy;

        self.s = s;
        self.xy = xy;
        self.yz = yz;
        self.zx = zx;
    }

    /// Normalizes the rotor, doing this is pretty important
    pub fn normalize(&mut self) {
        let mag_sqrd = self.s * self.s + self.xy * self.xy + self.yz * self.yz + self.zx * self.zx;
        let mag = mag_sqrd.sqrt();
        self.s /= mag;
        self.xy /= mag;
        self.yz /= mag;
        self.zx /= mag;
    }

    /// Creates a 4x4 rotation matrix (3x3 and padded to make it homogenous)
    // TODO: Optimize (zero calculations)
    #[rustfmt::skip]
    pub fn rotation_mat(&self) -> Mat4f32 {
        let new_x = self.rotated_vec(Vec3f32 { x: 1.0, y: 0.0, z: 0.0 });
        let new_y = self.rotated_vec(Vec3f32 { x: 0.0, y: 1.0, z: 0.0 });
        let new_z = self.rotated_vec(Vec3f32 { x: 0.0, y: 0.0, z: 1.0 });

        Mat4f32 {
            values: [
                new_x.x, new_y.x, new_z.x, 0.0,
                new_x.y, new_y.y, new_z.y, 0.0,
                new_x.z, new_y.z, new_z.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_rotation() {
        let a = Vec3f32::new(1.0, 0.0, 0.0);
        let b = Vec3f32::new(1.0, 0.0, 0.0);
        let rotor = Rot3Df32::new(a, b);

        let mut v = Vec3f32::new(1.0, 1.0, 1.0);
        rotor.rotate_vec(&mut v);
        assert!((0.9999..1.0001).contains(&v.x));
        assert!((0.9999..1.0001).contains(&v.y));
        assert!((0.9999..1.0001).contains(&v.z));
    }

    #[test]
    fn test_double_rotation_fix() {
        let a = Vec3f32::new(1.0, 0.0, 0.0);
        let b = Vec3f32::new(0.0, 1.0, 0.0);
        let rotor = Rot3Df32::new_exact(a, b);

        let mut v = Vec3f32::new(1.0, 0.0, 0.0);
        rotor.rotate_vec(&mut v);
        assert!((-0.0001..0.0001).contains(&v.x));
        assert!((0.9999..1.0001).contains(&v.y));
        assert!((-0.0001..0.0001).contains(&v.z));
    }

    #[test]
    fn test_double_rotation_fix_180() {
        let a = Vec3f32::new(1.0, 0.0, 0.0);
        let b = Vec3f32::new(-1.0, 0.0, 0.0);
        let rotor = Rot3Df32::new_exact(a, b);

        let mut v = Vec3f32::new(1.0, 0.0, 0.0);
        rotor.rotate_vec(&mut v);
        assert!((-1.0001..-0.9999).contains(&v.x));
        assert!((-0.0001..0.0001).contains(&v.y));
        assert!((-0.0001..0.0001).contains(&v.z));
    }

    #[test]
    fn test_append() {
        let a = Vec3f32::new(1.0, 0.0, 0.0);
        let mut b = Vec3f32::new(0.0, 1.0, 0.0);
        b.normalize();
        let mut rotor = Rot3Df32::new(a, b);
        rotor.append(rotor);
        let mut v = Vec3f32::new(1.0, 0.0, 0.0);
        rotor.rotate_vec(&mut v);
        assert!((0.99999..1.00001).contains(&v.x));
        assert!((-0.00001..0.00001).contains(&v.y));
        assert!((-0.00001..0.00001).contains(&v.z));
    }
}
