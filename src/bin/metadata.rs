use log::info;

use hyperliquid_rust_sdk::{AssetMapping, BaseUrl, Dex};

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = reqwest::Client::new();
    let assets = AssetMapping::new(
        &client,
        &BaseUrl::Mainnet.get_url(),
        vec![
            // Dex::Hyperliquid.dex_name().to_string(),
            // Dex::Hyena.dex_name().to_string(),
            Dex::Xyz.dex_name().to_string(),
        ],
    )
    .await
    .unwrap();

    info!("{:#?}", assets.symbol_to_asset_id);
    info!("{:#?}", assets.asset_name_to_symbol);
}
