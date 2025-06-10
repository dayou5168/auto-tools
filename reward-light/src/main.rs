mod sub_api;
mod sub_rpc;
mod util;
use clap::Parser;
use std::time::Instant;
use util::BlockRewardRecord;
use futures::stream;
use futures::StreamExt;

const BLOCKS_PER_DAY: u32 = 14400;
const CONCURRENCY: usize = 32; // 并发常量

/// Autonomys奖励统计工具,统计最近X天的所有区块奖励
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 查询天数,建议7天左右，过大查询慢，过小不准确
    #[arg(short, long, default_value_t = 7)]
    days: u32,

    /// 节点地址，建议连接自己的全节点
    #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
    ws_url: String,

    /// 奖励明细CSV文件名
    #[arg(long, default_value = "reward_detail.csv")]
    reward_detail_csv: String, // 修改字段名

    /// 按账户汇总CSV文件名
    #[arg(long, default_value = "sum_reward.csv")]
    summary_csv: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api = sub_api::connect_api(&args.ws_url).await?;
    let rpc = sub_rpc::connect_rpc(&args.ws_url).await?;

    let best_block_hash = sub_rpc::get_block_hash(&rpc, None).await?;
    let latest_block = sub_api::get_block_number(&api, best_block_hash).await?;
    let total_blocks = args.days * BLOCKS_PER_DAY;
    let start_block = latest_block - total_blocks;

    let start_time = Instant::now();
    println!("start block: {}", &start_block);
    println!("end block: {}", &latest_block);
    let block_range = start_block..=latest_block;

    let all_rewards: Vec<BlockRewardRecord> = stream::iter(block_range)
        .map(|n| {
            let rpc = rpc.clone();
            let api = api.clone();
            async move {
                let block_hash = sub_rpc::get_block_hash(&rpc, Some(n)).await.ok()?;
                sub_api::get_ev(&api, block_hash).await.ok()
            }
        })
        .buffer_unordered(CONCURRENCY)
        .filter_map(|rewards| async move { rewards })
        .flat_map(stream::iter)
        .collect()
        .await;
    let duration = start_time.elapsed();
    println!("查询区块记录耗时: {:.2?}", duration);

    let summary = util::summarize_rewards_by_account(&all_rewards);
    util::write_rewards_csv(&args.reward_detail_csv, &all_rewards)?; // 修改调用
    util::write_account_summary_csv(&args.summary_csv, &summary)?;
    println!("已输出到 {} 和 {}", &args.reward_detail_csv, &args.summary_csv);
    Ok(())
}
