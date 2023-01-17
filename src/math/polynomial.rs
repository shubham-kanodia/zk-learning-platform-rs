use std::iter::zip;
use fraction::{Fraction};
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Polynomial {
    pub coeffs: Vec<Fraction>
}

impl Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, other: &Polynomial) -> Polynomial {
        let mut new_coeffs: Vec<Fraction> = Vec::new();
        let (max_poly, min_poly) = if self.degree() > other.degree() {(self, other)} else {(other, self)};

        for (coeff1, coeff2) in zip(&self.coeffs, &other.coeffs) {
            new_coeffs.push(coeff1 + coeff2);
        }

        for idx in min_poly.coeffs.len()..max_poly.coeffs.len() {
            new_coeffs.push(max_poly.coeffs[idx]);
        }

        return Polynomial {coeffs: new_coeffs};
    }
}

impl Sub<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, other: &Polynomial) -> Polynomial {
        let mut new_coeffs: Vec<Fraction> = Vec::new();
        let (max_poly, min_poly) = if self.degree() > other.degree() {(self, other)} else {(other, self)};

        for (coeff1, coeff2) in zip(&self.coeffs, &other.coeffs) {
            new_coeffs.push(coeff1 - coeff2);
        }

        if self == min_poly {
            for idx in min_poly.coeffs.len()..max_poly.coeffs.len() {
                new_coeffs.push(max_poly.coeffs[idx].neg());
            }
        }
        else {
            for idx in min_poly.coeffs.len()..max_poly.coeffs.len() {
                new_coeffs.push(max_poly.coeffs[idx]);
            }
        }

        return Polynomial {coeffs: new_coeffs};
    }
}

impl Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, other: &Polynomial) -> Polynomial {
        let capacity = self.degree() + other.degree() + 1;
        let mut result_coeffs: Vec<Fraction> = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            result_coeffs.push(Fraction::new(0u8, 1u8));
        }

        for idx_a in 0..self.coeffs.len() {
            for idx_b in 0..other.coeffs.len() {
                result_coeffs[idx_a + idx_b] += self.coeffs[idx_a] * other.coeffs[idx_b];
            }
        }

        return Polynomial{coeffs: result_coeffs};
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        return &self.coeffs == &other.coeffs;
    }
}

impl Polynomial {
    fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_polynomials() {
        let coeffs_1 = vec![
            Fraction::new(1u8, 2u8),
            Fraction::new(2u8, 3u8),
            Fraction::new(1u8, 2u8)
        ];

        let coeffs_2 = vec![
            Fraction::new(1u8, 2u8),
            Fraction::new(1u8, 3u8),
            Fraction::new(1u8, 2u8)
        ];

        let result_coeffs = vec![
            Fraction::new(1u8, 1u8),
            Fraction::new(1u8, 1u8),
            Fraction::new(1u8, 1u8)
        ];

        let poly_1 = Polynomial{coeffs: coeffs_1};
        let poly_2 = Polynomial{coeffs: coeffs_2};
        let result_poly = Polynomial{coeffs: result_coeffs};

        assert_eq!((&poly_1 + &poly_2), result_poly);
    }

    #[test]
    fn can_sub_polynomials() {
        let coeffs_1 = vec![
            Fraction::new(1u8, 2u8),
            Fraction::new(2u8, 3u8)
        ];

        let coeffs_2 = vec![
            Fraction::new(1u8, 4u8),
            Fraction::new(1u8, 3u8),
            Fraction::new(1u8, 2u8)
        ];

        let result_coeffs = vec![
            Fraction::new(1u8, 4u8),
            Fraction::new(1u8, 3u8),
            Fraction::new(1u8, 2u8).neg()
        ];

        let poly_1 = Polynomial{coeffs: coeffs_1};
        let poly_2 = Polynomial{coeffs: coeffs_2};
        let result_poly = Polynomial{coeffs: result_coeffs};

        assert_eq!((&poly_1 - &poly_2), result_poly);
    }

    #[test]
    fn can_mul_polynomials() {
        let coeffs_1 = vec![
            Fraction::new(1u8, 2u8),
            Fraction::new(2u8, 3u8)
        ];

        let coeffs_2 = vec![
            Fraction::new(1u8, 4u8),
            Fraction::new(1u8, 3u8),
            Fraction::new(1u8, 2u8)
        ];

        let result_coeffs = vec![
            Fraction::new(1u8, 8u8),
            Fraction::new(1u8, 3u8),
            Fraction::new(17u8, 36u8),
            Fraction::new(1u8, 3u8)
        ];

        let poly_1 = Polynomial{coeffs: coeffs_1};
        let poly_2 = Polynomial{coeffs: coeffs_2};
        let result_poly = Polynomial{coeffs: result_coeffs};

        assert_eq!((&poly_1 * &poly_2), result_poly);
    }
}
