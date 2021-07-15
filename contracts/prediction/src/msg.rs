use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Position;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Bet { position: Position },
}
