use crate::{
    exchange::{cancel::CancelRequest, order::OrderRequest},
    signature::agent::mainnet::Agent,
};
use ethers::types::H160;
use serde::{Deserialize, Serialize};

use super::cancel::CancelRequestCloid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsdcTransfer {
    pub chain: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
    // optional fast-cancel flag. MUST be omitted when false: HL rejects actions
    // hashed with f: false. skip_serializing_if keeps it out of both the rmp
    // hash and the posted json so f: false stays byte-identical to no flag.
    #[serde(rename = "f", default, skip_serializing_if = "std::ops::Not::not")]
    pub fast: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
    #[serde(rename = "f", default, skip_serializing_if = "std::ops::Not::not")]
    pub fast: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentConnect {
    pub chain: String,
    pub agent: Agent,
    pub agent_address: H160,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCancel {
    pub time: Option<i64>,
}
