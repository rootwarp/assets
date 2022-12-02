use std::env;
use futures;

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
            let balance: f64 = values.0.iter()
                .map(|b| { b.amount.parse::<f64>().unwrap() })
                .sum();

            let staking: f64 = values.1.iter()
                .map(|v| { v.balance.amount.parse::<f64>().unwrap() })
                .sum();

            let reward: f64 = values.2.iter()
                .map(|v| { v.amount.parse::<f64>().unwrap() })
                .sum();
            
            let sum: f64 = [balance, staking, reward].iter().sum();

            println!("{:?}", values);
            println!("{:?} {:?} {:?}", balance, staking, reward);
            println!("Total {:?}", sum);
        }

        Err(err) => {
            println!("{}", err);
        }
    }
}