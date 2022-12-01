use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};
use nft_io::*;

#[cfg(debug_assertions)]
const PATH: &str = "../target/wasm32-unknown-unknown/debug/nft.opt.wasm";

#[cfg(not(debug_assertions))]
const PATH: &str = "../target/wasm32-unknown-unknown/release/nft.opt.wasm";

#[tokio::test]
async fn mint_test() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init_nft = InitNFT {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        base_uri: String::from(""),
        royalties: None,
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_nft.clone(),
            0,
            true,
            None,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(PATH, gclient::bytes_now(), init_nft, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn burn_test() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init_nft = InitNFT {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        base_uri: String::from(""),
        royalties: None,
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_nft.clone(),
            0,
            true,
            None,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(PATH, gclient::bytes_now(), init_nft, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let transaction_id = transaction_id + 1;

    let burn_payload = NFTAction::Burn {
        transaction_id,
        token_id: 0.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, burn_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, burn_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    // failures
    let burn_payload = NFTAction::Burn {
        transaction_id,
        token_id: 666.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, burn_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, burn_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn transfer_test() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init_nft = InitNFT {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        base_uri: String::from(""),
        royalties: None,
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_nft.clone(),
            0,
            true,
            None,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(PATH, gclient::bytes_now(), init_nft, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let transaction_id = transaction_id + 1;

    let transfer_payload = NFTAction::Transfer {
        transaction_id,
        to: ActorId::from(4u64),
        token_id: 0.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, transfer_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, transfer_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn owner_test() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init_nft = InitNFT {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        base_uri: String::from(""),
        royalties: None,
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_nft.clone(),
            0,
            true,
            None,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(PATH, gclient::bytes_now(), init_nft, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let owner_payload = NFTAction::Owner { token_id: 0.into() };

    let gas_info = api
        .calculate_handle_gas(None, program_id, owner_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, owner_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn approved_success() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init_nft = InitNFT {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        base_uri: String::from(""),
        royalties: None,
    }
    .encode();
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_nft.clone(),
            0,
            true,
            None,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(PATH, gclient::bytes_now(), init_nft, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let transaction_id: u64 = 0;
    use gear_lib::non_fungible_token::token::TokenMetadata;
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NFTAction::Mint {
        transaction_id,
        token_metadata,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, mint_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, mint_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    let transaction_id = transaction_id + 1;
    let owner_payload = NFTAction::Approve {
        transaction_id,
        to: ActorId::from(3),
        token_id: 0.into(),
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, owner_payload.encode(), 0, true, None)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, owner_payload, gas_info.min_limit, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}
