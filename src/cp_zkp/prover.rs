use num_traits::One;
use num_bigint::BigUint;
use rand::Rng;

#[derive(Default, Debug)]
pub struct Prover{
    //secret
    x: BigUint,
    g1: BigUint,
    g2: BigUint,
    pub p: BigUint,
    pub q: BigUint,
    pub c: BigUint,
    pub k: BigUint,
}

impl Prover{

    //construct bigints from bytes
    pub fn new(_x: &[u8], _p: &[u8], _q:&[u8]) -> Prover{
        let mut new_prover = Prover::default();
        new_prover.x = BigUint::from_bytes_be(_x);
        new_prover.p = BigUint::from_bytes_be(_p);
        new_prover.q = BigUint::from_bytes_be(_q);
        new_prover
    }
    pub fn set_generators(&mut self, _alpha: &BigUint, _beta: &BigUint){
        //should maybe require here that both values are less than q?
        self.g1 = _alpha.clone();
        self.g2 = _beta.clone();
    }
    pub fn set_random_generators(&mut self){
        let mut rng = rand::thread_rng();
        let alpha = BigUint::from_bytes_be(&(rng.gen::<[u8; 32]>()));
        let beta = BigUint::from_bytes_be(&(rng.gen::<[u8; 32]>()));
        self.g1 = alpha.modpow(&BigUint::one(), &self.q);
        self.g2 = beta.modpow(&BigUint::one(), &self.q);
    }

    pub fn get_generator_powers(&self, y: &BigUint) -> (BigUint, BigUint){
        (self.g1.modpow(y, &self.p), self.g2.modpow(y, &self.p))
    }

    pub fn get_k(&mut self) -> [u8; 32]{
        let mut rng = rand::thread_rng();
        let k_bytes = rng.gen::<[u8; 32]>();
        self.k = BigUint::from_bytes_be(&k_bytes);
        k_bytes
    }

    pub fn solve_challenge(&self) -> BigUint{
        let prod = &self.c * &self.x;
        if self.k >= prod{
            let diff = &self.k - &prod;
            diff.modpow(&BigUint::one(), &self.q) 

        }
        else{
            let diff = (&prod - &self.k) + &self.q;
            &self.q - &(diff.modpow(&BigUint::one(), &self.q))
        }
    }
}

#[cfg(test)]
mod test_prover{
    use super::*;

    #[test]
    fn test_new_prover(){
        let _x = (6 as u32).to_be_bytes();
        let _p = (59 as u32).to_be_bytes();
        let _q = (11 as u32).to_be_bytes();
        let prover = Prover::new(&_x, &_p, &_q);
    }

    #[test]
    fn test_set_random_generators(){
        let _x = (6 as u32).to_be_bytes();
        let _p = (59 as u32).to_be_bytes();
        let _q = (11 as u32).to_be_bytes();
        let mut prover = Prover::new(&_x, &_p, &_q);
        prover.set_random_generators();
        //println!("new prover with generators:\n{:?}", prover);
        assert_eq!(prover.g1.modpow(&prover.q, &prover.q), prover.g1);
        assert_eq!(prover.g2.modpow(&prover.q, &prover.q), prover.g2);
    }

    #[test]
    fn test_get_generator_powers(){
        let _x = (6 as u32).to_be_bytes();
        let _p = (59 as u32).to_be_bytes();
        let _q = (11 as u32).to_be_bytes();
        let _y = BigUint::from_bytes_be(&(23 as u32).to_be_bytes());
        let mut prover = Prover::new(&_x, &_p, &_q);
        prover.set_random_generators();
        let (alphak, betak) = prover.get_generator_powers(&_y);
        //println!("prover generator power:\n {:?}", alphak);
        //println!("prover generator power:\n {:?}", betak);

    }
    #[test]
    fn test_solve_challenge(){
        let _x = (6 as u32).to_be_bytes();
        let prover_x = BigUint::from_bytes_be(&_x);
        let _p = (59 as u32).to_be_bytes();
        let _q = (11 as u32).to_be_bytes();
        let _k = (7 as u32).to_be_bytes();
        let _c = (4 as u32).to_be_bytes();
        let mut prover = Prover::new(&_x, &_p, &_q);
        prover.c = BigUint::from_bytes_be(&_c); 
        prover.k = BigUint::from_bytes_be(&_k); 
        println!("prover:\n{:?}", prover);
        assert_eq!(prover.solve_challenge(), BigUint::from(5u32));
    }
}