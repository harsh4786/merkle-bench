use solana_merkle_tree::merkle_tree::MerkleTree;
mod receipt;
use crate::receipt::Receipt;




fn main() {
    // print!("{:?}", Signature::default())
    let rng = &mut rand::thread_rng();
    // let x = rng.gen_range(0..50000usize);
    let recs = Receipt::gen_random_receipts(rng);
    // let merkle = MerkleTree::new(recs.as_slice());
    print!("{:?}", recs)
}
