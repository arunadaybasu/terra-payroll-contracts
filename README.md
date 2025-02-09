
# Terra Payroll System

This is a **CosmWasm-based tokenized payroll system** for **Terra Classic**.

## Features
- **Vesting-based salary payments** (weekly, monthly, quarterly).
- **Multi-token support** (LUNC, USTC, ORB, yLUNC, CW20 tokens).
- **Governance-controlled salary adjustments**.
- **Auto-staking for unclaimed salaries**.
- **Slashing for early withdrawals or non-performance**.

## Setup
1. Install Rust & Cargo:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone this repo:
   ```sh
   git clone https://github.com/your-repo/terra-payroll-system.git
   cd terra-payroll-system
   ```
3. Build & test:
   ```sh
   cargo check
   cargo test
   ```

## Contracts
- `vesting_contract.rs` - Handles salary vesting & claims.
- `payroll_management.rs` - Manages employees & payments.
- `governance_control.rs` - DAO-controlled salary system.

## License
MIT

