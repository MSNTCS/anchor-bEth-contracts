use cosmwasm_std::{CanonicalAddr, ReadonlyStorage, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read};

const REWARD_CONTRACT_KEY: &[u8] = b"reward_contract";

pub fn read_reward_contract<S: ReadonlyStorage>(storage: &S) -> StdResult<CanonicalAddr> {
    singleton_read(storage, REWARD_CONTRACT_KEY).load()
}

pub fn store_reward_contract<S: Storage>(
    storage: &mut S,
    reward_contract: &CanonicalAddr,
) -> StdResult<()> {
    singleton(storage, REWARD_CONTRACT_KEY).save(reward_contract)
}
