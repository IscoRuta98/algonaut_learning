use algonaut::algod::v2::Algod;
use algonaut::error::AlgonautError;
use algonaut::indexer::v2::Indexer;
use algonaut::model::algod::v2::PendingTransaction;
use algonaut::transaction::account::Account;
use algonaut::transaction::builder::DestroyAsset;
use algonaut::transaction::{CreateAsset, TransferAsset, TxnBuilder};
use algonaut::{algod, indexer};
use std::env;
use std::process::id;
use std::time::{Duration, Instant};
use algonaut::model::indexer::v2::QueryAssetsInfo;
use anyhow;
use algonaut::transaction::tx_group::TxGroup;


// Issue DRT
pub fn issue_token(
    algod: &Algod,
    creator: &Account,
    creator_mnemonic: &str,
    UNIT_NAME: &str,
    ASSET_NAME: &str,
) -> anyhow::Result<()> {
    // Give functions parameters
    // function takes in parameters, and outputs the asset id of the token

    // Get creator's account
    let creator = Account::from_mnemonic(creator_mnemonic)?;
    println!("Creator: {:?}", creator.address());

    //
    let params = algod.suggested_transaction_params().await?;

    let create_token = TxnBuilder::with(
        params,
        CreateAsset::new(creator.address(), 1, 0, false)
            .unit_name(UNIT_NAME.to_owned())
            .asset_name(ASSET_NAME.to_owned())
            .manager(creator.address())
            .reserve(creator.address())
            .freeze(creator.address())
            .clawback(creator.address())
            .url("example.com".to_owned())
            .build(),
    )
    .build();

    // Creator signs the transaction
    let signed_t = creator.sign_transaction(&t)?;

    // Broadcast the transaction to the network and print out Transaction ID
    let send_response = algod.broadcast_signed_transaction(&signed_t).await?;
    println!("Transaction ID: {}", send_response.tx_id);

    // Print out Asset Index
    let pending_t = wait_for_pending_transaction(&algod, &send_response.tx_id).await?;
    println!("Asset index: {:?}", pending_t.map(|t| t.asset_index));

    Ok(())
}

pub fn check_token_redemption(indexer: &Indexer, TOKEN_ID: u64) -> _check_asset_status {
    // let indexer = Indexer::new(&env::var("INDEXER_URL")?)?;
    let _check_asset_status = indexer.assets_info(TOKEN_ID, QueryAssetsInfo);
}

// Implement redemption function.

pub fn reedem_token(
    algod: &Algod,
    creator: &Account,
    creator_mnemonic: &str,
    sender_mnemonic: &str,
    UNIT_NAME: &str,
    ASSET_NAME: &str,
) {
    // Get creator and sender's address
    let creator_address =  Account::from_mnemonic(creator_mnemonic)?;
    let sender_address = Account::from_mnemonic(sender_mnemonic)?;

    // Transaction 1: send asset back to owner

    transaction_1 = TxnBuilder::with(
        params,
        TransferAsset::new(
            sender_address.address(),
            ASSET_ID,
            1,
            creator_address.address()
        ).build(),
    )
    .build();

    // Transaction 2: Destroy Asset
    transaction_2 = TxnBuilder::with(
        params,
        DestroyAsset(creator_address, ASSET_ID)
    )

    // Group Transactions

    TxGroup::assign_group_id(vec![transaction_1,transaction_2]);

    let signed_t1 = account1.sign_transaction(&transaction_1)?;
    let signed_t2 = account2.sign_transaction(&transaction_2)?;

    let response = algod.broadcast_signed_transactions(&[signed_t1, signed_t2]).await;
    println!("response: {:?}", response)

}
