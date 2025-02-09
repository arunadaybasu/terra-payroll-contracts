# Terra Classic Tokenized Payroll System

## Overview
The **Terra Classic Tokenized Payroll System** is a simple smart contract built on the **Terra Classic blockchain** to facilitate basic payroll management using vesting schedules. It allows **teams, DAOs, and organizations** to distribute salaries in an automated and transparent manner.

## Features Implemented (Basic Version)
### 🔹 **Payroll Management**
- **Basic Employee Salary Management**: Admins can assign salaries to employees.
- **Employee Salary Claiming**: Employees can claim their vested salaries.
- **Manual Funding**: Any approved wallet can deposit LUNC into the contract.

### 🔹 **Vesting System**
- **Fixed Start & End Time Vesting**: Salaries are released over a period of time.
- **Basic Linear Vesting**: Employees receive their salaries at a constant rate.

### 🔹 **Storage & Optimizations**
- **Uses `cw-storage-plus` for efficient storage.**
- **Optimized contract size for lower deployment costs.**

## Installation & Usage
### 🚀 **Deploying the Contract**
1. **Compile the Contract**:
   ```sh
   cargo build --release --target wasm32-unknown-unknown
   ```

2. **Optimize Contract for Terra Classic**:
   ```sh
   docker run --rm -v "$(pwd)":/code \
     --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
     --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
     cosmwasm/rust-optimizer:0.14.0
   ```

3. **Store the Contract on Terra Classic**:
   ```sh
   terrad tx wasm store artifacts/terra_payroll_system.wasm --from mywallet --chain-id localterra --gas auto --gas-adjustment 1.5 -y
   ```

4. **Instantiate the Contract**:
   ```sh
   terrad tx wasm instantiate <code_id> '{}' --from mywallet --label "terra_payroll" --admin $(terrad keys show mywallet -a) --chain-id localterra --gas auto --gas-adjustment 1.5 -y
   ```

5. **Execute Payroll Transactions**:
   ```sh
   terrad tx wasm execute <contract_address> '{"claim_salary": {"employee": "terra1employee..."}}' --from employee_wallet --gas auto --gas-adjustment 1.5 -y
   ```

## Querying Contract Data
Retrieve payroll details using:
```sh
terrad query wasm contract-state smart <contract_address> '{"get_employee": {"address": "terra1employee..."}}' --chain-id localterra
```

## Known Issues & Fixes
### 🔹 **Gas Errors in Deployment**
- **Issue**: "Out of Gas"
- **Fix**: Use `--gas auto --gas-adjustment 1.5` to automatically adjust gas usage.

### 🔹 **LUNC Burn Tax Impact**
- **Issue**: Transactions failing due to insufficient funds.
- **Fix**: Increase gas price slightly (e.g., `--gas-prices 0.025uluna`).

## Future Enhancements (Planned)
🔹 **Implement Cliffs & Initial Unlocks**: Employees must wait for a certain period before claiming salaries.  
🔹 **Governance Enhancements**: DAO-based multi-sig approvals for payroll changes.  
🔹 **Batch Processing Optimization**: Reduce gas fees by handling multiple claims in a single transaction.

## License
MIT License. Open-source and available for contribution.

🚀 **Developed for the Terra Classic ecosystem – secure, transparent, and decentralized payroll management!**

