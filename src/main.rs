use std::env;
use futures;

use num_bigint::BigInt;
use assets::cosmos::{CosmoshubAccount, Account};

#[tokio::main]
async fn main() {
    let account = env::var("ACCOUNT").unwrap_or("".to_string());

    let cosmos = CosmoshubAccount{address: String::from(account)};
    let bal = cosmos.balance();
    let staking = cosmos.staking();
    let rewards = cosmos.rewards();

    let ret = futures::try_join!(bal, staking, rewards);

    match ret {
        Ok(values) => {
            let balance: BigInt = values.0;
            let staking: BigInt = values.1;
            let reward: BigInt = values.2;

            println!("Balance {:?}", balance);
            println!("Staking {:?}", staking);
            println!("Rewards {:?}", reward);
            
            let sum: BigInt = [balance, staking, reward].iter().sum();

            println!("Total {:?}", sum);
        }

        Err(err) => {
            println!("{}", err);
        }
    }
}