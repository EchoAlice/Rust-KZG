pub trait Field {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct PrimeFieldElement {
    element: u32,
    modulus: u32,
}

impl PrimeFieldElement {
    fn build(element: u32, modulus: u32) -> Result<PrimeFieldElement, &'static str> {
        if !is_prime(modulus) {
            return Err("Modulus isn't prime");
        }

        Ok(Self {
            element: element % modulus,
            modulus,
        })
    }
}

fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    }

    // TODO: Is this safe?  Are overflows possible here?
    for i in 2..=(x as f64).sqrt() as u32 {
        if x % i == 0 {
            return false;
        }
    }
    true
}

impl Field for PrimeFieldElement {
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
        let res = PrimeFieldElement::build(26, modulus).expect("Modulus should be prime in test");
        assert_eq!(
            res,
            PrimeFieldElement::build(9, modulus).expect("Modulus should be prime in test")
        )
    }

    // Field modulo arithmetic tests
    // ------------------------------------------------------
    #[test]
    fn add() {
        let modulus = 17;
        let a = PrimeFieldElement::build(9, modulus).expect("Modulus should be prime in test");
        let b = PrimeFieldElement::build(10, modulus).expect("Modulus should be prime in test");

        let res = a.add(&b);
        assert_eq!(
            res,
            PrimeFieldElement::build(2, modulus).expect("Modulus should be prime in test")
        );
    }

    #[test]
    fn sub() {
        let modulus = 17;
        let a = PrimeFieldElement::build(9, modulus).expect("Modulus should be prime in test");
        let b = PrimeFieldElement::build(10, modulus).expect("Modulus should be prime in test");

        let res = a.sub(&b);
        assert_eq!(
            res,
            PrimeFieldElement::build(16, modulus).expect("Modulus should be prime in test")
        );
    }

    #[test]
    fn mul() {
        let modulus = 17;
        let a = PrimeFieldElement::build(9, modulus).expect("Modulus should be prime in test");
        let b = PrimeFieldElement::build(10, modulus).expect("Modulus should be prime in test");

        let res = a.mul(&b);
        assert_eq!(
            res,
            PrimeFieldElement::build(5, modulus).expect("Modulus should be prime in test")
        );
    }

    #[test]
    fn div() {
        let modulus = 7;
        let a = PrimeFieldElement::build(3, modulus).expect("Modulus should be prime in test");
        let b = PrimeFieldElement::build(2, modulus).expect("Modulus should be prime in test");

        let res = a.div(&b);
        assert_eq!(
            res,
            PrimeFieldElement::build(5, modulus).expect("Modulus should be prime in test")
        );
    }
}
