use std::ops::Mul;

pub struct Mat4f32 {
    /// Row major order
    pub values: [f32; 16],
}

impl Mat4f32 {
    /// Returns a matrix with all values set to 0.0
    pub fn zero() -> Mat4f32 {
        Mat4f32 { values: [0.0; 16] }
    }

    /// Returns the identity matrix
    #[rustfmt::skip]
    pub fn identity() -> Mat4f32 {
        Mat4f32 { values: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]}
    }

    pub fn get_column_major(&self) -> [[f32; 4]; 4] {
        [
            [
                self.values[0],
                self.values[4],
                self.values[8],
                self.values[12],
            ],
            [
                self.values[1],
                self.values[5],
                self.values[9],
                self.values[13],
            ],
            [
                self.values[2],
                self.values[6],
                self.values[10],
                self.values[14],
            ],
            [
                self.values[3],
                self.values[7],
                self.values[11],
                self.values[15],
            ],
        ]
    }
}

impl Mul for &Mat4f32 {
    type Output = Mat4f32;

    #[rustfmt::skip]
    fn mul(self, rhs: Self) -> Self::Output {
        let ax1 = self.values[0];
        let ax2 = self.values[1];
        let ax3 = self.values[2];
        let ax4 = self.values[3];
        let ay1 = self.values[4];
        let ay2 = self.values[5];
        let ay3 = self.values[6];
        let ay4 = self.values[7];
        let az1 = self.values[8];
        let az2 = self.values[9];
        let az3 = self.values[10];
        let az4 = self.values[11];
        let aw1 = self.values[12];
        let aw2 = self.values[13];
        let aw3 = self.values[14];
        let aw4 = self.values[15];

        let bx1 = rhs.values[0];
        let bx2 = rhs.values[1];
        let bx3 = rhs.values[2];
        let bx4 = rhs.values[3];
        let by1 = rhs.values[4];
        let by2 = rhs.values[5];
        let by3 = rhs.values[6];
        let by4 = rhs.values[7];
        let bz1 = rhs.values[8];
        let bz2 = rhs.values[9];
        let bz3 = rhs.values[10];
        let bz4 = rhs.values[11];
        let bw1 = rhs.values[12];
        let bw2 = rhs.values[13];
        let bw3 = rhs.values[14];
        let bw4 = rhs.values[15];

        Mat4f32 { values: [
            ax1 * bx1 + ax2 * by1 + ax3 * bz1 + ax4 * bw1,
            ax1 * bx2 + ax2 * by2 + ax3 * bz2 + ax4 * bw2,
            ax1 * bx3 + ax2 * by3 + ax3 * bz3 + ax4 * bw3,
            ax1 * bx4 + ax2 * by4 + ax3 * bz4 + ax4 * bw4,

            ay1 * bx1 + ay2 * by1 + ay3 * bz1 + ay4 * bw1,
            ay1 * bx2 + ay2 * by2 + ay3 * bz2 + ay4 * bw2,
            ay1 * bx3 + ay2 * by3 + ay3 * bz3 + ay4 * bw3,
            ay1 * bx4 + ay2 * by4 + ay3 * bz4 + ay4 * bw4,

            az1 * bx1 + az2 * by1 + az3 * bz1 + az4 * bw1,
            az1 * bx2 + az2 * by2 + az3 * bz2 + az4 * bw2,
            az1 * bx3 + az2 * by3 + az3 * bz3 + az4 * bw3,
            az1 * bx4 + az2 * by4 + az3 * bz4 + az4 * bw4,

            aw1 * bx1 + aw2 * by1 + aw3 * bz1 + aw4 * bw1,
            aw1 * bx2 + aw2 * by2 + aw3 * bz2 + aw4 * bw2,
            aw1 * bx3 + aw2 * by3 + aw3 * bz3 + aw4 * bw3,
            aw1 * bx4 + aw2 * by4 + aw3 * bz4 + aw4 * bw4,
        ]}
    }
}

impl Mul for Mat4f32 {
    type Output = Mat4f32;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Approximates equality, should work for the small
    // numbers in the unit tests
    fn check_mat_equal(a: &Mat4f32, b: &Mat4f32) -> bool {
        for i in 0..16 {
            if !check_f32_equal(a.values[i], b.values[i]) {
                return false;
            }
        }
        true
    }

    fn check_f32_equal(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.0001
    }

    #[test]
    #[rustfmt::skip]
    fn mat_multiplication() {
        let left = Mat4f32 {
            values: [
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 10.0, 11.0, 12.0,
                13.0, 14.0, 15.0, 16.0,
            ],
        };
        let right = Mat4f32 {
            values: [
                17.0, 18.0, 19.0, 20.0,
                21.0, 22.0, 23.0, 24.0,
                25.0, 26.0, 27.0, 28.0,
                29.0, 30.0, 31.0, 32.0,
            ],
        };
        let correct_result = Mat4f32 {
            values: [
                250.0, 260.0, 270.0, 280.0,
                618.0, 644.0, 670.0, 696.0,
                986.0, 1028.0, 1070.0, 1112.0,
                1354.0, 1412.0, 1470.0, 1528.0,
            ]
        };

        assert!(check_mat_equal(&(left * right), &correct_result));
    }
}
