mod bindings;

use std::time::Duration;

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, Bytes, Uint, I256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};

use crate::bindings::{
    arenatoken::ArenaToken,
    liquidityprovider::{
        LiquidityProvider,
        LiquidityProvider::{ModifyLiquidityParams, PoolKey as LPoolKey},
    },
    poolmanager::{PoolManager, PoolManager::PoolKey},
};

#[tokio::main]
async fn main() {
    let anvil = Anvil::new().try_spawn().unwrap();

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint().parse().unwrap();

    let client = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);

    let pool_manager = PoolManager::deploy(client.clone(), Uint::from(5000))
        .await
        .unwrap();

    let currency0 = ArenaToken::deploy(
        client.clone(),
        "ARENA0".to_string(),
        "ARENA0".to_string(),
        18,
    )
    .await
    .unwrap();

    let currency1 = ArenaToken::deploy(
        client.clone(),
        "ARENA1".to_string(),
        "ARENA1".to_string(),
        18,
    )
    .await
    .unwrap();

    currency0
        .mint(Uint::from(2).pow(Uint::from(255)))
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
}
