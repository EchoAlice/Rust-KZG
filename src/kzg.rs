pub use c_kzg::{Blob, KzgSettings};

type KzgCommitment = [u8; 48];
pub const BYTES_PER_BLOB: usize = 131072;

// TODO:
//  - How *is* our blob be represented?
//  - How *should* our blob be represented?
pub fn blob_to_kzg_commitment(blob: &Blob, kzg: &KzgSettings) -> KzgCommitment {
    println!("Blob: {:?}", blob);
    println!("Trusted Setup: {:?}", kzg);
    blob_to_polynomial(&blob);

    polynomial_to_kzg_commitment();

    unimplemented!()
}

fn blob_to_polynomial(blob: &Blob) {
    println!("Implement blob_to_polynomial. Blob: {:?}", blob);
}
fn polynomial_to_kzg_commitment() {
    println!("Implement polynomial_to_kzg_commitment.");
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
