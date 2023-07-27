use rand::Rng;
use solana_sdk::signature::Signature;

#[derive(Debug)]
pub struct Receipt{
   pub signature: Signature,
   pub status: u8
}

impl Receipt{
   pub fn new_rand<R: Rng>(rng: &mut R) -> Self{
        Self { 
            signature: Signature::default(), 
            status: rng.gen() 
        }
    }
   pub fn gen_random_receipts<R: Rng>(rng: &mut R) -> Vec<Self>{
        let round_range = rng.gen_range(0..50000usize);
        let mut vec = Vec::new();
        for i in 0..round_range{
            vec.push(Self::new_rand(rng))
        }
        vec
    }
}