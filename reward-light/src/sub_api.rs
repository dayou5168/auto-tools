use subxt::events::EventDetails;
use subxt::{OnlineClient, PolkadotConfig,utils::H256};
use anyhow::Result;
use subxt::utils::AccountId32;
use sp_core::crypto::{AccountId32 as SpAccountId32, Ss58AddressFormat, Ss58Codec};
use parity_scale_codec::Decode;
use crate::BlockRewardRecord;



pub async fn connect_api(url: &str) -> Result<OnlineClient<PolkadotConfig>, subxt::Error> {
    if url.starts_with("ws://") {
        OnlineClient::<PolkadotConfig>::from_insecure_url(url).await
    } else {
        OnlineClient::<PolkadotConfig>::from_url(url).await
    }
}


pub async fn get_block_number(api: &OnlineClient<PolkadotConfig>, hash: H256)-> Result<u32>{
        // 获取区块信息
    let block = api.blocks().at(hash).await?;
    Ok(block.number())
}


pub async fn get_ev(api: &OnlineClient<PolkadotConfig>, hash: H256) -> Result<Vec<BlockRewardRecord>> {
    let events = api.events().at(hash).await?;
    let block_number = api.blocks().at(hash).await?.number();
    let mut records = Vec::new();

    for ev in events.iter() {
        let ev: EventDetails<PolkadotConfig> = ev?;
        if ev.pallet_name() == "Rewards" {
            let mut bytes = ev.field_bytes();
            let account = AccountId32::decode(&mut bytes)?;
            let raw_amount = u128::decode(&mut bytes)?;
            let amount = raw_amount as f64 / 1e18;
            // 用 sp_core::crypto::AccountId32 转换为 Su 开头地址
            let sp_account = SpAccountId32::from(account.0);
            let acount = sp_account.to_ss58check_with_version(Ss58AddressFormat::custom(6094));
            records.push(BlockRewardRecord {
                block_number,
                block_hash: hash,
                acount,
                amount,
            });
        }
    }
    Ok(records)
}