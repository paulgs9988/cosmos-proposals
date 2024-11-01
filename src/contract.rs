use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ProposalResponse};
use crate::state::{Config, Proposal, CONFIG, PROPOSALS, PROPOSAL_COUNT};

use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, entry_point, Order,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        sui_registry_address: msg.sui_registry_address.clone(),
        // We'll add oracle config later
    };
    CONFIG.save(deps.storage, &config)?;
    
    PROPOSAL_COUNT.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("sui_registry", msg.sui_registry_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateProposal { description } => {
            execute_create_proposal(deps, info, description)
        }
        ExecuteMsg::Vote { proposal_id, vote } => {
            execute_vote(deps, info, proposal_id, vote)
        }
    }
}

pub fn execute_create_proposal(
    deps: DepsMut,
    info: MessageInfo,
    description: String,
) -> Result<Response, ContractError> {
    // TODO: Add Oracle verification later
    // For now, always allow proposal creation
    
    let id = PROPOSAL_COUNT.load(deps.storage)? + 1;
    PROPOSAL_COUNT.save(deps.storage, &id)?;

    let proposal = Proposal {
        creator: info.sender.clone(),
        description: description.clone(),
        yes_votes: 0,
        no_votes: 0,
        voters: vec![],
        active: true,
    };

    PROPOSALS.save(deps.storage, id, &proposal)?;

    Ok(Response::new()
        .add_attribute("method", "create_proposal")
        .add_attribute("proposal_id", id.to_string())
        .add_attribute("creator", info.sender)
        .add_attribute("description", description))
}

pub fn execute_vote(
    deps: DepsMut,
    info: MessageInfo,
    proposal_id: u64,
    vote: bool,
) -> Result<Response, ContractError> {
    // TODO: Add Oracle verification later
    
    let mut proposal = PROPOSALS.load(deps.storage, proposal_id)?;
    
    if !proposal.active {
        return Err(ContractError::ProposalNotActive {});
    }
    
    if proposal.voters.contains(&info.sender) {
        return Err(ContractError::AlreadyVoted {});
    }

    if vote {
        proposal.yes_votes += 1;
    } else {
        proposal.no_votes += 1;
    }
    
    proposal.voters.push(info.sender.clone());
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

    Ok(Response::new()
        .add_attribute("method", "vote")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("voter", info.sender)
        .add_attribute("vote", if vote { "yes" } else { "no" }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetProposal { proposal_id } => {
            query_proposal(deps, proposal_id)
        }
        QueryMsg::ListProposals { start_after, limit } => {
            query_list_proposals(deps, start_after, limit)
        }
    }
}

fn query_proposal(deps: Deps, proposal_id: u64) -> StdResult<Binary> {
    let proposal = PROPOSALS.load(deps.storage, proposal_id)?;
    to_json_binary(&proposal)
}

fn query_list_proposals(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after;
    
    let proposals: StdResult<Vec<_>> = PROPOSALS
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit)
        .collect();

    to_json_binary(&proposals?)
}