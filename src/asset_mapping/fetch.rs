// src/asset_mapping/fetch.rs
use reqwest::Client;
use serde::Serialize;

use super::error::AssetMappingError;
use super::meta::{Meta, PerpDexMeta, SpotMeta};

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum InfoRequest {
    Meta {
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    SpotMeta,
    PerpDexs,
}

async fn post_info<T: serde::de::DeserializeOwned>(
    client: &Client,
    base_url: &str,
    request: &InfoRequest,
) -> Result<T, AssetMappingError> {
    let url = format!("{base_url}/info");
    let body = serde_json::to_string(request)
        .map_err(|e| AssetMappingError::JsonParse(e.to_string()))?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let status = response.status().as_u16();
    let text = response.text().await?;

    if status >= 400 {
        return Err(AssetMappingError::Api {
            status_code: status,
            message: text,
        });
    }

    serde_json::from_str(&text).map_err(|e| AssetMappingError::JsonParse(e.to_string()))
}

pub(super) async fn fetch_spot_meta(
    client: &Client,
    base_url: &str,
) -> Result<SpotMeta, AssetMappingError> {
    post_info(client, base_url, &InfoRequest::SpotMeta).await
}

pub(super) async fn fetch_meta(
    client: &Client,
    base_url: &str,
    dex: Option<String>,
) -> Result<Meta, AssetMappingError> {
    post_info(client, base_url, &InfoRequest::Meta { dex }).await
}

pub(super) async fn fetch_perp_dexs(
    client: &Client,
    base_url: &str,
) -> Result<Vec<Option<PerpDexMeta>>, AssetMappingError> {
    post_info(client, base_url, &InfoRequest::PerpDexs).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spot_meta_request_serialization() {
        let req = InfoRequest::SpotMeta;
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"type":"spotMeta"}"#);
    }

    #[test]
    fn test_meta_request_serialization_no_dex() {
        let req = InfoRequest::Meta { dex: None };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"type":"meta"}"#);
    }

    #[test]
    fn test_meta_request_serialization_with_dex() {
        let req = InfoRequest::Meta { dex: Some("hyna".to_string()) };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"type":"meta","dex":"hyna"}"#);
    }

    #[test]
    fn test_perp_dexs_request_serialization() {
        let req = InfoRequest::PerpDexs;
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"type":"perpDexs"}"#);
    }
}
