#![allow(unused_imports)]
use ark_bn254::{G1Projective, G1Affine, Fr, Fq};
use ark_ff::{BigInteger256, PrimeField};
use ark_std::{UniformRand};
use ark_ec::{ProjectiveCurve, AffineCurve, msm::VariableBaseMSM};
use std::{mem::transmute_copy};

pub const LIMBS: usize = 4;

/***************************** 'Field Struct *************************************/

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Field<const LIMBS: usize> {
    pub s: [u64; LIMBS],
}

impl<const LIMBS: usize> Default for Field<LIMBS> {
    fn default() -> Self {
        Field::zero()
    }
}

impl<const LIMBS: usize> Field<LIMBS> {
    pub fn zero() -> Self {
        Field {
            s: [0u64; LIMBS],
        }
    }

    pub fn one() -> Self {
        let mut s = [0u64; LIMBS];
        s[0] = 1;
        Field { s }
    }

    fn to_bytes_le(&self) -> Vec<u8> {
        self.s
            .iter()
            .map(|s| s.to_le_bytes().to_vec())
            .flatten()
            .collect::<Vec<_>>()
    }
}

/***************************** 'BaseField Struct *************************************/
pub type BaseField = Field<LIMBS>;

impl BaseField {
    pub fn base_limbs(&self) -> [u64; LIMBS] {
        self.s
    }

    pub fn from_limbs(value: &[u64]) -> BaseField {
        BaseField {
            s: Self::get_fixed_limbs(value),
        }
    }

    // Convert reference of 'BaseField' object to 'BigInteger256' object
    // using method from std::mem module to perform a byte-wise copy
    pub fn to_ark(&self) -> BigInteger256 {
        unsafe { transmute_copy::<BaseField, BigInteger256>(self) } 
    }

    pub fn from_ark_base(ark: BigInteger256) -> BaseField {
        unsafe { transmute_copy::<BigInteger256, BaseField>(&ark) }
    }

    fn get_fixed_limbs<const LIMBS: usize>(val: &[u64]) -> [u64; LIMBS] {
        match val.len() {
            n if n < LIMBS => {
                let mut padded: [u64; LIMBS] = [0; LIMBS];
                padded[..val.len()].copy_from_slice(&val);
                padded
            }
            n if n == LIMBS => val.try_into().unwrap(),
            _ => panic!("slice has to much elements"),
        }
    }
}

/***************************** 'AffinePoint Struct *************************************/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AffinePoint {
    pub x: BaseField,
    pub y: BaseField,
}

impl Default for AffinePoint {
    fn default() -> Self {
        AffinePoint {
            x: BaseField::zero(),
            y: BaseField::zero(),
        }
    }
}

impl AffinePoint {
    pub fn from_limbs(x: &[u64], y: &[u64]) -> Self {
        AffinePoint {
            x: BaseField {
                s: Field::get_fixed_limbs(x),
            },
            y: BaseField {
                s: Field::get_fixed_limbs(y),
            },
        }
    }

    pub fn to_ark_repr(&self) -> G1Affine {
        G1Affine::new(
            Fq::from_repr(self.x.to_ark()).unwrap(),
            Fq::from_repr(self.y.to_ark()).unwrap(),
            false,
        )
    }
}

/***************************** 'ProjectivePoint' Struct *************************************/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ProjectivePoint {
    pub x: BaseField,
    pub y: BaseField,
    pub z: BaseField,
}

impl Default for ProjectivePoint {
    fn default() -> Self {
        ProjectivePoint {
            x: BaseField::zero(),
            y: BaseField::zero(),
            z: BaseField::zero(),
        }
    }
}

impl ProjectivePoint {
    pub fn affine_form(&self) -> AffinePoint {
        AffinePoint {
            x: self.x,
            y: self.y,
        }
    }

    // Convert 'G1Projective' to 'Point' 
    pub fn from_projective(ark: G1Projective) -> ProjectivePoint {
        use ark_ff::Field;
        let z_inv = ark.z.inverse().unwrap();
        let z_invsq = z_inv * z_inv;
        let z_invq3 = z_invsq * z_inv;
        ProjectivePoint {
            x: BaseField::from_ark_base((ark.x * z_invsq).into_repr()),
            y: BaseField::from_ark_base((ark.y * z_invq3).into_repr()),
            z: BaseField::one(),
        }
    }

    pub fn to_ark(&self) -> G1Projective {
        //TODO: generic conversion
        self.to_ark_affine().into_projective()
    }

    pub fn to_ark_affine(&self) -> G1Affine {
        //TODO: generic conversion
        use ark_ff::Field;
        use std::ops::Mul;
        let proj_x_field = Fq::from_le_bytes_mod_order(&self.x.to_bytes_le());
        let proj_y_field = Fq::from_le_bytes_mod_order(&self.y.to_bytes_le());
        let proj_z_field = Fq::from_le_bytes_mod_order(&self.z.to_bytes_le());
        let inverse_z = proj_z_field.inverse().unwrap();
        let aff_x = proj_x_field.mul(inverse_z);
        let aff_y = proj_y_field.mul(inverse_z);
        G1Affine::new(aff_x, aff_y, false)
    }
}

/***************************** 'ScalarField' Struct *************************************/

pub type ScalarField = Field<LIMBS>;

impl ScalarField {
    pub fn scalar_limbs(&self) -> [u64; LIMBS] {
        self.s
    }

    pub fn from_ark_scalar(ark: BigInteger256) -> ScalarField {
        unsafe { transmute_copy::<BigInteger256, ScalarField>(&ark) }
    }
}

/***************************** 'Scalar' Struct *************************************/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Scalar {
    pub s: ScalarField,
}

impl Scalar {
    pub fn one() -> Self {
        Scalar {
            s: ScalarField::one(),
        }
    }

    pub fn zero() -> Self {
        Scalar {
            s: ScalarField::zero(),
        }
    }
    
    pub fn from_scalar(v: &Fr) -> Scalar {
        Scalar {
            s: ScalarField::from_ark_scalar(v.into_repr()),
        }
    }

    pub fn to_ark_mod_p(&self) -> Fr {
        Fr::new(self.s.to_ark())
    }
}