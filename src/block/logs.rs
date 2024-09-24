use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Axis {
    #[serde(rename = "x")]
    X,
    #[serde(rename = "y")]
    Y,
    #[serde(rename = "z")]
    Z,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct LogProperties {
    axis: Axis,
}
