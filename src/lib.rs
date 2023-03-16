pub mod points;
pub mod field;

#[cfg(test)]
mod tests {
    use crate::{points::*, field::*};
    use ark_ec::{msm::VariableBaseMSM};
    
    #[test]
    fn test_msm() {
        let count = 1 << 10;
        let seed: Option<u64> = None;

        let points = generate_projective_points(count, get_rng(seed));
        let scalars = generate_scalars(count, get_rng(seed));

        let point_ark: Vec<_> = points.iter().map(|x| x.to_ark_repr()).collect();
        let scalar_ark: Vec<_> = scalars.iter().map(|x| x.to_ark_mod_p().0).collect();

        let msm_result_ark = VariableBaseMSM::multi_scalar_mul(&point_ark, &scalar_ark);

        println!("MSM result is: {:?}", ProjectivePoint::from_projective(msm_result_ark).to_ark_affine());
    }
}
