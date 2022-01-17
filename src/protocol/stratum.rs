use crate::{client::write_to_socket_byte, state::Worker, util::hex_to_int};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncWrite, WriteHalf};

use super::{ethjson::EthClientObject, CLIENT_LOGIN};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StraumRoot {
    pub id: i64,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StraumResult {
    pub id: i64,
    pub jsonrpc: String,
    pub result: Vec<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StraumResultWorkNotify {
    pub id: i64,
    pub method: String,
    pub params: (String, String, String, bool),
}

pub struct StraumMiningNotify {
    pub id: i64,
    pub method: String,
    pub params: Vec<String>,
}

async fn login<W>(
    worker: &mut Worker,
    w: &mut WriteHalf<W>,
    rpc: &mut Box<dyn EthClientObject + Send + Sync>,
    worker_name: &mut String,
) -> Result<()>
where
    W: AsyncWrite,
{
    if let Some(wallet) = rpc.get_wallet() {
        rpc.set_id(CLIENT_LOGIN);
        let mut temp_worker = wallet.clone();
        let mut split = wallet.split(".").collect::<Vec<&str>>();
        if split.len() > 1 {
            worker.login(
                temp_worker.clone(),
                split.get(1).unwrap().to_string(),
                wallet.clone(),
            );
            *worker_name = temp_worker;
        } else {
            temp_worker.push_str(".");
            temp_worker = temp_worker + rpc.get_worker_name().as_str();
            worker.login(temp_worker.clone(), rpc.get_worker_name(), wallet.clone());
            *worker_name = temp_worker;
        }

        write_to_socket_byte(w, rpc.to_vec()?, &worker_name).await
    } else {
        bail!("请求登录出错。可能收到暴力攻击");
    }
}