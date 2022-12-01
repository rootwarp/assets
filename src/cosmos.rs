use std::error::Error;

use reqwest;
use async_trait::async_trait;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse {
    balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    denom: String,
    amount: String,
}

#[async_trait]
pub trait Account {
    async fn balance(&self) -> Result<Vec<Balance>, Box<dyn Error>>;
}

pub struct CosmoshubAccount {
    pub address: String,
}

#[async_trait]
impl Account for CosmoshubAccount {
    async fn balance(&self) -> Result<Vec<Balance>, Box<dyn Error>> {
        let base = "https://cosmos-mainnet-rpc-korea.allthatnode.com:1317";

        let url = format!("{}/cosmos/bank/v1beta1/balances/{}", base, self.address);
        let response = reqwest::get(url).await;
        match response {
            Ok(resp) => {
                let body = resp.text().await?;
                let ret: serde_json::Result<BalanceResponse> = serde_json::from_str(&*body);
                match ret {
                    Ok(balance) => Ok(balance.balances),
                    Err(err) => Err(Box::new(err)),
                }
            }
            Err(err) => Err(Box::new(err))
        }
    }
}
