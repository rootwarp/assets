use std::env;

use assets::cosmos::{CosmoshubAccount, Account};

#[tokio::main]
async fn main() {
    let account = env::var("ACCOUNT").unwrap_or("".to_string());

    let cosmos = CosmoshubAccount{address: String::from(account)};
    let ret = cosmos.balance().await;

    match ret {
        Ok(balance) => {
            println!("{:?}", balance);
        }

        Err(err) => {
            println!("{}", err);
        }
    }
}