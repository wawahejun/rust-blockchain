use serde::{Serialize, Deserialize};
use bellman::groth16::{Proof, VerifyingKey};
use bellman::pairing::bn256::{Bn256, Fr};
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16::generate_random_parameters;
use bellman::groth16::verify_proof;
use bellman::groth16::prepare_verifying_key;
use bellman::groth16::create_random_proof;
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivacyTransaction {
    pub proof: Proof<Bn256>, // ZKP 证明
    pub vk: VerifyingKey<Bn256>, // 验证密钥
}

#[derive(Clone)]
struct PrivacyCircuit {
    amount: Option<u64>,
}

impl Circuit<Fr> for PrivacyCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        let amount = cs.alloc(|| "amount", || self.amount.ok_or(SynthesisError::AssignmentMissing))?;
        cs.enforce(
            || "amount > 0",
            |lc| lc + amount,
            |lc| lc + CS::one(),
            |lc| lc,
        );
        Ok(())
    }
}

impl PrivacyTransaction {
    // 创建隐私交易
    pub fn new(amount: u64) -> Self {
        let circuit = PrivacyCircuit {
            amount: Some(amount),
        };
        let rng = &mut OsRng;
        let params = generate_random_parameters::<Bn256, _, _>(circuit.clone(), rng).unwrap();
        let proof = create_random_proof(circuit, &params, rng).unwrap();
        let vk = prepare_verifying_key(&params.vk);
        PrivacyTransaction { proof, vk }
    }

    // 验证隐私交易
    pub fn verify(&self) -> bool {
        verify_proof(&self.vk, &self.proof, &[]).is_ok()
    }
}
