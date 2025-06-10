use hex::decode;
use subxt::utils::H256;
use std::collections::HashMap;
use serde;
use std::fs::File;
use std::io::{Write, Result as IoResult};

#[derive(Debug,Clone,serde::Serialize)]
pub struct BlockRewardRecord {
    pub block_number: u32,
    pub block_hash: H256,
    pub acount: String,
    pub amount: f64, 
}

pub fn st_to_hash(s_option: Option<String>) -> H256 {
    let hash_str = s_option.unwrap();
    let hash_bytes = decode(hash_str.trim_start_matches("0x")).unwrap();
    let hash = H256::from_slice(&hash_bytes);
    hash
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AccountSummary {
    pub count: usize,
    pub total_reward: f64,
    pub ratio: f64, // 奖励占整体奖励的比例（0~1）
}

/// 按账户统计所有奖励记录，并求和
pub fn summarize_rewards_by_account(records: &[BlockRewardRecord]) -> HashMap<String, AccountSummary> {
    let mut map: HashMap<String, AccountSummary> = HashMap::new();
    let total_reward: f64 = records.iter().map(|r| r.amount).sum();

    for rec in records {
        let entry = map.entry(rec.acount.clone()).or_insert(AccountSummary {
            count: 0,
            total_reward: 0.0,
            ratio: 0.0,
        });
        entry.count += 1;
        entry.total_reward += rec.amount;
    }

    // 计算奖励占比
    for summary in map.values_mut() {
        summary.ratio = if total_reward > 0.0 {
            summary.total_reward / total_reward
        } else {
            0.0
        };
    }

    map
}

/// 输出 BlockRewardRecord 列表为 CSV
pub fn write_rewards_csv(path: &str, records: &[BlockRewardRecord]) -> IoResult<()> {
    let mut file = File::create(path)?;
    writeln!(file, "block_number,block_hash,account,amount")?;
    for r in records {
        writeln!(
            file,
            "{},0x{},{},{}",
            r.block_number,
            hex::encode(r.block_hash),
            r.acount,
            r.amount
        )?;
    }
    Ok(())
}

/// 输出账户统计为 CSV
pub fn write_account_summary_csv(path: &str, summary: &HashMap<String, AccountSummary>) -> IoResult<()> {
    let mut file = File::create(path)?;
    writeln!(file, "account,count,total_reward,ratio")?;
    for (account, s) in summary {
        writeln!(
            file,
            "{},{},{},{}",
            account,
            s.count,
            s.total_reward,
            s.ratio
        )?;
    }
    Ok(())
}


