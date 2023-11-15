/*
High Level Repository Notes:
-------------------------------------------------
TODO:
    1. Create a prime field that implements the field trait.
    2. Create THE field over which our curve is defined.
    3. Create the BLS12-381 G1 point type.
        - Express all functionality the point type needs.
-------------------------------------------------
 */

pub trait Field {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
}

// A field element can be thought of as a generalized version of "what a number is".
#[derive(Debug, PartialEq)]
pub struct FieldElement {
    element: u32,
    modulus: u32,
}

impl FieldElement {
    fn from(element: u32, modulus: u32) -> FieldElement {
        Self {
            element: element % modulus,
            modulus,
        }
    }
}

impl Field for FieldElement {
    fn add(&self, b: &Self) -> Self {
        assert_eq!(&self.modulus, &b.modulus);

        let res = (&self.element + b.element) % &self.modulus;
        Self {
            element: res,
            modulus: self.modulus,
        }
    }

    fn sub(&self, b: &Self) -> Self {
        assert_eq!(&self.modulus, &b.modulus);

        let a = i64::from(self.element);
        let b = i64::from(b.element.clone());
        let mut res = (a - b) % self.modulus as i64;

        if res < 0 {
            res += self.modulus as i64;
        }

        Self {
            element: (res as u32),
            modulus: self.modulus,
        }
    }

    fn mul(&self, b: &Self) -> Self {
        assert_eq!(&self.modulus, &b.modulus);

        let res = (&self.element * b.element) % &self.modulus;

        Self {
            element: res,
            modulus: self.modulus,
        }
    }

    fn div(&self, b: &Self) -> Self {
        assert_eq!(&self.modulus, &b.modulus);
        assert_ne!(b.element, 0 as u32);

        // Inverse of &self.element leveraging Fermat's little theorum:
        // TODO: *Why* does Fermat's little theorum work?
        let res = (self.element * (b.element.pow(self.modulus - 2))) % self.modulus;

        Self {
            element: res,
            modulus: self.modulus,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulo() {
        let modulus = 17;
        let res = FieldElement::from(26, modulus);
        assert_eq!(res, FieldElement::from(9, modulus))
    }

    // Field modulo arithmetic tests
    // ------------------------------------------------------
    #[test]
    fn add() {
        let modulus = 17;
        let a = FieldElement::from(9, modulus);
        let b = FieldElement::from(10, modulus);

        let res = a.add(&b);
        assert_eq!(res, FieldElement::from(2, modulus));
    }

    #[test]
    fn sub() {
        let modulus = 17;
        let a = FieldElement::from(9, modulus);
        let b = FieldElement::from(10, modulus);

        let res = a.sub(&b);
        assert_eq!(res, FieldElement::from(16, modulus));
    }

    #[test]
    fn mul() {
        let modulus = 17;
        let a = FieldElement::from(9, modulus);
        let b = FieldElement::from(10, modulus);

        let res = a.mul(&b);
        assert_eq!(res, FieldElement::from(5, modulus));
    }

    #[test]
    fn div() {
        let modulus = 7;
        let a = FieldElement::from(3, modulus);
        let b = FieldElement::from(2, modulus);

        let res = a.div(&b);
        assert_eq!(res, FieldElement::from(5, modulus));
    }
}
