use glassbench::*;
use merkle_bench::Receipt;
use solana_merkle_tree::MerkleTree;
// use receipt::Receipt;


fn bench_merkelization(bench: &mut Bench){
    bench.task("merkelization", |task|{
        task.iter(||{
            let rng = &mut rand::thread_rng();
            let recs = Receipt::gen_random_nodes(rng);
            MerkleTree::new(recs.as_slice())
        })
    })
}

glassbench!(
    "merkle bench",
    bench_merkelization,
);