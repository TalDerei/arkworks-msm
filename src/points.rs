#![allow(unused_imports)]
use rand::{RngCore, rngs::StdRng, SeedableRng};
use ark_bn254::{G1Projective, Fr, Fq};
use ark_ff::UniformRand;
use crate::field::*;

// Generate projective points and convert to affine points in base field
pub fn generate_projective_points(
    count: usize,
    mut rng: Box<dyn RngCore>,
) -> Vec<AffinePoint> {
    (0..count) 
        // The closure passed to the .map() function is a lambda function. Arkworks generates a 'G1Projective'
        // element, which is then converted 'AffinePoint'. These elements are collected into vector of AffinePoint. 
        .map(|_| ProjectivePoint::from_projective(G1Projective::rand(&mut rng)).affine_form())
        .collect()
}

// Generate scalars from scalar field
pub fn generate_scalars(
    count: usize,
    mut rng: Box<dyn RngCore>,
) -> Vec<Scalar> {
    (0..count) 
        .map(|_| Scalar::from_scalar(&Fr::rand(&mut rng)))
        .collect()
}

pub fn get_rng(seed: Option<u64>) -> Box<dyn RngCore> {
    let rng: Box<dyn RngCore> = match seed {
        Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
        None => Box::new(rand::thread_rng()),
    };
    rng
}