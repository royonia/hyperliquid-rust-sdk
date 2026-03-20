// src/asset_mapping/mapping.rs
use reqwest::Client;
use std::collections::HashMap;

use super::dex::Dex;
use super::error::AssetMappingError;
use super::fetch;

#[derive(Clone, Debug)]
pub struct AssetMapping {
    pub symbol_to_asset_id: HashMap<String, u32>,
    pub asset_name_to_symbol: HashMap<String, String>,
}

impl AssetMapping {
    pub async fn new(
        client: &Client,
        base_url: &str,
        interested_perp_dexs: Vec<String>,
    ) -> Result<Self, AssetMappingError> {
        let mut symbol_to_asset_id = HashMap::new();
        let mut asset_name_to_symbol = HashMap::new();

        // Spot meta
        {
            let spot_meta = fetch::fetch_spot_meta(client, base_url).await?;

            for spot_info in &spot_meta.universe {
                let asset = spot_info.index + 10_000;

                let [base, quote] = spot_info.tokens;
                if spot_meta.tokens.len() < base as _ {
                    eprintln!("Base token index out-of-bound: {base}");
                    continue;
                }

                let base_info = &spot_meta.tokens[base as usize];
                if base_info.index != base {
                    eprintln!(
                        "mismatch spot base asset index. expect {base}. got {}",
                        base_info.index
                    );
                    continue;
                }

                if spot_meta.tokens.len() < quote as _ {
                    eprintln!("Quote token index out-of-bound: {quote}");
                    continue;
                }
                let quote_info = &spot_meta.tokens[quote as usize];
                if quote_info.index != quote {
                    eprintln!(
                        "mismatch spot quote asset index. expect {quote}. got {}",
                        quote_info.index
                    );
                    continue;
                }

                let symbol = format!("{}/{}", base_info.name, quote_info.name);

                // PURR/USDC is a special case in HL
                if &base_info.name == "PURR" && &quote_info.name == "USDC" {
                    continue;
                }

                if !symbol_to_asset_id.insert(symbol.clone(), asset).is_none() {
                    eprintln!(
                        "Override entry for spot asset_name_to_id: {}",
                        spot_info.name
                    );
                }

                symbol_to_asset_id.insert(spot_info.name.clone(), asset);
                asset_name_to_symbol.insert(spot_info.name.clone(), symbol.clone());
            }
        }

        // Perp DEXes
        {
            let perp_dexs = fetch::fetch_perp_dexs(client, base_url).await?;

            for (idx, perp_dex) in perp_dexs.iter().enumerate() {
                let Some(perp_dex) = perp_dex.as_ref() else {
                    // Hyperliquid special case: index 0 is null
                    if idx == 0
                        && interested_perp_dexs
                            .contains(&Dex::Hyperliquid.dex_name().to_string())
                    {
                        let meta = fetch::fetch_meta(client, base_url, None).await?;
                        for (asset_ind, asset) in meta.universe.iter().enumerate() {
                            if symbol_to_asset_id.contains_key(&asset.name) {
                                eprintln!("duplicated perp asset entry: {}", asset.name);
                                continue;
                            }
                            assert!(symbol_to_asset_id
                                .insert(asset.name.clone(), asset_ind as u32)
                                .is_none());

                            asset_name_to_symbol
                                .insert(asset.name.clone(), asset.name.clone());
                        }
                    }
                    continue;
                };

                if !interested_perp_dexs.contains(&perp_dex.name) {
                    continue;
                }
                let offset = 100_000 + (idx as u32) * 10_000;

                let dex_meta =
                    fetch::fetch_meta(client, base_url, Some(perp_dex.name.to_owned())).await?;

                for (asset_ind, asset) in dex_meta.universe.iter().enumerate() {
                    if symbol_to_asset_id.contains_key(&asset.name) {
                        eprintln!(
                            "duplicated dex asset entry: {}, dex: {}",
                            asset.name, perp_dex.name
                        );
                        continue;
                    }
                    assert!(
                        symbol_to_asset_id
                            .insert(asset.name.clone(), asset_ind as u32 + offset)
                            .is_none(),
                        "duplicated dex asset entry: {}",
                        asset.name
                    );

                    asset_name_to_symbol.insert(asset.name.clone(), asset.name.clone());
                }
            }
        }

        Ok(Self {
            symbol_to_asset_id,
            asset_name_to_symbol,
        })
    }

    pub fn asset_id(&self, symbol: &str) -> Option<u32> {
        self.symbol_to_asset_id.get(symbol).cloned()
    }

    pub fn symbol(&self, asset_name: &str) -> Option<&String> {
        self.asset_name_to_symbol.get(asset_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_mapping_returns_none() {
        let mapping = AssetMapping {
            symbol_to_asset_id: HashMap::new(),
            asset_name_to_symbol: HashMap::new(),
        };
        assert_eq!(mapping.asset_id("BTC"), None);
        assert_eq!(mapping.symbol("BTC"), None);
    }

    #[test]
    fn test_lookup_after_insert() {
        let mut s2id = HashMap::new();
        s2id.insert("BTC".to_string(), 0u32);
        let mut n2s = HashMap::new();
        n2s.insert("@1".to_string(), "USOL/USDC".to_string());

        let mapping = AssetMapping {
            symbol_to_asset_id: s2id,
            asset_name_to_symbol: n2s,
        };
        assert_eq!(mapping.asset_id("BTC"), Some(0));
        assert_eq!(mapping.symbol("@1"), Some(&"USOL/USDC".to_string()));
    }
}
