
use cosmwasm_std::{Addr, Uint128, StdResult, Storage};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

const EMPLOYEE_STORAGE: Item<Employee> = Item::new("employees");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Employee {
    pub address: Addr,
    pub salary: Uint128,
    pub vesting_schedule: String,
}

pub fn add_employee(storage: &mut dyn Storage, employee: Employee) -> StdResult<()> {
    EMPLOYEE_STORAGE.save(storage, &employee)?;
    Ok(())
}
