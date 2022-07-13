use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub amount :Uint128,

}

// pub const STATE: Item<State> = Item::new("state");
pub const BALANCES: Map<&Addr, State> = Map::new("balances");
