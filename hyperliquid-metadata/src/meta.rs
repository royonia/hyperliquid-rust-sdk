// Minimal copies of meta structs from src/meta.rs.
// Fields unused by AssetMapping are intentionally omitted.
// serde's default behavior (ignore unknown fields) handles the extra API fields.
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(super) struct Meta {
    pub(super) universe: Vec<AssetMeta>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct AssetMeta {
    pub(super) name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct SpotMeta {
    pub(super) universe: Vec<SpotUniverse>,
    pub(super) tokens: Vec<SpotToken>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct SpotUniverse {
    pub(super) tokens: [u32; 2],
    pub(super) name: String,
    pub(super) index: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct SpotToken {
    pub(super) name: String,
    pub(super) index: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct PerpDexMeta {
    pub(super) name: String,
}
