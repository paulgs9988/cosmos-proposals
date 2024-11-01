use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub sui_registry_address: String,
    // We'll add Axelar config later
}

#[cw_serde]
pub struct Proposal {
    pub creator: Addr,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub voters: Vec<Addr>,
    pub active: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");