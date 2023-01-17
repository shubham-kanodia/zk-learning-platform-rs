use std::ops::Neg;
use fraction::{Fraction, ToPrimitive};
use crate::math::Polynomial;


pub struct R1CS {
    a: Vec<Vec<i32>>,
    b: Vec<Vec<i32>>,
    c: Vec<Vec<i32>>,
}

pub struct QAP {
    a: Vec<Polynomial>,
    b: Vec<Polynomial>,
    c: Vec<Polynomial>,
}

impl R1CS {
    fn _single_matrix_to_qap(matrix: &Vec<Vec<i32>>, lagrange_intermittent_polynomials: &Vec<Polynomial>) -> Vec<Polynomial> {
        let mut qap: Vec<Polynomial> = Vec::with_capacity(matrix[0].len());

        for idx_col in 0..matrix[0].len() {
            let mut qap_poly = Polynomial{coeffs: vec![Fraction::new(0u8, 1u8)]};

            for idx_row in 0..matrix.len() {
                let mul_polynomial;
                if matrix[idx_row][idx_col] < 0 {
                    mul_polynomial = Polynomial{coeffs: vec![Fraction::new((-1 * matrix[idx_row][idx_col]).to_u64().unwrap(), 1u64).neg()]};
                }
                else {
                    mul_polynomial = Polynomial{coeffs: vec![Fraction::new(matrix[idx_row][idx_col].to_u64().unwrap(), 1u64)]};
                }

                qap_poly = &qap_poly + &(&lagrange_intermittent_polynomials[idx_row] * &mul_polynomial);
            }

            qap.push(qap_poly);
        }

        qap
    }


    fn to_qap(&self) -> QAP {
        let mut lagrange_intermittent_polynomials: Vec<Polynomial> = Vec::with_capacity(self.a.len());

        for idx in 0..self.a.len() {
            let mut lagrange_intermittent_polynomial = Polynomial { coeffs: vec![Fraction::new(1u8, 1u8)] };
            let mut div: i32 = 1;

            for x in 1..=self.a.len() {
                if x != (idx + 1) {
                    lagrange_intermittent_polynomial = &lagrange_intermittent_polynomial * &Polynomial {
                        coeffs: vec![
                            Fraction::new(x.to_u64().unwrap(), 1u64).neg(),
                            Fraction::new(1u8, 1u8),
                        ]
                    };

                    div *= idx.to_i32().unwrap() + 1i32 - x.to_i32().unwrap()
                }
            }

            if div < 0 {
                let divisor_polynomial = &Polynomial{coeffs: vec![Fraction::new(1u64, (-1 * div).to_u64().unwrap()).neg()]};
                lagrange_intermittent_polynomials.push(&lagrange_intermittent_polynomial * divisor_polynomial);
            } else {
                let divisor_polynomial = &Polynomial{coeffs: vec![Fraction::new(1u64, div.to_u64().unwrap())]};
                lagrange_intermittent_polynomials.push(&lagrange_intermittent_polynomial * divisor_polynomial);
            }
        }
        let qap_a = Self::_single_matrix_to_qap(&self.a, &lagrange_intermittent_polynomials);
        let qap_b = Self::_single_matrix_to_qap(&self.b, &lagrange_intermittent_polynomials);
        let qap_c = Self::_single_matrix_to_qap(&self.c, &lagrange_intermittent_polynomials);

        QAP {a: qap_a, b: qap_b, c: qap_c}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_qap() {
        let a = vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0],
            vec![-42, 0, 0, 0, 1, 0],
        ];

        let b = vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![-1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
        ];

        let c = vec![
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
        ];

        let r1cs: R1CS = R1CS { a, b, c };
        let test = r1cs.to_qap();

        let expected_poly = Polynomial{coeffs: vec![
            Fraction::new(42u8, 1u8),
            Fraction::new(77u8, 1u8).neg(),
            Fraction::new(42u8, 1u8),
            Fraction::new(7u8, 1u8).neg()
        ]};

        assert_eq!(test.a[0], expected_poly);
    }
}