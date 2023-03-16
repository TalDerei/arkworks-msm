use ark_ff::{PrimeField, Field};
// We'll use the BLS12-381 G1 curve for this example.
use ark_bn254::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};
use ark_std::{Zero, UniformRand};
use ark_ec::{ProjectiveCurve, AffineCurve};

#[test]
fn test_msm() {
    
}