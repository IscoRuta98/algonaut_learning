use algonaut::algod::v2::Algod;
use algonaut::transaction::TransferAsset;
use algonaut::transaction::{account::Account, TxnBuilder};
//use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    //dotenv().ok();

    // let algod = Algod::new(&env::var("ALGOD_URL")?, &env::var("ALGOD_TOKEN")?)?;
    let algod = Algod::new("http://localhost:4001", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")?;

    let from = Account::from_mnemonic("maze fortune disagree inside sniff mail balcony parade error alert isolate acoustic choose dress music response room forget crash lottery chronic dragon second able mad")?;
    let to = Account::from_mnemonic("biology blouse spot woman clap demise exotic entire tumble bullet orange atom hood muscle material become income museum purchase laundry off answer orbit above snow")?;

    let params = algod.suggested_transaction_params().await?;

    let t = TxnBuilder::with(
        params,
        TransferAsset::new(from.address(),  8, 1, to.address()).build(),
    )
    .build();

    let sign_response = from.sign_transaction(&t)?;

    // Broadcast the transaction to the network
    // Note this transaction will get rejected because the accounts do not have any tokens
    let send_response = algod.broadcast_signed_transaction(&sign_response).await;
    println!("{:#?}", send_response);

    Ok(())
}