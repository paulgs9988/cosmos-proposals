use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub sui_registry_address: String,
    // We'll add Axelar config later
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateProposal { description: String },
    Vote { proposal_id: u64, vote: bool },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProposalResponse)]
    GetProposal { proposal_id: u64 },
    #[returns(Vec<ProposalResponse>)]
    ListProposals { start_after: Option<u64>, limit: Option<u32> },
}

#[cw_serde]
pub struct ProposalResponse {
    pub id: u64,
    pub creator: String,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub active: bool,
}