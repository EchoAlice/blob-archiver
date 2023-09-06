use ethereum_consensus::{
    deneb::mainnet::{BYTES_PER_BLOB, BYTES_PER_FIELD_ELEMENT},
    deneb::Blob,
    primitives::U256,
    ssz::ByteVector,
};

// TODO: Find bls modulus from blst
// rust bindings see what they exported.  How do i define this number.

// pub const BYTES_PER_BLOB: usize = 131072; // 32 x 4096
pub const BLS_MODULUS: U256 =
    52435875175126190479447740508185965837690552500527637822603658699938581184513;
pub const BYTES_PER_COMMITMENT: usize = 48;
pub const BYTES_PER_PROOF: usize = 48;
pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;

type BLSFieldElement = U256; // I'm not sure if this custon u256 type has enough functionality
type KZGCommitment = ByteVector<BYTES_PER_COMMITMENT>;
type Polynomial = Vec<BLSFieldElement>; // Should this polynomial type be an array?

/// Given a blob and a KZG proof, verify that the blob data corresponds to the provided commitment.
pub fn verify_blob_kzg_proof(
    blob: Blob<BYTES_PER_BLOB>,
    commitment_bytes: ByteVector<BYTES_PER_COMMITMENT>,
    proof_bytes: ByteVector<BYTES_PER_PROOF>,
) -> bool {
    assert_eq!(blob.len(), BYTES_PER_BLOB);
    assert_eq!(commitment_bytes.len(), BYTES_PER_COMMITMENT);
    assert_eq!(proof_bytes.len(), BYTES_PER_PROOF);

    let commitment = bytes_to_kzg_commitment(commitment_bytes);

    let polynomial = blob_to_polynomial(blob);
    true
    // unimplemented!()
}

/// Converts untrusted bytes to a trusted and validated KZGCommitment.
fn bytes_to_kzg_commitment(b: ByteVector<BYTES_PER_COMMITMENT>) -> KZGCommitment {
    validate_kzg_g1(&b);
    b
}

/// Performs BLS validation required by the types 'KZGProof' and 'KZGCommitment'
fn validate_kzg_g1(b: &ByteVector<BYTES_PER_COMMITMENT>) {
    /*
    let g1_point_at_infinite = ;
    if b = g1_point_at_infinite {
        return
    }

    */
    // unimplemented!()
}

// Converts blob chunks into bls field elements.  What're field scalars?
fn blob_to_polynomial(blob: Blob<BYTES_PER_BLOB>) -> Polynomial {
    let mut polynomial = Polynomial::default();

    for i in 0..FIELD_ELEMENTS_PER_BLOB {
        let start = i * BYTES_PER_FIELD_ELEMENT;
        let end = (i + 1) * BYTES_PER_FIELD_ELEMENT;

        let sub_slice = &blob[start..end];
        let mut array: [u8; BYTES_PER_FIELD_ELEMENT] = Default::default();
        array.copy_from_slice(sub_slice);

        let value = bytes_to_bls_field(&array);
        polynomial.push(value);
    }

    polynomial
}

/// Converts untrusted bytes to a trusted and validated BLS scalar field element.
/// This function doesn't accept inputs greater than the BLS modulus.
fn bytes_to_bls_field(b: &[u8; BYTES_PER_FIELD_ELEMENT]) -> BLSFieldElement {
    // "KZG bytes" are big endian. Does this mean the blob I've received is written in big endian?
    let b_le = swap_endianness(b);
    let field_element = U256::from_bytes_le(b_le);

    assert!(field_element < BLS_MODULUS);

    field_element
}

/// Non-crypto helper function
/// -------------------------------------------------------------------------
fn swap_endianness(input: &[u8]) -> [u8; BYTES_PER_FIELD_ELEMENT] {
    let mut output = [0; BYTES_PER_FIELD_ELEMENT];

    for (i, &byte) in input.iter().rev().enumerate() {
        output[i] = byte;
    }

    output
}
