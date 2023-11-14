use num_bigint::BigUint;
use rand::Rng;
use num_traits::One;

#[derive(Default, Debug)]
pub struct Verifier{
    g1: BigUint,
    g2: BigUint,
    g1x: BigUint,
    g2x: BigUint,
    pub p: BigUint,
    pub q: BigUint,
    pub c: BigUint,
    pub k: BigUint,
}

impl Verifier{
    pub fn new(_p: &[u8], _q: &[u8]) -> Verifier{
        let mut new_verifier = Verifier::default();
        new_verifier.p = BigUint::from_bytes_be(_p);
        new_verifier.q = BigUint::from_bytes_be(_q);
        new_verifier
    }
    pub fn get_c(&mut self) -> [u8;32]{
        let mut rng = rand::thread_rng();
        let c_bits = rng.gen::<[u8;32]>();
        self.c = BigUint::from_bytes_be(&c_bits);
        c_bits
    }
    pub fn verify(&self, s: &BigUint) -> bool{
        //should be able to recover generator power using s.
        //The chronology of this is not totally valid,
        //As the verifier gets k and the generator powers
        //before sending c to the prover
        let g1k = self.g1.modpow(&self.k, &self.p);
        let g2k = self.g2.modpow(&self.k, &self.p);
        let g1s = self.g1.modpow(s, &self.p); 
        let g2s = self.g2.modpow(s, &self.p); 
        
        let val1 = (&g1s * &(self.g1x.modpow(&self.c, &self.p))).modpow(&BigUint::one(), &self.p);
        let val2 = (&g2s* &(self.g2x.modpow(&self.c, &self.p))).modpow(&BigUint::one(), &self.p);

        val1 == g1k && val2 == g2k
    }
}

#[cfg(test)]
mod test_verifier{
    use super::*;

    #[test]
    fn test_new(){
        let _p = (59 as u32).to_be_bytes();
        let _q = (11 as u32).to_be_bytes();
        let verifier = Verifier::new(&_p, &_q);
        println!("new verifier:\n{:?}", verifier);
    }

    #[test]
    fn test_verify(){
        let _x = (6 as u32).to_be_bytes();
        let _p = (23 as u32).to_be_bytes();
        let _q = (11 as u32).to_be_bytes();
        let _c = (4 as u32).to_be_bytes();
        let _k = (7 as u32).to_be_bytes();
        let _g1 = (4 as u32).to_be_bytes();
        let _g2 = (9 as u32).to_be_bytes();
        let secret = BigUint::from_bytes_be(&_x);
        let mut verifier = Verifier::new(&_p, &_q);
        verifier.g1 = BigUint::from_bytes_be(&_g1);
        verifier.g2 = BigUint::from_bytes_be(&_g2);
        verifier.c = BigUint::from_bytes_be(&_c);
        verifier.k = BigUint::from_bytes_be(&_k);
        verifier.g1x = verifier.g1.modpow(&secret, &verifier.p); 
        verifier.g2x = verifier.g2.modpow(&secret, &verifier.p); 
        let s = BigUint::from_bytes_be(&((5 as u32).to_be_bytes()));
        assert!(verifier.verify(&s));
    }

    fn test_verify_sound(){
        let _x = (6 as u32).to_be_bytes();
        let _p = (23 as u32).to_be_bytes();
        let _q = (11 as u32).to_be_bytes();
        let _c = (4 as u32).to_be_bytes();
        let _k = (7 as u32).to_be_bytes();
        let _g1 = (4 as u32).to_be_bytes();
        let _g2 = (9 as u32).to_be_bytes();
        let secret = BigUint::from_bytes_be(&_x);
        let mut verifier = Verifier::new(&_p, &_q);
        verifier.g1 = BigUint::from_bytes_be(&_g1);
        verifier.g2 = BigUint::from_bytes_be(&_g2);
        verifier.c = BigUint::from_bytes_be(&_c);
        verifier.k = BigUint::from_bytes_be(&_k);
        verifier.g1x = verifier.g1.modpow(&secret, &verifier.p); 
        verifier.g2x = verifier.g2.modpow(&secret, &verifier.p); 
        //the correct value of s is 5. let's use something else
        let s = BigUint::from_bytes_be(&((6 as u32).to_be_bytes()));
        assert!(!verifier.verify(&s));
    }
}