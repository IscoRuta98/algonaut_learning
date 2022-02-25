use algonaut::algod::v2::Algod;
use algonaut::transaction::ClawbackAsset;
use algonaut::transaction::{account::Account, TxnBuilder};
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async pub fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let algod = Algod::new(&env::var("ALGOD_URL")?, &env::var("ALGOD_TOKEN")?)?;

    // The account specified as clawback when creating the asset.
    let sender = Account::from_mnemonic("kiss solar urge under dragon alone merry empty monkey hospital crisp over version tragic move cat company climb draft twin cram patrol say able sausage")?;
    let sender_address = sender.address();

    let receiver = Account::from_mnemonic("maze fortune disagree inside sniff mail balcony parade error alert isolate acoustic choose dress music response room forget crash lottery chronic dragon second able mad")?;
    let asset_receiver_address = receiver.address();


    let params = algod.suggested_transaction_params().await?;

    let t = TxnBuilder::with(
        params,
        ClawbackAsset::new(
            sender_address,
            ASSET_ID,
            1, // Assumes that only one token can be redeemed at once.
            sender_address,
            asset_receiver_address,
        )
        .build(),
    )
    .build();

    let sign_response = sender.sign_transaction(&t)?;

    // Broadcast the transaction to the network
    // Note this transaction will get rejected because the accounts do not have any tokens
    let send_response = algod.broadcast_signed_transaction(&sign_response).await;
    println!("{:#?}", send_response);

    Ok(())
}