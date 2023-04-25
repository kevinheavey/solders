use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
pub enum V2 {
    #[default]
    #[serde(rename = "2.0")]
    TwoPointOh,
}
