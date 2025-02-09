
use cosmwasm_std::{Addr, Uint128, StdResult, Storage};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

const PROPOSALS_STORAGE: Item<SalaryProposal> = Item::new("salary_proposals");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SalaryProposal {
    pub employee: Addr,
    pub new_salary: Uint128,
    pub reason: String,
}

pub fn vote_on_adjustment(storage: &mut dyn Storage, proposal: SalaryProposal) -> StdResult<()> {
    PROPOSALS_STORAGE.save(storage, &proposal)?;
    Ok(())
}
