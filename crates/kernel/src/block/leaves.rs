use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Distance {
    #[serde(rename = "0")]
    D0,
    #[serde(rename = "1")]
    D1,
    #[serde(rename = "2")]
    D2,
    #[serde(rename = "3")]
    D3,
    #[serde(rename = "4")]
    D4,
    #[serde(rename = "5")]
    D5,
    #[serde(rename = "6")]
    D6,
    #[serde(rename = "7")]
    D7,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct LeavesProperties {
    distance: Distance,
    persistent: bool,
    waterlogged: bool,
}
