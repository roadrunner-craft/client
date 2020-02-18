use std::fmt;
use std::ops;

#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone)]
pub struct m4(pub [[f32; 4]; 4]);

impl m4 {
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
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

    pub fn get(&self, y: usize, x: usize) -> f32 {
        self.0[y][x]
    }

    pub fn set(&mut self, y: usize, x: usize, value: f32) {
        self.0[y][x] = value
    }

    #[allow(dead_code)]
    pub fn transpose(&mut self) -> &mut Self {
        for y in 0..self.dimensions() - 1 {
            for x in (y + 1..self.dimensions()).rev() {
                let tmp = self.0[y][x];
                self.0[y][x] = self.0[x][y];
                self.0[x][y] = tmp;
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
                if self.get(d, d) == 0.0 {
                    self.set(d, d, 1.0e-18);
                }

                let scaler = self.get(y, d) / self.get(d, d);

                for x in 0..self.dimensions() {
                    self.set(y, x, self.get(y, x) - scaler * self.get(d, x));
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

        let mut det: f32 = m.get(0, 0);
        for d in 1..self.dimensions() {
            det *= m.get(d, d)
        }
        det
    }
}

impl ops::Mul for m4 {
    type Output = m4;

    fn mul(self, other: m4) -> Self::Output {
        let mut m = m4::zero();

        for y in 0..self.dimensions() {
            for x in 0..self.dimensions() {
                let mut sum = 0.0;
                for k in 0..self.dimensions() {
                    sum += self.get(y, k) * other.get(k, x)
                }
                m.set(y, x, sum);
            }
        }

        m
    }
}

impl fmt::Debug for m4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..4 {
            let line = format!(
                "\t[{}, {}, {}, {}]\n",
                self.get(y, 0),
                self.get(y, 1),
                self.get(y, 2),
                self.get(y, 3)
            );

            s.push_str(&line);
        }

        writeln!(f, "m4 (\n{})", s)
    }
}
