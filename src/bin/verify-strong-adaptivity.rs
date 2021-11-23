#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use strong_adaptivity::{Instance, Proof, Witness, data::puzzle_data};
use strong_adaptivity::{verify, prove, rogue_prove, utils::b2s_hash_to_field};
use strong_adaptivity::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use ark_std::{UniformRand};


fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let ck = puzzle_data();

    let mut rng = ChaChaRng::from_seed(*b"zkHack IPA puzzle for 2021-10-26");

    let (instance, witness, proof): (Instance, (Fr, Fr, Fr, Fr), Proof) = {
        // Your solution here!
        // Generate regular proof
        let a  = Fr::from(1);
        let r_1 = Fr::from(3);
        let r_2 = Fr::from(4);
        let comm_1 = ck.commit_with_explicit_randomness(a, r_1);
        let mut comm_2 = ck.commit_with_explicit_randomness(a, r_2); 
        let instance = Instance { comm_1, comm_2 };
        let witness = Witness { a, r_1, r_2 };
        let (a2, proof) = rogue_prove(&ck, &instance, &witness, &mut rng);
        let e = b2s_hash_to_field(&(ck, proof.commitment));
        
        comm_2 = ck.commit_with_explicit_randomness(a2, r_2); 
        let witness_rogue = (a, r_1, a2, r_2);
        let instance_rogue = Instance { comm_1, comm_2 };
                
        (instance_rogue, witness_rogue, proof)
    };
    
    let (a_1, r_1, a_2, r_2) = witness;


    assert!(verify(&ck, &instance, &proof));

    println!("Checking!");
    // Check that commitments are correct
    assert_eq!(ck.commit_with_explicit_randomness(a_1, r_1), instance.comm_1);
    assert_eq!(ck.commit_with_explicit_randomness(a_2, r_2), instance.comm_2);
    // Check that messages are unequal
    assert_ne!(a_1, a_2);

    println!("Soulved");
}
