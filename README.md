# Osmosis Voting Contract with SUI Registry Oracle

## Overview
This CosmWasm smart contract implements a proposal/voting system where voting rights are determined by a registry stored on the SUI blockchain. Before any user can create a proposal or vote, the contract verifies their eligibility by querying a SUI registry through an oracle.

## Key Components

### 1. Voting Contract (Osmosis)
- Manages proposals and votes
- Verifies user permissions through oracle before any action
- Maintains proposal state and vote counts

### 2. SUI Registry (External)
- Maintains list of authorized Osmosis addresses
- Acts as source of truth for voting permissions
- Located at: `0x9aa19e5cd14bc2cfd7f6a5a9e1830cf16ee1e8230f12e6b446283436cc9c44e5`

### 3. Oracle Integration
- Provides cross-chain verification of addresses
- Queries SUI registry for address validation
- Returns boolean indicating if address is authorized

## Contract Functions

### Instantiate
```rust
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult
```
Initializes the contract with:
- SUI registry address
- Oracle configuration

### Create Proposal
```rust
pub fn execute_create_proposal(
    deps: DepsMut,
    info: MessageInfo,
    description: String,
) -> Result
```
Creates a new proposal if:
- Sender is verified in SUI registry (via oracle)
- Description is valid

### Vote
```rust
pub fn execute_vote(
    deps: DepsMut,
    info: MessageInfo,
    proposal_id: u64,
    vote: bool,
) -> Result
```
Records a vote if:
- Sender is verified in SUI registry (via oracle)
- Proposal exists and is active
- Sender hasn't voted already

## Error Handling
- `NotRegistered`: Address not found in SUI registry
- `ProposalNotActive`: Attempting to vote on inactive proposal
- `AlreadyVoted`: User has already voted on proposal
- `OracleError`: Failed to verify address with oracle

## Example Usage

### Creating a Proposal
```bash
osmosisd tx wasm execute $CONTRACT_ADDRESS \
  '{"create_proposal":{"description":"First test proposal"}}' \
  --from $WALLET \
  --chain-id osmo-test-5 \
  -y
```

### Casting a Vote
```bash
osmosisd tx wasm execute $CONTRACT_ADDRESS \
  '{"vote":{"proposal_id":1,"vote":true}}' \
  --from $WALLET \
  --chain-id osmo-test-5 \
  -y
```

## Development Status
- ✅ Basic contract structure
- ✅ Proposal creation/voting logic
- ✅ Oracle integration structure
- 🚧 Oracle implementation
- 🚧 Testing

## Next Steps
1. Implement oracle integration
2. Add comprehensive tests
3. Deploy and test on testnet
4. Security audit
5. Mainnet deployment

## Security Considerations
- Oracle as single point of failure
- Cross-chain data verification
- Vote manipulation prevention
- Gas considerations for oracle queries