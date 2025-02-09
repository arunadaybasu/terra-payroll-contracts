#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::Addr;
    use terra_payroll_system::governance_control::{vote_on_adjustment, SalaryProposal};  // Use `crate::` instead of `terra_classic_tokenized_payroll::`

    #[test]
    fn test_vote_on_adjustment() {
        let mut deps = mock_dependencies();
        let proposal = SalaryProposal {
            employee: Addr::unchecked("terra1test"),
            new_salary: cosmwasm_std::Uint128::new(2500),
            reason: "Performance".to_string(),
        };

        let result = vote_on_adjustment(deps.as_mut().storage, proposal);
        assert!(result.is_ok());
    }
}
