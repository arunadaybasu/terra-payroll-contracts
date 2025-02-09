#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::Addr;
    use terra_payroll_system::payroll_management::{add_employee, Employee};  // Use `crate::` instead of `terra_classic_tokenized_payroll::`

    #[test]
    fn test_add_employee() {
        let mut deps = mock_dependencies();
        let employee = Employee {
            address: Addr::unchecked("terra1test"),
            salary: cosmwasm_std::Uint128::new(1000),
            vesting_schedule: "monthly".to_string(),
        };

        let result = add_employee(deps.as_mut().storage, employee);
        assert!(result.is_ok());
    }
}
