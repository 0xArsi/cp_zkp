#[cfg(test)]
mod cp_zkp_tests{
    use crate::cp_zkp::prover::Prover;
    use crate::cp_zkp::verifier::Verifier;
    use num_bigint::BigUint;
    use num_traits::One;
    use hex;

    #[test]
    fn test_cp_zkp_complete(){
        let _x = hex::decode("B9A3B3AE8FEFC1A2930496507086F8455D48943E").unwrap();
        let _p = hex::decode(String::from("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371")).unwrap();

        let _q = hex::decode(String::from("F518AA8781A8DF278ABA4E7D64B7CB9D49462353")).unwrap();

        let _g = hex::decode(String::from("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5")).unwrap();

       let p = BigUint::from_bytes_be(&_p);
       let g1 = BigUint::from_bytes_be(&_g);
       let g2 = g1.modpow(&BigUint::from(8454387u32), &p);
       
       let mut prover = Prover::new(&_x, &_p, &_q);
       prover.set_generators(&g1, &g2);

       let mut verifier = Verifier::new(&_p, &_q);

       //prover picks random k and tells it to verifier
       verifier.k = BigUint::from_bytes_be(&prover.get_k());
       
       //verifier picks random c and tells it to prover
       prover.c = BigUint::from_bytes_be(&verifier.get_c()); 

       assert!(verifier.verify(&prover.solve_challenge()));
    }
    //soundness already tested (lightly) in verifier tests
    #[test]
    fn test_cp_zkp_soundness(){
        println!("already tested");
    }
}