use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Binary, to_json_binary};
use crate::vesting_contract::claim_salary;
use crate::payroll_management::add_employee;
use crate::governance_control::vote_on_adjustment;

pub mod vesting_contract;
pub mod payroll_management;
pub mod governance_control;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: (),
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,  // ✅ Prefixed `_` to avoid warning
    _info: MessageInfo,  // ✅ Prefixed `_` to avoid warning
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::ClaimSalary { employee } => {
            claim_salary(deps.storage, employee)?;
            Ok(Response::new().add_attribute("action", "claim_salary"))
        }
        ExecuteMsg::AddEmployee { employee } => {
            add_employee(deps.storage, employee)?;
            Ok(Response::new().add_attribute("action", "add_employee"))
        }
        ExecuteMsg::VoteOnAdjustment { proposal } => {
            vote_on_adjustment(deps.storage, proposal)?;
            Ok(Response::new().add_attribute("action", "vote_on_adjustment"))
        }
    }
}

#[entry_point]
pub fn query(
    _deps: Deps,  // ✅ Fixed: Use `Deps` and prefix `_` to avoid warnings
    _env: Env,  // ✅ Prefixed `_` to avoid warning
    _msg: (),
) -> StdResult<Binary> {  // ✅ FIXED: Changed return type from `Response` to `Binary`
    to_json_binary("query method called")  // ✅ FIXED: Use `to_json_binary()` instead of `to_binary()`
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    ClaimSalary { employee: cosmwasm_std::Addr },
    AddEmployee { employee: crate::payroll_management::Employee },
    VoteOnAdjustment { proposal: crate::governance_control::SalaryProposal },
}
