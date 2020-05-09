use std::fmt;
use std::ops;

#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone, PartialEq)]
pub struct M4(pub [[f32; 4]; 4]);

impl M4 {
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn dimensions(&self) -> usize {
        4
    }

    #[allow(dead_code)]
    pub fn transpose(&mut self) -> &mut Self {
        for y in 0..self.dimensions() - 1 {
            for x in (y + 1..self.dimensions()).rev() {
                let tmp = self[y][x];
                self[y][x] = self[x][y];
                self[x][y] = tmp;
            }
        }

        self
    }

    #[allow(dead_code)]
    pub fn transposed(&self) -> Self {
        let mut m = self.clone();
        m.transpose();
        m
    }

    pub fn row_echelon_form(&mut self) -> &mut Self {
        for d in 0..self.dimensions() {
            for y in d + 1..self.dimensions() {
                if self[d][d] == 0.0 {
                    self[d][d] = 1.0e-18;
                }

                let scaler = self[y][d] / self[d][d];

                for x in 0..self.dimensions() {
                    self[y][x] = self[y][x] - scaler * self[d][x];
                }
            }
        }

        self
    }

    /// Returns the determinant of the matrix (read-only)
    ///
    /// This is done by finding the row echelon form of the matrix and then the determinant is the
    /// product of its diagonal.
    #[allow(dead_code)]
    pub fn determinant(&self) -> f32 {
        let mut m = self.clone();
        m.row_echelon_form();

        let mut det: f32 = m[0][0];
        for d in 1..self.dimensions() {
            det *= m[d][d]
        }
        det
    }
}

impl ops::Index<usize> for M4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for M4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::Mul for M4 {
    type Output = M4;

    fn mul(self, other: M4) -> Self::Output {
        let mut m = M4::zero();

        for y in 0..self.dimensions() {
            for x in 0..self.dimensions() {
                let mut sum = 0.0;
                for k in 0..self.dimensions() {
                    sum += self[y][k] * other[k][x];
                }
                m[y][x] = sum;
            }
        }

        m
    }
}

impl ops::Neg for M4 {
    type Output = M4;

    fn neg(self) -> Self::Output {
        let mut m = M4::zero();

        for y in 0..self.dimensions() {
            for x in 0..self.dimensions() {
                m[y][x] = -self[y][x];
            }
        }

        m
    }
}

impl fmt::Debug for M4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..4 {
            let line = format!(
                "\t[{}, {}, {}, {}]\n",
                self[y][0], self[y][1], self[y][2], self[y][3],
            );

            s.push_str(&line);
        }

        writeln!(f, "M4 (\n{})", s)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn zero_creates_zero_matrix() {
        let expects = super::M4([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(expects, super::M4::zero(), "Did not create a zero matrix");
    }

    #[test]
    fn identity_creates_identity_matrix() {
        let expects = super::M4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(expects, super::M4::identity(), "Did not create an identity matrix");
    }

    #[test]
    fn dimensions_return_4() {
        let expects = 4;
        let matrix = super::M4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(expects, matrix.dimensions(), "Did not return dimensions of 4");
    }

    #[test]
    fn transpose_transposes_matrix() {
        let expects = &mut super::M4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let matrix = &mut super::M4([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);

        assert_eq!(expects, matrix.transpose(), "Did not transpose matrix");
    }
}
