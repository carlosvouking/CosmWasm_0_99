use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr,  // wallet address or whatever...  // helps delete bad/garbage polls
}

pub const CONFIG: Item<Config> = Item::new(storage_key:"config");  // stored on the chain
