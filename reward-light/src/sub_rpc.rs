use anyhow::Result;
use subxt_rpcs::client::RpcParams;
use subxt_rpcs::RpcClient;
use subxt::utils::H256;
use crate::util;

pub async fn connect_rpc(url: &str) -> Result<RpcClient, subxt_rpcs::Error>{
    if url.starts_with("ws://") {
        RpcClient::from_insecure_url(url).await
    } else {
        RpcClient::from_url(url).await
    }
}


pub async fn get_block_hash(
    rpc: &RpcClient,
    number: Option<u32>,
) -> Result<H256> {
    let mut params = RpcParams::new();
    if let Some(n) = number {
        params.push(n)?;
    }
    let hash_str: Option<String> = rpc.request("chain_getBlockHash", params).await?;
    let hash = util::st_to_hash(hash_str);
    Ok(hash)
}

