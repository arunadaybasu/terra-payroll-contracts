#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::{Addr, Uint128};
    use terra_payroll_system::vesting_contract::{claim_salary, VestingSchedule};
    use cw_storage_plus::Item; // Use the correct storage library

    #[test]
    fn test_claim_salary() {
        let mut deps = mock_dependencies();
        let employee = Addr::unchecked("terra1test");

        // Create local storage variable
        let vesting_storage: Item<VestingSchedule> = Item::new("vesting_schedules");

        // Insert a sample employee before claiming salary
        let schedule = VestingSchedule {
            employee: employee.clone(),
            amount: Uint128::new(1000),
            token: "uluna".to_string(),
            schedule: "monthly".to_string(),
            claimed: Uint128::zero(),
        };
        vesting_storage.save(deps.as_mut().storage, &schedule).unwrap();

        // Now try to claim salary
        let result = claim_salary(deps.as_mut().storage, employee);
        assert!(result.is_ok());
    }
}
