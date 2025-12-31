use log::info;

use ethers::signers::LocalWallet;
use hyperliquid_rust_sdk::{BaseUrl, Dex, ExchangeClient};

#[tokio::main]
async fn main() {
    env_logger::init();
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: LocalWallet = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
        .parse()
        .unwrap();

    let exchange_client = ExchangeClient::new(
        None,
        wallet,
        Some(BaseUrl::Mainnet),
        vec![
            Dex::Hyperliquid.dex_name().to_string(),
            Dex::Hyena.dex_name().to_string(),
        ],
        None,
    )
    .await
    .unwrap();

    info!("{:#?}", exchange_client.symbol_to_asset);
}
