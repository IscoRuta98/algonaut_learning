use algonaut::algod::v2::Algod;
use algonaut::transaction::AcceptAsset;
use algonaut::transaction::{account::Account, TxnBuilder};
use dotenv::dotenv;
use std::env;
use std::error::Error;


// Before the receiver can recieve an asset, they have to opt-in to the transaction.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let algod = Algod::new(&env::var("ALGOD_URL")?, &env::var("ALGOD_TOKEN")?)?;

    let account = Account::from_mnemonic("biology blouse spot woman clap demise exotic entire tumble bullet orange atom hood muscle material become income museum purchase laundry off answer orbit above snow")?;

    let params = algod.suggested_transaction_params().await?;

    let t = TxnBuilder::with(params, AcceptAsset::new(account.address(), ASSET_ID).build()).build();

    let sign_response = account.sign_transaction(&t)?;

    // Broadcast the transaction to the network
    // Note this transaction will get rejected because the accounts do not have any tokens
    let send_response = algod.broadcast_signed_transaction(&sign_response).await;
    println!("{:#?}", send_response);

    Ok(())
}