mod consts;
mod find;
mod utils;

use std::time::Instant;

use find::find_hash_par;
use ore_api::state::Proof;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};
use tokio::task;
use utils::{get_proof, proof_pubkey};

pub async fn get_proof_with_authority(client: &RpcClient, authority: Pubkey) -> Proof {
    let proof_address = proof_pubkey(authority);
    get_proof(client, proof_address).await
}

#[tokio::main]
async fn main() {
    let cutoff_time = 60u64;
    let threads = 30u64;

    let proof1 = get_proof_by_key(
        "",
    )
    .await;
    // let proof2 = get_proof_by_key(
    //     "",
    // )
    // .await;
    // let proof3 = get_proof_by_key(
    //     "",
    // )
    // .await;
    // let proof4 = get_proof_by_key(
    //     "",
    // )
    // .await;

    // 使用 tokio::spawn 启动四个异步测试任务
    let handles = vec![
        task::spawn(find_hash_par_test_instance(cutoff_time, threads, proof1)),
        // task::spawn(find_hash_par_test_instance(cutoff_time, threads, proof2)),
        // task::spawn(find_hash_par_test_instance(cutoff_time, threads, proof3)),
        // task::spawn(find_hash_par_test_instance(cutoff_time, threads, proof4)),
    ];

    // 等待所有任务完成
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn get_proof_by_key(key: &str) -> Proof {
    let signer = Keypair::from_base58_string(key);
    let cluster = "https://api.mainnet-beta.solana.com";
    let rpc_client =
        RpcClient::new_with_commitment(cluster.to_string(), CommitmentConfig::confirmed());

    // 获取 proof
    let proof = get_proof_with_authority(&rpc_client, signer.pubkey()).await;
    proof
}

async fn find_hash_par_test_instance(cutoff_time: u64, threads: u64, proof: Proof) {
    // 开始计时
    let start_time = Instant::now();

    // 调用 find_hash_par 并测试其性能
    let (_solution, best_difficulty, _hash) = match find_hash_par(proof, cutoff_time, threads).await
    {
        Some((nonce, difficulty, hash)) => (nonce, difficulty, hash),
        None => {
            // 处理None情况，例如返回默认值或者终止程序
            println!("No valid hash found within the given cutoff time.");
            return; // 或者使用其他合适的处理方式
        }
    };

    // 结束计时
    let duration = start_time.elapsed();

    // 打印测试结果和耗时
    println!(
        "Test completed in {:?} Best difficulty found: {:?} ",
        duration, best_difficulty
    );
}
