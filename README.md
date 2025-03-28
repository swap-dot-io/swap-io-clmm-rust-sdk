# Swap.io CLMM Rust SDK

This repository provides a Rust Software Development Kit (SDK) to interact with the Swap.io CLMM smart contracts and integrate with the [jup.ag swap router program](https://station.jup.ag/docs/dex-integration). It is designed to streamline the process of integrating decentralized exchange (DEX) functionalities according to the requirements specified by jup.ag.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Core components](#core-components)
    - [PoolManager](#poolmanager)
    - [QuoteCalculator](#quotecalculator)
    - [InstructionBuilder](#instructionbuilder)
    - [Example Workflow](#example-workflow)
- [Integration with jup.ag](#integration-with-jupag)
- [Related Repositories](#related-repositories)
- [Contributing](#contributing)
- [License](#license)

## Overview

The Swap.io CLMM Rust SDK enables developers to interact with the underlying Swap.io CLMM smart contracts, facilitating smooth integration with the jup.ag swap router program. The SDK abstracts many of the low-level details, allowing you to focus on building robust applications on top of decentralized liquidity pools.

## Features

- **Pool Management:** Track and manage CLMM pool state with efficient tick array handling.
- **Price Discovery:** Calculate accurate swap quotes with slippage protection.
- **Instruction Building:** Generate optimized Solana instructions for swaps.
- **Token Fee Support:** Handle SPL Token 2022 transfer fees automatically.
- **Rust Native:** Built using Rust, offering strong performance and memory safety.
- **Abstraction Layer:** Simplified API for interacting with the Swap.io CLMM smart contracts.
- **Extensible:** Designed with modularity in mind to support future features and integrations.

## Installation

Add the SDK to your Cargo.toml:

```toml
[dependencies]
swap-io-clmm-rust-sdk = { git = "https://github.com/swap-dot-io/swap-io-clmm-rust-sdk" }
solana-sdk = "=1.16.25"
```

## Core Components

### PoolManager

Handles pool state tracking and tick array management:

```rust
// Initialize a pool manager from an account
let pool_manager = PoolManager::new(epoch, pool_key, program_id, pool_state_account)?;

// Update the pool manager with the latest data
pool_manager.update(accounts, up_tick_accounts, down_tick_accounts)?;

// Get the current pool state
let pool_state = pool_manager.get_pool_state();

```

### QuoteCalculator

Calculates price quotes for swaps with detailed fee information:

```rust
// Get a quote for swapping tokens
let quote = QuoteCalculator::calculate_quote(
    input_mint,
    output_mint,
    true, // exact input
    amount,
    &pool_manager
)?;

```

### InstructionBuilder

Generates Solana instructions for executing swaps:

```rust
// Build a swap instruction
let swap_instruction = InstructionBuilder::build_swap_instruction(
    &pool_manager,
    source_mint,
    destination_mint,
    source_token_account,
    destination_token_account
)?;

// Add to a transaction
transaction.add(swap_instruction);
```


### Example Workflow

```rust
// 1. Initialize pool manager
let pool_manager = PoolManager::new(epoch, pool_key, program_id, pool_state_account)?;

// 2. Get required accounts
let accounts_to_update = pool_manager.get_accounts_to_update();
// Fetch accounts from blockchain...

// 3. Update manager with latest data
pool_manager.update(accounts, up_tick_accounts, down_tick_accounts)?;

// 4. Calculate swap quote
let quote = QuoteCalculator::calculate_quote(
    token_a_mint, 
    token_b_mint, 
    true, // direction: A -> B
    1_000_000, 
    &pool_manager
)?;

// 5. Generate swap instruction
let instruction = InstructionBuilder::build_swap_instruction(
    &pool_manager,
    token_a_mint,
    token_b_mint,
    user_token_a_account,
    user_token_b_account
)?;
```

For more detailed examples and API documentation, please refer to the project's documentation.

## Integration with jup.ag
This SDK has been developed to meet the integration requirements specified in the [jup.ag swap router program documentation](https://station.jup.ag/docs/dex-integration). The SDK handles:

- Data formatting and communication with the jup.ag endpoint.
- Managing request/response cycles for swap quotes.
- Ensuring compatibility with the required protocol standards for DEX integrations.

Be sure to consult the jup.ag documentation for further details on configuring and using the swap router features.

## Related Repositories

- **Swap.io CLMM Smart Contracts:** The original smart contract implementation can be found in the [swap.io clmm repository](https://github.com/swap-dot-io/swap-io-clmm/).
- **jupiter-amm-interface adapter** Implementation of jupiter-amm-interface [Swap.io CLMM Jupiter Adapter](https://github.com/swap-dot-io/jupiter-swap-io-adapter).
- **jup.ag Documentation:** Detailed instructions and requirements are available at [jup.ag docs](https://station.jup.ag/docs/dex-integration).

## Contributing

Contributions to the SDK are welcome! Please review the [contributing guidelines](CONTRIBUTING.md) before submitting a pull request. For major changes, open an issue first to discuss what you would like to change.

## License

This project is licensed under the [MIT License](LICENSE).