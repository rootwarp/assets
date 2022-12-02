use std::error::Error;

use reqwest;
use async_trait::async_trait;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse {
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelegateResponse {
    pub delegation_responses: Vec<DelegateData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelegateData {
    pub delegation: Delegation,
    pub balance: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Delegation {
    pub delegator_address: String,
    pub validator_address: String,
    pub shares: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RewardsResponse {
    pub rewards: Vec<RewardsData>,
    pub total: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RewardsData {
    pub validator_address: String,
    pub reward: Vec<Balance>,
}

#[async_trait]
pub trait Account {
    async fn balance(&self) -> Result<Vec<Balance>, Box<dyn Error>>;
    async fn staking(&self) -> Result<Vec<DelegateData>, Box<dyn Error>>;
    async fn rewards(&self) -> Result<Vec<Balance>, Box<dyn Error>>;
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
                println!("balance {}", body);

                let ret: serde_json::Result<BalanceResponse> = serde_json::from_str(&*body);
                match ret {
                    Ok(balance) => Ok(balance.balances),
                    Err(err) => Err(Box::new(err)),
                }
            }
            Err(err) => Err(Box::new(err))
        }
    }

    async fn staking(&self) -> Result<Vec<DelegateData>, Box<dyn Error>> {
        let base = "https://cosmos-mainnet-rpc-korea.allthatnode.com:1317";

        let url = format!("{}/cosmos/staking/v1beta1/delegations/{}", base, self.address);
        let response = reqwest::get(url).await;
        match response {
            Ok(resp) => {
                let body = resp.text().await?;
                println!("staking {}", body);

                let ret: serde_json::Result<DelegateResponse> = serde_json::from_str(&*body);
                match ret {
                    Ok(data) => Ok(data.delegation_responses),
                    Err(err) => Err(Box::new(err)),
                }
            }
            Err(err) => Err(Box::new(err))
        }
    }

    async fn rewards(&self) -> Result<Vec<Balance>, Box<dyn Error>> {
        let base = "https://cosmos-mainnet-rpc-korea.allthatnode.com:1317";

        let url = format!("{}/cosmos/distribution/v1beta1/delegators/{}/rewards", base, self.address);
        let response = reqwest::get(url).await;
        match response {
            Ok(resp) => {
                let body = resp.text().await?;
                println!("rewards {}", body);

                let ret: serde_json::Result<RewardsResponse> = serde_json::from_str(&*body);
                match ret {
                    Ok(rewards) => Ok(rewards.total),
                    Err(err) => Err(Box::new(err)),
                }
            }
            Err(err) => Err(Box::new(err))
        }
    }
}
