use std::error::Error;

/*
High Level Repository Notes:
-------------------------------------------------
TODO:
    1. Create a prime field that implements this field trait.
    2. Create THE field over which our curve is defined.
    3. Create the BLS12-381 G1 point type.
        - Express all functionality the point type needs.
-------------------------------------------------
 */

// A generalized version of "what a number is"
pub trait Field {
    fn add(&self, other: &PrimeFieldElement) -> PrimeFieldElement;
    fn sub(&self, other: &PrimeFieldElement) -> PrimeFieldElement;
    fn mul(&self, other: &PrimeFieldElement) -> PrimeFieldElement;
    fn div(&self, other: &PrimeFieldElement) -> PrimeFieldElement;
}

// Create field elements via a finite_field.from()
// Note: This is a Finite Field Element!
//
// Should i call this a "prime field element"?
#[derive(Debug, PartialEq, Clone)]
pub struct PrimeFieldElement {
    int: u32,
    field: PrimeField,
}

// Big Question:
// What properties does our prime field need to have?

// Note: A prime field is a type of finite field.
// We need a prime field for our elliptic curve equation.
//
// TODO: Our modulus MUST be a prime number.
#[derive(Debug, PartialEq, Clone)]
struct PrimeField {
    modulus: u32,
    generator: u32,
    // TODO: Add a "zero?"
}

impl PrimeField {
    fn new(modulus: u32, generator: u32) -> Self {
        Self { modulus, generator }
    }

    fn from(&self, int: u32) -> Result<PrimeFieldElement, Box<dyn Error>> {
        // Verifies the *integer* is within the field.
        // Type conversion needed for division
        let out = f64::from(int) / f64::from(self.generator);
        assert_eq!(out, out.round());

        Ok(PrimeFieldElement {
            int: int % &self.modulus,
            field: self.clone(),
        })
    }
}

impl Field for PrimeFieldElement {
    fn add(&self, b: &Self) -> Self {
        assert_eq!(&self.field, &b.field);

        let res = (&self.int + b.int) % &self.field.modulus;
        Self {
            int: res,
            field: self.field.clone(),
        }
    }

    fn sub(&self, b: &Self) -> Self {
        assert_eq!(&self.field, &b.field);

        // Type conversion needed for subtraction
        let a = i64::from(self.int.clone());
        let b = i64::from(b.int.clone());
        let mut res = (a - b) % self.field.modulus as i64;

        if res < 0 {
            res += self.field.modulus as i64;
        }

        Self {
            int: (res as u32),
            field: self.field.clone(),
        }
    }

    fn mul(&self, b: &Self) -> Self {
        assert_eq!(&self.field, &b.field);
        let res = (&self.int * b.int) % &self.field.modulus;

        Self {
            int: res,
            field: self.field.clone(),
        }
    }

    fn div(&self, b: &Self) -> Self {
        assert_ne!(b.int, 0 as u32);

        // Inverse of &self.int leveraging Fermat's little theorum:
        let res = (self.int * (b.int.pow(self.field.modulus - 2))) % self.field.modulus;

        Self {
            int: res,
            field: self.field.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Prime field modulo arithmetic tests
    // ------------------------------------------------------
    #[test]
    fn add_elements() {
        let fq = PrimeField::new(17, 1);
        let a = fq.from(9).expect("hard coded values in test are valid");
        let b = fq.from(10).expect("hard coded values in test are valid");

        let res = a.add(&b);

        assert_eq!(
            res,
            fq.from(2).expect("hard coded values in test are valid")
        );
    }

    #[test]
    fn sub_elements() {
        let fq = PrimeField::new(17, 1);
        let a = fq.from(9).expect("hard coded values in test are valid");
        let b = fq.from(10).expect("hard coded values in test are valid");
        let res = a.sub(&b);

        assert_eq!(
            res,
            fq.from(16).expect("hard coded values in test are valid")
        );
    }

    #[test]
    fn mul_elements() {
        let fq = PrimeField::new(17, 1);
        let a = fq.from(9).expect("hard coded values in test are valid");
        let b = fq.from(10).expect("hard coded values in test are valid");

        assert_eq!(
            a.mul(&b),
            fq.from(5).expect("hard coded values in test are valid")
        );
    }

    #[test]
    fn div_elements() {
        let fq = PrimeField::new(7, 1);
        let a = fq.from(3).expect("hard coded values in test are valid");
        let b = fq.from(2).expect("hard coded values in test are valid");

        assert_eq!(
            a.div(&b),
            fq.from(5).expect("hard coded values in test are valid")
        );
    }

    // Elliptic Curve Point Tests
    // ------------------------------------------------------
}
