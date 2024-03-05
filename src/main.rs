extern crate bellman;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bls12_381::{Bls12, Fr};

struct DiplomaCircuit<F: Field> {
    university: Option<F>,
    grade: Option<F>,
    threshold: F,
}

impl<F: Field> Circuit<F> for DiplomaCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let university = cs.alloc(
            || "university",
            || self.university.ok_or(SynthesisError::AssignmentMissing),
        )?;
        let grade = cs.alloc(
            || "alloc",
            || self.grage.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // University must be A, B or C
        cs.enforce(
            || "valid university",
            |lc| lc + (F::one(), university) - F::one(), // encoded as 1
            |lc| lc + (F::from(2u32), university) - F::one(), // encoded as 2
            |lc| lc + (F::from(3u32), university) - F::one(), // encoded as 3
        );

        // Grade must be above threshold
        cs.enforce(
            || "grade above threshold",
            |lc| lc + grade,
            |lc| lc + CS::one(),
            |lc| lc + self.threshold,
        );
        Ok(())
    }
}

use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use rand::rngs::OsRng;

fn main() {
    let rng = &mut OsRng;

    let params = {
        let empty_circuit = DiplomaCircuit::<Fr> {
            university: None,
            grade: None,
            threshold: Fr::from(1u32), // Assuming binary encoding for grade
        };

        generate_random_parameters::<Bls12, _, _>(empty_circuit, rng)
            .expect("Parameter generation failed")
    };

    // Let's create a proof for having a diploma from university `A` (encoded as 1) with a passing grade (encoded as 1).
    let prover_circuit = DiplomaCircuit {
        university: Some(Fr::from(1u32)),
        grade: Some(Fr::from(1u32)),
        threshold: Fr::from(1u32),
    };

    let proof = create_random_proof(prover_circuit, &params, rng).expect("Proof generation failed");

    // Verify our proof
    let pvk = prepare_verifying_key(&params.vk);
    let public_inputs = vec![Fr::from(1u32)];

    assert!(verify_proof(&pvk, &proof, &public_inputs)).expect("Verification was failed");
}
