/*

// Elliptic Curve Points
// ------------------------------------------------------------------------------------------------------------------------------------------
// BLS12-381 curve info:
// * Base field: q = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
//       - The base field is the finite field over which our curve is defined.
// * Scalar field: r =
//   52435875175126190479447740508185965837690552500527637822603658699938581184513
// * valuation(q - 1, 2) = 1
// * valuation(r - 1, 2) = 32
// * G1 curve equation: y^2 = x^3 + 4
// * G2 curve equation: y^2 = x^3 + Fq2(4, 4)

// TODO: implement real modulus
const BLS_MODULUS: usize = 65537;

// TODO: Affine representation of G1 elliptic curve point
struct BlsPoint {
    x: usize,
    y: usize,
}

impl BlsPoint {
    fn new(x: &usize, y: &usize) -> Self {
        unimplemented!()
    }
    fn add(p: &BlsPoint, q: &BlsPoint) -> BlsPoint {
        unimplemented!()
    }
    fn multiply(p: &BlsPoint, scalar: &usize) {
        unimplemented!()
    }
}
*/
