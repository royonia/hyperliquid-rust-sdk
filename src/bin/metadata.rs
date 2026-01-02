use log::info;

use hyperliquid_rust_sdk::{AssetMapping, BaseUrl, Dex, InfoClient};

#[tokio::main]
async fn main() {
    env_logger::init();

    let assets = AssetMapping::new(
        InfoClient::new(None, Some(BaseUrl::Mainnet)).await.unwrap(),
        vec![
            Dex::Hyperliquid.dex_name().to_string(),
            Dex::Hyena.dex_name().to_string(),
        ],
    )
    .await
    .unwrap();

    info!("{:#?}", assets.symbol_to_asset_id);
    info!("{:#?}", assets.asset_name_to_symbol);
}
