use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cw20_base::allowances::{execute_decrease_allowance, execute_increase_allowance};
use cw20_base::contract::instantiate as cw20_instantiate;
use cw20_base::contract::query as cw20_query;
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw20_base::ContractError;

use crate::handler::*;
use crate::msg::TokenInstantiateMsg;
use crate::state::store_reward_contract;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: TokenInstantiateMsg,
) -> StdResult<Response> {
    let reward_raw = deps.api.addr_canonicalize(&msg.reward_contract)?;
    store_reward_contract(deps.storage, &reward_raw)?;

    cw20_instantiate(
        deps,
        env,
        info,
        InstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            decimals: msg.decimals,
            initial_balances: msg.initial_balances,
            mint: msg.mint,
        },
    )?;

    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            let recipient_addr = deps.api.addr_validate(&recipient)?;
            execute_transfer(deps, env, info, recipient_addr, amount)
        }
        ExecuteMsg::Burn { amount } => execute_burn(deps, env, info, amount),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => {
            let contract_addr = deps.api.addr_validate(&contract)?;
            execute_send(deps, env, info, contract_addr, amount, msg)
        }
        ExecuteMsg::Mint { recipient, amount } => {
            let recipient_addr = deps.api.addr_validate(&recipient)?;
            execute_mint(deps, env, info, recipient_addr, amount)
        }
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            let recipient_addr = deps.api.addr_validate(&recipient)?;
            execute_transfer_from(deps, env, info, owner_addr, recipient_addr, amount)
        }
        ExecuteMsg::BurnFrom { owner, amount } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            execute_burn_from(deps, env, info, owner_addr, amount)
        }
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => {
            let owner_addr = deps.api.addr_validate(&owner)?;
            let contract_addr = deps.api.addr_validate(&contract)?;
            execute_send_from(deps, env, info, owner_addr, contract_addr, amount, msg)
        }
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    cw20_query(deps, _env, msg)
}
