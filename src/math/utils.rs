use fraction::Fraction;
use crate::math::Polynomial;

fn add_polynomials(polynomials: &Vec<Polynomial>) -> Polynomial {
    let mut result = Polynomial{coeffs: vec![Fraction::new(0u8, 1u8)]};

    for polynomial in polynomials {
        result = &result + polynomial;
    }

    result
}

fn multiply_polynomials(polynomials: &Vec<Polynomial>) -> Polynomial {
    let mut result = Polynomial{coeffs: vec![Fraction::new(1u8, 1u8)]};

    for polynomial in polynomials {
        result = &result * polynomial;
    }

    result
}