use cosmwasm_std::{Addr, Uint128, StdResult, Storage};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

const VESTING_STORAGE: Item<VestingSchedule> = Item::new("vesting_schedules");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VestingSchedule {
    pub employee: Addr,
    pub amount: Uint128,
    pub token: String,
    pub schedule: String,
    pub claimed: Uint128,
}

pub fn claim_salary(storage: &mut dyn Storage, _employee: Addr) -> StdResult<()> {
    let schedule = VESTING_STORAGE.may_load(storage)?; // Use may_load to prevent crashing
    if let Some(mut schedule) = schedule {
        schedule.claimed += schedule.amount;
        VESTING_STORAGE.save(storage, &schedule)?;
        Ok(())
    } else {
        Err(cosmwasm_std::StdError::not_found("Vesting schedule not found"))
    }
}
